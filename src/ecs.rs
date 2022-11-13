//! A super-simple ECS framework.

use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

/// A vector in which each element may or may not be present.
type SparseVec<T> = Vec<Option<T>>;
/// A map from component types to component vectors.
type ComponentMap = HashMap<TypeId, Box<dyn ComponentVec>>;
/// A system, represented as a boxed closure.
type System = Box<dyn FnMut(&World)>;

/// A container to store all components and systems in use at any point.
#[derive(Default)]
pub struct World {
    /// The storages for all [`Component`]s in the world.
    components: ComponentMap,
    /// The current number of entities stored.
    entities: usize,
    /// The registered systems.
    systems: RefCell<Vec<System>>,
}

impl World {
    /// Constructs a new, empty `World`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates storage in the `World` for a specific [`Component`] type.
    pub fn register<T: Component>(&mut self) {
        assert_eq!(
            self.entities, 0,
            "Attempted to register a new component on an active World"
        );
        self.components.insert(
            TypeId::of::<T>(),
            Box::new(RefCell::new(SparseVec::<T>::new())),
        );
    }

    /// Inserts a new entity using a closure that populates the entity's components.
    pub fn add_entity(&mut self, f: impl FnOnce(&mut EntityBuilder) -> &mut EntityBuilder) -> Entity {
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

    /// Returns an immutable reference to a specific component vector.
    pub fn get<T: Component>(&self) -> Ref<SparseVec<T>> {
        self.cell().borrow()
    }

    /// Returns a mutable reference to a specific component vector.
    pub fn get_mut<T: Component>(&self) -> RefMut<SparseVec<T>> {
        self.cell().borrow_mut()
    }

    /// Adds a system to the world.
    pub fn system<T: FnMut(&World) + 'static>(&mut self, system: T) {
        self.systems.get_mut().push(Box::new(system))
    }

    /// Calls all systems in order.
    pub fn tick(&mut self) {
        let mut systems = self.systems.borrow_mut();
        for system in systems.iter_mut() {
            system(self)
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
    /// Adds a component to the entity.
    pub fn with_component<T: Component + 'static>(&mut self, component: T) -> &mut Self {
        self.0.insert(TypeId::of::<T>(), Box::new(component));
        self
    }
}

/// A marker trait indicating that a type may be used as component data.
pub trait Component: 'static {}

/// A trait for dynamically typed, dynamically borrow checked component vectors.
trait ComponentVec {
    /// Returns a reference to self as an [`Any`] object.
    fn as_any(&self) -> &dyn Any;
    /// Returns a mutable reference to self as an [`Any`] object.
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// Inserts a component into the vector.
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
