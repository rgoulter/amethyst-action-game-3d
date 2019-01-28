use amethyst;

use amethyst::{
    assets::{
        Completion, Handle, HotReloadBundle, Prefab, PrefabLoader, PrefabLoaderSystem,
        ProgressCounter, RonFormat, Loader,
    },
    core::{
        nalgebra::{UnitQuaternion, Vector3, Translation3, Quaternion},
        timing::Time,
        transform::{Transform, TransformBundle},
        Parent,
    },
    ecs::{NullStorage},
    ecs::prelude::{Component, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputBundle, InputHandler},
    prelude::*,
    renderer::{AmbientColor, Camera, DrawShaded, ElementState, Light, PosNormTex, VirtualKeyCode, Projection, MeshHandle, ObjFormat, Material, MaterialDefaults, PointLight, Rgba, DirectionalLight, Shape, MeshData, ShapeUpload,},
    ui::{UiBundle, UiCreator, UiFinder, UiText},
    utils::{
        application_root_dir,
        fps_counter::{FPSCounter, FPSCounterBundle},
        scene::BasicScenePrefab,
    },
    Error,
};
use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystem};
use std::f32::consts::PI;

// type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

#[derive(Default)]
struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut transforms, input, time): Self::SystemData) {
        let turn = input.axis_value("player_rotation").unwrap();
        let y_move = input.axis_value("player_acceleration").unwrap();

        let dt = time.delta_seconds();
        for (_, transform) in (&players, &mut transforms).join() {
            transform.move_forward(y_move as f32 * 2.0 * dt);
            // transform.move_local(Vector3::new(0.0, y_move as f32 * 2.0 * dt, 0.0));
            transform.yaw_local(turn as f32 * PI / 2.0 * dt);
            // transform.rotate_local(Vector3::z_axis(), turn as f32 * PI / 2.0 * dt);
        }
    }
}

#[derive(Clone)]
pub struct Assets {
    tank: MeshHandle,
    grid: MeshHandle,
    green_material: Material,
    grey_material: Material,
}

pub fn load_assets(world: &mut World, progress: &mut ProgressCounter) -> () {
    let assets = {
        let mesh_storage = world.read_resource();
        let tex_storage = world.read_resource();
        let mat_defaults = world.read_resource::<MaterialDefaults>();
        let loader = world.read_resource::<Loader>();


        let tank = {
            let p1: &mut ProgressCounter = progress;
            loader.load(
            "mesh/tank.obj",
            ObjFormat,
            (),
            p1,
            &mesh_storage,
        ) };

        let green_texture = {
            let p2: &mut ProgressCounter = progress;
            loader.load_from_data(
            [0.3, 1.0, 0.3, 1.0].into(),
            p2,
            &tex_storage
        ) };
        let green_material = Material {
            albedo: green_texture.clone(),
            // ambient_occlusion: green_texture.clone(),
            ..mat_defaults.0.clone()
        };

        let grey_texture = {
            let p2: &mut ProgressCounter = progress;
            loader.load_from_data(
            [0.3, 0.3, 0.3, 1.0].into(),
            p2,
            &tex_storage
        ) };
        let grey_material = Material {
            albedo: grey_texture.clone(),
            // ambient_occlusion: grey_texture.clone(),
            ..mat_defaults.0.clone()
        };

        let divisions = None;
        let grid_shape = Shape::Plane(divisions);
        let scale = Some((0.5, 0.5, 0.5));
        let grid_mesh_data = grid_shape.generate::<Vec<PosNormTex>>(scale);
        let grid = {
            let grid_progress: &mut ProgressCounter = progress;
            loader.load_from_data(grid_mesh_data, grid_progress, &mesh_storage)
        };

        Assets {
            tank,
            grid,
            green_material,
            grey_material,
        }
    };

    world.add_resource(assets);
}

