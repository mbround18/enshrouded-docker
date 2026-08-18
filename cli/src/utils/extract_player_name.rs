use regex::Regex;

/// Extracts the player name from a log line.
///
/// The log line is expected to contain a player name wrapped in single quotes,
/// e.g., "[server] Player 'mbround18' logged in with Permissions:".
///
/// # Arguments
///
/// * `log` - A string slice representing the log line.
///
/// # Returns
///
/// An `Option<String>` containing the player's name if found.
pub fn extract_player_joined_name(log: &str) -> Option<String> {
    // This regex looks for the word "Player" followed by one or more spaces,
    // then a single quote, then captures one or more characters that are not a single quote.
    let re = Regex::new(r"Player\s+'([^']+)'").unwrap();
    re.captures(log)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

/// Extracts the player name from a log line when a player leaves.
///
/// The log line is expected to follow the format:
/// `[server] Remove Player 'mbround18'`
///
/// # Arguments
///
/// * `log` - A string slice representing the log line.
///
/// # Returns
///
/// An `Option<String>` containing the player's name if the pattern is matched.
pub fn extract_player_left_name(log: &str) -> Option<String> {
    // This regex looks for the phrase "Remove Player", followed by one or more spaces,
    // then a single quote, then captures one or more characters that are not a single quote.
    let re = Regex::new(r"Remove Entity for Player\s+'([^']+)'").unwrap();
    re.captures(log)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}
