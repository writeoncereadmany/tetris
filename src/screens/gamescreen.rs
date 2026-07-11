use derive::Event;
use crate::screens::Screen;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use rand::{Rng, RngExt};
use rust_libretro::types::JoypadState;

#[derive(Event)]
struct NextPiecePlease();

#[derive(Copy, Clone)]
enum Block {
    Purple,
    Orange,
    Green,
    PaleBlue,
    Gold,
    DarkBlue,
    Pink
}

#[derive(Event, Eq, PartialEq)]
enum Action {
    Left,
    Right,
    Down,
    RotateClockwise,
    RotateAnticlockwise,
    ChangeTetromino // only for debugging purposes - we'll revisit this!
}

fn sprite(block: &Block) -> &'static str {
    match block {
        Block::Purple => "purple",
        Block::Orange => "orange",
        Block::Green => "green",
        Block::PaleBlue => "pale_blue",
        Block::Gold => "gold",
        Block::DarkBlue => "dark_blue",
        Block::Pink => "pink"
    }
}

#[derive(Copy, Clone)]
enum Tetromino {
    L,
    R,
    T,
    S,
    Z,
    O,
    I
}

fn next_tetromino( ) -> Tetromino {
    match rand::random_range(0..7) {
        0 => Tetromino::L,
        1 => Tetromino::R,
        2 => Tetromino::T,
        3 => Tetromino::S,
        4 => Tetromino::Z,
        5 => Tetromino::O,
        6 => Tetromino::I,
        __ => panic!("Out of range for tetromino")
    }
}

