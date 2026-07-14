use std::sync::Arc;
use std::time::Duration;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::fire_input_events;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::retroarch::{Application, ApplicationProperties};
use rust_libretro::contexts::AudioContext;
use rust_libretro::input_descriptors;
use rust_libretro::sys::{retro_input_descriptor, RETRO_DEVICE_ID_JOYPAD_A, RETRO_DEVICE_ID_JOYPAD_B, RETRO_DEVICE_ID_JOYPAD_DOWN, RETRO_DEVICE_ID_JOYPAD_LEFT, RETRO_DEVICE_ID_JOYPAD_RIGHT, RETRO_DEVICE_ID_JOYPAD_START, RETRO_DEVICE_ID_JOYPAD_UP, RETRO_DEVICE_JOYPAD};
use rust_libretro::types::JoypadState;
use crate::screens::gamescreen::GameScreen;
use crate::screens::loadscreen::LoadScreen;
use crate::screens::Screen;
use crate::screens::titlescreen::TitleScreen;
use crate::screens::transitions::{GameOver, Loaded, StartGame};

pub struct Tetris {
    assets: Arc<Assets>,
    screen: Box<dyn Screen>,
    previous_joypad_state: JoypadState,
}

const INPUT_DESCRIPTORS: &[retro_input_descriptor] = &input_descriptors!(
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP, "Up" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN, "Down" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT, "Left" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT, "Right" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A, "Rotate Clockwise" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_B, "Rotate Anticlockwise" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_START, "Start" },
);

impl Application for Tetris {
    fn new(assets: Arc<Assets>, logger_worker: Option<tracing_appender::non_blocking::WorkerGuard>) -> Self {
        Tetris {
            assets: assets.clone(),
            screen: Box::new(LoadScreen),
            previous_joypad_state : JoypadState::empty()
        }
    }

    fn update(&mut self, joypad_state: JoypadState, delta_time: u64, renderer: &mut AssetRenderer, events: &mut Events) {

        fire_input_events(joypad_state, self.previous_joypad_state, events);
        self.previous_joypad_state = joypad_state;
        self.process_events(renderer, events);

        let dt = Duration::from_micros(delta_time);

        events.elapse("Application", dt);
        self.process_events(renderer, events);

        events.fire(dt);
        self.process_events(renderer, events);
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        self.screen.draw(renderer);
    }

    fn play(&mut self, _ctx: &mut AudioContext) {
        // nothing yet
    }

    fn properties() -> ApplicationProperties {
        ApplicationProperties {
            width: 320,
            height: 200,
            name: "Tetris".to_string(),
            extensions: &["ttr"],
            input_descriptors: INPUT_DESCRIPTORS,
        }
    }
}

impl Tetris {
    fn on_event(&mut self, event: &Event, renderer: &mut AssetRenderer, events: &mut Events) {
        event.apply(|Loaded()| {
            self.screen = Box::new(TitleScreen::new(&self.assets, renderer));
        });
        event.apply(|StartGame()| {
            self.screen = Box::new(GameScreen::new(renderer, &self.assets, events));
        });
        event.apply(|GameOver { score }| {
            self.screen = Box::new(TitleScreen::new(&self.assets, renderer));
        });
    }

    fn process_events(&mut self, renderer: &mut AssetRenderer, events: &mut Events) {
        while let Some(event) = events.pop()
        {
            renderer.on_event(&event, events);
            self.on_event(&event, renderer, events);
            self.screen.on_event(&event, events);
        }
    }
}