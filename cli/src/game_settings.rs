use crate::environment::name;
use crate::utils::config_io::{load_config_with_defaults, save_config};
use crate::utils::env_overrides::apply_env_overrides;
use env_parse::env_parse;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents game settings in the server configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct GameSettings {
    /// Multiplier for player health (default: 1.0)
    pub player_health_factor: f32,
    /// Multiplier for player mana (default: 1.0)
    pub player_mana_factor: f32,
    /// Multiplier for player stamina (default: 1.0)
    pub player_stamina_factor: f32,
    /// Multiplier for player body heat (default: 1.0)
    pub player_body_heat_factor: f32,
    /// Enables item durability (default: true)
    pub enable_durability: bool,
    /// Enables starving debuff (default: false)
    pub enable_starving_debuff: bool,
    /// Multiplier for food buff duration (default: 1.0)
    pub food_buff_duration_factor: f32,
    /// Nanoseconds from hunger to starving (default: 600_000_000_000)
    pub from_hunger_to_starving: u64,
    /// Multiplier for shroud time (default: 1.0)
    pub shroud_time_factor: f32,
    /// Mode for tombstone behavior (default: "AddBackpackMaterials")
    pub tombstone_mode: String,
    /// Enables glider turbulences (default: true)
    pub enable_glider_turbulences: bool,
    /// Weather frequency (default: "Normal")
    pub weather_frequency: String,
    /// Multiplier for mining damage (default: 1.0)
    pub mining_damage_factor: f32,
    /// Multiplier for plant growth speed (default: 1.0)
    pub plant_growth_speed_factor: f32,
    /// Multiplier for resource drop stack amount (default: 1.0)
    pub resource_drop_stack_amount_factor: f32,
    /// Multiplier for factory production speed (default: 1.0)
    pub factory_production_speed_factor: f32,
    /// Multiplier for perk upgrade recycling (default: 0.5)
    pub perk_upgrade_recycling_factor: f32,
    /// Multiplier for perk cost (default: 1.0)
    pub perk_cost_factor: f32,
    /// Multiplier for combat experience (default: 1.0)
    pub experience_combat_factor: f32,
    /// Multiplier for mining experience (default: 1.0)
    pub experience_mining_factor: f32,
    /// Multiplier for exploration/quest experience (default: 1.0)
    pub experience_exploration_quests_factor: f32,
    /// Amount for random spawner (default: "Normal")
    pub random_spawner_amount: String,
    /// Amount for aggro pool (default: "Normal")
    pub aggro_pool_amount: String,
    /// Multiplier for enemy damage (default: 1.0)
    pub enemy_damage_factor: f32,
    /// Multiplier for enemy health (default: 1.0)
    pub enemy_health_factor: f32,
    /// Multiplier for enemy stamina (default: 1.0)
    pub enemy_stamina_factor: f32,
    /// Multiplier for enemy perception range (default: 1.0)
    pub enemy_perception_range_factor: f32,
    /// Multiplier for boss damage (default: 1.0)
    pub boss_damage_factor: f32,
    /// Multiplier for boss health (default: 1.0)
    pub boss_health_factor: f32,
    /// Threat bonus multiplier (default: 1.0)
    pub threat_bonus: f32,
    /// If true, pacifies all enemies (default: false)
    pub pacify_all_enemies: bool,
    /// Taming startle repercussion mode (default: "LoseSomeProgress")
    pub taming_startle_repercussion: String,
    /// Nanoseconds for day time duration (default: 1_800_000_000_000)
    pub day_time_duration: u64,
    /// Nanoseconds for night time duration (default: 720_000_000_000)
    pub night_time_duration: u64,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_health_factor: 1.0,
            player_mana_factor: 1.0,
            player_stamina_factor: 1.0,
            player_body_heat_factor: 1.0,
            enable_durability: true,
            enable_starving_debuff: false,
            food_buff_duration_factor: 1.0,
            from_hunger_to_starving: 600_000_000_000,
            shroud_time_factor: 1.0,
            tombstone_mode: "AddBackpackMaterials".to_string(),
            enable_glider_turbulences: true,
            weather_frequency: "Normal".to_string(),
            mining_damage_factor: 1.0,
            plant_growth_speed_factor: 1.0,
            resource_drop_stack_amount_factor: 1.0,
            factory_production_speed_factor: 1.0,
            perk_upgrade_recycling_factor: 0.5,
            perk_cost_factor: 1.0,
            experience_combat_factor: 1.0,
            experience_mining_factor: 1.0,
            experience_exploration_quests_factor: 1.0,
            random_spawner_amount: "Normal".to_string(),
            aggro_pool_amount: "Normal".to_string(),
            enemy_damage_factor: 1.0,
            enemy_health_factor: 1.0,
            enemy_stamina_factor: 1.0,
            enemy_perception_range_factor: 1.0,
            boss_damage_factor: 1.0,
            boss_health_factor: 1.0,
            threat_bonus: 1.0,
            pacify_all_enemies: false,
            taming_startle_repercussion: "LoseSomeProgress".to_string(),
            day_time_duration: 1_800_000_000_000,
            night_time_duration: 720_000_000_000,
        }
    }
}

