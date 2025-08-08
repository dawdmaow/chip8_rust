use raylib::core::audio::RaylibAudio;

pub struct Core {
    pub audio: RaylibAudio,
}

impl Core {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let audio = raylib::core::audio::RaylibAudio::init_audio_device()?;
        Ok(Self { audio })
    }
}
