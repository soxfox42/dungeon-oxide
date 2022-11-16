use crate::components::*;
use crate::ecs::{EntityBuilder, World};
use crate::Context;

use serde_json::{Map, Value};

fn get_int<T: Default + TryFrom<i64>>(obj: &Map<String, Value>, key: &str) -> T {
    match obj[key].as_i64().unwrap().try_into() {
        Ok(n) => n,
        Err(_) => T::default(),
    }
}

fn entity_loader(components: &Vec<Value>) -> impl FnOnce(&mut EntityBuilder) -> &mut EntityBuilder {
    let components: Vec<Value> = components.clone();
    move |entity| {
        for component in components {
            let obj = component.as_object().unwrap();
            match obj["type"].as_str().unwrap() {
                "pos" => {
                    entity.with_component(Pos::new(get_int(obj, "x"), get_int(obj, "y")));
                }
                "vel" => {
                    entity.with_component(Vel::new(get_int(obj, "x"), get_int(obj, "y")));
                }
                "spr" => {
                    entity.with_component(Spr(get_int(obj, "id")));
                }
                "player" => {
                    entity.with_component(Player::default());
                }
                "collider" => {
                    entity.with_component(Collider::new(get_int(obj, "w"), get_int(obj, "h")));
                }
                "health" => {
                    entity.with_component(Health(get_int(obj, "val")));
                }
                "healthmod" => {
                    entity.with_component(HealthMod {
                        health: get_int(obj, "val"),
                        cooldown: 0,
                    });
                }
                "follow" => {
                    entity.with_component(Follow(get_int(obj, "id")));
                }
                "push" => {
                    entity.with_component(Push);
                }
                other => panic!("unrecognised component {other}"),
            }
        }
        entity
    }
}

pub fn load_level(world: &mut World<Context>, level_str: &str) -> usize {
    let level: Value = serde_json::from_str(level_str).unwrap();
    for entity in level["entities"].as_array().unwrap() {
        world.add_entity(entity_loader(entity.as_array().unwrap()));
    }
    level["map"].as_u64().unwrap() as usize
}
