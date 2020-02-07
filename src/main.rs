#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::cast_precision_loss)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

use cgmath::{ortho, Matrix4, Vector3};
use luminance::{
    context::GraphicsContext,
    framebuffer::Framebuffer,
    linear::M44,
    pipeline::{BoundTexture, PipelineState},
    pixel::{NormRGB8UI, NormUnsigned},
    render_state::RenderState,
    shader::program::{Program, Uniform},
    tess::{Mode, Tess, TessBuilder},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

const VS_STR: &str = include_str!("vs.shader");
const FS_STR: &str = include_str!("fs.shader");

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
#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
    tex_coords: VertexTextureCoords,
}

#[derive(UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    projection: Uniform<M44>,
    #[uniform(unbound)]
    model: Uniform<M44>,
    #[uniform(unbound)]
    image: Uniform<&'static BoundTexture<'static, Flat, Dim2, NormUnsigned>>,
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let (width, height) = (800, 600);

    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(width, height),
        "No Tilearino",
        WindowOpt::default(),
    )
    .expect("unable to create surface");

    let tex = load_texture(&mut surface, "resources/sprites.png")?;

    let mut back_buffer = surface.back_buffer().unwrap();
    let mut resize = false;

    let renderer = SpriteRenderer::new(width, height);

    let mut registry = SpriteRegistry::new();

    registry.create_sprite("link", &tex, (1, 3), (17, 27));
    registry.create_sprite("sword", &tex, (10, 269), (18, 285));
    let link = registry.get_sprite("link").unwrap();
    let _sword = registry.get_sprite("sword").unwrap();

    'app: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app
                }
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                _ => {}
            }
        }

        if resize {
            back_buffer = surface.back_buffer().unwrap();
            resize = false;
        }

        renderer.queue_sprite_render(&mut surface, link, (400., 300.))?;

        renderer.render(&mut surface, &back_buffer);

        surface.swap_buffers();
    }

    Ok(())
}

type WorldPosition = (f32, f32);
type SpriteTexture = Texture<Flat, Dim2, NormRGB8UI>;
type TextureHandle = Rc<RefCell<SpriteTexture>>;

struct Sprite {
    texture: TextureHandle,
    width: u32,
    height: u32,
    vertices: [Vertex; 4],
}

impl Sprite {
    pub fn new(texture: &TextureInfo, top_left: (u32, u32), bottom_right: (u32, u32)) -> Self {
        let (width, height) = (bottom_right.0 - top_left.0, bottom_right.1 - top_left.1);

        let tex_coords = (
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
        );

        let vertices = [
            Vertex {
                position: VertexPosition::new([0., 0.]),
                tex_coords: VertexTextureCoords::new(tex_coords.0),
            },
            Vertex {
                position: VertexPosition::new([1., 0.]),
                tex_coords: VertexTextureCoords::new(tex_coords.1),
            },
            Vertex {
                position: VertexPosition::new([1., 1.]),
                tex_coords: VertexTextureCoords::new(tex_coords.2),
            },
            Vertex {
                position: VertexPosition::new([0., 1.]),
                tex_coords: VertexTextureCoords::new(tex_coords.3),
            },
        ];

        Sprite {
            texture: texture.handle.clone(),
            width,
            height,
            vertices,
        }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

struct SpriteRegistry(HashMap<String, Sprite>);

impl SpriteRegistry {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn create_sprite<T>(
        &mut self,
        name: T,
        texture: &TextureInfo,
        top_left: (u32, u32),
        bottom_right: (u32, u32),
    ) where
        T: Into<String>,
    {
        let sprite = Sprite::new(texture, top_left, bottom_right);
        self.0.insert(name.into(), sprite);
    }

    fn get_sprite<T>(&self, name: T) -> Option<&Sprite>
    where
        T: Into<String>,
    {
        self.0.get(&name.into())
    }
}

struct TextureInfo {
    handle: TextureHandle,
    width: u32,
    height: u32,
}

struct RenderCommand {
    tess: Tess,
    model: Matrix4<f32>,
    texture: TextureHandle,
}

struct SpriteRenderer {
    buf: RefCell<Vec<RenderCommand>>,
    projection: Matrix4<f32>,
    program: Program<VertexSemantics, (), ShaderInterface>,
}

impl SpriteRenderer {
    fn new(width: u32, height: u32) -> Self {
        let projection: Matrix4<f32> = ortho(0., width as f32, height as f32, 0., -1., 1.);

        let program = Program::<VertexSemantics, (), ShaderInterface>::from_strings(
            None, VS_STR, None, FS_STR,
        )
        .expect("Could not create shader program");

        if !program.warnings.is_empty() {
            eprintln!("Warnings: {:?}", program.warnings);
        }

        SpriteRenderer {
            buf: RefCell::new(vec![]),
            projection,
            program: program.program,
        }
    }
    fn queue_sprite_render(
        &self,
        ctx: &mut GlfwSurface,
        sprite: &Sprite,
        pos: WorldPosition,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let (width, height) = sprite.dimensions();
        let w_width = ctx.width();
        let w_height = ctx.height();
        let aspect_ratio = w_width as f32 / w_height as f32;
        let translate = Matrix4::<f32>::from_translation(Vector3::new(pos.0, pos.1, 0.));
        let scale = translate
            * Matrix4::<f32>::from_nonuniform_scale(
                width as f32,
                height as f32 * aspect_ratio,
                1.0,
            );

        let model = scale;

        let tess = TessBuilder::new(ctx)
            .add_vertices(&sprite.vertices[..])
            .set_mode(Mode::TriangleFan)
            .build()
            .map_err(|e| format!("Error creating Tess: {:?}", e))?;

        self.buf.borrow_mut().push(RenderCommand {
            tess,
            model,
            texture: sprite.texture.clone(),
        });

        Ok(())
    }
}

impl Renderable for SpriteRenderer {
    fn render(&self, surface: &mut GlfwSurface, frame_buffer: &Framebuffer<Flat, Dim2, (), ()>) {
        surface.pipeline_builder().pipeline(
            &frame_buffer,
            &PipelineState::default(),
            |pipeline, mut shading_gate| {
                for c in self.buf.borrow().iter() {
                    let tex = c.texture.borrow();
                    let bound_tex = pipeline.bind_texture(&tex);
                    shading_gate.shade(&self.program, |iface, mut render_gate| {
                        iface.projection.update(self.projection.into());
                        iface.model.update(c.model.into());
                        iface.image.update(&bound_tex);
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&c.tess);
                        });
                    });
                }
            },
        );
        self.buf.borrow_mut().clear();
    }
}

pub trait Renderable {
    fn render(&self, surface: &mut GlfwSurface, frame_buffer: &Framebuffer<Flat, Dim2, (), ()>);
}

fn load_texture<T>(surface: &mut GlfwSurface, path: T) -> Result<TextureInfo, image::ImageError>
where
    T: AsRef<Path>,
{
    println!("Loading texture ({})", path.as_ref().display(),);
    let img = image::open(path).map(|img| img.to_rgb())?;
    let (width, height) = img.dimensions();
    let texels = img.into_raw();
    println!("Loaded: {}x{}", width, height);

    // create the luminance texture; the third argument is the number of mipmaps we want (leave it
    // to 0 for now) and the latest is the sampler to use when sampling the texels in the
    // shader (we’ll just use the default one)
    let tex = Texture::new(surface, [width, height], 0, Sampler::default())
        .expect("luminance texture creation");

    // the first argument disables mipmap generation (we don’t care so far)
    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    Ok(TextureInfo {
        handle: Rc::new(RefCell::new(tex)),
        width,
        height,
    })
}
