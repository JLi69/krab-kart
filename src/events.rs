use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::EventPump;
use std::collections::HashSet;

/*
 * Events struct is a wrapper around
 * the sdl event module to make working
 * with events easier
 * */

#[derive(PartialEq, Eq)]
enum ClickState {
    Unheld,
    Held,
    Clicked,
}

pub struct Events {
    pressed_keys: HashSet<Keycode>,
    event_pump: EventPump,
    click_state: ClickState,
    pub mouse_state: MouseState,
    pub can_quit: bool,
}

impl Events {
    pub fn new(context: &sdl2::Sdl) -> Result<Events, String> {
        let new_event_pump = context.event_pump().map_err(|e| e.to_string())?;
        let new_mouse_state = new_event_pump.mouse_state();

        Ok(Events {
            pressed_keys: HashSet::<Keycode>::new(),
            event_pump: new_event_pump,
            click_state: ClickState::Unheld,
            mouse_state: new_mouse_state,
            can_quit: false,
        })
    }

    pub fn update(&mut self) {
        self.mouse_state = self.event_pump.mouse_state();

        if self.mouse_state.left() && self.click_state == ClickState::Unheld {
            self.click_state = ClickState::Held;
        }

        if !self.mouse_state.left() && self.click_state == ClickState::Clicked {
            self.click_state = ClickState::Unheld;
        }

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.can_quit = true,
                //Keydown, add keycode to hashset of keys pressed
                Event::KeyDown {
                    keycode: Some(k),
                    repeat: false,
                    ..
                } => {
                    self.pressed_keys.insert(k);
                }
                //Key up, remove keycode from hashset of keys pressed
                Event::KeyUp {
                    keycode: Some(k), ..
                } => {
                    self.pressed_keys.remove(&k);
                }
                _ => {}
            }
        }
    }

    pub fn key_is_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn key_is_pressed_once(&mut self, keycode: Keycode) -> bool {
        let pressed = self.pressed_keys.contains(&keycode);
        self.pressed_keys.remove(&keycode);
        pressed
    }

    pub fn left_is_clicked(&mut self) -> bool {
        if self.click_state == ClickState::Held {
            self.click_state = ClickState::Clicked;
            return true;
        }

        false
    }
}
