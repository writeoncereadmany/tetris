use std::time::Duration;
use engine::events::event::{Event, Events};
use engine::renderer::asset_renderer::AssetRenderer;
use crate::screens::Screen;
use crate::screens::transitions::Loaded;

pub struct LoadScreen;

impl Screen for LoadScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|_dt: &Duration| {
            events.fire(Loaded());
        });
    }

    fn draw(&mut self, _renderer: &mut AssetRenderer) {
    }
}