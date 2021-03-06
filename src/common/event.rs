use core::char;

use common::string::*;

use syscall::call::*;

pub enum EventOption {
    Mouse(MouseEvent),
    Key(KeyEvent),
    Redraw(RedrawEvent),
    Open(OpenEvent),
    Unknown(Event),
    None
}

#[derive(Copy, Clone)]
pub struct Event {
    pub code: char,
    pub a: isize,
    pub b: isize,
    pub c: isize,
    pub d: isize,
    pub e: isize
}

impl Event {
    pub fn to_option(self) -> EventOption {
        match self.code {
            'm' => EventOption::Mouse(MouseEvent::from_event(self)),
            'k' => EventOption::Key(KeyEvent::from_event(self)),
            'r' => EventOption::Redraw(RedrawEvent::from_event(self)),
            'o' => EventOption::Open(OpenEvent::from_event(self)),
            '\0' => EventOption::None,
            _ => EventOption::Unknown(self)
        }
    }

    pub fn trigger(&self){
        unsafe{
            sys_trigger(self);
        }
    }
}

#[derive(Copy, Clone)]
pub struct MouseEvent {
    pub x: isize,
    pub y: isize,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub valid: bool
}

impl MouseEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: 'm',
            a: self.x,
            b: self.y,
            c: self.left_button as isize,
            d: self.middle_button as isize,
            e: self.right_button as isize
        }
    }

    pub fn from_event(event: Event) -> MouseEvent {
        MouseEvent {
            x: event.a,
            y: event.b,
            left_button: event.c > 0,
            middle_button: event.d > 0,
            right_button: event.e > 0,
            valid: true
        }
    }

    pub fn trigger(&self){
        self.to_event().trigger();
    }
}

pub const K_ESC: u8 = 0x01;
pub const K_BKSP: u8 = 0x0E;
pub const K_TAP: u8 = 0x0F;
pub const K_CTRL: u8 = 0x1D;
pub const K_ALT: u8 = 0x38;
pub const K_F1: u8 = 0x3B;
pub const K_F2: u8 = 0x3C;
pub const K_F3: u8 = 0x3D;
pub const K_F4: u8 = 0x3E;
pub const K_F5: u8 = 0x3F;
pub const K_F6: u8 = 0x40;
pub const K_F7: u8 = 0x41;
pub const K_F8: u8 = 0x42;
pub const K_F9: u8 = 0x43;
pub const K_F10: u8 = 0x44;
pub const K_HOME: u8 = 0x47;
pub const K_UP: u8 = 0x48;
pub const K_PGUP: u8 = 0x49;
pub const K_LEFT: u8 = 0x4B;
pub const K_RIGHT: u8 = 0x4D;
pub const K_END: u8 = 0x4F;
pub const K_DOWN: u8 = 0x50;
pub const K_PGDN: u8 = 0x51;
pub const K_DEL: u8 = 0x53;
pub const K_F11: u8 = 0x57;
pub const K_F12: u8 = 0x58;

#[derive(Copy, Clone)]
pub struct KeyEvent {
    pub character: char,
    pub scancode: u8,
    pub pressed: bool
}

impl KeyEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: 'k',
            a: self.character as isize,
            b: self.scancode as isize,
            c: self.pressed as isize,
            d: 0,
            e: 0
        }
    }

    pub fn from_event(event: Event) -> KeyEvent {
        match char::from_u32(event.a as u32) {
            Option::Some(character) => KeyEvent {
                character: character,
                scancode: event.b as u8,
                pressed: event.c > 0,
            },
            Option::None => KeyEvent {
                character: '\0',
                scancode: event.b as u8,
                pressed: event.c > 0,
            }
        }
    }

    pub fn trigger(&self){
        self.to_event().trigger();
    }
}

pub const REDRAW_NONE: usize = 0;
pub const REDRAW_CURSOR: usize = 1;
pub const REDRAW_ALL: usize = 2;

pub struct RedrawEvent {
    pub redraw: usize
}

impl RedrawEvent {
    pub fn to_event(&self) -> Event {
        Event {
            code: 'r',
            a: self.redraw as isize,
            b: 0,
            c: 0,
            d: 0,
            e: 0
        }
    }

    pub fn from_event(event: Event) -> RedrawEvent {
        RedrawEvent {
            redraw: event.a as usize
        }
    }

    pub fn trigger(&self){
        self.to_event().trigger();
    }
}

pub struct OpenEvent {
    pub url_string: String
}

impl OpenEvent {
    pub fn to_event(&self) -> Event {
        unsafe{
            Event {
                code: 'o',
                a: self.url_string.to_c_str() as isize,
                b: 0,
                c: 0,
                d: 0,
                e: 0
            }
        }
    }

    pub fn from_event(event: Event) -> OpenEvent {
        unsafe{
            let ret = OpenEvent {
                url_string: String::from_c_str(event.a as *const u8)
            };
            sys_unalloc(event.a as usize);
            return ret;
        }
    }

    pub fn trigger(&self){
        self.to_event().trigger();
    }
}
