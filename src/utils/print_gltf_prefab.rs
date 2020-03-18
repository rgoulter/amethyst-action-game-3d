use amethyst::{
    assets::{AssetStorage, Handle},
    prelude::*,
};
use amethyst_gltf::{GltfPrefab, GltfSceneAsset};

pub fn print_info_about_gltf_prefab(gltf_prefab: &mut GltfPrefab) -> () {
    println!("info about GLTF Node:");
    if let Some(name) = &gltf_prefab.name {
        println!("- name = {}", name.name);
    }
    if let Some(_transform) = &gltf_prefab.transform {
        println!("- has transform");
    }
    if let Some(_material) = &gltf_prefab.material {
        println!("- has material");
    }
}

pub fn print_gltf_info(
    world: &mut World,
    gltf_asset_handle: &Handle<GltfSceneAsset>
) -> () {
    let asset_storage: &mut AssetStorage<GltfSceneAsset> =
        &mut world.write_resource();
    let gltf_asset: Option<&mut GltfSceneAsset> =
        asset_storage.get_mut(gltf_asset_handle);
    if let Some(gltf_asset) = gltf_asset {
        let len = gltf_asset.len();
        println!("gltf_asset (Prefab<GltfPrefab>) has length = {}", len);

        for idx in 0..len {
            let gltf_prefab = gltf_asset.data_or_default(idx);

            print_info_about_gltf_prefab(gltf_prefab);
        }
    } else {
        println!("Can't find GltfSceneAsset for given handle.");
    }
}
