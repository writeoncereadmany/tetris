use crate::screens::Screen;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use rust_libretro::types::JoypadState;

#[derive(Clone)]
enum Block {
    Purple,
    Orange,
    Green,
    PaleBlue,
    Gold,
    DarkBlue,
    Pink
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

enum Tetromino {
    L,
    R,
    T,
    S,
    Z,
    O,
    I
}

enum Rotation {
    UP,
    RIGHT,
    DOWN,
    LEFT
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

fn blocks(tetromino: &Tetromino, rotation: &Rotation) -> (Block, [(i32, i32); 4]) {
    match tetromino {
        Tetromino::L => {
            match rotation {
                Rotation::UP => (Block::Orange, [(0, 0), (0, -1), (0, -2), (1, -2)]),
                Rotation::RIGHT => (Block::Orange, [(0, -1), (1, -1), (2, -1), (0, -2)]),
                Rotation::DOWN => (Block::Orange, [(1, -1), (1, -2), (1, -3), (0, -1)]),
                Rotation::LEFT => (Block::Orange, [(-1, -2), (0, -2), (1, -2), (1, -1)]),
            }
        }
        Tetromino::R => {
            match rotation {
                Rotation::UP => (Block::DarkBlue, [(1, 0), (1, -1), (1, -2), (0, -2)]),
                Rotation::RIGHT => (Block::DarkBlue, [(0, -1), (0, -2), (1, -2), (2, -2)]),
                Rotation::DOWN => (Block::DarkBlue, [(0, -1), (0, -2), (0, -3), (1, -1)]),
                Rotation::LEFT => (Block::DarkBlue, [(-1, -1), (0, -1), (1, -1), (1, -2)]),
            }
        }
        Tetromino::S => {
            match rotation {
                Rotation::UP | Rotation::DOWN => (Block::Green, [(0, 0), (0, -1), (1, -1), (1, -2)]),
                Rotation::RIGHT | Rotation::LEFT => (Block::Green, [(1, -1), (2, -1), (1, -2), (0, -2)]),
            }
        }
        Tetromino::Z => {
            match rotation {
                Rotation::UP | Rotation::DOWN => (Block::Gold, [(1, 0), (1, -1), (0, -1), (0, -2)]),
                Rotation::RIGHT | Rotation::LEFT => (Block::Gold, [(-1, -1), (0, -1), (0, -2), (1, -2)]),
            }
        }
        Tetromino::T => {
            match rotation {
                Rotation::UP => (Block::PaleBlue, [(0, -1), (0, 0), (-1, -1), (1, -1)]),
                Rotation::RIGHT => (Block::PaleBlue, [(0, -1), (0, 0), (1, -1), (0, -2)]),
                Rotation::DOWN => (Block::PaleBlue, [(0, -1), (1, -1), (-1, -1), (0, -2)]),
                Rotation::LEFT => (Block::PaleBlue, [(0, -1), (-1, -1), (0, -0), (0, -2)]),
            }
        }
        Tetromino::I => {
            match rotation {
                Rotation::UP | Rotation::DOWN=> (Block::Pink, [(0, 0), (0, -1), (0, -2), (0, -3)]),
                Rotation::RIGHT | Rotation::LEFT => (Block::Pink, [(-1, -1), (0, -1), (1, -1), (2, -1)]),
            }
        }
        Tetromino::O => (Block::Purple, [(0, 0), (0, -1), (1, 0), (1, -1)]),
    }
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
            tetromino: Tetromino::L,
            position: (4,19),
            rotation: Rotation::UP,
        }
    }

    fn attempt_move(&mut self, button: &JoypadState) {
        let (x, y) = self.position;
        match button {
            &JoypadState::LEFT => self.position = (x - 1, y),
            &JoypadState::RIGHT => self.position = (x + 1, y),
            &JoypadState::DOWN => self.position = (x, y - 1),
            &JoypadState::A => self.rotation = clockwise(&self.rotation),
            &JoypadState::B => self.rotation = anti_clockwise(&self.rotation),
            &JoypadState::Y => self.tetromino = next(&self.tetromino),
            _ => {}
        }
    }
}

impl Screen for GameScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            self.attempt_move(button);
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        let (block, positions) = blocks(&self.tetromino, &self.rotation);
        let (px, py) = self.position;
        for (bx, by) in positions {
            renderer.draw_sprite(sprite(&block), ((bx + px) * 8) + 120, (by + py) * 8 + 40, false)
        }
    }
}