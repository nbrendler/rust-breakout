use cgmath::{Matrix, Matrix4, Vector3, Vector4};
use specs::{storage::DenseVecStorage, Component};

use crate::collidable::Collidable;
use crate::components::Transform;
use crate::types::{TextureInfo, Vertex, VertexPosition, VertexTextureCoords};

#[derive(Copy, Clone)]
pub struct Sprite {
    pub texture: TextureInfo,
    pub offsets: [f32; 2],
    width: u32,
    height: u32,
    vertices: [Vertex; 4],
}

impl Component for Sprite {
    type Storage = DenseVecStorage<Self>;
}

impl Sprite {
    pub fn new(texture: &TextureInfo, top_left: (u32, u32), bottom_right: (u32, u32)) -> Self {
        let (width, height) = (bottom_right.0 - top_left.0, bottom_right.1 - top_left.1);

        let tex_coords = [
            [
                top_left.0 as f32 / texture.width as f32,
                top_left.1 as f32 / texture.height as f32,
            ],
            [
                bottom_right.0 as f32 / texture.width as f32,
                top_left.1 as f32 / texture.height as f32,
            ],
            [
                bottom_right.0 as f32 / texture.width as f32,
                bottom_right.1 as f32 / texture.height as f32,
            ],
            [
                top_left.0 as f32 / texture.width as f32,
                bottom_right.1 as f32 / texture.height as f32,
            ],
        ];

        let vertices = [
            Vertex {
                position: VertexPosition::new([0., 0.]),
                tex_coords: VertexTextureCoords::new(tex_coords[0]),
            },
            Vertex {
                position: VertexPosition::new([1., 0.]),
                tex_coords: VertexTextureCoords::new(tex_coords[1]),
            },
            Vertex {
                position: VertexPosition::new([1., 1.]),
                tex_coords: VertexTextureCoords::new(tex_coords[2]),
            },
            Vertex {
                position: VertexPosition::new([0., 1.]),
                tex_coords: VertexTextureCoords::new(tex_coords[3]),
            },
        ];

        Sprite {
            texture: *texture,
            width,
            height,
            vertices,
            offsets: [0., 0.],
        }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let (w, h) = self.dimensions();

        // offsets are in uv space
        let offsets =
            Matrix4::<f32>::from_translation(Vector3::new(-self.offsets[0], -self.offsets[1], 0.));

        let model = Matrix4::<f32>::from_nonuniform_scale(w as f32, -1.0 * h as f32, 1.0);

        model * offsets
    }
}

impl Collidable for Sprite {
    fn get_hitbox(&self) -> ((f32, f32), (f32, f32)) {
        let m = self.get_model_matrix();

        let v1 = m * Vector4::new(0., 1., 0., 1.);
        let v2 = m * Vector4::new(1., 0., 0., 1.);

        ((v1.x, v1.y), (v2.x, v2.y))
    }
}