#[derive(Copy, Clone)]
enum Rotation {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn block(tetronimo: &Tetromino) -> Block {
    match tetronimo {
        Tetromino::L => Block::Orange,
        Tetromino::R => Block::DarkBlue,
        Tetromino::T => Block::PaleBlue,
        Tetromino::S => Block::Green,
        Tetromino::Z => Block::Purple,
        Tetromino::O => Block::Gold,
        Tetromino::I => Block::Pink
    }
}

fn clockwise(rotation: &Rotation) -> Rotation {
    match rotation {
        Rotation::UP => Rotation::RIGHT,
        Rotation::RIGHT => Rotation::DOWN,
        Rotation::DOWN => Rotation::LEFT,
        Rotation::LEFT => Rotation::UP
    }
}

fn anti_clockwise(rotation: &Rotation) -> Rotation {
    match rotation {
        Rotation::UP => Rotation::LEFT,
        Rotation::RIGHT => Rotation::UP,
        Rotation::DOWN => Rotation::RIGHT,
        Rotation::LEFT => Rotation::DOWN
    }
}

fn positions(tetromino: &Tetromino, rotation: &Rotation, (px, py): &(i32, i32)) -> [(i32, i32); 4] {
    let positions = match tetromino {
        Tetromino::L => {
            match rotation {
                Rotation::UP => [(0, 0), (0, -1), (0, -2), (1, -2)],
                Rotation::RIGHT => [(0, -1), (1, -1), (2, -1), (0, -2)],
                Rotation::DOWN => [(1, -1), (1, -2), (1, -3), (0, -1)],
                Rotation::LEFT => [(-1, -2), (0, -2), (1, -2), (1, -1)],
            }
        }
        Tetromino::R => {
            match rotation {
                Rotation::UP => [(1, 0), (1, -1), (1, -2), (0, -2)],
                Rotation::RIGHT => [(0, -1), (0, -2), (1, -2), (2, -2)],
                Rotation::DOWN => [(0, -1), (0, -2), (0, -3), (1, -1)],
                Rotation::LEFT => [(-1, -1), (0, -1), (1, -1), (1, -2)],
            }
        }
        Tetromino::S => {
            match rotation {
                Rotation::UP | Rotation::DOWN => [(0, 0), (0, -1), (1, -1), (1, -2)],
                Rotation::RIGHT | Rotation::LEFT => [(1, -1), (2, -1), (1, -2), (0, -2)],
            }
        }
        Tetromino::Z => {
            match rotation {
                Rotation::UP | Rotation::DOWN => [(1, 0), (1, -1), (0, -1), (0, -2)],
                Rotation::RIGHT | Rotation::LEFT => [(-1, -1), (0, -1), (0, -2), (1, -2)],
            }
        }
        Tetromino::T => {
            match rotation {
                Rotation::UP => [(0, -1), (0, 0), (-1, -1), (1, -1)],
                Rotation::RIGHT => [(0, -1), (0, 0), (1, -1), (0, -2)],
                Rotation::DOWN => [(0, -1), (1, -1), (-1, -1), (0, -2)],
                Rotation::LEFT => [(0, -1), (-1, -1), (0, -0), (0, -2)],
            }
        }
        Tetromino::I => {
            match rotation {
                Rotation::UP | Rotation::DOWN=> [(0, 0), (0, -1), (0, -2), (0, -3)],
                Rotation::RIGHT | Rotation::LEFT => [(-1, -1), (0, -1), (1, -1), (2, -1)],
            }
        }
        Tetromino::O => [(0, 0), (0, -1), (1, 0), (1, -1)],
    };
    positions.map(|(x, y)| (x + px, y + py))
}

fn next(tetronimo : &Tetromino) -> Tetromino {
    match tetronimo {
        Tetromino::L => Tetromino::R,
        Tetromino::R => Tetromino::T,
        Tetromino::T => Tetromino::S,
        Tetromino::S => Tetromino::Z,
        Tetromino::Z => Tetromino::O,
        Tetromino::O => Tetromino::I,
        Tetromino::I => Tetromino::L
    }
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
            tetromino: next_tetromino(),
            position: (4,19),
            rotation: Rotation::UP,
        }
    }

    fn listen_to_input(&mut self, button: &JoypadState, events: &mut Events) {
        let (x, y) = self.position;
        match button {
            &JoypadState::LEFT => events.fire(Action::Left),
            &JoypadState::RIGHT => events.fire(Action::Right),
            &JoypadState::DOWN => events.fire(Action::Down),
            &JoypadState::A => events.fire(Action::RotateClockwise),
            &JoypadState::B => events.fire(Action::RotateAnticlockwise),
            &JoypadState::Y => events.fire(Action::ChangeTetromino),
            _ => {}
        }
    }

    fn attempt_move(&mut self, action: &Action, events: &mut Events) {
        let (mut x, mut y) = self.position;
        let mut rotation = self.rotation;
        let mut tetromino = self.tetromino;
        match action {
            &Action::Left => x = x - 1,
            &Action::Right => x  = x + 1,
            &Action::Down => y = y - 1,
            &Action::RotateClockwise => rotation = clockwise(&rotation),
            &Action::RotateAnticlockwise => rotation = anti_clockwise(&rotation),
            &Action::ChangeTetromino => tetromino = next(&tetromino),
        }
        let new_positions = positions(&tetromino, &rotation, &(x, y));

        if new_positions.iter().all(|position| self.is_available(position)) {
            self.position = (x, y);
            self.rotation = rotation;
            self.tetromino = tetromino;
        }
        else
        {
            if action == &Action::Down {
                let old_positions = positions(&self.tetromino, &self.rotation, &self.position);
                let block = block(&self.tetromino);
                for (x, y) in old_positions {
                    self.well[y as usize][x as usize] = Some(block);
                }
                events.fire(NextPiecePlease())
            }
        }
    }

    fn is_available(&self, &(x, y): &(i32, i32)) -> bool {
        x >= 0 && x < 10 && y >= 0 && self.well[y as usize][x as usize].is_none()
    }
}

fn draw_block(renderer: &mut AssetRenderer, block: &Block, x: i32, y: i32) {
    renderer.draw_sprite(sprite(&block), x * 8 + 120, y * 8 + 40, false)
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
            self.tetromino = next_tetromino();
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        let block = block(&self.tetromino);
        let positions = positions(&self.tetromino, &self.rotation, &self.position);
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