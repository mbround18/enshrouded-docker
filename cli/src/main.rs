mod environment;
mod game_settings;
mod utils;

use crate::environment::name;
use clap::{Parser, Subcommand};
use gsm_cron::{begin_cron_loop, register_job};
use gsm_instance::{Instance, InstanceConfig};
use gsm_monitor::LogRules;
use gsm_notifications::notifications::{StandardServerEvents, send_notifications};
use gsm_shared::{fetch_var, is_env_var_truthy};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

#[derive(Parser)]
#[command(
    name = "enshrouded",
    version = "1.1",
    about = "Manage Enshrouded Server"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup runtime environment (directories, permissions, config)
    Setup {
        #[arg(long, default_value = "/home/steam/enshrouded")]
        path: PathBuf,
    },
    Install {
        #[arg(long, default_value = "/home/steam/enshrouded")]
        path: PathBuf,
    },
    /// Start the server only (without monitoring jobs)
    Start,
    /// Monitor the server: start the server and then run scheduled jobs and watch logs.
    Monitor {
        #[arg(long)]
        update_job: bool,
        #[arg(long)]
        restart_job: bool,
    },
    Stop,
    Restart,
    Update {
        #[arg(long)]
        check: bool,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    debug!("Tracing subscriber initialized.");

    fn setup_configuration(game_root: &Path) {
        let config_path = game_root.join("enshrouded_server.json");
        debug!("Loading or creating config at: {:?}", config_path);
        game_settings::load_or_create_config(&config_path);
        debug!("Config load or creation completed.");
    }

    // Launches the server via Proton. Used by every code path that starts the
    // server (not just the `start` subcommand) so that `restart`, the
    // scheduled-restart job, and the post-update restart all agree on how to
    // launch it — `Instance::start()`/`restart()` from gsm-instance only know
    // how to launch via wine64 directly, which isn't on PATH on the
    // proton-runtime image.
    fn start_server_via_proton(config: &InstanceConfig) -> Result<(), Box<dyn std::error::Error>> {
        setup_configuration(&config.working_dir);

        utils::setup::setup_proton_env()?;
        utils::setup::create_steam_appid(&config.working_dir, 2278520)?;

        let proton_path = utils::setup::get_proton_executable()?;
        let server_exe = config.working_dir.join("enshrouded_server.exe");
        utils::setup::spawn_server(&proton_path, &server_exe, config)
    }

    // Set the TZ environment variable to your desired timezone.
    #[cfg(unix)]
    unsafe {
        env::set_var("TZ", fetch_var("TZ", "America/Los_Angeles"));
    }

    let cli = Cli::parse();
    let instance_config = InstanceConfig {
        app_id: 2278520, // Enshrouded Steam App ID
        name: name(),
        command: "enshrouded_server.exe".to_string(),
        install_args: vec![],
        launch_args: vec![],
        force_windows: true,
        working_dir: PathBuf::from("/home/steam/enshrouded"),
    };
    debug!("Instance configuration set: {:?}", instance_config);

    // Use tokio::sync::Mutex for async locking.
    let instance = Arc::new(Mutex::new(Instance::new(instance_config)));
    debug!("Instance created and wrapped in Arc<Mutex<>>");

    match cli.command {
        Commands::Setup { path } => {
            info!("Setting up runtime environment");
            utils::setup::print_system_info();
            
            match utils::setup::initialize_runtime(&path) {
                Ok(_) => {
                    setup_configuration(&path);
                    info!("Setup completed successfully");
                }
                Err(e) => {
                    error!("Setup failed: {}", e);
                    exit(1);
                }
            }
        }
        Commands::Install { path } => {
            info!("Installing Enshrouded server to: {:?}", path);
            debug!("Acquiring lock for installation...");
            let inst = instance.lock().await;
            if let Err(e) = inst.install() {
                error!("Installation failed: {}", e);
            } else {
                debug!("Installation successful.");
                setup_configuration(&path);
                info!("Enshrouded server installed successfully at: {:?}", path);
            }
        }
        Commands::Start => {
            info!("Starting server with Proton...");
            let inst = instance.lock().await;
            if let Err(e) = start_server_via_proton(&inst.config) {
                error!("Failed to start server: {}", e);
                exit(1);
            }
        }
        Commands::Monitor {
            update_job,
            restart_job,
        } => {
            // Start your server and schedule jobs as needed...
            // Then, to watch the logs:
            let working_dir = {
                let inst = instance.lock().await;
                inst.config.working_dir.clone()
            };

            let rules = LogRules::default();

            if env::var("WEBHOOK_URL").is_ok() {
                rules.add_rule(
                    |line| line.contains("[Session] 'HostOnline' (up)!"),
                    |_| {
                        send_notifications(StandardServerEvents::Started)
                            .expect("Failed to send webhook event! Invalid url?")
                    },
                    false,
                    None,
                );

                rules.add_rule(
                    |line| line.contains("logged in with Permissions:"),
                    |line| match utils::extract_player_joined_name(line) {
                        Some(name) => send_notifications(StandardServerEvents::PlayerJoined(name))
                            .expect("Failed to send webhook event! Invalid url?"),
                        None => error!("Failed to extract player name from:\n{line}"),
                    },
                    false,
                    None,
                );

                rules.add_rule(
                    |line| line.contains("[server] Remove Entity for Player"),
                    |line| match utils::extract_player_left_name(line) {
                        Some(name) => send_notifications(StandardServerEvents::PlayerLeft(name))
                            .expect("Failed to send webhook event! Invalid url?"),
                        None => error!("Failed to extract player name from:\n{line}"),
                    },
                    false,
                    None,
                );
            }

            // Start monitoring the instance log files.
            gsm_monitor::start_instance_log_monitor(working_dir, rules);

            if update_job || is_env_var_truthy("AUTO_UPDATE") {
                debug!("Auto-update job condition met.");
                let update_schedule = fetch_var("AUTO_UPDATE_SCHEDULE", "0 3 * * *");
                debug!("Auto-update schedule: {}", update_schedule);
                let instance_clone = Arc::clone(&instance);
                register_job("auto-update", &update_schedule, move || {
                    debug!("Auto-update job triggered.");
                    let instance_clone_inner = Arc::clone(&instance_clone);
                    tokio::spawn(async move {
                        let inst = instance_clone_inner.lock().await;
                        if inst.update_available() {
                            warn!("Update available! Stopping server...");
                            if let Err(e) = inst.stop() {
                                error!("Failed to stop server: {}", e);
                                return;
                            }
                            info!("Updating server...");
                            if let Err(e) = inst.update() {
                                error!("Update failed: {}", e);
                                return;
                            }
                            info!("Restarting server...");
                            if let Err(e) = start_server_via_proton(&inst.config) {
                                error!("Failed to start server: {}", e);
                            }
                        } else {
                            debug!("No updates available during auto-update check.");
                        }
                    });
                });
            } else {
                debug!("Auto-update job not enabled.");
            }

            if restart_job || is_env_var_truthy("SCHEDULED_RESTART") {
                debug!("Scheduled restart job condition met.");
                let restart_schedule = fetch_var("SCHEDULED_RESTART_SCHEDULE", "0 4 * * *");
                debug!("Scheduled restart schedule: {}", restart_schedule);
                let instance_clone = Arc::clone(&instance);
                register_job("scheduled-restart", &restart_schedule, move || {
                    debug!("Scheduled restart job triggered.");
                    let instance_clone_inner = Arc::clone(&instance_clone);
                    tokio::spawn(async move {
                        let inst = instance_clone_inner.lock().await;
                        warn!("Restarting server...");
                        if let Err(e) = inst.stop() {
                            error!("Failed to stop server: {}", e);
                            return;
                        }
                        if let Err(e) = start_server_via_proton(&inst.config) {
                            error!("Failed to restart server: {}", e);
                        }
                    });
                });
            } else {
                debug!("Scheduled restart job not enabled.");
            }

            debug!("Entering cron loop (monitoring logs and scheduled tasks)...");
            begin_cron_loop().await;
            debug!("Cron loop ended.");
        }
        Commands::Stop => {
            let webhook_enabled = env::var("WEBHOOK_URL").is_ok();
            if webhook_enabled {
                if let Ok(delay_str) = env::var("STOP_DELAY") {
                    match delay_str.parse::<u64>() {
                        Ok(delay_sec) => {
                            send_notifications(StandardServerEvents::Stopping)
                                .expect("Failed to send webhook event! Invalid url?");
                            tokio::time::sleep(Duration::from_secs(delay_sec)).await;
                        }
                        Err(_) => {
                            error!("Invalid STOP_DELAY value: {}", delay_str);
                        }
                    }
                }
            }

            warn!("Stopping Enshrouded server...");
            debug!("Acquiring lock to stop the server...");

            let inst = instance.lock().await;
            match inst.stop() {
                Err(e) => {
                    error!("Failed to stop: {}", e);
                }
                Ok(_) => {
                    if webhook_enabled {
                        send_notifications(StandardServerEvents::Stopped)
                            .expect("Failed to send webhook event! Invalid url?");
                    }
                    debug!("Server stopped successfully.");
                }
            }
        }
        Commands::Restart => {
            warn!("Restarting Enshrouded server...");
            debug!("Acquiring lock to restart the server...");
            let inst = instance.lock().await;
            if let Err(e) = inst.stop() {
                error!("Failed to stop server: {}", e);
            } else if let Err(e) = start_server_via_proton(&inst.config) {
                error!("Failed to restart server: {}", e);
            } else {
                debug!("Server restarted successfully.");
            }
        }
        Commands::Update { check } => {
            debug!("Update command initiated with check = {}", check);
            let inst = instance.lock().await;
            if check {
                debug!("Performing update check...");
                if inst.update_available() {
                    info!("Update available!");
                    exit(1);
                } else {
                    info!("Server is up to date.");
                    exit(0);
                }
            } else {
                info!("Checking for updates without enforcing check flag...");
                if inst.update_available() {
                    warn!("Update available! Updating...");
                    if let Err(e) = inst.update() {
                        error!("Update failed: {}", e);
                    } else {
                        debug!("Update applied successfully.");
                    }
                } else {
                    debug!("Server is up to date; no update needed.");
                }
            }
        }
    }
}
