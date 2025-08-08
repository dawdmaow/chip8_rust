use std::path::Path;

pub fn load_rom(
    cpu: &mut crate::cpu::Cpu,
    rom_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    cpu.reset();
    cpu.memory.load_rom(rom_path)?;
    eprintln!(
        "Loaded ROM: {} ({} bytes)",
        rom_path,
        Path::new(rom_path).metadata()?.len()
    );
    Ok(())
}
