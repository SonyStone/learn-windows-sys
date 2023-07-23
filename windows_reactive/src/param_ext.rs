use windows::Win32::Foundation::{LPARAM, WPARAM};

pub trait ParamExt {
    fn get_loword(self) -> u32;
    fn get_hiword(self) -> u32;
    fn get_x(self) -> i32;
    fn get_y(self) -> i32;
}

impl ParamExt for WPARAM {
    fn get_loword(self: WPARAM) -> u32 {
        (self.0 & 0xffff) as u32
    }

    fn get_hiword(self: WPARAM) -> u32 {
        ((self.0 >> 16) & 0xffff) as u32
    }

    fn get_x(self: WPARAM) -> i32 {
        (self.0 & 0xffff) as i32
    }

    fn get_y(self: WPARAM) -> i32 {
        (self.0 >> 16) as i32
    }
}

impl ParamExt for LPARAM {
    fn get_loword(self: LPARAM) -> u32 {
        (self.0 & 0xffff) as u32
    }

    fn get_hiword(self: LPARAM) -> u32 {
        ((self.0 >> 16) & 0xffff) as u32
    }

    fn get_x(self: LPARAM) -> i32 {
        (self.0 & 0xffff) as i32
    }

    fn get_y(self: LPARAM) -> i32 {
        (self.0 >> 16) as i32
    }
}
