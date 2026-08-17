use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::{debug, info, error};
use flate2::read::GzDecoder;
use tar::Archive;

/// Initialize runtime environment: directories, permissions, and environment variables
pub fn initialize_runtime(game_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    info!("🧹 Initializing runtime environment");

    // Ensure game directory exists
    create_directory_if_needed(game_root)?;
    create_directory_if_needed(&game_root.join("logs"))?;

    // Set up WINE environment
    setup_wine_environment()?;

    // Install Proton if not already installed
    setup_proton()?;

    // Cleanup cache
    cleanup_cache()?;

    // Ensure binary is executable
    make_executable("/usr/local/bin/enshrouded")?;

    info!("✅ Runtime initialization complete");
    Ok(())
}

/// Create directory if it doesn't exist
fn create_directory_if_needed(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        debug!("Creating directory: {:?}", path);
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Setup WINE environment variables
fn setup_wine_environment() -> Result<(), Box<dyn std::error::Error>> {
    debug!("Setting up WINE environment");
    
    // Set WINEPREFIX
    if std::env::var("WINEPREFIX").is_err() {
        unsafe {
            std::env::set_var("WINEPREFIX", "/home/steam/.wine");
        }
        debug!("Set WINEPREFIX=/home/steam/.wine");
    }

    // Set DISPLAY
    if std::env::var("DISPLAY").is_err() {
        unsafe {
            std::env::set_var("DISPLAY", ":1");
        }
        debug!("Set DISPLAY=:1");
    }

    Ok(())
}

/// Clean up cache directories
fn cleanup_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache_paths = vec![
        "/home/steam/.cache",
    ];

    for cache_path in cache_paths {
        if Path::new(cache_path).exists() {
            debug!("Removing cache: {}", cache_path);
            fs::remove_dir_all(cache_path).ok();
        }
    }

    Ok(())
}

/// Install/setup Proton-GE for game server execution
pub fn setup_proton() -> Result<(), Box<dyn std::error::Error>> {
    let proton_path = PathBuf::from("/home/steam/.proton");
    let proton_binary = proton_path.join("proton");
    
    // Check if Proton is already installed and functional
    if proton_binary.exists() {
        debug!("Proton already installed at {:?}", proton_path);
        return Ok(());
    }

    info!("📦 Installing Proton-GE from GitHub...");
    create_directory_if_needed(&proton_path)?;

    // Fetch the latest Proton-GE release
    let release = fetch_latest_proton_ge_release()?;
    info!("Found Proton-GE release: {}", release.tag_name);

    // Download and extract
    download_and_extract_proton_ge(&release, &proton_path)?;

    // Make proton binary executable
    if proton_binary.exists() {
        make_executable(proton_binary.to_str().unwrap())?;
        info!("✅ Proton-GE ({}) installed successfully", release.tag_name);
        return Ok(());
    }

    Err("Failed to extract Proton-GE binary".into())
}

/// GitHub release metadata
#[derive(serde::Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(serde::Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// Fetch the latest Proton-GE release from GitHub
fn fetch_latest_proton_ge_release() -> Result<GitHubRelease, Box<dyn std::error::Error>> {
    debug!("Fetching latest Proton-GE release from GitHub");
    let url = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "enshrouded-docker")
        .send()?;
    
    let release: GitHubRelease = response.json()?;
    Ok(release)
}

/// Download and extract Proton-GE to the installation path
fn download_and_extract_proton_ge(release: &GitHubRelease, install_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Find the tar.gz asset
    let asset = release.assets
        .iter()
        .find(|a| a.name.ends_with(".tar.gz"))
        .ok_or("No tar.gz asset found in release")?;
    
    info!("Downloading {}", asset.name);
    debug!("URL: {}", asset.browser_download_url);
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&asset.browser_download_url)
        .header("User-Agent", "enshrouded-docker")
        .send()?;
    
    // Create a temporary file for the tarball
    let temp_tar = std::env::temp_dir().join(&asset.name);
    let mut file = fs::File::create(&temp_tar)?;
    let mut content = std::io::Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    
    // Extract the tarball
    info!("Extracting to {:?}", install_path);
    let tar_gz = fs::File::open(&temp_tar)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    
    // Extract and move to final location
    let extract_path = std::env::temp_dir().join("proton-extract");
    fs::create_dir_all(&extract_path)?;
    archive.unpack(&extract_path)?;
    
    // Find the proton directory (usually named GE-Proton*)
    let entries = fs::read_dir(&extract_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.file_name().unwrap().to_string_lossy().contains("Proton") {
            // Move contents to install_path
            for item in fs::read_dir(&path)? {
                let item = item?;
                let item_path = item.path();
                let dest = install_path.join(item.file_name());
                if item_path.is_dir() {
                    fs::create_dir_all(&dest)?;
                    copy_dir_all(&item_path, &dest)?;
                } else {
                    fs::copy(&item_path, &dest)?;
                }
            }
            break;
        }
    }
    
    // Cleanup
    fs::remove_file(&temp_tar).ok();
    fs::remove_dir_all(&extract_path).ok();
    
    Ok(())
}

