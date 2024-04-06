//! # Blank
//!
//! A module that homes the simplest generator - one that generates literally
//! nothing!

use crate::world::{Generator, MacawWorld};
use async_trait::async_trait;

pub struct BlankGenerator;

#[async_trait]
impl Generator for BlankGenerator {
    fn name(&self) -> &'static str {
        "Blank Generator"
    }

    fn description(&self) -> &'static str {
        "A world generator that creates blank worlds."
    }

    async fn pre_generate(&mut self, seed: u64) -> MacawWorld {
        MacawWorld::default().await
    }

    async fn generate(
        &mut self,
        _world: &mut MacawWorld,
        _chunks: (
            crate::world::coordinates::ChunkBlockCoordinate,
            crate::world::coordinates::ChunkBlockCoordinate,
        ),
    ) {
    }
}
