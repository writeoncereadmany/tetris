use crate::screens::transitions::{GameOver, StartGame};
use crate::screens::Screen;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::{CENTER, LEFT, RIGHT};
use engine::renderer::spritefont::VerticalAlignment::BOTTOM;
use rust_libretro::types::JoypadState;
use std::sync::Arc;

pub struct HighScore {
    name: String,
    score: u32
}

impl HighScore {
    pub fn new(name: String, score: u32) -> Self {
        Self { name, score }
    }
}

pub struct HighScoreScreen {
    high_scores: Arc<Vec<HighScore>>
}

impl HighScoreScreen {
    pub fn new(high_scores: Arc<Vec<HighScore>>, score: u32, renderer: &mut AssetRenderer) -> Self {
        renderer.clear();
        HighScoreScreen { high_scores }
    }
}

impl Screen for HighScoreScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(| ButtonPressed(button) | {
            if (button == &JoypadState::START) {
                events.fire(StartGame())
            }
            if (button == &JoypadState::SELECT) {
                events.fire(GameOver { score: 0 })
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        renderer.draw_text("High Scores", "Spritefont_Medium", 160, 144, Alignment::aligned(CENTER, BOTTOM));
        for (i, HighScore { name, score }) in self.high_scores.iter().enumerate() {
            renderer.draw_text(&name, "Spritefont_Medium", 80, 120 - (i*16) as i32, Alignment::aligned(LEFT, BOTTOM));
            renderer.draw_text(&format!("{score}"), "Spritefont_Medium", 240, 120 - (i*16) as i32, Alignment::aligned(RIGHT, BOTTOM));
        }
    }
}