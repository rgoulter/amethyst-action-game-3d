use std::borrow::Cow;
use std::collections::HashMap;

use amethyst;
use amethyst::{
    assets::Handle,
    core::{
        Named,
        Parent,
    },
    derive::SystemDesc,
    ecs::prelude::{
        Entities, Entity, Join, ReadStorage, System, SystemData, WriteStorage,
    },
    renderer::Material,
};

use crate::replace_material::ReplaceMaterial;

#[derive(Default, SystemDesc)]
pub struct ReplaceMaterialSystem;

fn has_ancestor_with_name<'a>(
    name: &Cow<'static, str>,
    component: &Parent,
    named: &ReadStorage<'a, Named>,
    parents: &ReadStorage<'a, Parent>
) -> bool {
    let mut parent = Some(component.entity);

    println!("has ancestor with name={}?:", name);
    while let Some(parent_entity) = parent {
        println!("- found parent");
        if let Some(parent_name) = named.get(parent_entity).map(|n| &n.name) {
            println!("-- parent has name: {}", parent_name);

            if name == parent_name {
                println!("-- MATCHES!");
                return true
            }
        }

        parent = parents.get(parent_entity).map(|p| p.entity);
    }

    false
}

impl<'a> System<'a> for ReplaceMaterialSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ReplaceMaterial>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Parent>,
        WriteStorage<'a, Handle<Material>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut replacements, named, parents, mut materials) =
            data;

        let mut replacement_map: HashMap<Entity, Handle<Material>> =
            HashMap::new();

        // Build map of entities with the material we want to replace
        //
        // Outer loop: loop over ReplaceMaterials
        // Inner loop: loop over named entities with material to find descendants
        for (replacement_name, replacement)
        in (&named, &mut replacements).join() {
            if let Some(replacement_material) = &replacement.replacement {
                let targets = &mut replacement.targets;

                for (entity, name, parent, _material)
                in (&entities, &named, &parents, &materials).join() {
                    let name = &name.name;

                    if targets.contains(name) {
                        let is_descendant =
                            has_ancestor_with_name(
                                &replacement_name.name,
                                parent,
                                &named,
                                &parents
                            );
                        if is_descendant {
                            println!("found entity to replace material");
                            replacement_map.insert(entity, replacement_material.clone());

                            targets.remove(name);
                        }
                    }
                }

                if targets.is_empty() {
                    replacement.replacement = None;
                }
            }
        }

        // Replace the material
        for (entity, material_handle) in replacement_map {
            materials.insert(entity, material_handle);
        }
    }
}
