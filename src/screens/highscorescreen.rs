use crate::screens::transitions::{Loaded, StartGame};
use crate::screens::Screen;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::{CENTER, LEFT, RIGHT};
use engine::renderer::spritefont::VerticalAlignment::BOTTOM;
use rust_libretro::types::JoypadState;
use std::cell::RefCell;
use std::rc::Rc;

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
    high_scores: Rc<RefCell<Vec<HighScore>>>,
    new_score_name: Option<String>,
    new_score_index: Option<usize>,
    current_letter: char,
}

impl HighScoreScreen {
    pub fn new(high_scores: Rc<RefCell<Vec<HighScore>>>, score: u32, renderer: &mut AssetRenderer) -> Self {
        renderer.clear();
        let new_score_index = Self::find_score_position(&high_scores, score);
        if let Some(index) = new_score_index {
            let mut updated_scores = high_scores.borrow_mut();
            updated_scores.insert(index, HighScore::new("".to_string(), score));
            updated_scores.truncate(5);
        }
        let new_score_name = new_score_index.map(|index| String::new());
        HighScoreScreen { high_scores, new_score_name, new_score_index, current_letter: 'A' }
    }

    fn find_score_position(high_scores: &Rc<RefCell<Vec<HighScore>>>, score: u32) -> Option<usize> {
        for (index, high_score) in high_scores.borrow().iter().enumerate() {
            if high_score.score < score {
                return Some(index);
            }
        }
        None
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

    fn set_name(&mut self) {
        if let (Some(index), Some(name)) = (self.new_score_index, self.new_score_name.as_ref()) {
            self.high_scores.borrow_mut().get_mut(index).unwrap().name = name.clone();
        }
        self.new_score_name = None;
        self.new_score_index = None;
    }
}

impl Screen for HighScoreScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(| ButtonPressed(button) | {
            if (button == &JoypadState::START) {
                events.fire(StartGame())
            }
            if (button == &JoypadState::SELECT) {
                events.fire(Loaded())
            }
            if (button == &JoypadState::RIGHT || button == &JoypadState::LEFT) {
                self.update_letter(button);
            }
            if (button == &JoypadState::A || button == &JoypadState::B) {
                if self.current_letter == ' ' {
                    self.set_name();
                }
                else {
                    self.new_score_name.as_mut().map(|name| name.push(self.current_letter));
                    if self.new_score_name.as_ref().map(|name| name.len() == 5).unwrap_or(false) {
                        self.set_name();
                    }
                }
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        renderer.draw_text("High Scores", "Spritefont_Medium", 160, 144, Alignment::aligned(CENTER, BOTTOM));
        for (i, HighScore { name, score }) in self.high_scores.borrow().iter().enumerate() {
            renderer.draw_text(&name, "Spritefont_Medium", 80, 120 - (i*16) as i32, Alignment::aligned(LEFT, BOTTOM));
            renderer.draw_text(&format!("{score}"), "Spritefont_Medium", 240, 120 - (i*16) as i32, Alignment::aligned(RIGHT, BOTTOM));
        }
        if let (Some(index), Some(name)) = (self.new_score_index, self.new_score_name.as_ref()) {
            let mut name = name.clone();
            name.push(self.current_letter);
            renderer.draw_text(&name, "Spritefont_Medium", 80, 120 - (index*16) as i32, Alignment::aligned(LEFT, BOTTOM));
        }
    }
}