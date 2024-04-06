use super::{coordinates::ChunkBlockCoordinate, MacawWorld};
use async_trait::async_trait;
use std::sync::Arc;

pub mod biomes;
pub mod generators;

const MAX_GEN_HEIGHT: u32 = 256;

#[async_trait]
pub trait Generator {
    /// The name of this generator.
    fn name(&self) -> &'static str;

    /// The description of this generator.
    fn description(&self) -> &'static str;

    /// Creates a new MacawWorld and returns it.
    async fn pre_generate(&mut self, seed: u64) -> MacawWorld;

    /// Given an existing world and two chunk coordinates, generates the chunks in between.
    async fn generate(
        &mut self,
        world: &mut MacawWorld,
        chunks: (ChunkBlockCoordinate, ChunkBlockCoordinate),
    );
}

pub struct GeneratorWrapper(pub Arc<dyn Generator>);

impl GeneratorWrapper {
    pub fn new(generator: impl Generator + 'static) -> Self {
        Self(Arc::new(generator))
    }
}

impl core::fmt::Debug for GeneratorWrapper {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GeneratorWrapper({} - {})",
            self.0.name(),
            self.0.description()
        )
    }
}

impl Clone for GeneratorWrapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
