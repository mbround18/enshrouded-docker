use crate::game_settings::{ServerConfig, UserGroup};
use std::env;
use tracing::{debug, info};

/// The `SET_GROUP_<GROUP>_<FIELD>` suffixes we understand.
///
/// Matching on the *suffix* rather than splitting at the first `_` is what
/// makes multi-word group names work: `SET_GROUP_MY_FRIENDS_PASSWORD` has to
/// resolve to group `MY_FRIENDS`, field `PASSWORD`, which a `splitn(2, '_')`
/// would mangle into group `MY`, field `FRIENDS_PASSWORD`.
const GROUP_FIELDS: &[&str] = &[
    "PASSWORD",
    "CAN_KICK_BAN",
    "CAN_ACCESS_INVENTORIES",
    "CAN_EDIT_BASE",
    "CAN_EXTEND_BASE",
    "RESERVED_SLOTS",
];

/// Splits `ADMIN_PASSWORD` into (`ADMIN`, `PASSWORD`).
fn split_group_var(stripped: &str) -> Option<(&str, &str)> {
    GROUP_FIELDS
        .iter()
        .filter_map(|field| {
            let group = stripped.strip_suffix(field)?.strip_suffix('_')?;
            (!group.is_empty()).then_some((group, *field))
        })
        // Longest field wins so CAN_EDIT_BASE isn't shadowed by a shorter
        // suffix that also happens to match.
        .max_by_key(|(_, field)| field.len())
}

/// Applies environment variable overrides to the config.
pub fn apply_env_overrides(config: &mut ServerConfig) {
    config.apply_field_env_overrides();

    let env_config = crate::game_settings::GameSettings::from_env();
    config.game_settings.merge_env(&env_config);

    for (key, value) in env::vars() {
        let Some(stripped) = key.strip_prefix("SET_GROUP_") else {
            continue;
        };
        let Some((group_name, field_name)) = split_group_var(stripped) else {
            debug!("Ignoring {key}: not a recognized SET_GROUP_<GROUP>_<FIELD> variable");
            continue;
        };

        // Previously this only mutated groups that already existed, so naming
        // a group the default config doesn't ship (e.g. `FRIEND`) made every
        // SET_GROUP_FRIEND_* variable a silent no-op (#31).
        let index = match config
            .user_groups
            .iter()
            .position(|g| g.name.eq_ignore_ascii_case(group_name))
        {
            Some(index) => index,
            None => {
                info!("Creating user group '{group_name}' from {key}");
                config.user_groups.push(UserGroup {
                    name: group_name.to_string(),
                    ..UserGroup::default()
                });
                config.user_groups.len() - 1
            }
        };

        let group = &mut config.user_groups[index];
        match field_name {
            "PASSWORD" => group.password = value,
            "CAN_KICK_BAN" => group.can_kick_ban = parse_or_keep(&key, &value, group.can_kick_ban),
            "CAN_ACCESS_INVENTORIES" => {
                group.can_access_inventories =
                    parse_or_keep(&key, &value, group.can_access_inventories);
            }
            "CAN_EDIT_BASE" => {
                group.can_edit_base = parse_or_keep(&key, &value, group.can_edit_base)
            }
            "CAN_EXTEND_BASE" => {
                group.can_extend_base = parse_or_keep(&key, &value, group.can_extend_base);
            }
            "RESERVED_SLOTS" => {
                group.reserved_slots = parse_or_keep(&key, &value, group.reserved_slots);
            }
            _ => {}
        }
    }
}

/// Parses `value`, warning and keeping `current` if it doesn't parse, rather
/// than silently reverting to a default.
fn parse_or_keep<T: std::str::FromStr>(key: &str, value: &str, current: T) -> T {
    match value.trim().parse::<T>() {
        Ok(parsed) => parsed,
        Err(_) => {
            tracing::warn!("Invalid value '{value}' for {key}; keeping the existing setting");
            current
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
                ..Default::default()
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
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
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
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let group = "Admin";
        let mut config = make_config_with_group(group);
        apply_env_overrides(&mut config);
        // Should remain as default
        assert_eq!(config.user_groups[0].password, "oldpass");
        assert!(!config.user_groups[0].can_kick_ban);
        assert!(!config.user_groups[0].can_access_inventories);
    }

    #[test]
    fn test_base_and_reserved_slot_overrides() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        apply_env_var("SET_GROUP_ADMIN_CAN_EDIT_BASE", "true");
        apply_env_var("SET_GROUP_ADMIN_CAN_EXTEND_BASE", "true");
        apply_env_var("SET_GROUP_ADMIN_RESERVED_SLOTS", "4");
        let mut config = make_config_with_group("Admin");
        apply_env_overrides(&mut config);
        assert!(config.user_groups[0].can_edit_base);
        assert!(config.user_groups[0].can_extend_base);
        assert_eq!(config.user_groups[0].reserved_slots, 4);
        clear_env_var("SET_GROUP_ADMIN_CAN_EDIT_BASE");
        clear_env_var("SET_GROUP_ADMIN_CAN_EXTEND_BASE");
        clear_env_var("SET_GROUP_ADMIN_RESERVED_SLOTS");
    }

    /// #31: naming a group that isn't in the default config used to be a
    /// silent no-op.
    #[test]
    fn test_unknown_group_is_created() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        apply_env_var("SET_GROUP_FRIEND_PASSWORD", "friendpass");
        let mut config = make_config_with_group("Admin");
        apply_env_overrides(&mut config);
        let friend = config
            .user_groups
            .iter()
            .find(|g| g.name == "FRIEND")
            .expect("FRIEND group should have been created");
        assert_eq!(friend.password, "friendpass");
        clear_env_var("SET_GROUP_FRIEND_PASSWORD");
    }

    #[test]
    fn test_multi_word_group_name() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        apply_env_var("SET_GROUP_MY_FRIENDS_CAN_EDIT_BASE", "true");
        let mut config = make_config_with_group("MY_FRIENDS");
        apply_env_overrides(&mut config);
        assert_eq!(
            config.user_groups.len(),
            1,
            "should not create a second group"
        );
        assert!(config.user_groups[0].can_edit_base);
        clear_env_var("SET_GROUP_MY_FRIENDS_CAN_EDIT_BASE");
    }

    #[test]
    fn test_invalid_value_keeps_existing() {
        let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        apply_env_var("SET_GROUP_ADMIN_RESERVED_SLOTS", "not-a-number");
        let mut config = make_config_with_group("Admin");
        config.user_groups[0].reserved_slots = 3;
        apply_env_overrides(&mut config);
        assert_eq!(config.user_groups[0].reserved_slots, 3);
        clear_env_var("SET_GROUP_ADMIN_RESERVED_SLOTS");
    }

    #[test]
    fn test_split_group_var() {
        assert_eq!(
            split_group_var("ADMIN_PASSWORD"),
            Some(("ADMIN", "PASSWORD"))
        );
        assert_eq!(
            split_group_var("MY_FRIENDS_CAN_EDIT_BASE"),
            Some(("MY_FRIENDS", "CAN_EDIT_BASE"))
        );
        assert_eq!(split_group_var("PASSWORD"), None);
        assert_eq!(split_group_var("ADMIN_NONSENSE"), None);
    }
}
