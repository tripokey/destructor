use amethyst::prelude::*;
use amethyst::ecs::{Entity, EntityBuilder};

#[derive(Default)]
struct ManagedResource {
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
}

pub trait ManagedWorld {
    fn initialize_managed_world(&mut self);
    fn create_managed_entity(&mut self) -> EntityBuilder;
    fn push_state(&mut self);
    fn pop_state(&mut self);
}

impl ManagedWorld for World {
    fn initialize_managed_world(&mut self) {
        self.res.entry::<ManagedResource>().or_insert_with(|| ManagedResource::default());
    }

    fn create_managed_entity(&mut self) -> EntityBuilder {
        self.write_resource::<ManagedResource>().create_entity(self)
    }

    fn push_state(&mut self) {
        self.write_resource::<ManagedResource>().push_state();
    }

    fn pop_state(&mut self) {
        let entities = self.write_resource::<ManagedResource>().pop_state();
        self.delete_entities(&entities).unwrap_or(());
    }
}