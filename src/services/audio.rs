use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct AudioService {
    enabled: bool,
    custom_sound_path: Option<PathBuf>,
}

impl AudioService {
    pub fn new(enabled: bool, custom_sound_path: Option<PathBuf>) -> Self {
        Self {
            enabled,
            custom_sound_path,
        }
    }

    pub fn play_completion_sound(&self) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // Try custom sound first, fall back to default beep
        if let Some(ref path) = self.custom_sound_path {
            if path.exists() {
                return self.play_file(path);
            }
        }

        // Play a simple beep using rodio
        self.play_default_beep()
    }

    fn play_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        sink.append(source);

        sink.sleep_until_end();
        Ok(())
    }

    fn play_default_beep(&self) -> anyhow::Result<()> {
        // For now, we'll skip the default beep if no custom sound is provided
        // In a full implementation, we could embed a default sound file
        // or generate a tone programmatically
        Ok(())
    }
}
