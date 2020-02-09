use std::cell::RefCell;

use cgmath::{ortho, Matrix4, Vector3};
use luminance::{
    context::GraphicsContext as _,
    linear::M44,
    pipeline::{BoundTexture, PipelineState},
    pixel::{NormRGB8UI, NormUnsigned},
    render_state::RenderState,
    shader::program::{Program, Uniform},
    tess::{Mode, Tess, TessBuilder},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_derive::UniformInterface;
use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
use specs::{Join, ReadStorage, System, World, WriteExpect};

use crate::asset_manager::AssetManager;
use crate::components::Sprite;
use crate::types::{TextureId, VertexSemantics, WindowState, WorldPosition};

const VS_STR: &str = include_str!("../vs.shader");
const FS_STR: &str = include_str!("../fs.shader");

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

pub struct RenderingSystem {
    assets: Vec<Texture<Flat, Dim2, NormRGB8UI>>,
    buf: RefCell<Vec<RenderCommand>>,
    projection: Matrix4<f32>,
    program: Program<VertexSemantics, (), ShaderInterface>,
    surface: RefCell<GlfwSurface>,
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Sprite>, WriteExpect<'a, WindowState>);
    fn run(&mut self, (sprites, mut window_state): Self::SystemData) {
        let mut resize = false;
        for event in self.surface.borrow_mut().poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    //TODO: Some event thing here
                }
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                _ => {}
            }
        }

        if resize {
            let width = self.surface.borrow().width();
            let height = self.surface.borrow().height();
            window_state.width = width;
            window_state.height = height;
            self.resize(width, height);
        }
        for sprite in sprites.join() {
            self.queue_sprite_render(&sprite, (100., 100.));
        }

        self.render();
    }

    fn setup(&mut self, world: &mut World) {
        println!("setup");
        let mut asset_manager = world.fetch_mut::<AssetManager>();
        let surface = self.surface.get_mut();
        let assets = &mut self.assets;
        asset_manager.upload_textures(|w, h, raw| {
            let tex =
                Texture::<Flat, Dim2, NormRGB8UI>::new(surface, [w, h], 0, Sampler::default())
                    .expect("luminance texture creation");

            tex.upload_raw(GenMipmaps::No, raw.as_slice()).unwrap();
            assets.push(tex);
        });
    }
}

impl RenderingSystem {
    pub fn new(width: u32, height: u32) -> Self {
        let surface = GlfwSurface::new(
            WindowDim::Windowed(width, height),
            "No Tilearino",
            WindowOpt::default(),
        )
        .expect("unable to create surface");

        let projection: Matrix4<f32> = ortho(0., width as f32, height as f32, 0., -1., 1.);

        let program = Program::<VertexSemantics, (), ShaderInterface>::from_strings(
            None, VS_STR, None, FS_STR,
        )
        .expect("Could not create shader program");

        if !program.warnings.is_empty() {
            eprintln!("Warnings: {:?}", program.warnings);
        }

        RenderingSystem {
            buf: RefCell::new(vec![]),
            projection,
            program: program.program,
            surface: RefCell::new(surface),
            assets: vec![],
        }
    }

    fn render(&self) {
        let frame_buffer = self.surface.borrow_mut().back_buffer().unwrap();
        self.surface.borrow_mut().pipeline_builder().pipeline(
            &frame_buffer,
            &PipelineState::default(),
            |pipeline, mut shading_gate| {
                for c in self.buf.borrow().iter() {
                    let tex = self.assets.get(c.texture).unwrap();

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
        self.surface.borrow_mut().swap_buffers();
    }

    fn queue_sprite_render(&mut self, sprite: &Sprite, pos: WorldPosition) {
        let (width, height) = sprite.dimensions();
        let w_width = self.surface.borrow().width();
        let w_height = self.surface.borrow().height();
        let aspect_ratio = w_width as f32 / w_height as f32;
        let translate = Matrix4::<f32>::from_translation(Vector3::new(pos.0, pos.1, 0.));
        let scale = translate
            * Matrix4::<f32>::from_nonuniform_scale(
                width as f32,
                height as f32 * aspect_ratio,
                1.0,
            );

        let model = scale;

        let tess = TessBuilder::new(self.surface.get_mut())
            .add_vertices(&sprite.vertices[..])
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();

        self.buf.borrow_mut().push(RenderCommand {
            tess,
            model,
            texture: sprite.texture.id,
        });
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.projection = ortho(0., width as f32, height as f32, 0., -1., 1.);
    }
}
