use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};

/// Initialize runtime environment: directories, permissions, and environment variables
pub fn initialize_runtime(game_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    info!("🧹 Initializing runtime environment");

    // Ensure game directory exists
    create_directory_if_needed(game_root)?;
    create_directory_if_needed(&game_root.join("logs"))?;

    // Set up WINE environment
    setup_wine_environment()?;

    // Link the steam user's own steamclient.so into ~/.steam/sdk32|64 (see
    // setup_steam_client_symlinks for why this can't be done at image build time)
    setup_steam_client_symlinks()?;

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

/// Link `~/.steam/sdk32`/`sdk64` (and the `steamservice.so` alias inside them) to
/// where steamcmd actually puts its files under this user's `$HOME`.
///
/// `scripts/docker/setup-steam.sh` already does this exact linking during the
/// Docker build, but that RUN instruction executes as root with `$HOME=/root`
/// (before `USER steam` switches), so it only ever creates
/// `/root/.steam/sdk32|64` — a directory the runtime `steam` user can't even
/// read (root's home is 700). `LD_LIBRARY_PATH` points at
/// `/home/steam/.steam/sdk32|64`, which as a result never exists, so anything
/// that dlopen()s steamclient.so via it (Steamworks-integrated games running
/// under Wine/Proton) fails or hangs. Symlinks can dangle until steamcmd
/// actually populates the target on first install, so this is safe to do
/// unconditionally and early.
fn setup_steam_client_symlinks() -> Result<(), Box<dyn std::error::Error>> {
    let home = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/home/steam".to_string()));
    let steamcmd_dir = home.join(".local/share/Steam/steamcmd");

    // steamcmd only populates ~/.local/share/Steam/steamcmd/linux32|64 (and the
    // steamclient.so inside it) as a side effect of actually running under this
    // user, which happens during `enshrouded install`. But entrypoint.sh skips
    // `install` whenever enshrouded_server.exe already exists (e.g. a container
    // restarted against an already-populated volume), so on a persisted volume
    // this directory can be permanently missing even though the game itself is
    // installed. Bootstrap it here with a plain `+quit`, which just makes
    // steamcmd self-update and exit — no game/app install needed.
    if fs::symlink_metadata(&steamcmd_dir.join("linux64").join("steamclient.so")).is_err() {
        debug!("Bootstrapping steamcmd SDK libraries for {:?}", home);
        std::process::Command::new("steamcmd")
            .arg("+quit")
            .status()
            .ok();
    }

    for arch_dir in ["linux32", "linux64"] {
        let sdk_link = home.join(".steam").join(match arch_dir {
            "linux32" => "sdk32",
            _ => "sdk64",
        });
        create_directory_if_needed(&home.join(".steam"))?;

        // Use symlink_metadata (lstat), not exists()/is_dir(): those follow the
        // symlink and report false while it's still dangling (before steamcmd has
        // populated the target), which would make this re-create it every run.
        if fs::symlink_metadata(&sdk_link).is_err() {
            debug!(
                "Linking {:?} -> {:?}",
                sdk_link,
                steamcmd_dir.join(arch_dir)
            );
            std::os::unix::fs::symlink(steamcmd_dir.join(arch_dir), &sdk_link)?;
        }

        let steamservice_link = sdk_link.join("steamservice.so");
        if fs::symlink_metadata(&steamservice_link).is_err() {
            std::os::unix::fs::symlink("steamclient.so", &steamservice_link).ok();
        }
    }

    Ok(())
}

/// Clean up cache directories
fn cleanup_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache_paths = vec!["/home/steam/.cache"];

    for cache_path in cache_paths {
        if Path::new(cache_path).exists() {
            debug!("Removing cache: {}", cache_path);
            fs::remove_dir_all(cache_path).ok();
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
/// Fill in the Proton variables the base image hasn't already set, and make
/// sure the directories they name exist.
///
/// The base image's `20-proton-init.sh` exports `STEAM_COMPAT_DATA_PATH`,
/// `STEAM_COMPAT_CLIENT_INSTALL_PATH` and `WINEPREFIX` before handing control
/// to our entrypoint, so we defer to those rather than overwriting them --
/// pointing the compat data path somewhere else than the prefix the base
/// already initialized would mean Proton building a second one from scratch.
/// The defaults here are only for running outside that image.
pub fn setup_proton_env() -> Result<(), Box<dyn std::error::Error>> {
    let proton_data = env_path_or("STEAM_COMPAT_DATA_PATH", "/home/steam/.proton");
    let proton_steam = env_path_or(
        "STEAM_COMPAT_CLIENT_INSTALL_PATH",
        "/home/steam/.steam/steam",
    );

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

/// Read `var` as a path, falling back to `default` when it is unset or empty.
fn env_path_or(var: &str, default: &str) -> PathBuf {
    match std::env::var(var) {
        Ok(value) if !value.trim().is_empty() => PathBuf::from(value),
        _ => PathBuf::from(default),
    }
}

/// Locate the Proton executable installed in the base image.
///
/// `PROTON_PATH` is the base image's own answer to this question, recomputed at
/// runtime by `20-proton-init.sh` and exported to us, so it wins. It is not
/// trusted blindly though: the image also ships a *build-time* `ENV PROTON_PATH`
/// pointing at `.steam/steam/compatibilitytools.d/current/proton`, a path that
/// does not exist, and that stale value is what a container sees if anything
/// bypasses the base entrypoint. So the variable is used only when it actually
/// resolves to a file, and otherwise we look where `dockerify install proton`
/// puts things -- preferring the `current` symlink, which tracks the pinned
/// version, over whichever `GE-Proton*` directory happens to sort last.
pub fn get_proton_executable() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(from_env) = std::env::var_os("PROTON_PATH") {
        let candidate = PathBuf::from(from_env);
        if candidate.is_file() {
            debug!("Using Proton from PROTON_PATH: {:?}", candidate);
            return Ok(candidate);
        }
        debug!("Ignoring PROTON_PATH={candidate:?}: not a file");
    }

    let compat_dir = PathBuf::from("/home/steam/.steam/root/compatibilitytools.d");
    let current = compat_dir.join("current").join("proton");
    if current.is_file() {
        debug!("Using Proton from {:?}", current);
        return Ok(current);
    }

    if let Ok(entries) = fs::read_dir(&compat_dir) {
        let mut found: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path().join("proton"))
            .filter(|path| path.is_file())
            .collect();
        found.sort();
        if let Some(path) = found.pop() {
            debug!("Using Proton from {:?}", path);
            return Ok(path);
        }
    }

    Err(format!(
        "Proton not found. Looked at $PROTON_PATH and under {}.",
        compat_dir.display()
    )
    .into())
}

/// Spawn the server under Proton as a background process, redirecting output to
/// the instance's log files.
///
/// Deliberately does *not* record a pid file: `child.id()` here is Proton's
/// own launcher process, not the `enshrouded_server.exe` process it execs
/// under Wine. If we wrote that pid to `config.pid_file()`,
/// `Instance::stop()` (gsm-instance) would find it and SIGINT that pid
/// directly -- which only hits the Proton wrapper and never reaches the
/// actual game process, so it gets no chance to save before the container's
/// stop grace period expires and it's SIGKILLed. Leaving the pid file absent
/// makes `stop()` fall through to its name-based fallback
/// (`shutdown::blocking_shutdown`), which finds the real
/// `enshrouded_server.exe` process and signals -- and waits on -- that one.
pub fn spawn_server(
    proton_path: &Path,
    server_exe: &Path,
    config: &gsm_instance::InstanceConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    create_directory_if_needed(&config.log_dir())?;

    // Retry: setup_steam_client_symlinks() also runs during `setup`, but at that
    // point steamcmd hasn't necessarily populated ~/.local/share/Steam/steamcmd
    // yet, so the steamservice.so link (created inside the sdk32/64 symlink
    // target) can silently fail. By the time we're spawning the server, install
    // has definitely run.
    setup_steam_client_symlinks()?;

    let stdout = fs::File::create(config.stdout())?;
    let stderr = fs::File::create(config.stderr())?;

    // `runinprefix` skips Proton's Steam-client-directory bridging setup
    // (setup_steam_dir_drive()), which lsteamclient (Proton's Linux Steamworks
    // bridge) needs — without it the game hard-crashes with an assertion failure
    // in steamclient_main.c. `run` is slower to reach the game's own init (it also
    // launches Proton's Xalia accessibility helper first) but is the mode that
    // actually wires up Steamworks correctly.
    let child = std::process::Command::new(proton_path)
        .arg("run")
        .arg(server_exe)
        .current_dir(&config.working_dir)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()?;

    info!(
        "Server started in background (Proton launcher pid {})",
        child.id()
    );

    Ok(())
}

/// Make a file executable
fn make_executable(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(path).exists() {
        debug!("Making {} executable", path);
        let perms = fs::Permissions::from_mode(0o755);
        // The binary is already executable from the Docker build (COPY --chmod / cargo build
        // output) but may be owned by root while we're running as the unprivileged steam user,
        // so this chmod is best-effort: lacking permission to change it further isn't fatal.
        if let Err(e) = fs::set_permissions(path, perms) {
            debug!("Could not update permissions on {}: {}", path, e);
        }
    } else {
        error!("File not found: {}", path);
    }
    Ok(())
}

/// Print system information for debugging
pub fn print_system_info() {
    use std::process::Command;

    info!("──────────────────────────────────────────────────────────");
    info!(
        "🚀 Enshrouded Docker - {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
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