fn init_grid(world: &mut World, assets: Assets) -> Entity {
    let mut transform = Transform::default();

    let grid_root = world
        .create_entity()
        .with(transform)
        .build();

    let grid_num_rows = 8;
    let grid_num_cols = 8;

    for x in 0..grid_num_rows {
        for y in 0..grid_num_rows {
            if (x + y) % 2 == 1 {
                let mut transform = Transform::default();
                let tx = -0.5 + (x as f32 - (grid_num_rows / 2) as f32);
                let ty = 0.5 + (y as f32 - (grid_num_cols / 2) as f32);
                // println!("make grid at {}, {} for {}, {}", tx, ty, x, y);
                transform.set_xyz(tx, 0.0, ty);
                transform.set_xyz(tx, 0.0, ty);
                transform.rotate_local(Vector3::x_axis(), -PI / 2.0);

                let material = assets.grey_material.clone();
                let grid_mesh = assets.grid.clone();
                world
                    .create_entity()
                    .with(transform)
                    .with(grid_mesh)
                    .with(material)
                    .with(Parent { entity: grid_root })
                    .build();
            }
        }
    }

    grid_root
}

fn init_player(world: &mut World, assets: Assets) -> Entity {
    let mut transform = Transform::default();

    let player = world
        .create_entity()
        .with(transform.clone())
        .with(Player)
        .build();

    // front of the model in Blender is -Y;
    // export from blender with Z foward, Y up.
    let tank_mesh = assets.tank.clone();
    let tank_entity = world
        .create_entity()
        .with(transform.clone())
        .with(tank_mesh)
        .with(assets.green_material.clone())
        .with(Parent { entity: player })
        .build();

    player
}

// fn init_camera(world: &mut World, parent: Entity) {
fn init_camera(world: &mut World) {
    // let position = Translation3::new(0.0, -20.0, 10.0);
    let position = Translation3::new(0.0, 15.0, 15.0);
    // let rotation = UnitQuaternion::from_euler_angles(0.7933533, 0.6087614, 0.0);
    // let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    // let rotation = UnitQuaternion::new_normalize(rotation);
    let rotation = UnitQuaternion::from_euler_angles(
        -PI / 4.0,
        0.0,
        0.0
    );

    let scale = Vector3::new(1.0, 1.0, 1.0);
    let transform = Transform::new(position, rotation, scale);
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(
          1.3,
          1.0471975512,
        )))
        // .with(Parent { entity: parent })
        .with(transform)
        .build();
}

fn init_lighting(world: &mut World) {
    let position = Translation3::new(15.0, 6.0, 0.0);
    let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    let rotation = UnitQuaternion::new_normalize(rotation);
    let scale = Vector3::new(1.0, 1.0, 1.0);
    let transform = Transform::new(position, rotation, scale);
    let point_light = PointLight {
        color: Rgba(1.0, 1.0, 1.0, 1.0),
        intensity: 50.0,
        radius: 1000.0,
        smoothness: 1.0,
    };
    // world
    //     .create_entity()
    //     .with(Light::Point(point_light))
    //     .with(transform.clone())
    //     .build();

    world.exec(
        |mut color: Write<'_, AmbientColor>| {
            color.0 = [0.5; 4].into();
        },
    );


    let direction_light = DirectionalLight {
        color: Rgba(0.5, 0.5, 0.5, 1.0),
        // direction: [0.0, 0.05, 0.03]
        direction: [-0.1, -0.1, 1.0]
    };
    world
        .create_entity()
        .with(Light::Directional(direction_light))
        .with(transform.clone())
        .build();
}

#[derive(Default)]
struct Loading {
    progress: ProgressCounter,
    // prefab: Option<Handle<Prefab<MyPrefabData>>>,
}

struct Main {
    // scene: Handle<Prefab<MyPrefabData>>,
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // self.prefab = Some(data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/renderable.ron", RonFormat, (), &mut self.progress)
        // }));

        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress);
        });

        load_assets(data.world, &mut self.progress);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Failed => {
                println!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded ({}/{}), swapping state",
                         self.progress.num_finished(),
                         self.progress.num_assets());
                if let Some(entity) = data
                    .world
                    .exec(|finder: UiFinder<'_>| finder.find("loading"))
                {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(Main {
                    // XXX no need to do anything here;
                    // we write assets as resource
                    // scene: self.prefab.as_ref().unwrap().clone(),
                }))
            }
            Completion::Loading => {
                Trans::None
            }
        }
    }
}

impl SimpleState for Main {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let assets = world.read_resource::<Assets>().clone();

