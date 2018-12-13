use amethyst::prelude::*;
use amethyst::ecs::{Entity, EntityBuilder, Write, world::{EntitiesRes, EntityResBuilder}};

#[derive(Default)]
pub struct ManagedResource {
    states: Vec<Vec<Entity>>,
}

impl ManagedResource {
    fn push_state(&mut self) {
        self.states.push(Vec::default());
    }

    fn pop_state(&mut self) -> Vec<Entity> {
        self.states.pop().expect("There is no active state")
    }

    fn create_entity<'a>(&mut self, world: &'a World) -> EntityBuilder<'a> {
        let builder = world.create_entity_unchecked();
        self.states.last_mut().expect("There is no active state").push(builder.entity);
        builder
    }

    fn push_entity(&mut self, entity: Entity) {
        self.states.last_mut().expect("There is no active state").push(entity);
    }
}

pub trait ManagedWorld {
    fn create_managed_entity(&mut self) -> EntityBuilder;
    fn push_state(&mut self);
    fn pop_state(&mut self);
}

impl ManagedWorld for World {
    fn create_managed_entity(&mut self) -> EntityBuilder {
        self.write_resource::<ManagedResource>().create_entity(self)
    }

    fn push_state(&mut self) {
        self.res.entry::<ManagedResource>().or_insert_with(|| ManagedResource::default()).push_state();
    }

    fn pop_state(&mut self) {
        let entities = self.write_resource::<ManagedResource>().pop_state();
        self.delete_entities(&entities).unwrap_or(());
    }
}

// TODO implement Managed exactly like Entities so that it can be used as Read
// TODO implement world managed_maintain that first handles all the managed entities created
// and then goes on to call world::maintain
/// A wrapper for the Managed resource, needs the Entities resource to create managed entities.
/// type SystemData = (Entities<'a>, Managed<'a>);
pub type Managed<'a> = Write<'a, ManagedResource>;

impl ManagedResource {
    pub fn create_managed(&mut self, entities: &EntitiesRes) -> Entity {
        let entity = entities.create();
        self.push_entity(entity);

        entity
    }

    // TODO implement create_managed_iter
    //pub fn create_managed_iter(&mut self, entities: &EntitiesRes) -> CreateManagedIter

    pub fn build_managed_entity<'a>(&mut self, entities: &'a EntitiesRes) -> EntityResBuilder<'a> {
        let builder = entities.build_entity();
        self.push_entity(builder.entity);

        builder
    }
}