// adapted from shape.rs from amethyst_renderer.
//
// sprite_grid is for creating a tiled grid of 2D sprites.

use std::sync::Arc;

use amethyst::{
    error::Error,
    assets::{
        Format,
        FormatValue,
        Handle,
        Source,
    },
    renderer::{
        rendy::mesh::{
            MeshBuilder,
            Normal,
            Position,
            PosNormTex,
            PosNormTangTex,
            PosTex,
            Tangent,
            TexCoord,
        },
        sprite::TextureCoordinates,
        types::MeshData,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
};
use genmesh::{
    generators::Plane,
    MapVertex, Quad, Triangulate, Vertices,
};
use nalgebra::{
    Vector3,
};
use ron::de::from_bytes as from_ron_bytes;
use serde::{Deserialize, Serialize,};

// Shape generators
#[derive(Clone, Debug)]
pub struct SpriteGrid {
    pub sprite_sheet: SpriteSheet,
    pub grid: Vec<Vec<usize>>,
    pub num_rows: usize,
    pub num_cols: usize,
}

// Smells
// Pos, Norm, Tex, Tangent
pub type VertexFormat = ([f32; 3], [f32; 3], [f32; 2], [f32; 3]);

/// Internal Shape, used for transformation from `genmesh` to `MeshData`
#[derive(Debug)]
pub struct InternalShape(Vec<VertexFormat>);

impl SpriteGrid {
    // Generate `MeshData` for the `SpriteGrid`
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
    //     * `Vec<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>`
     pub fn generate<'a, V>(&self, scale: Option<(f32, f32, f32)>) -> MeshBuilder<'a>
     where
         V: From<InternalShape> + Into<MeshBuilder<'a>>,
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
    ) -> Vec<VertexFormat> {
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
                position: Position([v.0[0], v.0[1], v.0[2]]),
                tex_coord: TexCoord([v.2[0], v.2[1]]),
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
                position: Position([v.0[0], v.0[1], v.0[2]]),
                normal: Normal([v.1[0], v.1[1], v.1[2]]),
                tex_coord: TexCoord([v.2[0], v.2[1]]),
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
                position: Position([v.0[0], v.0[1], v.0[2]]),
                normal: Normal([v.1[0], v.1[1], v.1[2]]),
                tex_coord: TexCoord([v.2[0], v.2[1]]),
                tangent: Tangent([v.3[0], v.3[1], v.3[2], 1.0]),
            })
            .collect()
    }
}

impl From<InternalShape> for (Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>) {
    fn from(shape: InternalShape) -> Self {
        let v: Vec<PosNormTangTex> = shape.into();
        let mut vecs = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for pntt in v {
            let (positions, normals, tangents, tex_coords) = &mut vecs;
            positions.push(pntt.position);
            normals.push(pntt.normal);
            tex_coords.push(pntt.tex_coord);
            tangents.push(pntt.tangent);
        }
        vecs
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SerializedSpriteGrid {
    /// Width of the sprite sheet
    pub spritesheet_path: String,
    /// Description of the sprites
    pub grid: Vec<Vec<usize>>,
}

#[derive(Clone, Debug)]
pub struct SpriteGridFormat {
    pub texture: Handle<Texture>
}

impl Format<MeshData> for SpriteGridFormat {
    fn name(&self) -> &'static str {
        "SpriteGrid"
    }

    fn import(
        &self,
        name: String,
        source: Arc<dyn Source>,
        _create_reload:  Option<Box<dyn Format<MeshData>>>,
    ) -> Result<FormatValue<MeshData>, Error> {
        #[cfg(feature = "profiler")]
        profile_scope!("import_asset");
        // NOTE: create_reload is IGNORED.
        // Thus, no hot reloading.
        // To reload, would need to impl. Reload for multiple files.

        let bytes = source
            .load(&name)
            .map_err(|_| Error::from_string("error loading asset from source"))?;

        let load_data: SerializedSpriteGrid = from_ron_bytes(&bytes).map_err(|_| {
            Error::from_string(
                "Failed to parse Ron file for SpriteGrid",
            )
        })?;

        // smell: just assume grid is the right size
        let num_rows = load_data.grid.len();
        let num_cols = load_data.grid[0].len();

        // My understanding: typically Prefab is used for an easy
        //  way to make an Asset which depends on other Assets being loaded.
        // But this SpriteGrid wants to refer to the SpriteSheet (and its sprites),
        //  so would have to wait for the spritesheet's loading to be completed
        //  before it could even begin to load the MeshData from SpriteGrid.
        // So, I'm just loading the spritesheet using SpriteSheetFormat.
        // This doesn't feel idiomatic.
        let sprite_sheet_bytes = source
            .load(&load_data.spritesheet_path)
            .map_err(|_| Error::from_string("error loading asset from source"))?;
        let sprite_sheet = {
            let format = SpriteSheetFormat(self.texture.clone());
            format.import_simple(sprite_sheet_bytes)?
        };

        let sprite_grid = SpriteGrid {
            sprite_sheet,
            grid: load_data.grid,
            num_rows,
            num_cols,
        };

        // smell
        let data = sprite_grid.generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(
            Some((2.0 * num_cols as f32, 2.0 * num_rows as f32, 1.0))
        ).into();

        Ok(FormatValue::data(data))
    }
}
