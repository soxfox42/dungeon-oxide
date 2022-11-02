
//! A super-simple ECS framework.

use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

type SparseVec<T> = Vec<Option<T>>;

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
        assert_eq!(self.entities, 0, "Attempted to register a new component on an active World");
        self.components.insert(
            TypeId::of::<T>(),
            Box::new(RefCell::new(SparseVec::<T>::new())),
        );
    }

    /// Inserts a new entity using a closure that populates the entity's components.
    pub fn entity(&mut self, f: impl FnOnce(&mut EntityBuilder) -> &mut EntityBuilder) -> Entity {
        let mut builder = EntityBuilder::default();
        f(&mut builder);
        self.insert(builder.0)
    }

    /// Inserts a new entity given a map of types to components.
    fn insert(&mut self, mut components: HashMap<TypeId, Box<dyn Any>>) -> Entity {
        self.entities += 1;
        for (typeid, vec) in &mut self.components {
            vec.insert(components.remove(typeid));
        }
        Entity(self.entities - 1)
    }

    /// Retrieves the [`std::cell::RefCell`] containing a [`Component`]'s storage.
    fn cell<T: Component>(&self) -> &RefCell<SparseVec<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .expect("Attempted to borrow non-existent component vector")
            .as_any()
            .downcast_ref::<RefCell<SparseVec<T>>>()
            .unwrap()
    }

    pub fn execute<'a, T: Fetch<'a>>(&'a mut self, mut system: impl FnMut(T)) {
        system(T::fetch(self));
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

impl<T: Component> ComponentVec for RefCell<SparseVec<T>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn insert(&mut self, component: Option<Box<dyn Any>>) {
        self.get_mut()
            .push(component.map(|x| *x.downcast().unwrap()));
    }
}

/// A type that may be fetched from a `World`.
pub trait Fetch<'a>: Sized {
    fn fetch(world: &'a World) -> Self;
}

pub struct Read<'a, T> {
    ptr: *mut SparseVec<T>,
    index: usize,
    _borrow: Ref<'a, SparseVec<T>>,
}
pub struct ReadWrite<'a, T> {
    ptr: *mut SparseVec<T>,
    index: usize,
    _borrow: RefMut<'a, SparseVec<T>>,
}

impl<'a, T: Component> Fetch<'a> for Read<'a, T> {
    fn fetch(world: &'a World) -> Self {
        let cell = world.cell();
        let borrow = cell.borrow();
        Self {
            ptr: cell.as_ptr(),
            index: 0,
            _borrow: borrow,
        }
    }
}
impl<'a, T: Component> Fetch<'a> for ReadWrite<'a, T> {
    fn fetch(world: &'a World) -> Self {
        let cell = world.cell();
        let borrow = cell.borrow_mut();
        Self {
            ptr: cell.as_ptr(),
            index: 0,
            _borrow: borrow,
        }
    }
}

impl<'a, T: Component> Iterator for Read<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        // SAFETY: See ReadWrite.
        let vec = unsafe { &mut *self.ptr };
        vec.get(index).map(Option::as_ref)
    }
}
impl<'a, T: Component> Iterator for ReadWrite<'a, T> {
    type Item = Option<&'a mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        // SAFETY:
        // - The Vec behind self.ptr is guaranteed to last at least as long as self.borrow,
        //   as long as self.borrow is never modified.
        // - Due to the strictly increasing self.index, this will never return multiple mutable
        //   references to the same cell.
        // - Finally, thanks to the RefCell used to hold the component storages, Read and ReadWrite
        //   follow standard Rust aliasing rules.
        let vec = unsafe { &mut *self.ptr };
        vec.get_mut(index).map(Option::as_mut)
    }
}

// macro_rules! fetch_tuple {
//     ($($name:ident),+) => {
//         impl<'a, $($name),+> Fetch<'a> for ($($name,)+)
//         where
//         $(
//             $name: Fetch<'a>,
//         )+
//         {
//             #[allow(non_snake_case)]
//             fn fetch(world: &'a World, idx: usize) -> Option<Self> {
//                 $(
//                     let $name = $name::fetch(world, idx)?;
//                 )+
//                 Some(($($name,)+))
//             }
//         }
//     };
// }
//
// macro_rules! fetch_tuples {
//     ($name:ident) => {
//         fetch_tuple!($name);
//     };
//     ($name:ident, $($names:ident),+) => {
//         fetch_tuple!($name, $($names),+);
//         fetch_tuples!($($names),+);
//     };
// }
//
// // Implement up to 8-tuple fetches
// fetch_tuples!(A, B, C, D, E, F, G, H);
