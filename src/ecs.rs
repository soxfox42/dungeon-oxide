#![allow(dead_code)]

//! A super-simple ECS framework.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A container to store all components and systems in use at any point.
#[derive(Default)]
pub struct World {
    components: HashMap<TypeId, Box<dyn ComponentVec>>,
    entities: usize,
}

impl World {
    /// Constructs a new, empty `World`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates storage in the `World` for a specific [`Component`] type.
    pub fn register<T: Component>(&mut self) {
        if self.entities != 0 {
            panic!("Attempted to register a new component on an active World");
        }
        self.components
            .insert(TypeId::of::<T>(), Box::new(Vec::<Option<T>>::new()));
    }

    /// Inserts a new entity using a closure that populates the entity's components.
    pub fn entity(
        &mut self,
        f: impl FnOnce(&mut EntityBuilder) -> &mut EntityBuilder,
    ) -> Entity {
        let mut builder = EntityBuilder::default();
        f(&mut builder);
        self.insert(builder.0)
    }

    /// Inserts a new entity given a map of types to components.
    fn insert(&mut self, mut components: HashMap<TypeId, Box<dyn Any>>) -> Entity {
        self.entities += 1;
        for (typeid, vec) in self.components.iter_mut() {
            vec.insert(components.remove(typeid));
        }
        Entity(self.entities - 1)
    }

    /// Borrows the vector corresponding to a [`Component`] type.
    pub fn component<T: Component>(&self) -> &Vec<Option<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .expect("Attempted to borrow non-existent component vector")
            .as_any()
            .downcast_ref()
            .unwrap()
    }

    /// Mutably borrows the vector corresponding to a [`Component`] type.
    pub fn component_mut<T: Component>(&mut self) -> &mut Vec<Option<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .expect("Attempted to borrow non-existent component vector")
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    pub fn execute<'a, T: Fetch<'a>>(&'a mut self, mut system: impl FnMut(T)) {
        for i in 0..self.entities {
            if let Some(data) = T::fetch(self, i) {
                system(data);
            }
        }
    }
}

/// A representation of a single object in the game.
/// Modelled internally as a single index into the component storage vectors.
pub struct Entity(usize);

/// A helper struct for constructing new entities.
#[derive(Default)]
pub struct EntityBuilder(HashMap<TypeId, Box<dyn Any>>);

impl EntityBuilder {
    pub fn component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.0.insert(TypeId::of::<T>(), Box::new(component));
        self
    }
}

/// A marker trait indicating that a type may be used as component data.
pub trait Component: 'static {}

/// A trait for dynamically typed component vectors.
trait ComponentVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn insert(&mut self, component: Option<Box<dyn Any>>);
}

impl<T: Component> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn insert(&mut self, component: Option<Box<dyn Any>>) {
        self.push(component.map(|x| *x.downcast().unwrap()));
    }
}

pub trait Fetch<'a>: Sized {
    fn fetch(world: &'a World, idx: usize) -> Option<Self>;
}

impl<'a, T: Component> Fetch<'a> for &'a T {
    fn fetch(world: &'a World, idx: usize) -> Option<Self> {
        world.component::<T>()[idx].as_ref()
    }
}

macro_rules! fetch_tuple {
    ($($name:ident),+) => {
        impl<'a, $($name),+> Fetch<'a> for ($($name,)+)
        where
        $(
            $name: Fetch<'a>,
        )+
        {
            #[allow(non_snake_case)]
            fn fetch(world: &'a World, idx: usize) -> Option<Self> {
                $(
                    let $name = $name::fetch(world, idx)?;
                )+
                Some(($($name,)+))
            }
        }
    };
}

macro_rules! fetch_tuples {
    ($name:ident) => {
        fetch_tuple!($name);
    };
    ($name:ident, $($names:ident),+) => {
        fetch_tuple!($name, $($names),+);
        fetch_tuples!($($names),+);
    };
}

// Implement up to 8-tuple fetches
fetch_tuples!(A, B, C, D, E, F, G, H);
