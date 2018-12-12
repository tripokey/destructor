extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};
use amethyst::ecs::{Entity, EntityBuilder};

#[derive(Default, Debug)]
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

struct ManagedState;

impl SimpleState for ManagedState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.initialize_managed_world();
        data.world.push_state();
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.pop_state();
    }
}

trait ManagedWorld {
    fn initialize_managed_world(&mut self);
    fn create_managed_entity(&self) -> EntityBuilder;
    fn push_state(&self);
    fn pop_state(&mut self);
}

impl ManagedWorld for World {
    fn initialize_managed_world(&mut self) {
        self.res.entry::<ManagedResource>().or_insert_with(|| ManagedResource::default());
    }

    fn create_managed_entity(&self) -> EntityBuilder {
        self.write_resource::<ManagedResource>().create_entity(self)
    }

    fn push_state(&self) {
        self.write_resource::<ManagedResource>().push_state();
    }

    fn pop_state(&mut self) {
        let entities = self.write_resource::<ManagedResource>().pop_state();
        self.delete_entities(&entities).unwrap_or(());
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new("./", ManagedState, game_data)?;

    game.run();

    Ok(())
}
