use std::fmt::Debug;

use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::CREATESTRUCTW,
};

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

pub trait LParamExt {
    fn get_any<T>(&self) -> T
    where
        T: Debug;
}

impl LParamExt for LPARAM {
    fn get_any<T>(self: &LPARAM) -> T
    where
        T: Debug,
    {
        let create_struct = self.0 as *mut CREATESTRUCTW;
        let create_struct = unsafe { *create_struct };
        let l_param = create_struct.lpCreateParams as *mut T;
        let l_param = unsafe { Box::from_raw(l_param) };
        *l_param
    }
}
