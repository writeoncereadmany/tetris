use crate::screens::Screen;
use crate::screens::transitions::{GameOver, StartGame};
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::LEFT;
use engine::renderer::spritefont::VerticalAlignment::BOTTOM;
use rust_libretro::types::JoypadState;

pub struct TitleScreen {
    selection: Selection,
}

enum Selection {
    PlayTetris,
    HighScores,
}

impl TitleScreen {
    pub fn new(assets: &Assets, renderer: &mut AssetRenderer) -> Self {
        let titlescreen = assets.maps.get("titlescreen").unwrap();
        renderer.clear();
        for tile in &titlescreen.tiles {
            renderer.draw_background_tile(&tile.tile_set_name, tile.id, tile.x * 8, tile.y * 8)
        }
        renderer.draw_background_text(
            "Play Tetris",
            "Spritefont_Medium",
            120,
            32,
            Alignment::aligned(LEFT, BOTTOM),
        );
        renderer.draw_background_text(
            "High Scores",
            "Spritefont_Medium",
            120,
            16,
            Alignment::aligned(LEFT, BOTTOM),
        );
        TitleScreen {
            selection: Selection::PlayTetris,
        }
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                match self.selection {
                    Selection::PlayTetris => events.fire(StartGame()),
                    Selection::HighScores => events.fire(GameOver { score: 0 })
                }
            }
            if button == &JoypadState::UP || button == &JoypadState::DOWN {
                match self.selection {
                    Selection::PlayTetris => self.selection = Selection::HighScores,
                    Selection::HighScores => self.selection = Selection::PlayTetris,
                }
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        let y_pos = match self.selection {
            Selection::PlayTetris => 32,
            Selection::HighScores => 16,
        };
        renderer.draw_sprite("cursor", 104, y_pos, false);
    }
}
