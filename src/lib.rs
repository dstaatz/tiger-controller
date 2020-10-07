/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use rdev::{Event, EventType, Key};


#[derive(Debug)]
enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug)]
struct ControlKey {
    key: Key,
    state: KeyState,
}

#[derive(Debug)]
pub struct ControllerState {
    forward: ControlKey,
    reverse: ControlKey,
    right: ControlKey,
    left: ControlKey,
}

impl ControllerState {
    pub fn default() -> Self {
        Self {
            forward: ControlKey {
                key: Key::UpArrow,
                state: KeyState::Released
            },
            reverse: ControlKey {
                key: Key::DownArrow,
                state: KeyState::Released
            },
            right: ControlKey {
                key: Key::RightArrow,
                state: KeyState::Released
            },
            left: ControlKey {
                key: Key::LeftArrow,
                state: KeyState::Released
            },
        }
    }

    pub fn process_event(&mut self, event: Event) {
        
        match event.event_type {
            EventType::KeyPress(key) => {
                if key == self.forward.key {
                    self.forward.state = KeyState::Pressed;
                } else if key == self.reverse.key {
                    self.reverse.state = KeyState::Pressed;
                } else if key == self.right.key {
                    self.right.state = KeyState::Pressed;
                } else if key == self.left.key {
                    self.left.state = KeyState::Pressed;
                }
            },
            EventType::KeyRelease(key) => {
                if key == self.forward.key {
                    self.forward.state = KeyState::Released;
                } else if key == self.reverse.key {
                    self.reverse.state = KeyState::Released;
                } else if key == self.right.key {
                    self.right.state = KeyState::Released;
                } else if key == self.left.key {
                    self.left.state = KeyState::Released;
                }
            },
            _ => (),
        };
    }

    pub fn get_drive(&self) -> f64 {
        match self.forward.state {
            KeyState::Pressed => match self.reverse.state {
                KeyState::Pressed => 0.0,
                KeyState::Released => 1.0,
            },
            KeyState::Released => match self.reverse.state {
                KeyState::Pressed => -1.0,
                KeyState::Released => 0.0,
            },
        }
    }

    pub fn get_steering(&self) -> f64 {
        match self.right.state {
            KeyState::Pressed => match self.left.state {
                KeyState::Pressed => 0.0,
                KeyState::Released => 1.0,
            },
            KeyState::Released => match self.left.state {
                KeyState::Pressed => -1.0,
                KeyState::Released => 0.0,
            },
        }
    }
}

// TODO: tests