macro_rules! env_field_mapping {
    ($($field:ident => $env_var:literal),*) => {
        pub fn from_env() -> Self {
            Self {
                $(
                    $field: env_parse!($env_var, Self::default().$field, _),
                )*
            }
        }

        pub fn merge_env(&mut self, env_config: &GameSettings) {
            $(
                if std::env::var($env_var).is_ok() {
                    self.$field = env_config.$field.clone();
                }
            )*
        }
    };
}

impl GameSettings {
    env_field_mapping! {
        player_health_factor => "PLAYER_HEALTH_FACTOR",
        player_mana_factor => "PLAYER_MANA_FACTOR",
        player_stamina_factor => "PLAYER_STAMINA_FACTOR",
        player_body_heat_factor => "PLAYER_BODY_HEAT_FACTOR",
        enable_durability => "ENABLE_DURABILITY",
        enable_starving_debuff => "ENABLE_STARVING_DEBUFF",
        food_buff_duration_factor => "FOOD_BUFF_DURATION_FACTOR",
        from_hunger_to_starving => "FROM_HUNGER_TO_STARVING",
        shroud_time_factor => "SHROUD_TIME_FACTOR",
        tombstone_mode => "TOMBSTONE_MODE",
        enable_glider_turbulences => "ENABLE_GLIDER_TURBULENCES",
        weather_frequency => "WEATHER_FREQUENCY",
        mining_damage_factor => "MINING_DAMAGE_FACTOR",
        plant_growth_speed_factor => "PLANT_GROWTH_SPEED_FACTOR",
        resource_drop_stack_amount_factor => "RESOURCE_DROP_STACK_AMOUNT_FACTOR",
        factory_production_speed_factor => "FACTORY_PRODUCTION_SPEED_FACTOR",
        perk_upgrade_recycling_factor => "PERK_UPGRADE_RECYCLING_FACTOR",
        perk_cost_factor => "PERK_COST_FACTOR",
        experience_combat_factor => "EXPERIENCE_COMBAT_FACTOR",
        experience_mining_factor => "EXPERIENCE_MINING_FACTOR",
        experience_exploration_quests_factor => "EXPERIENCE_EXPLORATION_QUESTS_FACTOR",
        random_spawner_amount => "RANDOM_SPAWNER_AMOUNT",
        aggro_pool_amount => "AGGRO_POOL_AMOUNT",
        enemy_damage_factor => "ENEMY_DAMAGE_FACTOR",
        enemy_health_factor => "ENEMY_HEALTH_FACTOR",
        enemy_stamina_factor => "ENEMY_STAMINA_FACTOR",
        enemy_perception_range_factor => "ENEMY_PERCEPTION_RANGE_FACTOR",
        boss_damage_factor => "BOSS_DAMAGE_FACTOR",
        boss_health_factor => "BOSS_HEALTH_FACTOR",
        threat_bonus => "THREAT_BONUS",
        pacify_all_enemies => "PACIFY_ALL_ENEMIES",
        taming_startle_repercussion => "TAMING_STARTLE_REPERCUSSION",
        day_time_duration => "DAY_TIME_DURATION",
        night_time_duration => "NIGHT_TIME_DURATION"
    }
}

/// Represents a user group and its permissions for the game server.
///
/// # Fields
/// - `name`: The name of the user group (e.g., "Admin", "Guest").
/// - `password`: The password required to join this group.
/// - `can_kick_ban`: Whether users in this group can kick or ban other players.
/// - `can_access_inventories`: Whether users can access other players' inventories.
/// - `can_edit_base`: Whether users can edit the base.
/// - `can_extend_base`: Whether users can extend the base.
/// - `reserved_slots`: Number of reserved slots for this group.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct UserGroup {
    pub name: String,
    pub password: String,
    pub can_kick_ban: bool,
    pub can_access_inventories: bool,
    pub can_edit_base: bool,
    pub can_extend_base: bool,
    pub reserved_slots: u8,
}

