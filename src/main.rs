extern crate amethyst;

use amethyst::{
    ecs::{Component, Join, NullStorage, ReadStorage, System},
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

mod managed;

use crate::managed::Managed;
use crate::managed::ManagedEntities;
use crate::managed::ManagedWorld;

#[derive(Default)]
pub struct Alive;

impl Component for Alive {
    type Storage = NullStorage<Self>;
}

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
    type SystemData = (Managed<'a>, ReadStorage<'a, Alive>);

    fn run(
        &mut self,
        ((managed_resource, entities, mut managed_storage), alive): Self::SystemData,
    ) {
        println!("ExampleSystem::run");

        let mut wrong_count = 0;
        for _ in entities.join() {
            wrong_count = wrong_count + 1;
        }
        println!("Number of entities wrong {}", wrong_count);

        let mut alive_count = 0;
        for (_, _) in (&entities, &alive).join() {
            alive_count = alive_count + 1;
        }
        println!("Number of entities alive {}", alive_count);

        let mut entity_count = 0;
        for (_, _) in (&entities, &alive).join() {
            entity_count = entity_count + 1;
        }
        println!("Number of entities {}", entity_count);
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
