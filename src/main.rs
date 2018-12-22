extern crate amethyst;

use amethyst::{
    ecs::{Join, System},
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

mod managed;

use crate::managed::Managed;
use crate::managed::ManagedEntities;
use crate::managed::ManagedWorld;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("Example::on_start");
        data.world.push_state();
        data.world.create_managed_entity().build();
        data.world.create_managed_entity().build();
        data.world.create_entity().build();
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        println!("Example::on_stop");
        data.world.pop_state();
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        println!("Example::update");
        Trans::Switch(Box::new(Example))
    }
}

pub struct ExampleSystem;

impl<'a> System<'a> for ExampleSystem {
    type SystemData = Managed<'a>;

    fn run(&mut self, (managed_resource, entities, mut managed_storage): Self::SystemData) {
        println!("ExampleSystem::run");
        for entity in entities.join() {
            if entities.is_alive(entity) {
                println!("{:?}", entity);
            }
        }
        (managed_resource, entities).create_managed(&mut managed_storage);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with(ExampleSystem, "example_system", &[]);

    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
