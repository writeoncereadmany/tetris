use crate::game::block::Block;

#[derive(Copy, Clone)]
pub enum Tetromino {
    L,
    R,
    T,
    S,
    Z,
    O,
    I
}

pub fn next_tetromino( ) -> Tetromino {
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

pub fn block(tetronimo: &Tetromino) -> Block {
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

pub fn positions(tetromino: &Tetromino, rotation: &Rotation, (px, py): &(i32, i32)) -> [(i32, i32); 4] {
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

#[derive(Copy, Clone)]
pub enum Rotation {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

pub fn clockwise(rotation: &Rotation) -> Rotation {
    match rotation {
        Rotation::UP => Rotation::RIGHT,
        Rotation::RIGHT => Rotation::DOWN,
        Rotation::DOWN => Rotation::LEFT,
        Rotation::LEFT => Rotation::UP
    }
}

pub fn anti_clockwise(rotation: &Rotation) -> Rotation {
    match rotation {
        Rotation::UP => Rotation::LEFT,
        Rotation::RIGHT => Rotation::UP,
        Rotation::DOWN => Rotation::RIGHT,
        Rotation::LEFT => Rotation::DOWN
    }
}