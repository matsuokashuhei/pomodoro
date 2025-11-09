use notify_rust::Notification;

pub struct NotificationService {
    enabled: bool,
}

impl NotificationService {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn send_work_complete(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // Ignore notification errors (e.g., when no display is available in tests)
        let _ = Notification::new()
            .summary("Work Session Complete! 🎉")
            .body("Great job! Time for a break.")
            .show();

        Ok(())
    }

    pub fn send_break_complete(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let _ = Notification::new()
            .summary("Break Complete! ⏰")
            .body("Ready to start another work session?")
            .show();

        Ok(())
    }

    pub fn send_long_break_suggestion(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let _ = Notification::new()
            .summary("Time for a Long Break! 🌟")
            .body("You've completed 4 work sessions. Take a longer break!")
            .show();

        Ok(())
    }

    pub fn send_custom(&self, title: &str, message: &str) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let _ = Notification::new().summary(title).body(message).show();

        Ok(())
    }
}
