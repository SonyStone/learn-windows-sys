use std::mem::transmute;

use windows::Win32::{
    Foundation::{HWND, LPARAM, POINT, WPARAM},
    UI::WindowsAndMessaging::CREATESTRUCTW,
};

pub trait ParamExt {
    fn get_loword(&self) -> u32;
    fn get_hiword(&self) -> u32;
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;

    fn get_point(&self) -> POINT {
        POINT {
            x: self.get_x(),
            y: self.get_y(),
        }
    }
}

impl ParamExt for WPARAM {
    fn get_loword(&self) -> u32 {
        (self.0 & 0xffff) as u32
    }

    fn get_hiword(&self) -> u32 {
        ((self.0 >> 16) & 0xffff) as u32
    }

    fn get_x(&self) -> i32 {
        (self.0 & 0xffff) as i32
    }

    fn get_y(&self) -> i32 {
        (self.0 >> 16) as i32
    }
}

impl ParamExt for LPARAM {
    fn get_loword(&self) -> u32 {
        (self.0 & 0xffff) as u32
    }

    fn get_hiword(&self) -> u32 {
        ((self.0 >> 16) & 0xffff) as u32
    }

    fn get_x(&self) -> i32 {
        (self.0 & 0xffff) as i32
    }

    fn get_y(&self) -> i32 {
        (self.0 >> 16) as i32
    }
}

pub trait LParamExt {
    fn get_create_struct(&self) -> &CREATESTRUCTW;
    fn get_create_data<T>(&self) -> Box<T>;
    fn get_child_handle(&self) -> HWND;
    fn get_mouse_position(&self) -> (i32, i32);
}

impl LParamExt for LPARAM {
    fn get_create_struct(&self) -> &CREATESTRUCTW {
        unsafe { transmute(*self) }
    }

    fn get_create_data<T>(&self) -> Box<T> {
        let create_struct = self.get_create_struct();
        let l_param = create_struct.lpCreateParams as *mut T;
        unsafe { Box::from_raw(l_param) }
    }

    fn get_child_handle(&self) -> HWND {
        HWND(self.0)
    }

    fn get_mouse_position(&self) -> (i32, i32) {
        (self.get_x(), self.get_y())
    }
}

pub trait WParamExt: ParamExt {
    fn get_pointer_id(&self) -> u32 {
        self.get_loword()
    }
}

impl WParamExt for WPARAM {}
