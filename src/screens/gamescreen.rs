use crate::game::block::Block;
use crate::game::tetromino::{Rotation, Tetromino};
use crate::game::{block, tetromino};
use crate::screens::Screen;
use derive::Event;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use rust_libretro::types::JoypadState;

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
}

impl GameScreen {
    pub fn new(renderer: &mut AssetRenderer, assets: &Assets) -> GameScreen {
        let gameboard = assets.maps.get("gameboard").unwrap();
        renderer.clear();
        for tile in &gameboard.tiles {
            renderer.draw_background_tile(&tile.tile_set_name, tile.id, tile.x * 8, tile.y * 8)
        }
        renderer.clear_sprites();

        GameScreen {
            well: vec![vec![None; 10]; 20],
            tetromino: tetromino::next_tetromino(),
            position: (4, 19),
            rotation: Rotation::UP,
        }
    }

    fn listen_to_input(&mut self, button: &JoypadState, events: &mut Events) {
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
        let mut tetromino = self.tetromino;
        match action {
            &Action::Left => x = x - 1,
            &Action::Right => x = x + 1,
            &Action::Down => y = y - 1,
            &Action::RotateClockwise => rotation = tetromino::clockwise(&rotation),
            &Action::RotateAnticlockwise => rotation = tetromino::anti_clockwise(&rotation),
        }
        let new_positions = tetromino::positions(&tetromino, &rotation, &(x, y));

        if new_positions.iter().all(|position| self.is_available(position))
        {
            self.position = (x, y);
            self.rotation = rotation;
            self.tetromino = tetromino;
        } else if action == &Action::Down {
            self.set_tetromino(events);
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
        self.well
            .retain(|row| row.iter().any(|item| item.is_none()));
        while self.well.len() < 20 {
            self.well.push(vec![None; 10]);
        }
    }
}

fn draw_block(renderer: &mut AssetRenderer, block: &Block, x: i32, y: i32) {
    renderer.draw_sprite(block::sprite(&block), x * 8 + 120, y * 8 + 40, false)
}

impl Screen for GameScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            self.listen_to_input(button, events);
        });
        event.apply(|action| self.attempt_move(action, events));
        event.apply(|NextPiecePlease()| {
            self.position = (4, 19);
            self.rotation = Rotation::UP;
            self.tetromino = tetromino::next_tetromino();
        });
        event.apply(|CheckForLines()| self.check_for_lines());
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        let block = tetromino::block(&self.tetromino);
        let positions = tetromino::positions(&self.tetromino, &self.rotation, &self.position);
        for (x, y) in positions {
            draw_block(renderer, &block, x, y);
        }
        for (y, row) in self.well.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if let Some(block) = block {
                    draw_block(renderer, &block, x as i32, y as i32)
                }
            }
        }
    }
}