        world.register::<MeshData>();
        init_grid(world, assets.clone());
        init_player(world, assets.clone());
        init_camera(world);
        init_lighting(world);
        // world.create_entity().with(self.scene.clone()).build();

    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let w = data.world;
        if let StateEvent::Window(event) = &event {
            // Exit if user hits Escape or closes the window
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
            match get_key(&event) {
                Some((VirtualKeyCode::R, ElementState::Pressed)) => {
                    w.exec(|mut state: Write<'_, DemoState>| {
                        state.light_color = [0.8, 0.2, 0.2, 1.0];
                    });
                }
                Some((VirtualKeyCode::G, ElementState::Pressed)) => {
                    w.exec(|mut state: Write<'_, DemoState>| {
                        state.light_color = [0.2, 0.8, 0.2, 1.0];
                    });
                }
                Some((VirtualKeyCode::B, ElementState::Pressed)) => {
                    w.exec(|mut state: Write<'_, DemoState>| {
                        state.light_color = [0.2, 0.2, 0.8, 1.0];
                    });
                }
                // Some((VirtualKeyCode::W, ElementState::Pressed)) => {
                //     w.exec(|mut state: Write<'_, DemoState>| {
                //         state.light_color = [1.0, 1.0, 1.0, 1.0];
                //     });
                // }
                Some((VirtualKeyCode::L, ElementState::Pressed)) => {
                    w.exec(
                        |(mut state, mut color): (
                            Write<'_, DemoState>,
                            Write<'_, AmbientColor>,
                        )| {
                            if state.ambient_light {
                                state.ambient_light = false;
                                color.0 = [0.0; 3].into();
                            } else {
                                state.ambient_light = true;
                                color.0 = [0.01; 3].into();
                            }
                        },
                    );
                }
                // Some((VirtualKeyCode::D, ElementState::Pressed)) => {
                //     w.exec(
                //         |(mut state, mut lights): (
                //             Write<'_, DemoState>,
                //             WriteStorage<'_, Light>,
                //         )| {
                //             if state.directional_light {
                //                 state.directional_light = false;
                //                 for light in (&mut lights).join() {
                //                     if let Light::Directional(ref mut d) = *light {
                //                         d.color = [0.0; 4].into();
                //                     }
                //                 }
                //             } else {
                //                 state.directional_light = true;
                //                 for light in (&mut lights).join() {
                //                     if let Light::Directional(ref mut d) = *light {
                //                         d.color = [0.2; 4].into();
                //                     }
                //                 }
                //             }
                //         },
                //     );
                // }
                Some((VirtualKeyCode::P, ElementState::Pressed)) => {
                    w.exec(|mut state: Write<'_, DemoState>| {
                        if state.point_light {
                            state.point_light = false;
                            state.light_color = [0.0; 4].into();
                        } else {
                            state.point_light = true;
                            state.light_color = [1.0; 4].into();
                        }
                    });
                }
                _ => (),
            }
        }
        Trans::None
    }
}

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
        // .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_bundle(
            InputBundle::<String, String>::new()
                .with_bindings_from_file(input_config_path)?,
        )?
        .with(MovementSystem, "movement", &[])
        .with::<ExampleSystem>(ExampleSystem::default(), "example_system", &[])
        .with_bundle(TransformBundle::new().with_dep(&["example_system"]))?
        // .with(
        //     GltfSceneLoaderSystem::default(),
        //     "gltf_loader",
        //     &["scene_loader"], // This is important so that entity instantiation is performed in a single frame.
        //     )
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(FPSCounterBundle::default())?
        .with_basic_renderer(display_config_path, DrawShaded::<PosNormTex>::new(), true)?;
    let mut game = Application::build(resources_directory, Loading::default())?.build(game_data)?;
    game.run();
    Ok(())
}

struct DemoState {
    light_angle: f32,
    light_color: [f32; 4],
    ambient_light: bool,
    point_light: bool,
    directional_light: bool,
    camera_angle: f32,
}

impl Default for DemoState {
    fn default() -> Self {
        DemoState {
            light_angle: 0.0,
            light_color: [1.0; 4],
            ambient_light: true,
            point_light: true,
            directional_light: true,
            camera_angle: 0.0,
        }
    }
}

#[derive(Default)]
struct ExampleSystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for ExampleSystem {
    type SystemData = (
        WriteStorage<'a, Light>,
        Read<'a, Time>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
        Write<'a, DemoState>,
        WriteStorage<'a, UiText>,
        Read<'a, FPSCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut lights, time, camera, mut transforms, mut state, mut ui_text, fps_counter, finder) =
            data;
        if let None = self.fps_display {
            if let Some(fps_entity) = finder.find("fps_text") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }
}
