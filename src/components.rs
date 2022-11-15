use crate::ecs::Component;

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

pub struct Spr(pub usize);
impl Component for Spr {}

pub struct Player;
impl Component for Player {}
