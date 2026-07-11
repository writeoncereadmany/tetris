use crate::screens::Screen;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::renderer::asset_renderer::AssetRenderer;

pub struct GameScreen;

impl GameScreen {
    pub fn new(renderer: &mut AssetRenderer, assets: &Assets) -> GameScreen {
        let gameboard = assets.maps.get("gameboard").unwrap();
        renderer.clear();
        for tile in &gameboard.tiles {
            renderer.draw_background_tile(&tile.tile_set_name, tile.id, tile.x * 8, tile.y * 8)
        }
        renderer.clear_sprites();

        GameScreen
    }
}

impl Screen for GameScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {

    }
}