/// Provides default values for a `UserGroup`.
///
/// - `name`: "Guest"
/// - `password`: "GuestXXXXXXXX"
/// - `can_kick_ban`: false
/// - `can_access_inventories`: true
/// - `can_edit_base`: true
/// - `can_extend_base`: true
/// - `reserved_slots`: 0
impl Default for UserGroup {
    fn default() -> Self {
        Self {
            name: "Guest".to_string(),
            password: "GuestXXXXXXXX".to_string(),
            can_kick_ban: false,
            can_access_inventories: true,
            can_edit_base: true,
            can_extend_base: true,
            reserved_slots: 0,
        }
    }
}

/// Represents the full server configuration, including server info, game settings, and user groups.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct ServerConfig {
    pub name: String,
    pub save_directory: String,
    pub log_directory: String,
    pub ip: String,
    pub query_port: u16,
    pub slot_count: u8,
    pub voice_chat_mode: String,
    pub enable_voice_chat: bool,
    pub enable_text_chat: bool,
    pub game_settings_preset: String,
    pub game_settings: GameSettings,
    pub user_groups: Vec<UserGroup>,
    pub game_port: i32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: name(),
            save_directory: "./savegame".to_string(),
            log_directory: "./logs".to_string(),
            ip: "0.0.0.0".to_string(),
            game_port: 15636,
            query_port: 15637,
            slot_count: 16,
            voice_chat_mode: "Proximity".to_string(),
            enable_voice_chat: false,
            enable_text_chat: false,
            game_settings_preset: "Default".to_string(),
            game_settings: GameSettings::default(),
            user_groups: vec![
                UserGroup {
                    name: "Admin".to_string(),
                    password: "AdminXXXXXXXX".to_string(),
                    can_kick_ban: true,
                    can_access_inventories: true,
                    can_edit_base: true,
                    can_extend_base: true,
                    reserved_slots: 0,
                },
                UserGroup::default(),
            ],
        }
    }
}

