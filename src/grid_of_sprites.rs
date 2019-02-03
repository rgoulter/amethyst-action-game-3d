// adapted from shape.rs from amethyst_renderer.
//
// grid_of_sprites is for creating a tiled grid of 2D sprites.

use std::marker::PhantomData;

use amethyst::{
    assets::{
        AssetStorage, Handle, Loader, PrefabData, PrefabError,
        Progress, ProgressCounter,
    },
    core::{
        nalgebra::{Vector2, Vector3},
        specs::prelude::{Entity, Read, ReadExpect, WriteStorage},
    },
    renderer::{
        ComboMeshCreator, MeshData, Normal, Position, PosTex, PosNormTex,
        PosNormTangTex, Separate, SpriteSheet, Tangent, TexCoord,
        TextureCoordinates,
    },
};

use genmesh::{
    generators::{
        Circle, Cone, Cube, Cylinder, IcoSphere, IndexedPolygon,
        Plane, SharedVertex, SphereUv, Torus,
    },
    EmitTriangles, MapVertex, Quad, Triangulate, Vertex, Vertices,
};

// Shape generators
#[derive(Clone, Debug)]
// #[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GridOfSprites {
    pub sprite_sheet: SpriteSheet,
    pub grid: Vec<Vec<usize>>,
    pub num_rows: usize,
    pub num_cols: usize,
}

pub type VertexFormat = ([f32; 3], [f32; 3], [f32; 2], [f32; 3]);

/// Internal Shape, used for transformation from `genmesh` to `MeshData`
#[derive(Debug)]
pub struct InternalShape(Vec<VertexFormat>);

impl GridOfSprites {
    // Generate `MeshData` for the `GridOfSprites`
    //
    // ### Parameters:
    //
    // - `scale`: Scale the shape by the given amounts along the x, y, z axes
    //
    // ### Type parameters:
    //
    // `V`: Vertex format to use, must to be one of:
    //     * `Vec<PosTex>`
    //     * `Vec<PosNormTex>`
    //     * `Vec<PosNormTangTex>`
    //     * `ComboMeshCreator`
    pub fn generate<V>(&self, scale: Option<(f32, f32, f32)>) -> MeshData
    where
        V: From<InternalShape> + Into<MeshData>,
    {
        V::from(self.generate_internal(scale)).into()
    }

    fn generate_internal(&self, scale: Option<(f32, f32, f32)>) -> InternalShape {
        let vertices =
            self.generate_vertices(
                Plane::subdivide(self.num_cols, self.num_rows),
                scale,
            );
        InternalShape(vertices)
    }

    fn tex_coords_for_index(&self, index: usize) -> TextureCoordinates {
        let row = index / self.num_cols;
        let col = index % self.num_cols;
        // TODO: default to the first sprite if the (sprite) index is bad.
        let sprite_index = self.grid[row][col];
        let sprite = &self.sprite_sheet.sprites[sprite_index]; // smells
        sprite.tex_coords.clone()
    }

    fn generate_vertices(
        &self,
        plane: Plane,
        scale: Option<(f32, f32, f32)>
    ) -> Vec<VertexFormat>
    {
        plane
            .enumerate()
            .map(|(i, Quad{x: v0, y: v1, z: v2, w: v3})| {
                let tex_coords = self.tex_coords_for_index(i);
                Quad::new(
                    (v0, [tex_coords.left,  tex_coords.bottom]),
                    (v1, [tex_coords.right, tex_coords.bottom]),
                    (v2, [tex_coords.right, tex_coords.top]),
                    (v3, [tex_coords.left,  tex_coords.top])
                ).map_vertex(|(vertex, uv)| {
                    let pos = scale
                        .map(|(sx, sy, sz)| Vector3::new(
                            vertex.pos.x * sx,
                            vertex.pos.y * sy,
                            vertex.pos.z * sz
                        ))
                        .unwrap_or_else(|| Vector3::from(vertex.pos));
                    let normal = scale
                        .map(|(sx, sy, sz)| {
                            Vector3::new(
                                vertex.normal.x * sx,
                                vertex.normal.y * sy,
                                vertex.normal.z * sz
                            ).normalize()
                        })
                        .unwrap_or_else(|| Vector3::from(vertex.normal));
                    let up = Vector3::y();
                    let tangent = normal.cross(&up).cross(&normal);
                    (
                        pos.into(),
                        normal.into(),
                        uv,
                        tangent.into(),
                    )
                })
            })
            .triangulate()
            .vertices()
            .collect::<Vec<VertexFormat>>()
    }
}

impl From<InternalShape> for Vec<PosTex> {
    fn from(shape: InternalShape) -> Self {
        shape
            .0
            .iter()
            .map(|v| PosTex {
                position: Vector3::new(v.0[0], v.0[1], v.0[2]),
                tex_coord: Vector2::new(v.2[0], v.2[1]),
            })
            .collect()
    }
}

impl From<InternalShape> for Vec<PosNormTex> {
    fn from(shape: InternalShape) -> Self {
        shape
            .0
            .iter()
            .map(|v| PosNormTex {
                position: Vector3::new(v.0[0], v.0[1], v.0[2]),
                tex_coord: Vector2::new(v.2[0], v.2[1]),
                normal: Vector3::new(v.1[0], v.1[1], v.1[2]),
            })
            .collect()
    }
}

impl From<InternalShape> for Vec<PosNormTangTex> {
    fn from(shape: InternalShape) -> Self {
        shape
            .0
            .iter()
            .map(|v| PosNormTangTex {
                position: Vector3::new(v.0[0], v.0[1], v.0[2]),
                tex_coord: Vector2::new(v.2[0], v.2[1]),
                normal: Vector3::new(v.1[0], v.1[1], v.1[2]),
                tangent: Vector3::new(v.3[0], v.3[1], v.3[2]),
            })
            .collect()
    }
}

impl From<InternalShape> for ComboMeshCreator {
    fn from(shape: InternalShape) -> Self {
        ComboMeshCreator::new((
            shape
                .0
                .iter()
                .map(|v| Separate::<Position>::new(v.0))
                .collect(),
            None,
            Some(
                shape
                    .0
                    .iter()
                    .map(|v| Separate::<TexCoord>::new(v.2))
                    .collect(),
            ),
            Some(
                shape
                    .0
                    .iter()
                    .map(|v| Separate::<Normal>::new(v.1))
                    .collect(),
            ),
            Some(
                shape
                    .0
                    .iter()
                    .map(|v| Separate::<Tangent>::new(v.3))
                    .collect(),
            ),
        ))
    }
}
