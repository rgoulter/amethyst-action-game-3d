use amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{
        palette::LinSrgba,
        rendy::{
            mesh::{Normal, Position, Tangent, TexCoord},
            texture::palette::load_from_linear_rgba,
        },
        shape::Shape,
        sprite::SpriteSheetHandle,
        formats::texture::ImageFormat,
        Material,
        MaterialDefaults,
        Mesh,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
};
use amethyst_gltf::{
    GltfSceneAsset,
    GltfSceneFormat,
    GltfSceneOptions,
};

use crate::sprite_grid::{
    SpriteGridFormat,
};

// The basic map sprite sheet has incides:
// 0: road
// 1: sand
// 2: sand (dark)
// 3: grass
// 4: grass (dark)
// 5: water
// 6: water (dark)

#[derive(Clone)]
pub struct Assets {
    pub tank_gltf: Handle<GltfSceneAsset>,
    pub tank_blue_material: Handle<Material>,
    pub grid: Handle<Mesh>,
    pub green_material: Handle<Material>,
    pub grey_material: Handle<Material>,
    pub map_texture_material: Handle<Material>,
    pub map_sprite_sheet: SpriteSheetHandle,
    pub sprite_grid: Handle<Mesh>,
}

pub fn load_assets(world: &mut World, progress: &mut ProgressCounter) -> () {
    let assets = {
        let mesh_storage = world.read_resource();
        let tex_storage = world.read_resource::<AssetStorage<Texture>>();
        let mtl_storage = world.read_resource::<AssetStorage<Material>>();
        let gltf_prefab_storage = world.write_resource();
        let sprite_sheet_storage =
            world.read_resource::<AssetStorage<SpriteSheet>>();
        let mat_defaults = world.read_resource::<MaterialDefaults>();
        let loader = world.read_resource::<Loader>();

        let gltf_options = GltfSceneOptions::default();
        let tank_gltf = {
            let tank_gltf_progress: &mut ProgressCounter = progress;
            loader.load(
                "mesh/tank.gltf",
                GltfSceneFormat(gltf_options),
                tank_gltf_progress,
                &gltf_prefab_storage,
            )
        };

        let tank_blue_texture = {
            let tex_progress: &mut ProgressCounter = progress;
            loader.load(
                "mesh/tank_texture_blue.png",
                ImageFormat::default(),
                tex_progress,
                &tex_storage,
            )
        };
        let tank_blue_material = {
            let mtl_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                Material {
                    albedo: tank_blue_texture.clone(),
                    ..mat_defaults.0.clone()
                },
                mtl_progress,
                &mtl_storage,
            )
        };

        let green_texture = {
            let texture_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(0.3, 1.0, 0.3, 1.0)).into(),
                texture_progress,
                &tex_storage
            )
        };
        let green_material = {
            let mtl_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                Material {
                    albedo: green_texture.clone(),
                    ..mat_defaults.0.clone()
                },
                mtl_progress,
                &mtl_storage,
            )
        };

        let grey_texture = {
            let texture_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(0.3, 0.3, 0.3, 1.0)).into(),
                texture_progress,
                &tex_storage
            )
        };
        let grey_material = {
            let mtl_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                Material {
                    albedo: grey_texture.clone(),
                    ..mat_defaults.0.clone()
                },
                mtl_progress,
                &mtl_storage,
            )
        };

        let divisions = None;
        let grid_shape = Shape::Plane(divisions);
        let scale = Some((0.5, 0.5, 0.5));
        let grid_mesh_data =
            grid_shape
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(scale)
                .into();
        let grid = {
            let grid_progress: &mut ProgressCounter = progress;
            loader.load_from_data(grid_mesh_data, grid_progress, &mesh_storage)
        };

        let map_texture = {
            let tex_progress: &mut ProgressCounter = progress;
            loader.load(
                "texture/basic_map_tiles.png",
                ImageFormat::default(),
                tex_progress,
                &tex_storage,
            )
        };

        let map_texture_material = {
            let mtl_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                Material {
                    albedo: map_texture.clone(),
                    ..mat_defaults.0.clone()
                },
                mtl_progress,
                &mtl_storage,
            )
        };

        let map_sprite_sheet = {
            let sprite_sheet_progress: &mut ProgressCounter = progress;
            loader.load(
                "texture/basic_map_spritesheet.ron",
                SpriteSheetFormat(map_texture.clone()),
                sprite_sheet_progress,
                &sprite_sheet_storage,
            )
        };

        let sprite_grid = {
            let grid_progress: &mut ProgressCounter = progress;
            loader.load(
                "texture/basic_map.ron",
                SpriteGridFormat { texture: map_texture.clone() },
                grid_progress,
                &mesh_storage
            )
        };

        Assets {
            tank_gltf,
            tank_blue_material,
            grid,
            green_material,
            grey_material,
            map_texture_material,
            map_sprite_sheet,
            sprite_grid,
        }
    };

    world.insert(assets);
}
