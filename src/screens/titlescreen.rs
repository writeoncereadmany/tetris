use crate::screens::transitions::StartGame;
use crate::screens::Screen;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::{Alignment, HorizontalAlignment, VerticalAlignment};
use rust_libretro::types::JoypadState;
use HorizontalAlignment::CENTER;
use VerticalAlignment::MIDDLE;

pub struct TitleScreen;

impl TitleScreen {
    pub fn new(renderer: &mut AssetRenderer) -> Self {
        renderer.clear();
        renderer.draw_background_text("Tetris", "Spritefont_Medium", 160, 100, Alignment::aligned(CENTER, MIDDLE));
        renderer.clear_sprites();
        TitleScreen
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(| ButtonPressed(button) | {
            if (button == &JoypadState::START)
            {
                events.fire(StartGame())
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {

    }
}