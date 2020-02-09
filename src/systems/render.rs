use std::cell::RefCell;

use cgmath::{ortho, Matrix4, Vector3};
use luminance::{
    context::GraphicsContext as _,
    framebuffer::Framebuffer,
    linear::M44,
    pipeline::{BoundTexture, PipelineState},
    pixel::NormUnsigned,
    render_state::RenderState,
    shader::program::{Program, Uniform},
    tess::{Mode, Tess, TessBuilder},
    texture::{Dim2, Flat},
};
use luminance_derive::UniformInterface;
use luminance_glfw::{GlfwSurface, Surface as _};
use specs::{Join, ReadStorage, World};

use crate::asset_manager::AssetManager;
use crate::components::Sprite;
use crate::game_error::GameError;
use crate::types::{TextureId, VertexSemantics, WorldPosition};

const VS_STR: &str = include_str!("../vs.shader");
const FS_STR: &str = include_str!("../fs.shader");

pub struct RenderingSystem {
    renderer: SpriteRenderer,
}

impl RenderingSystem {
    pub fn new(width: u32, height: u32) -> Self {
        RenderingSystem {
            renderer: SpriteRenderer::new(width, height),
        }
    }

    pub fn render(&self, surface: &mut GlfwSurface, world: &mut World, assets: &AssetManager) {
        let frame_buffer = { surface.back_buffer().unwrap() };

        let sprites = world.system_data::<ReadStorage<Sprite>>();

        for sprite in sprites.join() {
            self.renderer
                .queue_sprite_render(surface, &sprite, (100., 100.));
        }
        self.renderer.render(surface, &frame_buffer, &assets);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }
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

struct RenderCommand {
    tess: Tess,
    model: Matrix4<f32>,
    texture: TextureId,
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
    fn render(
        &self,
        surface: &mut GlfwSurface,
        frame_buffer: &Framebuffer<Flat, Dim2, (), ()>,
        assets: &AssetManager,
    ) {
        surface.pipeline_builder().pipeline(
            &frame_buffer,
            &PipelineState::default(),
            |pipeline, mut shading_gate| {
                for c in self.buf.borrow().iter() {
                    let tex = assets.get(c.texture).unwrap();

                    let bound_tex = pipeline.bind_texture(tex);
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
    fn queue_sprite_render(
        &self,
        ctx: &mut GlfwSurface,
        sprite: &Sprite,
        pos: WorldPosition,
    ) -> Result<(), GameError> {
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
            .map_err(|e| GameError(format!("Error creating Tess: {:?}", e)))?;

        self.buf.borrow_mut().push(RenderCommand {
            tess,
            model,
            texture: sprite.texture.id,
        });

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.projection = ortho(0., width as f32, height as f32, 0., -1., 1.);
    }
}
