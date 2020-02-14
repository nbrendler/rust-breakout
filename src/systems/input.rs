use luminance_glfw::{Action, Key};
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::resources::InputState;
use crate::types::{GameEvent, InputEvent};

#[derive(Default)]
pub struct InputSystem {
    reader: Option<ReaderId<GameEvent>>,
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (Read<'a, EventChannel<GameEvent>>, Write<'a, InputState>);

    fn run(&mut self, (events, mut state): Self::SystemData) {
        let mut r = self.reader.as_mut().expect("Event reader uninitialized");
        for event in events.read(&mut r) {
            match event {
                GameEvent::Input(InputEvent::Key(Key::Left, Action::Press))
                | GameEvent::Input(InputEvent::Key(Key::A, Action::Press)) => {
                    state.left = true;
                }
                GameEvent::Input(InputEvent::Key(Key::Right, Action::Press))
                | GameEvent::Input(InputEvent::Key(Key::D, Action::Press)) => {
                    state.right = true;
                }
                GameEvent::Input(InputEvent::Key(Key::Up, Action::Press))
                | GameEvent::Input(InputEvent::Key(Key::W, Action::Press)) => {
                    state.up = true;
                }
                GameEvent::Input(InputEvent::Key(Key::Down, Action::Press))
                | GameEvent::Input(InputEvent::Key(Key::S, Action::Press)) => {
                    state.down = true;
                }
                GameEvent::Input(InputEvent::Key(Key::Left, Action::Release))
                | GameEvent::Input(InputEvent::Key(Key::A, Action::Release)) => {
                    state.left = false;
                }
                GameEvent::Input(InputEvent::Key(Key::Right, Action::Release))
                | GameEvent::Input(InputEvent::Key(Key::D, Action::Release)) => {
                    state.right = false;
                }
                GameEvent::Input(InputEvent::Key(Key::Up, Action::Release))
                | GameEvent::Input(InputEvent::Key(Key::W, Action::Release)) => {
                    state.up = false;
                }
                GameEvent::Input(InputEvent::Key(Key::Down, Action::Release))
                | GameEvent::Input(InputEvent::Key(Key::S, Action::Release)) => {
                    state.down = false;
                }
                _ => {}
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        println!("input setup");
        world.insert(InputState::default());
        {
            let mut ch = world.fetch_mut::<EventChannel<GameEvent>>();
            self.reader = Some(ch.register_reader());
        }
    }
}
