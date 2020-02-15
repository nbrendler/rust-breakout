use std::cell::RefCell;
use std::fmt;

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
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::asset_manager::AssetManager;
use crate::components::{Sprite, Transform};
use crate::types::{GameEvent, InputEvent, ScreenContext, TextureId, VertexSemantics};

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

impl fmt::Debug for RenderCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RenderCommand {{ tex: {}, model: {:?} }}",
            self.texture, self.model
        )
    }
}

pub struct RenderingSystem {
    assets: Vec<Texture<Flat, Dim2, NormRGB8UI>>,
    buf: RefCell<Vec<RenderCommand>>,
    screen_context: ScreenContext,
    program: Program<VertexSemantics, (), ShaderInterface>,
    surface: RefCell<GlfwSurface>,
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Transform>,
        Write<'a, EventChannel<GameEvent>>,
        WriteExpect<'a, ScreenContext>,
    );
    fn run(&mut self, (sprites, transforms, mut event_channel, mut screen_ctx): Self::SystemData) {
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
            screen_ctx.set_dimensions(self.screen_context.dimensions());
            screen_ctx.set_transform(self.screen_context.transform());
        }
        for (sprite, transform) in (&sprites, &transforms).join() {
            self.queue_sprite_render(&sprite, &transform);
        }

        self.render();
    }

    fn setup(&mut self, world: &mut World) {
        println!("render setup");
        let surface = self.surface.get_mut();
        {
            let mut asset_manager = world.fetch_mut::<AssetManager>();
            let assets = &mut self.assets;
            asset_manager.upload_textures(|w, h, raw| {
                let tex =
                    Texture::<Flat, Dim2, NormRGB8UI>::new(surface, [w, h], 0, Sampler::default())
                        .expect("luminance texture creation");

                tex.upload_raw(GenMipmaps::No, raw.as_slice()).unwrap();
                assets.push(tex);
            });
        }
        world.insert(self.screen_context);
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

        let screen_context = ScreenContext::new(Matrix4::<f32>::identity(), width, height);
        let mut s = RenderingSystem {
            buf: RefCell::new(vec![]),
            program: program.program,
            surface: RefCell::new(surface),
            assets: vec![],
            screen_context,
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
                        iface.world.update(self.screen_context.transform().into());
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

        let model = transform.get_matrix();

        self.buf.borrow_mut().push(RenderCommand {
            tess,
            model,
            texture: sprite.texture.id,
        });
    }

    fn resize(&mut self, width: u32, height: u32) {
        let (w, h) = (width as f32, height as f32);

        let world: Matrix4<f32> = ortho(0., w, 0., h, -1., 1.);

        let position = Matrix4::<f32>::from_translation(Vector3::new((w - 500.) / 2.0, 10., 0.));

        self.screen_context.set_transform(world * position);
        self.screen_context.set_dimensions((width, height));
    }
}
