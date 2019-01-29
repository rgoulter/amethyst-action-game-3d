use amethyst;

use amethyst::{
    assets::{HotReloadBundle},
    core::transform::{TransformBundle},
    input::{InputBundle},
    prelude::*,
    renderer::{DrawShaded, PosNormTex},
    ui::{UiBundle},
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
mod systems;

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
        .with_basic_renderer(display_config_path, DrawShaded::<PosNormTex>::new(), true)?;
    let mut game = Application::build(resources_directory, Loading::default())?.build(game_data)?;
    game.run();
    Ok(())
}
