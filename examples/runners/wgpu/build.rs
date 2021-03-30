use spirv_builder::SpirvBuilder;
use std::error::Error;

fn build_shader(path_to_create: &str) -> Result<(), Box<dyn Error>> {
    SpirvBuilder::new(path_to_create)
        .spirv_version(1, 0)
        .target("spirv-unknown-vulkan1.1")
        .build()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    build_shader("../../shaders/sky-shader")?;
    build_shader("../../shaders/simplest-shader")?;
    build_shader("../../shaders/compute-shader")?;
    build_shader("../../shaders/mouse-shader")?;
    Ok(())
}
