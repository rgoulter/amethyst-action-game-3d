use std::borrow::Cow;
use std::collections::HashSet;

use amethyst::{
    ecs::{DenseVecStorage},
    ecs::prelude::Component,
    renderer::TextureHandle,
};

#[derive(Default)]
pub struct ReplaceMaterial {
    // pub targets: HashSet<String>,
    pub targets: HashSet<Cow<'static, str>>,
    pub replacement: Option<TextureHandle>,
}

impl Component for ReplaceMaterial {
    type Storage = DenseVecStorage<Self>;
}
