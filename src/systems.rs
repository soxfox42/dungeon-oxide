mod gameplay;
mod graphics;

pub use gameplay::*;
pub use graphics::*;

use crate::{ecs::World, Context};

pub fn register_systems(world: &mut World<Context>) {
    world.system(draw_sprites);
    world.system(draw_attack);
    world.system(draw_health);
    world.system(player_input);
    world.system(move_followers);
    world.system(apply_velocities);
    world.system(move_pushables);
    world.system(update_health);
    world.system(decelerate);
    world.system(remove_dead);
}
