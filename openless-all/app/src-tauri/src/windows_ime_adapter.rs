//! Windows IME Integration Deep Module (Adapter).
//! Encapsulates Windows TSF Profile registration, IPC pipe messaging, and IME Session state.
//! Provides minimal interface (`sync_status`, `insert_text`) and automatic macOS/Linux cross-platform stubs.

use crate::types::{UserPreferences, WindowsImeInstallState, WindowsImeStatus};

pub struct WindowsImeAdapter;

impl WindowsImeAdapter {
    pub fn new() -> Self {
        Self
    }

    #[cfg(target_os = "windows")]
    pub fn is_supported(&self) -> bool {
        true
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_supported(&self) -> bool {
        false
    }

    pub fn current_status(&self) -> WindowsImeStatus {
        crate::windows_ime_profile::get_windows_ime_status()
    }

    pub fn sync_profile_with_preferences(&self, _prefs: &UserPreferences) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_ime_adapter_cross_platform_stub() {
        let adapter = WindowsImeAdapter::new();
        if cfg!(target_os = "windows") {
            assert!(adapter.is_supported());
        } else {
            assert!(!adapter.is_supported());
            assert_eq!(
                adapter.current_status().state,
                WindowsImeInstallState::NotWindows
            );
        }
    }
}
