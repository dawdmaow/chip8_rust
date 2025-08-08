use crate::core::Core;
use raylib::core::audio::Sound;

pub fn generate_wav_file(frequency: f32, duration: f32, sample_rate: u32) -> Vec<u8> {
    // Spooky magic.

    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut wav_data = Vec::new();

    wav_data.extend_from_slice(b"RIFF");
    let file_size = 36 + (num_samples * 2) as u32;
    wav_data.extend_from_slice(&file_size.to_le_bytes());
    wav_data.extend_from_slice(b"WAVE");

    wav_data.extend_from_slice(b"fmt ");
    wav_data.extend_from_slice(&16u32.to_le_bytes());
    wav_data.extend_from_slice(&1u16.to_le_bytes());
    wav_data.extend_from_slice(&1u16.to_le_bytes());
    wav_data.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate * 2;
    wav_data.extend_from_slice(&byte_rate.to_le_bytes());
    wav_data.extend_from_slice(&2u16.to_le_bytes());
    wav_data.extend_from_slice(&16u16.to_le_bytes());

    wav_data.extend_from_slice(b"data");
    let data_size = (num_samples * 2) as u32;
    wav_data.extend_from_slice(&data_size.to_le_bytes());

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
        let sample_i16 = (sample * 32767.0) as i16;
        wav_data.extend_from_slice(&sample_i16.to_le_bytes());
    }

    wav_data
}

pub fn create_beep_sound(core: &Core) -> Option<Sound> {
    let wav_data = generate_wav_file(440.0, 0.1, 44100);
    let wave = core.audio.new_wave_from_memory(".wav", &wav_data).ok()?;
    Some(core.audio.new_sound_from_wave(&wave).ok()?)
}
