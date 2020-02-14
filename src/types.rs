use cgmath::Matrix4;
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::Key;

pub type TextureId = usize;

pub struct WindowState {
    pub width: u32,
    pub height: u32,
}

impl WindowState {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState {
            width: 800,
            height: 600,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextureInfo {
    pub id: TextureId,
    pub width: u32,
    pub height: u32,
}

impl TextureInfo {
    pub fn new(id: TextureId, width: u32, height: u32) -> Self {
        TextureInfo { id, width, height }
    }
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(
        name = "texture_coords",
        repr = "[f32; 2]",
        wrapper = "VertexTextureCoords"
    )]
    TextureCoords,
}

#[allow(dead_code)]
#[derive(Vertex, Copy, Clone)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
    pub tex_coords: VertexTextureCoords,
}

pub enum InputEvent {
    KeyUp(Key),
}

pub enum GameEvent {
    CloseWindow,
    Input(InputEvent),
}
