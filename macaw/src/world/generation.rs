use super::MacawWorld;

pub mod biomes;
pub mod generators;

const MAX_GEN_HEIGHT: u32 = 256;

pub trait Generator {
    fn generate(&mut self) -> MacawWorld;
}
