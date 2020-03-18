use amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{
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

static ASSET_MESH_TANK_GLTF: &str = "mesh/tank.gltf";
static ASSET_TEXTURE_TANK_BLUE: &str = "mesh/tank_texture_blue.png";
static ASSET_TEXTURE_MAP_TILES: &str = "texture/basic_map_tiles.png";
static ASSET_SPRITESHEET_MAP_TILES: &str = "texture/basic_map_spritesheet.ron";
static ASSET_RON_BASIC_MAP: &str = "texture/basic_map.ron";

#[derive(Clone)]
pub struct Assets {
    pub tank_gltf: Handle<GltfSceneAsset>,
    pub tank_blue_material: Handle<Material>,
    pub map_sprite_sheet_material: Handle<Material>,
    pub map_sprite_sheet: SpriteSheetHandle,
    pub sprite_grid: Handle<Mesh>,
}

pub fn load_assets(world: &mut World, progress: &mut ProgressCounter) -> () {
    let assets = {
        let mesh_storage = world.read_resource();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let material_storage = world.read_resource::<AssetStorage<Material>>();
        let gltf_prefab_storage = world.write_resource();
        let sprite_sheet_storage =
            world.read_resource::<AssetStorage<SpriteSheet>>();
        let material_defaults = world.read_resource::<MaterialDefaults>();
        let loader = world.read_resource::<Loader>();

        let tank_gltf = {
            let pc: &mut ProgressCounter = progress;
            loader.load(
                ASSET_MESH_TANK_GLTF,
                GltfSceneFormat(GltfSceneOptions::default()),
                pc,
                &gltf_prefab_storage,
            )
        };

        let tank_blue_texture = {
            let pc: &mut ProgressCounter = progress;
            loader.load(
                ASSET_TEXTURE_TANK_BLUE,
                ImageFormat::default(),
                pc,
                &texture_storage,
            )
        };
        let tank_blue_material = {
            let pc: &mut ProgressCounter = progress;
            loader.load_from_data(
                Material {
                    albedo: tank_blue_texture.clone(),
                    ..material_defaults.0.clone()
                },
                pc,
                &material_storage,
            )
        };

        let map_sprite_sheet_texture = {
            let pc: &mut ProgressCounter = progress;
            loader.load(
                ASSET_TEXTURE_MAP_TILES,
                ImageFormat::default(),
                pc,
                &texture_storage,
            )
        };
        let map_sprite_sheet_material = {
            let pc: &mut ProgressCounter = progress;
            loader.load_from_data(
                Material {
                    albedo: map_sprite_sheet_texture.clone(),
                    ..material_defaults.0.clone()
                },
                pc,
                &material_storage,
            )
        };

        let map_sprite_sheet = {
            let pc: &mut ProgressCounter = progress;
            loader.load(
                ASSET_SPRITESHEET_MAP_TILES,
                SpriteSheetFormat(map_sprite_sheet_texture.clone()),
                pc,
                &sprite_sheet_storage,
            )
        };
        let sprite_grid = {
            let pc: &mut ProgressCounter = progress;
            loader.load(
                ASSET_RON_BASIC_MAP,
                SpriteGridFormat { texture: map_sprite_sheet_texture.clone() },
                pc,
                &mesh_storage
            )
        };

        Assets {
            tank_gltf,
            tank_blue_material,
            map_sprite_sheet_material,
            map_sprite_sheet,
            sprite_grid,
        }
    };

    world.insert(assets);
}
