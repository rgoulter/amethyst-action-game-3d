use amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{
        ComboMeshCreator, Material, MaterialDefaults, MeshHandle,
        PngFormat, Shape, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, TextureHandle, TextureMetadata,
    },
};
use amethyst_gltf::{
    GltfSceneAsset,
    GltfSceneFormat,
    GltfSceneOptions,
};

use crate::grid_of_sprites::{
    GridOfSpritesFormat,
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
pub struct UndergroundBaseAnimations {
    // bottom_left
    // bottom_right
    // open
    // top_left
    pub top_right: Handle<GltfSceneAsset>,
    pub unanimated: Handle<GltfSceneAsset>,
}

#[derive(Clone)]
pub struct Assets {
    pub tank_gltf: Handle<GltfSceneAsset>,
    pub tank_blue_texture: TextureHandle,
    pub grid: MeshHandle,
    pub green_material: Material,
    pub grey_material: Material,
    pub map_texture_material: Material,
    pub map_sprite_sheet: SpriteSheetHandle,
    pub grid_of_sprites: MeshHandle,
    pub underground_base: UndergroundBaseAnimations,
}

pub fn load_assets(world: &mut World, progress: &mut ProgressCounter) -> () {
    let assets = {
        let mesh_storage = world.read_resource();
        let tex_storage = world.read_resource();
        let gltf_prefab_storage = world.write_resource();
        let sprite_sheet_storage =
            world.read_resource::<AssetStorage<SpriteSheet>>();
        let mat_defaults = world.read_resource::<MaterialDefaults>();
        let loader = world.read_resource::<Loader>();

        let mut gltf_options = GltfSceneOptions::default();
        gltf_options.flip_v_coord = true;
        let tank_gltf = {
            let tank_gltf_progress: &mut ProgressCounter = progress;
            loader.load(
                "mesh/tank.gltf",
                GltfSceneFormat,
                gltf_options,
                tank_gltf_progress,
                &gltf_prefab_storage,
            )
        };

        let tank_blue_texture = {
            let tex_progress: &mut ProgressCounter = progress;
            loader.load(
                "mesh/tank_texture_blue.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                tex_progress,
                &tex_storage,
            )
        };

        let green_texture = {
            let texture_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                [0.3, 1.0, 0.3, 1.0].into(),
                texture_progress,
                &tex_storage
            )
        };
        let green_material = Material {
            albedo: green_texture.clone(),
            ..mat_defaults.0.clone()
        };

        let grey_texture = {
            let texture_progress: &mut ProgressCounter = progress;
            loader.load_from_data(
                [0.3, 0.3, 0.3, 1.0].into(),
                texture_progress,
                &tex_storage
            )
        };
        let grey_material = Material {
            albedo: grey_texture.clone(),
            ..mat_defaults.0.clone()
        };

        let divisions = None;
        let grid_shape = Shape::Plane(divisions);
        let scale = Some((0.5, 0.5, 0.5));
        let grid_mesh_data = grid_shape.generate::<ComboMeshCreator>(scale);
        let grid = {
            let grid_progress: &mut ProgressCounter = progress;
            loader.load_from_data(grid_mesh_data, grid_progress, &mesh_storage)
        };

        let map_texture = {
            let tex_progress: &mut ProgressCounter = progress;
            loader.load(
                "texture/basic_map_tiles.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                tex_progress,
                &tex_storage,
            )
        };

        let map_texture_material = Material {
            albedo: map_texture.clone(),
            ..mat_defaults.0.clone()
        };

        let map_sprite_sheet = {
            let sprite_sheet_progress: &mut ProgressCounter = progress;
            loader.load(
                "texture/basic_map_spritesheet.ron",
                SpriteSheetFormat,
                map_texture.clone(),
                sprite_sheet_progress,
                &sprite_sheet_storage,
            )
        };

        let grid_of_sprites = {
            let grid_progress: &mut ProgressCounter = progress;
            loader.load(
                "texture/basic_map.ron",
                GridOfSpritesFormat,
                map_texture.clone(),
                grid_progress,
                &mesh_storage
            )
        };

        // underground_base animations:
        // 0: lift bottom left
        // 1: lift bottom right
        // 2: lift open
        // 3: lift top left
        // 4: lift top right
        // 5: scene

        let base_anim_scene_top_right = {
            let gltf_progress: &mut ProgressCounter = progress;
            loader.load(
                "mesh/underground_base.gltf",
                GltfSceneFormat,
                GltfSceneOptions {
                    load_animations: true,
                    flip_v_coord: true,
                    scene_index: Some(4),
                    ..GltfSceneOptions::default()
                },
                gltf_progress,
                &gltf_prefab_storage,
            )
        };

        let base_unanimated = {
            let gltf_progress: &mut ProgressCounter = progress;
            loader.load(
                "mesh/underground_base.gltf",
                GltfSceneFormat,
                GltfSceneOptions {
                    load_animations: false,
                    flip_v_coord: true,
                    scene_index: Some(5),
                    ..GltfSceneOptions::default()
                },
                gltf_progress,
                &gltf_prefab_storage,
            )
        };

        let underground_base = UndergroundBaseAnimations {
            top_right: base_anim_scene_top_right,
            unanimated: base_unanimated,
        };

        Assets {
            tank_gltf,
            tank_blue_texture,
            grid,
            green_material,
            grey_material,
            map_texture_material,
            map_sprite_sheet,
            grid_of_sprites,
            underground_base,
        }
    };

    world.add_resource(assets);
}