/// Loads the configuration from a file or creates a new one with defaults.
/// Environment variables override both file values and defaults.
pub fn load_or_create_config(path: &Path) -> ServerConfig {
    tracing::debug!("Loading config from path: {:?}", path);

    let mut config = load_config_with_defaults::<ServerConfig>(path);
    tracing::debug!("Config loaded: {:?}", config.game_settings);

    let original_config = config.clone();

    tracing::debug!("Config loaded, applying environment overrides");
    apply_env_overrides(&mut config);

    let config_changed =
        serde_json::to_string(&config).unwrap() != serde_json::to_string(&original_config).unwrap();
    tracing::debug!("Config changed after env overrides: {}", config_changed);

    if path.exists() && config_changed {
        let timestamp = chrono::Local::now().format("%Y-%m-%d-%H.%M.%S");
        let backup_path = path.with_extension(format!("bak.{timestamp}.json"));
        tracing::debug!("Creating backup at: {:?}", backup_path);
        let _ = std::fs::copy(path, &backup_path);

        if let Some(parent) = path.parent() {
            let prefix = path.file_stem().unwrap_or_default().to_string_lossy();
            let mut backups: Vec<_> = std::fs::read_dir(parent)
                .unwrap_or_else(|_| std::fs::read_dir(".").unwrap())
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with(&format!("{prefix}.bak."))
                        && entry.file_name().to_string_lossy().ends_with(".json")
                })
                .collect();

            if backups.len() > 5 {
                backups.sort_by_key(|entry| {
                    entry
                        .metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::UNIX_EPOCH)
                });
                for old_backup in backups.iter().take(backups.len() - 5) {
                    let _ = std::fs::remove_file(old_backup.path());
                }
            }
        }
    }

    tracing::debug!("Saving config to: {:?}", path);
    save_config(path, &config);

    tracing::debug!("Config loading completed");
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::sync::Mutex;

    lazy_static::lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    fn clear_env_vars() {
        let vars = [
            "PLAYER_HEALTH_FACTOR",
            "PLAYER_MANA_FACTOR",
            "PLAYER_STAMINA_FACTOR",
            "PLAYER_BODY_HEAT_FACTOR",
            "EXPERIENCE_COMBAT_FACTOR",
            "TOMBSTONE_MODE",
            "THREAT_BONUS",
        ];
        for var in vars {
            unsafe {
                env::remove_var(var);
            }
        }
    }

    #[test]
    fn test_default_settings() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        clear_env_vars();

        let settings = GameSettings::default();

        assert_eq!(settings.player_health_factor, 1.0);
        assert_eq!(settings.player_mana_factor, 1.0);
        assert_eq!(settings.tombstone_mode, "AddBackpackMaterials");
    }

    #[test]
    fn test_env_override_f32() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        clear_env_vars();

        unsafe {
            env::set_var("PLAYER_HEALTH_FACTOR", "1.5");
            env::set_var("EXPERIENCE_COMBAT_FACTOR", "3.0");
        }

        let settings = GameSettings::from_env();
        assert_eq!(settings.player_health_factor, 1.5);
        assert_eq!(settings.experience_combat_factor, 3.0);
    }

    #[test]
    fn test_env_override_string() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        clear_env_vars();
        unsafe {
            env::set_var("TOMBSTONE_MODE", "Nothing");
        }

        let settings = GameSettings::from_env();
        assert_eq!(settings.tombstone_mode, "Nothing");
    }

    #[test]
    fn test_new_config_creation_with_env() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        clear_env_vars();

        unsafe {
            env::set_var("THREAT_BONUS", "5.0");
        }

        let path = Path::new("./tmp/test_new_config.json");
        fs::create_dir_all("./tmp").unwrap();
        let _ = fs::remove_file(path);

        let config = load_or_create_config(path);
        assert!(path.exists());
        assert!((config.game_settings.threat_bonus - 5.0).abs() < f32::EPSILON);

        let raw = fs::read_to_string(path).expect("failed to read config");
        let json: serde_json::Value = serde_json::from_str(&raw).expect("invalid JSON");
        let file_threat_bonus = json["gameSettings"]["threatBonus"]
            .as_f64()
            .expect("missing threatBonus");
        assert!((file_threat_bonus - 5.0).abs() < f64::EPSILON);

        unsafe {
            std::env::remove_var("THREAT_BONUS");
        }
    }

    #[test]
    fn test_load_or_create_config_saves_only_on_new_file() {
        use tempfile::TempDir;

        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        clear_env_vars();

        let tmp_dir = TempDir::new().expect("create temp dir");
        let config_path = tmp_dir.path().join("existing_config.json");

        let original_config = ServerConfig {
            name: "CustomName".to_string(),
            game_port: 54321,
            ..Default::default()
        };
        save_config(&config_path, &original_config);

        let loaded_config = load_or_create_config(&config_path);
        assert_eq!(loaded_config.name, "CustomName");
        assert_eq!(loaded_config.game_port, 54321);

        let raw = fs::read_to_string(&config_path).expect("failed to read config");
        let json: serde_json::Value = serde_json::from_str(&raw).expect("invalid JSON");
        let file_name = json["name"].as_str().expect("missing name");
        let file_port = json["gamePort"].as_i64().expect("missing gamePort");
        assert_eq!(file_name, "CustomName");
        assert_eq!(file_port, 54321);
    }

    #[test]
    fn test_preserve_and_override_with_env() {
        use std::fs;
        use std::path::PathBuf;
        use tempfile::TempDir;

        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        clear_env_vars();

        let tmp_dir = TempDir::new().expect("create temp dir");
        let config_path: PathBuf = tmp_dir.path().join("test_config.json");

        let config = GameSettings {
            player_health_factor: 42.0,
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, json).unwrap();

        let loaded: GameSettings = load_config_with_defaults(&config_path);
        assert_eq!(loaded.player_health_factor, 42.0);

        unsafe {
            std::env::set_var("PLAYER_HEALTH_FACTOR", "99.0");
        }
        let loaded2 = GameSettings::from_env();
        assert_eq!(loaded2.player_health_factor, 99.0);

        let mut config2: GameSettings = load_config_with_defaults(&config_path);
        if let Ok(val) = std::env::var("PLAYER_HEALTH_FACTOR") {
            config2.player_health_factor = val.parse().unwrap();
        }
        assert_eq!(config2.player_health_factor, 99.0);

        unsafe {
            std::env::remove_var("PLAYER_HEALTH_FACTOR");
        }
    }
}
