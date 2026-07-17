use crate::application::savegame::{NewHighScore, SavableHighScore, Savegame};
use crate::input::{KeyRepeater, KeysRepeater};
use crate::screens::transitions::{Loaded, StartGame};
use crate::screens::Screen;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::{CENTER, LEFT, RIGHT};
use engine::renderer::spritefont::VerticalAlignment::BOTTOM;
use rust_libretro::types::JoypadState;
use std::time::Duration;

pub struct HighScoreScreen {
    high_scores: Savegame,
    score: u32,
    score_submitted: bool,
    new_score_name: String,
    current_letter: char,
    keys_repeater: KeysRepeater
}

impl HighScoreScreen {
    pub fn new(savegame: Savegame, score: u32, renderer: &mut AssetRenderer) -> Self {
        renderer.clear();
        let new_score_name = String::new();
        HighScoreScreen {
            high_scores: savegame,
            score,
            new_score_name,
            score_submitted: false,
            current_letter: 'A',
            keys_repeater: KeysRepeater::new(vec![
                KeyRepeater::new(JoypadState::LEFT, Duration::from_secs_f64(0.3), Duration::from_secs_f64(0.1)),
                KeyRepeater::new(JoypadState::RIGHT, Duration::from_secs_f64(0.3), Duration::from_secs_f64(0.1)),
            ]),
        }
    }

    fn update_letter(&mut self, input: &JoypadState) {
        if self.current_letter == ' ' {
            if input == &JoypadState::RIGHT {
                self.current_letter = 'A';
            } else if input == &JoypadState::LEFT {
                self.current_letter = 'Z';
            }
            return;
        }

        let mut repr: u32 = self.current_letter.into();
        if input == &JoypadState::RIGHT {
            repr += 1;
        } else if input == &JoypadState::LEFT {
            repr -= 1;
        }
        let mut new_char = char::from_u32(repr).unwrap();
        if new_char < 'A' {
            new_char = ' ';
        }
        if new_char > 'Z' {
            new_char = ' ';
        }
        self.current_letter = new_char;
    }

    fn set_name(&mut self, events: &mut Events) {
        events.fire(NewHighScore(self.new_score_name.clone(), self.score));
        self.score_submitted = true;
    }
}

impl Screen for HighScoreScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        self.keys_repeater.on_event(event, events);
        event.apply(| ButtonPressed(button) | {
            if button == &JoypadState::START {
                events.fire(Loaded())
            }
            if button == &JoypadState::RIGHT || button == &JoypadState::LEFT {
                self.update_letter(button);
            }
            if button == &JoypadState::A || button == &JoypadState::B {
                if self.score_submitted {
                    return;
                }
                if self.current_letter == ' ' {
                    self.set_name(events);
                }
                else {
                        self.new_score_name.push(self.current_letter);
                        if self.new_score_name.len() == 5 {
                            self.set_name(events);
                        }
                }
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        renderer.draw_text("High Scores", "Spritefont_Medium", 160, 144, Alignment::aligned(CENTER, BOTTOM));
        let mut new_score_drawn = false;

        for (i, SavableHighScore { name, score }) in self.high_scores.high_scores.iter().enumerate() {
            if &self.score > score && !new_score_drawn {
                let mut name = self.new_score_name.clone();
                if !self.score_submitted {
                    name.push(self.current_letter);
                }
                renderer.draw_text(&name, "Spritefont_Medium", 100, 120 - (i * 16) as i32, Alignment::aligned(LEFT, BOTTOM));
                renderer.draw_text(&format!("{}", self.score), "Spritefont_Medium", 220, 120 - (i*16) as i32, Alignment::aligned(RIGHT, BOTTOM));
                new_score_drawn = true;
            }

            let i = i + if new_score_drawn { 1 } else { 0 };

            if i < 5 {
                renderer.draw_text(&str::from_utf8(name).unwrap(), "Spritefont_Medium", 100, 120 - (i*16) as i32, Alignment::aligned(LEFT, BOTTOM));
                renderer.draw_text(&format!("{score}"), "Spritefont_Medium", 220, 120 - (i*16) as i32, Alignment::aligned(RIGHT, BOTTOM));
            }
        }
    }
}