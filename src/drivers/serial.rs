use common::pio::*;

use programs::common::*;

pub struct Serial {
    pub port: u16,
    pub irq: u8,
    pub escape: bool,
    pub cursor_control: bool
}

impl Serial {
    pub fn new(port: u16, irq: u8) -> Serial{
        return Serial {
            port: port,
            irq: irq,
            escape: false,
            cursor_control: false
        };
    }
}

impl SessionItem for Serial {
    fn on_irq(&mut self, irq: u8){
        if irq == self.irq {
            unsafe{
                loop {
                    if inb(self.port + 5) & 1 == 1 {
                        break;
                    }
                }

                let mut c = inb(self.port) as char;
                let mut sc = 0;

                if self.escape {
                    self.escape = false;

                    if c == '['{
                        self.cursor_control = true;
                    }

                    c = '\0';
                }else if self.cursor_control {
                    self.cursor_control = false;

                    if c == 'A'{
                        sc = K_UP;
                    }else if c == 'B'{
                        sc = K_DOWN;
                    }else if c == 'C'{
                        sc = K_RIGHT;
                    }else if c == 'D'{
                        sc = K_LEFT;
                    }

                    c = '\0';
                }else if c == '\x1B' {
                    self.escape = true;
                    c = '\0';
                }else if c == '\r' {
                    c = '\n';
                }else if c == '\x7F' {
                    sc = K_BKSP;
                    c = '\0';
                }

                if c != '\0' || sc != 0 {
                    KeyEvent {
                        character: c,
                        scancode: sc,
                        pressed: true
                    }.trigger();
                }
            }
        }
    }
}
