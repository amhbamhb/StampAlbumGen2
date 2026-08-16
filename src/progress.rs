use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A progress indicator that shows a spinning animation with status messages
pub struct ProgressIndicator {
    pb: Arc<Mutex<Option<ProgressBar>>>,
    current_step: Arc<Mutex<String>>,
    enabled: bool,
}

impl ProgressIndicator {
    /// Create a new progress indicator
    pub fn new() -> Self {
        Self::new_with_config(true)
    }

    /// Create a new progress indicator with configuration
    pub fn new_with_config(enabled: bool) -> Self {
        let pb = if enabled {
            let progress_bar = ProgressBar::new_spinner();

            // Set up the spinning animation with custom style
            progress_bar.set_style(
                ProgressStyle::with_template("{spinner:.green} {msg}")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );

            // Enable steady tick to keep the spinner moving
            progress_bar.enable_steady_tick(Duration::from_millis(120));

            Some(progress_bar)
        } else {
            None
        };

        Self {
            pb: Arc::new(Mutex::new(pb)),
            current_step: Arc::new(Mutex::new("Starting...".to_string())),
            enabled,
        }
    }

    /// Update the current step message
    pub fn set_message(&self, message: &str) {
        if !self.enabled {
            return;
        }

        // TEMPORARY: Print each message on a new line for testing order
        println!("🔄 {}", message);

        if let (Ok(pb_opt), Ok(mut step)) = (self.pb.lock(), self.current_step.lock()) {
            *step = message.to_string();
            if let Some(pb) = pb_opt.as_ref() {
                pb.set_message(message.to_string());
            }
        }
    }

    /// Update with a formatted message
    pub fn set_message_format(&self, args: std::fmt::Arguments) {
        self.set_message(&format!("{}", args));
    }

    /// Finish the progress indicator with a success message
    pub fn finish_with_message(&self, message: &str) {
        if !self.enabled {
            return;
        }

        // TEMPORARY: Print final success message on new line
        println!("✅ {}", message);

        if let Ok(pb_opt) = self.pb.lock() {
            if let Some(pb) = pb_opt.as_ref() {
                pb.finish_with_message(format!("✅ {}", message));
            }
        }
    }

    /// Finish the progress indicator with an error message
    pub fn finish_with_error(&self, message: &str) {
        if !self.enabled {
            return;
        }

        // TEMPORARY: Print error message on new line
        println!("❌ {}", message);

        if let Ok(pb_opt) = self.pb.lock() {
            if let Some(pb) = pb_opt.as_ref() {
                pb.finish_with_message(format!("❌ {}", message));
            }
        }
    }

    /// Clear the progress indicator
    pub fn clear(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(pb_opt) = self.pb.lock() {
            if let Some(pb) = pb_opt.as_ref() {
                pb.finish_and_clear();
            }
        }
    }
}

impl Drop for ProgressIndicator {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Default for ProgressIndicator {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro to easily update progress with formatted messages
#[macro_export]
macro_rules! progress_update {
    ($progress:expr, $($arg:tt)*) => {
        $progress.set_message(&format!($($arg)*))
    };
}

/// Convenience function to create a new progress indicator
pub fn new_progress() -> ProgressIndicator {
    ProgressIndicator::new()
}

/// Convenience function to create a new progress indicator with configuration
pub fn new_progress_with_config(enabled: bool) -> ProgressIndicator {
    ProgressIndicator::new_with_config(enabled)
}
