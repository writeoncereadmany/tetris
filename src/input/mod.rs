use std::collections::HashMap;
use std::time::Duration;
use engine::events::event::{Event, Events};
use engine::events::input::{ButtonPressed, ButtonReleased};
use engine::events::timer::{TimerId, NO_TIMER_ID};
use rust_libretro::types::JoypadState;

#[derive(Clone, Copy)]
pub struct KeyRepeater {
    key: JoypadState,
    initial_repeat: Duration,
    subsequent_repeat: Duration,
    repeat_timer: TimerId
}

impl KeyRepeater {
    pub fn new(key: JoypadState, initial_repeat: Duration, subsequent_repeat: Duration) -> Self {
        KeyRepeater {
            key, initial_repeat, subsequent_repeat, repeat_timer: NO_TIMER_ID
        }
    }
}

pub struct KeysRepeater {
    repeaters: HashMap<JoypadState, KeyRepeater>,
}

impl KeysRepeater {
    pub fn new(repeaters : Vec<KeyRepeater>) -> Self {
        let repeaters: HashMap<JoypadState, KeyRepeater> = repeaters.iter().map(|r| (r.key, *r)).collect();
        Self { repeaters }
    }

    pub fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| self.queue_repeat(button, events));
        event.apply(|ButtonReleased(button)| self.cancel_repeat(button, events));
    }

    fn queue_repeat(&mut self, button: &JoypadState, events: &mut Events) {
        if let Some(repeater) = self.repeaters.get_mut(button) {
            let repeat_duration = if repeater.repeat_timer == NO_TIMER_ID { repeater.initial_repeat } else { repeater.subsequent_repeat };
            repeater.repeat_timer = events.schedule("Application", repeat_duration, ButtonPressed(button.clone()));
        }
    }

    fn cancel_repeat(&mut self, button: &JoypadState, events: &mut Events) {
        if let Some(repeater) = self.repeaters.get_mut(button) {
            events.cancel("Application", &repeater.repeat_timer);
            repeater.repeat_timer = NO_TIMER_ID;
        }
    }
}