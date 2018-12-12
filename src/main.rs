extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

mod managed;

use crate::managed::ManagedWorld;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.initialize_managed_world();
        data.world.push_state();
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.pop_state();
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

    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
