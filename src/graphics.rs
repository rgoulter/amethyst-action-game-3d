use amethyst;

use amethyst::{
    assets::{Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{
        Material, MaterialDefaults, MeshHandle, ObjFormat,
        PosNormTex, Shape,
    },
};
use amethyst_gltf::{
    GltfSceneAsset,
    GltfSceneFormat,
    GltfSceneOptions,
};

#[derive(Clone)]
pub struct Assets {
    pub tank: MeshHandle,
    pub tank_gltf: Handle<GltfSceneAsset>,
    pub grid: MeshHandle,
    pub green_material: Material,
    pub grey_material: Material,
}

pub fn load_assets(world: &mut World, progress: &mut ProgressCounter) -> () {
    let assets = {
        let mesh_storage = world.read_resource();
        let tex_storage = world.read_resource();
        let gltf_prefab_storage = world.write_resource();
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

        let tank_gltf = {
            let tank_gltf_progress: &mut ProgressCounter = progress;
            loader.load(
            "mesh/tank.gltf",
            GltfSceneFormat,
            GltfSceneOptions::default(),
            tank_gltf_progress,
            &gltf_prefab_storage,
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
            tank_gltf,
            grid,
            green_material,
            grey_material,
        }
    };

    world.add_resource(assets);
}
