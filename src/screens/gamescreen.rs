use std::time::Duration;
use crate::game::block::Block;
use crate::game::tetromino::{Rotation, Tetromino};
use crate::game::{block, tetromino};
use crate::screens::Screen;
use derive::Event;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::events::timer::TimerId;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::{Alignment, HorizontalAlignment, VerticalAlignment};
use rust_libretro::types::JoypadState;
use crate::input::{KeyRepeater, KeysRepeater};
use crate::screens::transitions::GameOver;

#[derive(Event)]
struct NextPiecePlease();

#[derive(Event)]
struct CheckForLines();

#[derive(Event, Eq, PartialEq)]
enum Action {
    Left,
    Right,
    Down,
    RotateClockwise,
    RotateAnticlockwise,
}

pub struct GameScreen {
    well: Vec<Vec<Option<Block>>>, // outer vec is Y coord, inner is X, to simplify line removal
    tetromino: Tetromino,
    position: (i32, i32),
    rotation: Rotation,
    next_down_timer: TimerId,
    key_repeater: KeysRepeater,
    score: u32,
    lines: u32,
    level: u32
}

impl GameScreen {
    pub fn new(renderer: &mut AssetRenderer, assets: &Assets, events: &mut Events) -> GameScreen {
        let gameboard = assets.maps.get("gameboard").unwrap();
        renderer.clear();
        for tile in &gameboard.tiles {
            renderer.draw_background_tile(&tile.tile_set_name, tile.id, tile.x * 8, tile.y * 8)
        }
        renderer.clear_sprites();

        let next_down_timer = events.schedule("Game", Self::drop_time(1), Action::Down);

        GameScreen {
            well: vec![vec![None; 10]; 20],
            tetromino: tetromino::next_tetromino(),
            position: (4, 19),
            rotation: Rotation::UP,
            next_down_timer,
            key_repeater: KeysRepeater::new(vec![
                KeyRepeater::new(JoypadState::LEFT, Duration::from_secs_f64(0.2), Duration::from_secs_f64(0.06)),
                KeyRepeater::new(JoypadState::RIGHT, Duration::from_secs_f64(0.2), Duration::from_secs_f64(0.06)),
                KeyRepeater::new(JoypadState::DOWN, Duration::from_secs_f64(0.06), Duration::from_secs_f64(0.06)),
            ]),
            score: 0,
            lines: 0,
            level: 0
        }
    }

    fn drop_time(level: u32) -> Duration {
        match level {
            1 => Duration::from_millis(1_000),
            2 => Duration::from_millis(750),
            3 => Duration::from_millis(500),
            4 => Duration::from_millis(400),
            5 => Duration::from_millis(325),
            6 => Duration::from_millis(250),
            7 => Duration::from_millis(200),
            8 => Duration::from_millis(150),
            9 => Duration::from_millis(125),
            10 => Duration::from_millis(100),
            11 => Duration::from_millis(80),
            _ => Duration::from_millis(60),
        }

    }

    fn listen_to_press(&mut self, button: &JoypadState, events: &mut Events) {
        match button {
            &JoypadState::LEFT => events.fire(Action::Left),
            &JoypadState::RIGHT => events.fire(Action::Right),
            &JoypadState::DOWN => events.fire(Action::Down),
            &JoypadState::A => events.fire(Action::RotateClockwise),
            &JoypadState::B => events.fire(Action::RotateAnticlockwise),
            _ => {}
        }
    }

    fn attempt_move(&mut self, action: &Action, events: &mut Events) {
        let (mut x, mut y) = self.position;
        let mut rotation = self.rotation;
        match action {
            &Action::Left => x = x - 1,
            &Action::Right => x = x + 1,
            &Action::Down => y = y - 1,
            &Action::RotateClockwise => rotation = tetromino::clockwise(&rotation),
            &Action::RotateAnticlockwise => rotation = tetromino::anti_clockwise(&rotation),
        }
        let new_positions = tetromino::positions(&self.tetromino, &rotation, &(x, y));

        if new_positions.iter().all(|position| self.is_available(position))
        {
            self.position = (x, y);
            self.rotation = rotation;
        } else if action == &Action::Down {
            self.set_tetromino(events);
        }

        if action == &Action::Down {
            events.cancel("Game", &self.next_down_timer);
            self.next_down_timer = events.schedule("Game", Self::drop_time(self.level), Action::Down);
        }
    }