/// Recursively copy a directory
fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dst.join(&file_name);
        
        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

/// Create steam_appid.txt in game directory to prevent Steam ownership checks
pub fn create_steam_appid(game_dir: &Path, app_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let appid_file = game_dir.join("steam_appid.txt");
    debug!("Creating steam_appid.txt with app_id: {}", app_id);
    fs::write(&appid_file, app_id.to_string())?;
    Ok(())
}

/// Setup Proton environment variables for running a game server
pub fn setup_proton_env() -> Result<(), Box<dyn std::error::Error>> {
    let proton_data = PathBuf::from("/home/steam/.proton_data");
    let proton_steam = PathBuf::from("/home/steam/.steam/steam");
    
    create_directory_if_needed(&proton_data)?;
    create_directory_if_needed(&proton_steam)?;

    unsafe {
        // STEAM_COMPAT_DATA_PATH: Where Proton creates the fake C: drive (WINEPREFIX)
        std::env::set_var("STEAM_COMPAT_DATA_PATH", &proton_data);
        debug!("Set STEAM_COMPAT_DATA_PATH={:?}", proton_data);

        // STEAM_COMPAT_CLIENT_INSTALL_PATH: Fake Steam installation path
        std::env::set_var("STEAM_COMPAT_CLIENT_INSTALL_PATH", &proton_steam);
        debug!("Set STEAM_COMPAT_CLIENT_INSTALL_PATH={:?}", proton_steam);

        // Enable Proton logging for debugging
        std::env::set_var("PROTON_LOG", "1");
        debug!("Set PROTON_LOG=1");

        // LC_ALL for locale support
        std::env::set_var("LC_ALL", "C");
        debug!("Set LC_ALL=C");
    }

    Ok(())
}

/// Get the path to the Proton executable
pub fn get_proton_executable() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let proton_path = PathBuf::from("/home/steam/.proton/proton");
    if proton_path.exists() {
        Ok(proton_path)
    } else {
        Err("Proton not found. Run 'setup' command first.".into())
    }
}

/// Spawn the server under Proton as a background process, redirecting output to
/// the instance's log files and recording its pid so `stop`/`restart`/`monitor`
/// can find it afterward.
pub fn spawn_server(
    proton_path: &Path,
    server_exe: &Path,
    config: &gsm_instance::InstanceConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    create_directory_if_needed(&config.log_dir())?;

    let pid_file = config.pid_file();
    if pid_file.exists() {
        fs::remove_file(&pid_file)?;
    }

    let stdout = fs::File::create(config.stdout())?;
    let stderr = fs::File::create(config.stderr())?;

    let child = std::process::Command::new(proton_path)
        .arg("run")
        .arg(server_exe)
        .current_dir(&config.working_dir)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()?;

    info!("Server started in background with pid {}", child.id());
    fs::write(&pid_file, child.id().to_string())?;

    Ok(())
}

/// Make a file executable
fn make_executable(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(path).exists() {
        debug!("Making {} executable", path);
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(path, perms)?;
    } else {
        error!("File not found: {}", path);
    }
    Ok(())
}

/// Print system information for debugging
pub fn print_system_info() {
    use std::process::Command;

    info!("──────────────────────────────────────────────────────────");
    info!("🚀 Enshrouded Docker - {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    info!("──────────────────────────────────────────────────────────");

    // Hostname
    if let Ok(output) = Command::new("hostname").output() {
        if let Ok(hostname) = String::from_utf8(output.stdout) {
            info!("🔹 Hostname: {}", hostname.trim());
        }
    }

    // Kernel
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        if let Ok(kernel) = String::from_utf8(output.stdout) {
            info!("🔹 Kernel: {}", kernel.trim());
        }
    }

    // User info
    if let Ok(output) = Command::new("whoami").output() {
        if let Ok(user) = String::from_utf8(output.stdout) {
            info!("👤 Running as user: {}", user.trim());
        }
    }

    info!("──────────────────────────────────────────────────────────");
}
