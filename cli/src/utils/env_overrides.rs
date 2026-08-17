use crate::game_settings::ServerConfig;
use std::env;

/// Applies environment variable overrides to the config.
pub fn apply_env_overrides(config: &mut ServerConfig) {
    let env_config = crate::game_settings::GameSettings::from_env();
    config.game_settings.merge_env(&env_config);

    for (key, value) in env::vars() {
        if let Some(stripped) = key.strip_prefix("SET_GROUP_") {
            let mut parts = stripped.splitn(2, '_');
            if let (Some(group_name), Some(field_name)) = (parts.next(), parts.next()) {
                if let Some(group) = config
                    .user_groups
                    .iter_mut()
                    .find(|g| g.name.eq_ignore_ascii_case(group_name))
                {
                    match field_name.to_lowercase().as_str() {
                        "password" => group.password = value,
                        "can_kick_ban" => {
                            group.can_kick_ban = value.parse().unwrap_or(group.can_kick_ban)
                        }
                        "can_access_inventories" => {
                            group.can_access_inventories =
                                value.parse().unwrap_or(group.can_access_inventories)
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_settings::{ServerConfig, UserGroup};
    use std::sync::Mutex;
    lazy_static::lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    fn make_config_with_group(name: &str) -> ServerConfig {
        ServerConfig {
            user_groups: vec![UserGroup {
                name: name.to_string(),
                password: "oldpass".to_string(),
                can_kick_ban: false,
                can_access_inventories: false,
                can_edit_base: false,
                can_extend_base: false,
                reserved_slots: 0,
            }],
            ..Default::default()
        }
    }

    fn clear_env_var(var: &str) {
        unsafe {
            std::env::remove_var(var);
        }
    }

    fn apply_env_var(var: &str, value: &str) {
        unsafe {
            std::env::set_var(var, value);
        }
    }

    #[test]
    fn test_password_override() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let group = "Admin";
        let env_var = format!("SET_GROUP_{group}_PASSWORD");
        apply_env_var(&env_var, "newpass");
        let mut config = make_config_with_group(group);
        apply_env_overrides(&mut config);
        assert_eq!(config.user_groups[0].password, "newpass");
        clear_env_var(&env_var);
    }

    #[test]
    fn test_can_kick_ban_override() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let group = "Admin";
        let env_var = format!("SET_GROUP_{group}_CAN_KICK_BAN");
        apply_env_var(&env_var, "true");
        let mut config = make_config_with_group(group);
        apply_env_overrides(&mut config);
        assert!(config.user_groups[0].can_kick_ban);
        clear_env_var(&env_var);
    }

    #[test]
    fn test_can_access_inventories_override() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let group = "Admin";
        let env_var = format!("SET_GROUP_{group}_CAN_ACCESS_INVENTORIES");
        apply_env_var(&env_var, "true");
        let mut config = make_config_with_group(group);
        apply_env_overrides(&mut config);
        assert!(config.user_groups[0].can_access_inventories);
        clear_env_var(&env_var);
    }

    #[test]
    fn test_no_override_for_unset_env() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let group = "Admin";
        let mut config = make_config_with_group(group);
        apply_env_overrides(&mut config);
        // Should remain as default
        assert_eq!(config.user_groups[0].password, "oldpass");
        assert!(!config.user_groups[0].can_kick_ban);
        assert!(!config.user_groups[0].can_access_inventories);
    }
}