    fn set_tetromino(&mut self, events: &mut Events) {
        let old_positions = tetromino::positions(&self.tetromino, &self.rotation, &self.position);
        let block = tetromino::block(&self.tetromino);
        for (x, y) in old_positions {
            self.well[y as usize][x as usize] = Some(block);
        }
        events.fire(CheckForLines());
        events.fire(NextPiecePlease());
    }

    fn is_available(&self, &(x, y): &(i32, i32)) -> bool {
        x >= 0 && x < 10 && y >= 0 && self.well[y as usize][x as usize].is_none()
    }

    fn check_for_lines(&mut self) {
        let mut lines_to_remove = Vec::<usize>::new();
        for (index, row) in self.well.iter().enumerate() {
            if row.iter().all(|block| block.is_some()) {
                lines_to_remove.push(index);
            }
        }
        self.lines += lines_to_remove.len() as u32;
        self.score += match lines_to_remove.len() {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0
        };
        self.level = self.lines / 10 + 1;
        lines_to_remove.reverse();
        for index in lines_to_remove {
            self.well.remove(index);
        }
        while self.well.len() < 20 {
            self.well.push(vec![None; 10]);
        }
    }

    fn draw_current_tetromino(&mut self, renderer: &mut AssetRenderer) {
        let block = tetromino::block(&self.tetromino);
        let positions = tetromino::positions(&self.tetromino, &self.rotation, &self.position);
        for (x, y) in positions {
            draw_block(renderer, &block, x, y);
        }
    }

    fn draw_well(&mut self, renderer: &mut AssetRenderer) {
        for (y, row) in self.well.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if let Some(block) = block {
                    draw_block(renderer, &block, x as i32, y as i32)
                }
            }
        }
    }

    fn draw_stats(&mut self, renderer: &mut AssetRenderer) {
        renderer.draw_text(
            &format!("{}", self.score),
            "Spritefont_Medium",
            280,
            160,
            Alignment::aligned(HorizontalAlignment::RIGHT, VerticalAlignment::BOTTOM)
        );
        renderer.draw_text(
            &format!("{}", self.lines),
            "Spritefont_Medium",
            280,
            120,
            Alignment::aligned(HorizontalAlignment::RIGHT, VerticalAlignment::BOTTOM)
        );
        renderer.draw_text(
            &format!("{}", self.level),
            "Spritefont_Medium",
            280,
            80,
            Alignment::aligned(HorizontalAlignment::RIGHT, VerticalAlignment::BOTTOM)
        );
    }

    fn next_tetromino_please(&mut self, events: &mut Events) {
        let new_position = (4, 19);
        let new_rotation = Rotation::UP;
        let new_tetromino = tetromino::next_tetromino();
        let next_start_positions = tetromino::positions(&new_tetromino, &new_rotation, &new_position);
        if next_start_positions.iter().all(|position| self.is_available(position)) {
            self.position = (4, 19);
            self.rotation = Rotation::UP;
            self.tetromino = tetromino::next_tetromino();
        } else {
            events.fire(GameOver());
        }
    }
}

fn draw_block(renderer: &mut AssetRenderer, block: &Block, x: i32, y: i32) {
    renderer.draw_sprite(block::sprite(&block), x * 8 + 120, y * 8 + 40, false)
}

impl Screen for GameScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        self.key_repeater.on_event(event, events);

        event.apply(|dt| events.elapse("Game", *dt));

        event.apply(|ButtonPressed(button)| self.listen_to_press(button, events));
        event.apply(|action| self.attempt_move(action, events));
        event.apply(|NextPiecePlease()| self.next_tetromino_please(events));
        event.apply(|CheckForLines()| self.check_for_lines());
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        self.draw_current_tetromino(renderer);
        self.draw_well(renderer);
        self.draw_stats(renderer);
    }
}