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
    },
    ecs::{NullStorage},
    ecs::prelude::{Component, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{get_key, is_close_requested, is_key_down, InputBundle, InputHandler},
    prelude::*,
    renderer::{AmbientColor, Camera, DrawShaded, ElementState, Light, PosNormTex, VirtualKeyCode, Projection, MeshHandle, ObjFormat, Material, MaterialDefaults, PointLight, Rgba,},
    ui::{UiBundle, UiCreator, UiFinder, UiText},
    utils::{
        application_root_dir,
        fps_counter::{FPSCounter, FPSCounterBundle},
        scene::BasicScenePrefab,
    },
    Error,
};
use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystem};

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
    );

    fn run(&mut self, (players, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, transform) in (&players, &mut transforms).join() {
            transform.translate_x(x_move as f32 * 5.0);
            transform.translate_y(y_move as f32 * 5.0);
        }
    }
}

#[derive(Clone)]
pub struct Assets {
    tank: MeshHandle,
    green_material: Material,
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
            progress,
            &tex_storage
        ) };
        let green_material = Material {
            albedo: green_texture,
            ..mat_defaults.0.clone()
        };

        Assets {
            tank,
            green_material,
        }
    };

    world.add_resource(assets);
}

fn init_player(world: &mut World, assets: Assets) -> Entity {
// fn init_player(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    let tank_mesh = assets.tank.clone();
    world
        .create_entity()
        .with(transform)
        // .with(Player)
        .with(tank_mesh)
        .with(assets.green_material.clone())
        .build()
}

// fn init_camera(world: &mut World, parent: Entity) {
fn init_camera(world: &mut World) {
    // let position = Translation3::new(0.0, -20.0, 10.0);
    let position = Translation3::new(0.0, -20.0, 10.0);
    // let rotation = UnitQuaternion::from_euler_angles(0.7933533, 0.6087614, 0.0);
    let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    let rotation = UnitQuaternion::new_normalize(rotation);
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
    let position = Translation3::new(15.0, 0.0, 6.0);
    let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    let rotation = UnitQuaternion::new_normalize(rotation);
    let scale = Vector3::new(1.0, 1.0, 1.0);
    let transform = Transform::new(position, rotation, scale);
    let point_light = PointLight {
        color: Rgba(1.0, 1.0, 1.0, 1.0),
        intensity: 50.0,
        radius: 100.0,
        smoothness: 1.0,
    };
    world
        .create_entity()
        .with(Light::Point(point_light))
        .with(transform)
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

        init_player(world, assets);
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
                Some((VirtualKeyCode::W, ElementState::Pressed)) => {
                    w.exec(|mut state: Write<'_, DemoState>| {
                        state.light_color = [1.0, 1.0, 1.0, 1.0];
                    });
                }
                Some((VirtualKeyCode::A, ElementState::Pressed)) => {
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
                Some((VirtualKeyCode::D, ElementState::Pressed)) => {
                    w.exec(
                        |(mut state, mut lights): (
                            Write<'_, DemoState>,
                            WriteStorage<'_, Light>,
                        )| {
                            if state.directional_light {
                                state.directional_light = false;
                                for light in (&mut lights).join() {
                                    if let Light::Directional(ref mut d) = *light {
                                        d.color = [0.0; 4].into();
                                    }
                                }
                            } else {
                                state.directional_light = true;
                                for light in (&mut lights).join() {
                                    if let Light::Directional(ref mut d) = *light {
                                        d.color = [0.2; 4].into();
                                    }
                                }
                            }
                        },
                    );
                }
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

    let game_data = GameDataBuilder::default()
        // .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
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
        .with_basic_renderer(display_config_path, DrawShaded::<PosNormTex>::new(), true)?
        .with_bundle(InputBundle::<String, String>::new())?;
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
        // let light_angular_velocity = -1.0;
        // let light_orbit_radius = 15.0;
        // let light_z = 6.0;

        // let camera_angular_velocity = 0.1;

        // state.light_angle += light_angular_velocity * time.delta_seconds();
        // state.camera_angle += camera_angular_velocity * time.delta_seconds();

        // let delta_rot = UnitQuaternion::from_axis_angle(
        //     &Vector3::z_axis(),
        //     camera_angular_velocity * time.delta_seconds(),
        // );
        // for (_, transform) in (&camera, &mut transforms).join() {
        //     // Append the delta rotation to the current transform.
        //     *transform.isometry_mut() = delta_rot * transform.isometry();
        // }
        for (_, transform) in (&camera, &mut transforms).join() {
            println!("Camera Transform: translation: {}, rotation: {}", transform.translation(), transform.rotation());
        }

        // for (point_light, transform) in
        //     (&mut lights, &mut transforms)
        //         .join()
        //         .filter_map(|(light, transform)| {
        //             if let Light::Point(ref mut point_light) = *light {
        //                 Some((point_light, transform))
        //             } else {
        //                 None
        //             }
        //         })
        // {
        //     transform.set_xyz(
        //         light_orbit_radius * state.light_angle.cos(),
        //         light_orbit_radius * state.light_angle.sin(),
        //         light_z,
        //     );

        //     point_light.color = state.light_color.into();
        // }

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
