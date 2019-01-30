use amethyst;

use amethyst::{
    assets::{AssetStorage, Completion, Handle, Prefab, ProgressCounter},
    core::{
        nalgebra::{Translation3, UnitQuaternion, Vector3,},
        transform::{Transform},
        Named, Parent,
    },
    ecs::{NullStorage},
    ecs::prelude::{
        Component, Entity, Join, Write,
    },
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        AmbientColor, Camera, DirectionalLight, Light, Mesh, MeshData,
        Projection, Rgba, VirtualKeyCode,
    },
    ui::{UiCreator, UiFinder},
};
use amethyst_gltf::{GltfPrefab};
use std::f32::consts::PI;

use crate::graphics::*;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}


fn init_grid(world: &mut World, assets: Assets) -> Entity {
    let transform = Transform::default();

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
    let transform = Transform::default();

    let player = world
        .create_entity()
        .with(transform.clone())
        .with(Player)
        .build();

    let tank_gltf_prefab_handle = assets.tank_gltf.clone();
    // let gltf_prefab_storage: AssetStorage<Prefab<GltfPrefab>> = world.read_resource();
    // let tank_gltf_prefab: Prefab<GltfPrefab> = gltf_prefab_storage.get(tank_gltf_prefab_handle);
    // println!("tank gltf prefab has {} entities", tank_gltf_prefab.len());
    /*
    for gltf_entity in tank_gltf_prefab.entities() {
        if let Some(name) = gltf_entity.name {
            println!("gltf_entity {} is named {}", gltf_entity, name);
        }
    }
    // */

    {
        let meshes = world.read_storage::<Handle<Mesh>>();
        let names = world.read_storage::<Named>();
        println!("init_player:: exec'ing to list Named:");
        for (name) in (&names).join() {
            println!("+ found Named entity! name={}", name.name);
        }
        println!("init_player:: exec'ing to list Named, Mesh:");
        for (name, mesh) in (&names, &meshes).join() {
            println!("+ found Named,Mesh! name={}", name.name);
        }
    }

    // front of the model in Blender is -Y;
    // export from blender with Z foward, Y up.
    let tank_mesh = assets.tank.clone();
    let tank_gltf_mesh = assets.tank_gltf.clone();
    let _tank_entity = world
        .create_entity()
        .with(transform.clone())
        // .with(tank_mesh)
        .with(tank_gltf_mesh)
        // .with(assets.green_material.clone())
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
    world.exec(
        |mut color: Write<'_, AmbientColor>| {
            color.0 = [0.5; 4].into();
        },
    );

    // let position = Translation3::new(15.0, 6.0, 0.0);
    // let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    // let rotation = UnitQuaternion::new_normalize(rotation);
    // let scale = Vector3::new(1.0, 1.0, 1.0);
    // let transform = Transform::new(position, rotation, scale);

    let direction_light = DirectionalLight {
        color: Rgba(0.5, 0.5, 0.5, 1.0),
        direction: [-0.1, -0.1, 1.0]
    };
    world
        .create_entity()
        .with(Light::Directional(direction_light))
        .build();
}

#[derive(Default)]
pub struct Loading {
    progress: ProgressCounter,
    // prefab: Option<Handle<Prefab<MyPrefabData>>>,
}

pub struct Main {
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
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Exit if user hits Escape or closes the window
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}
