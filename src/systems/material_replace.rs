use std::borrow::Cow;

use amethyst;

use amethyst::{
    core::{
        Named,
        Parent,
    },
    ecs::prelude::{
        Join, ReadStorage, System, WriteStorage,
    },
    renderer::Material,
};

use crate::replace_material::ReplaceMaterial;

#[derive(Default)]
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
        WriteStorage<'a, ReplaceMaterial>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Parent>,
        WriteStorage<'a, Material>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut replacements, named, parents, mut materials) =
            data;

        for (replacement_name, replacement)
        in (&named, &mut replacements).join() {
            if let Some(replacement_material) = &replacement.replacement {
                let targets = &mut replacement.targets;
                for (name, parent, mut material)
                in (&named, &parents, &mut materials).join() {
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
                            material.albedo = replacement_material.clone();

                            targets.remove(name);
                        }
                    }
                }

                if targets.is_empty() {
                    replacement.replacement = None;
                }
            }
        }
    }
}
