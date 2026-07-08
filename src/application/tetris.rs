use std::sync::Arc;
use engine::assets::Assets;
use engine::events::event::Events;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::{Alignment, HorizontalAlignment, VerticalAlignment};
use engine::retroarch::{Application, ApplicationProperties};
use rust_libretro::contexts::AudioContext;
use rust_libretro::input_descriptors;
use rust_libretro::sys::{retro_input_descriptor, RETRO_DEVICE_ID_JOYPAD_A, RETRO_DEVICE_ID_JOYPAD_B, RETRO_DEVICE_ID_JOYPAD_DOWN, RETRO_DEVICE_ID_JOYPAD_LEFT, RETRO_DEVICE_ID_JOYPAD_RIGHT, RETRO_DEVICE_ID_JOYPAD_START, RETRO_DEVICE_ID_JOYPAD_UP, RETRO_DEVICE_JOYPAD};
use rust_libretro::types::JoypadState;

pub struct Tetris {
    _assets: Arc<Assets>,
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
            _assets: assets.clone(),
        }

    }

    fn update(&mut self, input: JoypadState, delta_time: u64, renderer: &mut AssetRenderer, events: &mut Events) {
        // nothing yet
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        let gameboard = self._assets.maps.get("gameboard");
        renderer.clear();
        for tile in &gameboard.unwrap().tiles {
            renderer.draw_background_tile(&tile.tile_set_name, tile.id, tile.x * 8, tile.y * 8)
        }
        renderer.clear_sprites();
    }

    fn play(&mut self, _ctx: &mut AudioContext) {
        // nothing yet
    }

    fn properties() -> ApplicationProperties {
        ApplicationProperties {
            width: 320,
            height: 240,
            name: "Tetris".to_string(),
            extensions: &["ttr"],
            input_descriptors: INPUT_DESCRIPTORS,
        }
    }
}