use std::cell::RefCell;

use cgmath::{ortho, Matrix4, SquareMatrix, Vector3};
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
use specs::{shrev::EventChannel, Join, ReadStorage, System, World, Write};

use crate::asset_manager::AssetManager;
use crate::components::{Sprite, Transform};
use crate::constants::*;
use crate::types::{GameEvent, InputEvent, TextureId, VertexSemantics};

const VS_STR: &str = include_str!("../vs.shader");
const FS_STR: &str = include_str!("../fs.shader");

#[derive(UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    world: Uniform<M44>,
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
    world_projection: Matrix4<f32>,
    program: Program<VertexSemantics, (), ShaderInterface>,
    surface: RefCell<GlfwSurface>,
    square_size: u32,
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Transform>,
        Write<'a, EventChannel<GameEvent>>,
    );
    fn run(&mut self, (sprites, transforms, mut event_channel): Self::SystemData) {
        let mut resize = false;
        for event in self.surface.borrow_mut().poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    event_channel.single_write(GameEvent::WindowEvent(event));
                    event_channel.single_write(GameEvent::CloseWindow);
                }
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                WindowEvent::Key(k, _scancode, action, _mods) => {
                    event_channel.single_write(GameEvent::Input(InputEvent::Key(k, action)))
                }
                _ => {}
            }
        }

        if resize {
            let width = self.surface.borrow().width();
            let height = self.surface.borrow().height();
            self.resize(width, height);
        }
        for (sprite, transform) in (&sprites, &transforms).join() {
            self.queue_sprite_render(&sprite, &transform);
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

        let program = Program::<VertexSemantics, (), ShaderInterface>::from_strings(
            None, VS_STR, None, FS_STR,
        )
        .expect("Could not create shader program");

        if !program.warnings.is_empty() {
            eprintln!("Warnings: {:?}", program.warnings);
        }

        let mut s = RenderingSystem {
            buf: RefCell::new(vec![]),
            world_projection: Matrix4::<f32>::identity(),
            program: program.program,
            surface: RefCell::new(surface),
            assets: vec![],
            square_size: 0,
        };

        s.resize(width, height);
        s
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
                        iface.world.update(self.world_projection.into());
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

    fn queue_sprite_render(&mut self, sprite: &Sprite, transform: &Transform) {
        let tess = TessBuilder::new(self.surface.get_mut())
            .add_vertices(sprite.get_vertices())
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();

        let t = transform.get_matrix();

        let scale = Matrix4::<f32>::from_nonuniform_scale(1.0 / SCALE_FACTOR, 1.0 / 0.95, 1.0);

        let model = t
            * scale
            * Matrix4::<f32>::from_translation(Vector3::new(
                -transform.offsets[0],
                -transform.offsets[1],
                1.,
            ));

        self.buf.borrow_mut().push(RenderCommand {
            tess,
            model,
            texture: sprite.texture.id,
        });
    }

    fn resize(&mut self, width: u32, height: u32) {
        let (w, h) = (width as f32, height as f32);

        let constraint = w.min(h);
        self.square_size = (constraint * SCALE_FACTOR).floor() as u32;
        let world: Matrix4<f32> = ortho(0., WORLD_WIDTH, 0., WORLD_HEIGHT, -1., 1.);

        let aspect_ratio = w / h;
        if w >= h {
            self.world_projection =
                Matrix4::<f32>::from_nonuniform_scale(SCALE_FACTOR / aspect_ratio, 0.95, 1.0)
                    * world;
        } else {
            self.world_projection =
                Matrix4::<f32>::from_nonuniform_scale(SCALE_FACTOR, 0.95 * aspect_ratio, 1.0)
                    * world;
        }
    }
}
