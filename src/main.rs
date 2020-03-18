use amethyst;
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::{Application, GameDataBuilder},
    renderer::{
        plugins::{
            RenderDebugLines,
            RenderPbr3D,
            RenderToWindow,
        },
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{
        fps_counter::{FpsCounterBundle},
        application_root_dir,
    },
    Error,
};
use amethyst_gltf::GltfSceneLoaderSystemDesc;

use crate::systems::{
    DebugSystem,
    MovementSystem,
    ReplaceMaterialSystem,
    UISystem,
};
use crate::states::{
    MainMenu,
};

mod assets;
mod level;
mod player;
mod replace_material;
mod sprite_grid;
mod states;
mod systems;
mod utils;

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // Add our meshes directory to the asset loader.
    let resources_directory = app_root.join("assets");

    let display_config_path = app_root.join("resources").join("display_config.ron");

    let input_config_path = app_root.join("resources").join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(input_config_path)?
        )?
        .with_system_desc(
            GltfSceneLoaderSystemDesc::default(),
            "gltf_loader",
            &[]
        )
        .with_system_desc(MovementSystem, "movement", &[])
        .with_system_desc(UISystem::default(), "game_ui_system", &[])
        .with_system_desc(DebugSystem::default(), "game_debug_system", &[])
        .with_system_desc(ReplaceMaterialSystem::default(), "replace_material_system", &[])
        .with_bundle(TransformBundle::new().with_dep(&[]))?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderPbr3D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default())
        )?;

    let init_state = MainMenu::default();

    let mut game = Application::build(resources_directory, init_state)?
        .build(game_data)?;

    game.run();
    Ok(())
}
