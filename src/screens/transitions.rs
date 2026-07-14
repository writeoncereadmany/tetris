use derive::Event;

#[derive(Event)]
pub struct StartGame();

#[derive(Event)]
pub struct Loaded();

#[derive(Event)]
pub struct GameOver { pub score: u32 }
