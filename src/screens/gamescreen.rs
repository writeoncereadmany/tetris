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
    LEFT,
    DOWN,
    RIGHT
}

fn clockwise(rotation: &Rotation) -> Rotation {
    match rotation {
        Rotation::UP => Rotation::LEFT,
        Rotation::LEFT => Rotation::DOWN,
        Rotation::DOWN => Rotation::RIGHT,
        Rotation::RIGHT => Rotation::UP
    }
}

fn anti_clockwise(rotation: &Rotation) -> Rotation {
    match rotation {
        Rotation::UP => Rotation::RIGHT,
        Rotation::LEFT => Rotation::UP,
        Rotation::DOWN => Rotation::LEFT,
        Rotation::RIGHT => Rotation::DOWN
    }
}

fn blocks(tetromino: &Tetromino, rotation: &Rotation) -> (Block, [(i32, i32); 4]) {
    match tetromino {
        Tetromino::L => {
            match rotation {
                Rotation::UP => (Block::Orange, [(0, 0), (0, -1), (0, -2), (1, -2)]),
                Rotation::LEFT => (Block::Orange, [(0, -1), (1, -1), (2, -1), (0, -2)]),
                Rotation::DOWN => (Block::Orange, [(1, -1), (1, -2), (1, -3), (0, -1)]),
                Rotation::RIGHT => (Block::Orange, [(-1, -2), (0, -2), (1, -2), (1, -1)]),
            }
        }
        _otherwise => (Block::Purple, [(0, 0), (0, -1), (1, 0), (1, -1)]),
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