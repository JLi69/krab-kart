use std::collections::HashSet;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/*
 * Events struct is a wrapper around
 * the sdl event module to make working
 * with events easier
 * */

pub struct Events {
	pressed_keys: HashSet<Keycode>,
	event_pump: EventPump,
	pub can_quit: bool 
}

impl Events {
	pub fn new(context: &sdl2::Sdl) -> Events {
		Events { 
			pressed_keys: HashSet::<Keycode>::new(),
			event_pump: context.event_pump().unwrap(),
			can_quit: false
		}
	}

	pub fn update(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit {..} => { self.can_quit = true },
				//Keydown, add keycode to hashset of keys pressed
				Event::KeyDown { keycode: Some(k), .. } => { self.pressed_keys.insert(k); },
				//Key up, remove keycode from hashset of keys pressed
				Event::KeyUp { keycode: Some(k), .. } => { self.pressed_keys.remove(&k); },
				_ => {}
			}
		}
	}

	pub fn key_is_pressed(&self, keycode: Keycode) -> bool {
		self.pressed_keys.contains(&keycode)
	}
}