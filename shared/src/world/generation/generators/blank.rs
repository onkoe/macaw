//! # Blank
//!
//! A module that homes the simplest generator - one that generates literally
//! nothing!

use crate::{
    util::built_info,
    world::{metadata::GeneratorId, Generator, MacawWorld},
};
use async_trait::async_trait;

pub struct BlankGenerator;

impl Default for BlankGenerator {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Generator for BlankGenerator {
    fn name(&self) -> &'static str {
        "Blank Generator"
    }

    fn description(&self) -> &'static str {
        "A world generator that creates blank worlds."
    }

    fn id(&self) -> GeneratorId {
        GeneratorId::new("org", built_info::PKG_NAME, "generator", Some("blank"))
    }

    async fn pre_generate(&mut self, _seed: u64) -> MacawWorld {
        MacawWorld::default()
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
