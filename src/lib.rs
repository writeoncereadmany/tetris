pub mod application;
pub mod screens;
pub mod game;

use engine::events::event::Events;
use engine::retroarch::RetroarchCore;
use rust_libretro::retro_core;

retro_core!(RetroarchCore::<application::tetris::Tetris> {
    application: None,
    renderer: None,
    events: Events::new()
});
