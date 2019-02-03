use amethyst;

use amethyst::{
    assets::{HotReloadBundle,},
    core::transform::{TransformBundle},
    input::{InputBundle},
    prelude::*,
    renderer::{
        pipe::*,
        ALPHA,
        ColorMask,
        DepthMode,
        DisplayConfig,
        DrawPbmSeparate,
        RenderBundle,
    },
    ui::{DrawUi, UiBundle},
    utils::{
        application_root_dir,
        fps_counter::{FPSCounterBundle},
    },
    Error,
};
use amethyst_gltf::GltfSceneLoaderSystem;

use crate::systems::*;
use crate::game::*;

mod game;
mod graphics;
mod grid_of_sprites;
mod level;
mod player;
mod systems;
mod utils;

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    // Add our meshes directory to the asset loader.
    let resources_directory = format!("{}/assets/", app_root);

     let display_config_path = format!(
         "{}/resources/display_config.ron",
         app_root
     );

     let input_config_path = format!(
         "{}/resources/input.ron",
         app_root
     );

    let display_config = DisplayConfig::load(display_config_path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1024.0)
            .with_pass(DrawPbmSeparate::new()
                 .with_transparency(
                     ColorMask::all(),
                     ALPHA,
                     Some(DepthMode::LessEqualWrite)
                 ))
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new()
                .with_bindings_from_file(input_config_path)?,
        )?
        .with(MovementSystem, "movement", &[])
        .with::<UISystem>(UISystem::default(), "game_ui_system", &[])
        .with(GltfSceneLoaderSystem::default(), "gltf_loader", &[])
        .with_bundle(TransformBundle::new().with_dep(&[]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config))
                                  .with_sprite_sheet_processor())?;

    let init_state = Loading::default();

    let mut game = Application::build(resources_directory, init_state)?
        .build(game_data)?;

    game.run();
    Ok(())
}
