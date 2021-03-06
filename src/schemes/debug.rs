use common::scheduler::*;

use programs::common::*;

pub struct DebugResource;

impl Resource for DebugResource {
    fn url(&self) -> URL {
        return URL::from_str("debug://");
    }

    fn stat(&self) -> ResourceType {
        return ResourceType::File;
    }

    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        unsafe {
            loop {
                let reenable = start_no_ints();

                if (*::debug_command).len() > 0 {
                    break;
                }

                end_no_ints(reenable);

                sys_yield();
            }

            let reenable = start_no_ints();

            //TODO: Unicode
            let mut i = 0;
            while i < buf.len() {
                match (*::debug_command).vec.remove(0) {
                    Option::Some(c) => buf[i] = c as u8,
                    Option::None => break
                }
                i += 1;
            }

            end_no_ints(reenable);

            return Option::Some(i);
        }
    }

    fn write(&mut self, buf: &[u8]) -> Option<usize> {
        for byte in buf {
            unsafe{
                sys_debug(*byte);
            }
        }
        return Option::Some(buf.len());
    }

    fn seek(&mut self, pos: ResourceSeek) -> Option<usize> {
        return Option::None;
    }

    fn flush(&mut self) -> bool {
        return true;
    }
}

pub struct DebugScheme;

impl SessionItem for DebugScheme {
    fn scheme(&self) -> String {
        return "debug".to_string();
    }

    fn open(&mut self, url: &URL) -> Box<Resource>{
        return box DebugResource;
    }
}
