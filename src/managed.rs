use amethyst::ecs::{world::EntityResBuilder, Entities, Entity, EntityBuilder, Read, WriteStorage};
use amethyst::prelude::*;
use amethyst::utils::removal::{exec_removal, Removal};

#[derive(Default)]
pub struct ManagedResource {
    active_state: usize,
}

impl ManagedResource {
    fn push_state(&mut self) {
        println!(
            "push_state: {}->{}",
            self.active_state,
            self.active_state + 1
        );
        self.active_state += 1;
    }

    fn pop_state(&mut self) -> usize {
        assert!(self.active_state > 0);
        println!(
            "pop_state: {}->{}",
            self.active_state,
            self.active_state - 1
        );
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
        let active_state = self.read_resource::<ManagedResource>().active_state;
        println!("create_managed_entity for state {}", active_state);
        self.create_entity().with(Removal::new(active_state))
    }

    fn push_state(&mut self) {
        println!("Entering new state");
        self.res
            .entry::<ManagedResource>()
            .or_insert_with(|| ManagedResource { active_state: 0 })
            .push_state();
    }

    fn pop_state(&mut self) {
        let stale_state = self.write_resource::<ManagedResource>().pop_state();
        println!("Leaving state {}", stale_state);
        exec_removal(
            &self.entities(),
            &self.read_storage::<Removal<usize>>(),
            stale_state,
        );
        self.maintain();
    }
}

pub type Managed<'a> = (
    Read<'a, ManagedResource>,
    Entities<'a>,
    WriteStorage<'a, Removal<usize>>,
);

pub trait ManagedEntities {
    fn create_managed(&self, storage: &mut WriteStorage<Removal<usize>>) -> Entity;
    fn build_managed_entity(&self, storage: &mut WriteStorage<Removal<usize>>) -> EntityResBuilder;
}

impl ManagedEntities for (Read<'_, ManagedResource>, Entities<'_>) {
    fn create_managed(&self, storage: &mut WriteStorage<Removal<usize>>) -> Entity {
        self.build_managed_entity(storage).build()
    }

    fn build_managed_entity(&self, storage: &mut WriteStorage<Removal<usize>>) -> EntityResBuilder {
        let (managed_resource, entities) = self;

        println!(
            "Create system entity for state {}",
            managed_resource.active_state
        );
        entities
            .build_entity()
            .with(Removal::new(managed_resource.active_state), storage)
    }
}

impl ManagedEntities for (Entities<'_>, Read<'_, ManagedResource>) {
    fn create_managed(&self, storage: &mut WriteStorage<Removal<usize>>) -> Entity {
        self.build_managed_entity(storage).build()
    }

    fn build_managed_entity(&self, storage: &mut WriteStorage<Removal<usize>>) -> EntityResBuilder {
        let (entities, managed_resource) = self;

        println!(
            "Create system entity for state {}",
            managed_resource.active_state
        );
        entities
            .build_entity()
            .with(Removal::new(managed_resource.active_state), storage)
    }
}
