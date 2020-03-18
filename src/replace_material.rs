use std::borrow::Cow;
use std::collections::HashSet;

use amethyst::{
    assets::Handle,
    ecs::DenseVecStorage,
    ecs::prelude::Component,
    renderer::Material,
};

#[derive(Default)]
pub struct ReplaceMaterial {
    pub targets: HashSet<Cow<'static, str>>,
    pub replacement: Option<Handle<Material>>,
}

impl Component for ReplaceMaterial {
    type Storage = DenseVecStorage<Self>;
}
