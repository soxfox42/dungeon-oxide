mod gameplay;
mod graphics;

pub use gameplay::*;
pub use graphics::*;

use crate::{ecs::World, Context};

pub fn register_systems(world: &mut World<Context>) {
    world.system(draw_sprites);
    world.system(player_input);
    world.system(apply_velocities);
}
