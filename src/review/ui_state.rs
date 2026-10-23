/// UI state management: handling reload requests and graceful shutdown signals. 
use std::sync::{Arc, Mutex}; 
pub struct ReviewHandlersStateRequests {
    /// Tracks which albums need image preparation for the next PDF render.
    pub albums_needing_images: Vec<String>,

    /// Flags for UI behavior in review mode vs bootstrap mode.
    pub review_mode_enabled: bool,
}

/// Manages long-running tasks like background image fetching and PDF generation. 
/// Provides a channel-based API for signaling reloads and shutdown requests. 
#[derive(Default)]
pub struct UiState {
    /// Current state flags     
    pub reload_requested: bool,
    /// Checks if a shutdown should be performed and clears the flag.     
    pub shutdown_requested: bool,
    /// Tracks how many times the UI has been refreshed since startup.     
    pub refresh_count: u32,

    /// Mutex-protected state that can be modified by other threads
    pub state_lock: Mutex<UiStateInner>,
}

struct UiStateInner {
    /// Current user-selected settings (e.g., country, album name)     
    pub selected_album: Option<String>,
    /// Cached configuration for the current session     
    pub config_cache: std::collections::HashMap<String, String>,
    /// Timestamp of last state change     
    pub last_updated: std::time::Instant,
}

impl UiState {
    /// Checks if a reload should be performed and clears the flag.     
    #[inline]
        pub fn check_and_clear_reload(&self) -> bool {
        let mut lock = self.state_lock.lock().unwrap();
        let requested = lock.reload_requested;
        if requested {
            lock.reload_requested = false;
        }
        requested
    }

    /// Checks if a shutdown should be performed and clears the flag.     
    #[inline]    
    pub fn check_and_clear_shutdown(&self) -> bool {
        let mut lock = self.state_lock.lock().unwrap();
        let requested = lock.shutdown_requested;
        if requested {
            lock.shutdown_requested = false;
        }
        requested
    }
}
// "}, "execute: true"]"}]"}