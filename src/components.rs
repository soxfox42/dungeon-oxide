use crate::ecs::{Component, World};
use crate::Context;

pub fn register_components(world: &mut World<Context>) {
    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Spr>();
    world.register::<Player>();
    world.register::<Collider>();
    world.register::<Health>();
    world.register::<HealthMod>();
    world.register::<Follow>();
    world.register::<Push>();
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}
impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Component for Pos {}

#[derive(Clone, Copy)]
pub struct Vel {
    pub x: i32,
    pub y: i32,
}
impl Vel {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Component for Vel {}

pub struct Spr(pub i32);
impl Component for Spr {}

#[derive(Default)]
pub struct Player {
    pub attack: bool,
}
impl Component for Player {}

#[derive(Clone, Copy)]
pub struct Collider {
    pub w: i32,
    pub h: i32,
}
impl Collider {
    pub fn new(w: i32, h: i32) -> Self {
        Self { w, h }
    }
}
impl Component for Collider {}

pub struct Health(pub i32);
impl Component for Health {}

pub struct HealthMod {
    pub health: i32,
    pub cooldown: u32,
}
impl Component for HealthMod {}

pub struct Follow(pub usize);
impl Component for Follow {}

pub struct Push;
impl Component for Push {}
