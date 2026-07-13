use crate::game::block::Block;
use crate::game::tetromino::{Rotation, Tetromino};
use crate::game::{block, tetromino};
use crate::input::{KeyRepeater, KeysRepeater};
use crate::screens::transitions::GameOver;
use crate::screens::Screen;
use derive::Event;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::events::timer::TimerId;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::{Alignment, HorizontalAlignment, VerticalAlignment};
use rust_libretro::types::JoypadState;
use std::time::{Duration, Instant, SystemTime};

#[derive(Event)]
struct NextPiecePlease();

#[derive(Event)]
struct CheckForLines();

#[derive(Event)]
struct RemoveLines(Vec<usize>);

#[derive(Event)]
struct ClearLines();

#[derive(Event, Eq, PartialEq)]
enum Action {
    Left,
    Right,
    Down,
    RotateClockwise,
    RotateAnticlockwise,
    HoldTetromino,
}

const PULSE_FRAMES: [&'static str; 6] = ["pulse_1", "pulse_2", "pulse_3", "pulse_4", "pulse_3", "pulse_2"];

pub struct GameScreen {
    well: Vec<Vec<Option<Block>>>, // outer vec is Y coord, inner is X, to simplify line removal
    tetromino: Tetromino,
    next_tetromino: Tetromino,
    held_tetromino: Option<Tetromino>,
    lines_being_removed: Vec<usize>,
    game_over: bool,
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
            next_tetromino: tetromino::next_tetromino(),
            held_tetromino: None,
            lines_being_removed: Vec::new(),
            game_over: false,
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
            level: 1
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
            &JoypadState::Y => events.fire(Action::HoldTetromino),
            _ => {}
        }
    }

    fn attempt_move(&mut self, action: &Action, events: &mut Events) {
        if !self.lines_being_removed.is_empty() {
            return;
        }

        let (mut x, mut y) = self.position;
        let mut rotation = self.rotation;
        let mut tetromino = self.tetromino;
        match action {
            &Action::Left => x = x - 1,
            &Action::Right => x = x + 1,
            &Action::Down => y = y - 1,
            &Action::RotateClockwise => rotation = tetromino::clockwise(&rotation),
            &Action::RotateAnticlockwise => rotation = tetromino::anti_clockwise(&rotation),
            &Action::HoldTetromino => tetromino = self.held_tetromino.unwrap_or(self.next_tetromino),
        }
        let new_positions = tetromino::positions(&tetromino, &rotation, &(x, y));

        if new_positions.iter().all(|position| self.is_available(position))
        {
            if action == &Action::HoldTetromino {
                let current = self.tetromino;
                if let Some(held) = self.held_tetromino {
                    self.tetromino = held;
                    self.held_tetromino = Some(current);
                } else {
                    self.tetromino = self.next_tetromino;
                    self.next_tetromino = tetromino::next_tetromino();
                    self.held_tetromino = Some(current);
                }
            }
            else {
                self.position = (x, y);
                self.rotation = rotation;

                if action == &Action::Down {
                    events.cancel("Game", &self.next_down_timer);
                    self.next_down_timer = events.schedule("Game", Self::drop_time(self.level), Action::Down);
                }
            }
        } else if action == &Action::Down {
            self.set_tetromino(events);
        }
    }

    fn set_tetromino(&mut self, events: &mut Events) {
        events.cancel("Game", &self.next_down_timer);
        let old_positions = tetromino::positions(&self.tetromino, &self.rotation, &self.position);
        let block = tetromino::block(&self.tetromino);
        for (x, y) in old_positions {
            self.well[y as usize][x as usize] = Some(block);
        }
        events.fire(CheckForLines());
    }

    fn is_available(&self, &(x, y): &(i32, i32)) -> bool {
        x >= 0 && x < 10 && y >= 0 && self.well[y as usize][x as usize].is_none()
    }

    fn check_for_lines(&mut self, events: &mut Events) {
        let mut lines_to_remove = Vec::<usize>::new();
        for (index, row) in self.well.iter().enumerate() {
            if row.iter().all(|block| block.is_some()) {
                lines_to_remove.push(index);
            }
        }
        events.fire(RemoveLines(lines_to_remove));
    }

    fn clear_lines(&mut self, events: &mut Events) {
        self.lines += self.lines_being_removed.len() as u32;
        self.score += match self.lines_being_removed.len() {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("Too many lines! Too many lines!") // rip sheamus
        } * (self.level + 1);
        self.level = self.lines / 10 + 1;
        self.lines_being_removed.reverse();
        for index in self.lines_being_removed.iter() {
            self.well.remove(*index);
        }
        while self.well.len() < 20 {
            self.well.push(vec![None; 10]);
        }
        self.lines_being_removed.clear();

        events.fire(NextPiecePlease());
    }

    fn draw_current_tetromino(&mut self, renderer: &mut AssetRenderer) {
        if !self.lines_being_removed.is_empty() {
            return;
        }
        let block = tetromino::block(&self.tetromino);
        let positions = tetromino::positions(&self.tetromino, &self.rotation, &self.position);
        for (x, y) in positions {
            draw_block(renderer, &block, x, y);
        }
    }

    fn draw_next_tetromino(&mut self, renderer: &mut AssetRenderer) {
        let tetromino = self.next_tetromino;
        self.draw_preview_tetromino(tetromino, 80, 128, renderer);
    }

    fn draw_held_tetromino(&mut self, renderer: &mut AssetRenderer) {
        if let Some(tetromino) = self.held_tetromino {
            self.draw_preview_tetromino(tetromino, 80, 64, renderer);
        }
    }

    fn draw_preview_tetromino(&mut self, tetromino: Tetromino, sx:i32, sy: i32, renderer: &mut AssetRenderer) {
        let block = tetromino::block(&tetromino);
        let positions = tetromino::positions(&tetromino, &Rotation::UP, &(0, 0));
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for (x, y) in positions {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        let center_x = (max_x + min_x) * 4;
        let center_y = (max_y + min_y) * 4;
        for (x, y) in positions {
            renderer.draw_sprite(block::sprite(&block), x * 8 + sx - center_x, y * 8 + sy - center_y, false)
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

    fn draw_line_removal(&mut self, renderer: &mut AssetRenderer) {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let pulse_frame = PULSE_FRAMES[(now / 75 % 6) as usize];
        for row in self.lines_being_removed.iter() {
            for x in 0..10 {
                renderer.draw_sprite(pulse_frame, x * 8 + 120, *row as i32 * 8 + 24, false)
            }
        }
    }

    fn draw_stats(&mut self, renderer: &mut AssetRenderer) {
        renderer.draw_text(
            &format!("{}", self.score),
            "Spritefont_Medium",
            280,
            144,
            Alignment::aligned(HorizontalAlignment::RIGHT, VerticalAlignment::BOTTOM)
        );
        renderer.draw_text(
            &format!("{}", self.lines),
            "Spritefont_Medium",
            280,
            104,
            Alignment::aligned(HorizontalAlignment::RIGHT, VerticalAlignment::BOTTOM)
        );
        renderer.draw_text(
            &format!("{}", self.level),
            "Spritefont_Medium",
            280,
            64,
            Alignment::aligned(HorizontalAlignment::RIGHT, VerticalAlignment::BOTTOM)
        );
    }

    fn draw_game_over(&mut self, renderer: &mut AssetRenderer) {
        if self.game_over
        {
            renderer.draw_text(
                "Game Over",
                "Spritefont_Medium",
                160,
                100,
                Alignment::aligned(HorizontalAlignment::CENTER, VerticalAlignment::MIDDLE)
            );
        }
    }

    fn remove_lines(&mut self, lines: Vec<usize>, events: &mut Events) {
        self.lines_being_removed = lines;
        let duration = match self.lines_being_removed.len() {
            0 => Duration::from_millis(0),
            1 => Duration::from_millis(500),
            2 => Duration::from_millis(800),
            3 => Duration::from_millis(1000),
            4 => Duration::from_millis(1500),
            _ => panic!("Too many lines! Too many lines!")
        };
        events.schedule("Game", duration, ClearLines());
    }

    fn next_tetromino_please(&mut self, events: &mut Events) {
        let new_position = (4, 19);
        let new_rotation = Rotation::UP;
        let new_tetromino = self.next_tetromino;
        let next_start_positions = tetromino::positions(&new_tetromino, &new_rotation, &new_position);
        if next_start_positions.iter().all(|position| self.is_available(position)) {
            self.position = (4, 19);
            self.rotation = Rotation::UP;
            self.tetromino = new_tetromino;
            self.next_tetromino = tetromino::next_tetromino();
            self.next_down_timer = events.schedule("Game", Self::drop_time(self.level), Action::Down);
        } else {
            self.game_over = true;
            events.schedule("Game", Duration::from_secs_f64(2.0), GameOver());
        }
    }
}

fn draw_block(renderer: &mut AssetRenderer, block: &Block, x: i32, y: i32) {
    renderer.draw_sprite(block::sprite(&block), x * 8 + 120, y * 8 + 24, false)
}

impl Screen for GameScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        self.key_repeater.on_event(event, events);

        event.apply(|dt| events.elapse("Game", *dt));

        event.apply(|ButtonPressed(button)| self.listen_to_press(button, events));
        event.apply(|action| self.attempt_move(action, events));
        event.apply(|NextPiecePlease()| self.next_tetromino_please(events));
        event.apply(|CheckForLines()| self.check_for_lines(events));
        event.apply(|RemoveLines(lines)| self.remove_lines(lines.clone(), events));
        event.apply(|ClearLines()| self.clear_lines(events));
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        self.draw_current_tetromino(renderer);
        self.draw_next_tetromino(renderer);
        self.draw_held_tetromino(renderer);
        self.draw_well(renderer);
        self.draw_line_removal(renderer);
        self.draw_stats(renderer);
        self.draw_game_over(renderer)
    }
}