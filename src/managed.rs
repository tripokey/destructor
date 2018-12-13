use amethyst::prelude::*;
use amethyst::ecs::{
    Join, Component, EntityBuilder, DenseVecStorage, ReadStorage, Entities, Entity, Read, WriteStorage,
    world::EntityResBuilder,
};

pub struct ManagedComponent {
    owning_state: usize,
}

impl Component for ManagedComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ManagedResource {
    active_state: usize,
}

impl ManagedResource {
    fn push_state(&mut self) {
        self.active_state += 1;
    }

    fn pop_state(&mut self) -> usize {
        assert!(self.active_state > 0);
        let prev_state = self.active_state;
        self.active_state -= 1;

        prev_state
    }
}

pub trait ManagedWorld {
    fn create_managed_entity(&mut self) -> EntityBuilder;
    fn push_state(&mut self);
    fn pop_state(&mut self);
}

impl ManagedWorld for World {
    fn create_managed_entity(&mut self) -> EntityBuilder {
        self.register::<ManagedComponent>();
        let active_state = self.read_resource::<ManagedResource>().active_state;
        self.create_entity().with(ManagedComponent { owning_state: active_state })
    }

    fn push_state(&mut self) {
        self.res.entry::<ManagedResource>().or_insert_with(|| ManagedResource{active_state: 0}).push_state();
    }

    fn pop_state(&mut self) {
        let stale_state = self.write_resource::<ManagedResource>().pop_state();
        self.exec(|(owning_states, entities): (ReadStorage<ManagedComponent>, Entities)| {
            for (owning_state, entity) in (&owning_states, &*entities).join() {
                if owning_state.owning_state == stale_state {
                    entities.delete(entity).expect("Failed to delete entity");
                }
            }
        });
    }
}

pub type Managed<'a> = (
    Read<'a, ManagedResource>,
    Entities<'a>,
    WriteStorage<'a, ManagedComponent>
);

pub trait ManagedEntities {
    fn create_managed(&self, storage: &mut WriteStorage<ManagedComponent>) -> Entity;
    fn build_managed_entity(&self, storage: &mut WriteStorage<ManagedComponent>) -> EntityResBuilder;
}

impl ManagedEntities for (Read<'_, ManagedResource>, Entities<'_>) {
    fn create_managed(&self, storage: &mut WriteStorage<ManagedComponent>) -> Entity {
        self.build_managed_entity(storage).build()
    }

    fn build_managed_entity(&self, storage: &mut WriteStorage<ManagedComponent>) -> EntityResBuilder {
        let (managed_resource, entities) = self;

        entities.build_entity().with(ManagedComponent { owning_state: managed_resource.active_state }, storage)
    }
}

impl ManagedEntities for (Entities<'_>, Read<'_, ManagedResource>) {
    fn create_managed(&self, storage: &mut WriteStorage<ManagedComponent>) -> Entity {
        self.build_managed_entity(storage).build()
    }

    fn build_managed_entity(&self, storage: &mut WriteStorage<ManagedComponent>) -> EntityResBuilder {
        let (entities, managed_resource) = self;

        entities.build_entity().with(ManagedComponent { owning_state: managed_resource.active_state }, storage)
    }
}