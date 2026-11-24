use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

/// The maximum number of entries kept in `AppState::debug_messages`.
pub const MAX_DEBUG_MESSAGES: usize = 100;

/// The level an entry in the Debug screen is prefixed with, so the frontend can filter without guessing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugLevel {
    Error,
    Ok,
    Info,
}

impl DebugLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            DebugLevel::Error => "ERROR",
            DebugLevel::Ok => "OK",
            DebugLevel::Info => "INFO",
        }
    }
}

/// Collects the flow entries for the in-app Debug screen (Dev Mode > Debug messages) while an action is reduced.
///
/// Reducers write to it through a shared handle, so the entries survive a reducer that fails halfway: `command.rs`
/// drains them into `AppState::debug_messages` after every action, whether the state update succeeded or not.
///
/// Write entries with [`debug_log_info!`](crate::debug_log_info), [`debug_log_ok!`](crate::debug_log_ok) and
/// [`debug_log_error!`](crate::debug_log_error). They also write every entry to the developer log, with the module and
/// line they are called from as the log target, e.g. `identity_wallet::state::qr_code::reducers::accept_connection:86`.
///
/// Entries must only contain IDs, types, counts, Issuer/Verifier names and hosts, statuses and durations, never
/// Credential contents, claim values, tokens or keys.
#[derive(Clone, Default, Debug)]
pub struct DebugLog(Arc<Mutex<Vec<String>>>);

impl DebugLog {
    /// Writes an entry to the developer log under `target` and to the Debug screen. Called by the `debug_log_*!`
    /// macros, which pass the module and line of their call site as `target`.
    #[doc(hidden)]
    pub fn record(&self, level: DebugLevel, target: &str, message: String) {
        let log_level = match level {
            DebugLevel::Error => log::Level::Error,
            DebugLevel::Ok | DebugLevel::Info => log::Level::Info,
        };
        log::log!(target: target, log_level, "{message}");
        self.push(level, message);
    }

    fn push(&self, level: DebugLevel, message: impl Display) {
        if let Ok(mut entries) = self.0.lock() {
            entries.push(format_debug_message(level, message));
        }
    }

    /// Moves the collected entries into `debug_messages`, dropping the oldest ones beyond [`MAX_DEBUG_MESSAGES`].
    pub fn drain_into(&self, debug_messages: &mut VecDeque<String>) {
        if let Ok(mut entries) = self.0.lock() {
            debug_messages.extend(entries.drain(..));
        }
        truncate_debug_messages(debug_messages);
    }
}

/// Writes an INFO entry to the Debug screen and the developer log, e.g.
/// `debug_log_info!(debug_log, "Scanned {}", summary)`. The log target is the calling module and line.
#[macro_export]
macro_rules! debug_log_info {
    ($debug_log:expr, $($arg:tt)+) => {
        $debug_log.record(
            $crate::state::core_utils::debug_log::DebugLevel::Info,
            concat!(module_path!(), ":", line!()),
            format!($($arg)+),
        )
    };
}

/// Writes an OK entry to the Debug screen and an INFO line to the developer log, for a flow that succeeded. The log
/// target is the calling module and line.
#[macro_export]
macro_rules! debug_log_ok {
    ($debug_log:expr, $($arg:tt)+) => {
        $debug_log.record(
            $crate::state::core_utils::debug_log::DebugLevel::Ok,
            concat!(module_path!(), ":", line!()),
            format!($($arg)+),
        )
    };
}

/// Writes an ERROR entry to the Debug screen and the developer log. The log target is the calling module and line.
#[macro_export]
macro_rules! debug_log_error {
    ($debug_log:expr, $($arg:tt)+) => {
        $debug_log.record(
            $crate::state::core_utils::debug_log::DebugLevel::Error,
            concat!(module_path!(), ":", line!()),
            format!($($arg)+),
        )
    };
}

/// Formats a Debug screen entry as `[YYYY-MM-DD][HH:MM:SS] LEVEL message`.
pub fn format_debug_message(level: DebugLevel, message: impl Display) -> String {
    format!(
        "{} {} {message}",
        chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
        level.as_str()
    )
}

/// Pushes an entry directly into `debug_messages`, for code that runs outside a reducer (e.g. the action timeout).
pub fn push_debug_message(debug_messages: &mut VecDeque<String>, level: DebugLevel, message: impl Display) {
    debug_messages.push_back(format_debug_message(level, message));
    truncate_debug_messages(debug_messages);
}

fn truncate_debug_messages(debug_messages: &mut VecDeque<String>) {
    while debug_messages.len() > MAX_DEBUG_MESSAGES {
        debug_messages.pop_front();
    }
}

/// Formats an error and its chain of sources with `Display`, e.g. `Failed to load stronghold: invalid file`.
pub fn error_chain(error: &(dyn std::error::Error + 'static)) -> String {
    let mut message = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        let cause_message = cause.to_string();
        // Some errors repeat their source in their own message.
        if !message.contains(&cause_message) {
            message.push_str(": ");
            message.push_str(&cause_message);
        }
        source = cause.source();
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_are_prefixed_with_timestamp_and_level() {
        let debug_log = DebugLog::default();
        crate::debug_log_ok!(debug_log, "Received 1 Credential(s) from Issuer: University Degree");
        crate::debug_log_error!(debug_log, "Token request failed");

        let mut debug_messages = VecDeque::new();
        debug_log.drain_into(&mut debug_messages);

        let rest: Vec<_> = debug_messages
            .iter()
            .map(|message| message.split_once(' ').unwrap().1)
            .collect();
        assert_eq!(
            rest,
            [
                "OK Received 1 Credential(s) from Issuer: University Degree",
                "ERROR Token request failed"
            ]
        );
        assert!(debug_messages[0].starts_with('['));
    }

    #[test]
    fn drain_empties_the_log_and_caps_the_messages() {
        let debug_log = DebugLog::default();
        let mut debug_messages: VecDeque<String> = (0..MAX_DEBUG_MESSAGES).map(|i| i.to_string()).collect();
        crate::debug_log_info!(debug_log, "new entry");

        debug_log.drain_into(&mut debug_messages);
        assert_eq!(debug_messages.len(), MAX_DEBUG_MESSAGES);
        assert_eq!(debug_messages.front().unwrap(), "1");
        assert!(debug_messages.back().unwrap().ends_with("INFO new entry"));

        debug_log.drain_into(&mut debug_messages);
        assert_eq!(debug_messages.len(), MAX_DEBUG_MESSAGES);
    }

    #[test]
    fn clones_share_the_same_entries() {
        let debug_log = DebugLog::default();
        crate::debug_log_info!(debug_log.clone(), "from a clone");

        let mut debug_messages = VecDeque::new();
        debug_log.drain_into(&mut debug_messages);
        assert_eq!(debug_messages.len(), 1);
    }

    #[test]
    fn error_chain_joins_sources() {
        #[derive(Debug, thiserror::Error)]
        #[error("inner")]
        struct Inner;
        #[derive(Debug, thiserror::Error)]
        #[error("outer")]
        struct Outer(#[source] Inner);

        assert_eq!(error_chain(&Outer(Inner)), "outer: inner");
    }
}
