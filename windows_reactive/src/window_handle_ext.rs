use std::{ffi::OsString, fmt::Debug, os::windows::prelude::OsStringExt};

use windows::{
    core::IntoParam,
    Win32::{
        Foundation::{HMODULE, HWND},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetClassNameW, IsWindow, MoveWindow, SetWindowLongPtrW,
            SetWindowPlacement, SetWindowPos, SetWindowTextW, ShowWindow, GWLP_HINSTANCE, GWLP_ID,
            GWLP_USERDATA, GWLP_WNDPROC, GWL_EXSTYLE, GWL_STYLE, HMENU, SET_WINDOW_POS_FLAGS,
            SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, WINDOWPLACEMENT, WINDOW_EX_STYLE, WINDOW_STYLE,
        },
    },
};

use crate::pcwstr_handler::{AsPCWSTR, AsWide};

/// [SetWindowLongPtrW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
pub trait WindowHandleExt {
    /// [CreateWindowExW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    fn new<P0, P1>(
        ex_style: WINDOW_EX_STYLE,
        class_name: &str,
        window_name: &str,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: P0,
        menu: P1,
    ) -> Self
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>;

    fn create_window<P0, P1, P2>(
        ex_style: WINDOW_EX_STYLE,
        class_name: &str,
        window_name: &str,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: P0,
        menu: P1,
        l_param: Option<P2>,
    ) -> Self
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>,
        P2: IntoLParam + Debug;

    /// Sets a new [extended window style](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
    fn set_extended_window_style(&self, ptr: isize);

    /// Sets a new application instance handle.
    fn set_instance(&self, ptr: isize);

    /// Sets a new identifier of the child window.
    /// The window cannot be a top-level window.
    fn set_identifier(&self, ptr: isize);

    /// Sets a new [window style](https://learn.microsoft.com/en-us/windows/desktop/winmsg/window-styles).
    fn set_style(&self, ptr: isize);

    /// Sets the user data associated with the window.
    /// This data is intended for use by the application that created the window.
    /// Its value is initially zero.
    fn set_user_data(&self, ptr: isize);

    /// Sets a new address for the window procedure.
    fn set_window_procedure(&self, ptr: isize);

    fn set_window_placement(&self, lpwndpl: *const WINDOWPLACEMENT);

    /// [SetWindowPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
    fn set_window_pos(
        &self,
        handle_insert_after: HWND,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: SET_WINDOW_POS_FLAGS,
    );

    /// [SetWindowTextW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
    /// Changes the text of the specified window's title bar (if it has one).
    fn set_window_text(&self, string: &str);

    fn move_window(&self, x: i32, y: i32, width: i32, height: i32, repaint: bool);

    fn get_class_name(&self) -> String;

    fn is_window(&self) -> Result<(), windows::core::Error>;
    fn minimize(&self);
    fn maximize(&self);
    fn restore(&self);
}

impl WindowHandleExt for HWND {
    fn new<P0, P1>(
        ex_style: WINDOW_EX_STYLE,
        class_name: &str,
        window_name: &str,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: P0,
        menu: P1,
    ) -> Self
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>,
    {
        let class_name = class_name.as_wide();
        let window_name = window_name.as_wide();

        unsafe {
            CreateWindowExW(
                ex_style,
                class_name.as_pcwstr(),
                window_name.as_pcwstr(),
                style,
                x,
                y,
                width,
                height,
                parent,
                menu,
                get_current_instance(),
                None,
            )
        }
    }

    fn create_window<P0, P1, P2>(
        ex_style: WINDOW_EX_STYLE,
        class_name: &str,
        window_name: &str,
        style: WINDOW_STYLE,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: P0,
        menu: P1,
        l_param: Option<P2>,
    ) -> Self
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>,
        P2: IntoLParam + Debug,
    {
        unsafe {
            let h_instance = get_current_instance();
            let class_name = class_name.as_wide();
            let window_name = window_name.as_wide();
            // let menu = menu.into_param().abi();
            // let parent = parent.into_param().abi();

            let handle = CreateWindowExW(
                ex_style,
                class_name.as_pcwstr(),
                window_name.as_pcwstr(),
                style,
                x,
                y,
                width,
                height,
                parent,
                menu,
                h_instance,
                l_param.map(|l_param| l_param.into_l_param()),
            );

            handle
        }
    }

    fn set_extended_window_style(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self, GWL_EXSTYLE, ptr) };
    }

    fn set_instance(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self, GWLP_HINSTANCE, ptr) };
    }

    fn set_identifier(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self, GWLP_ID, ptr) };
    }

    fn set_style(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self, GWL_STYLE, ptr) };
    }

    fn set_user_data(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self, GWLP_USERDATA, ptr) };
    }

    fn set_window_procedure(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self, GWLP_WNDPROC, ptr) };
    }

    fn set_window_placement(&self, lpwndpl: *const WINDOWPLACEMENT) {
        unsafe {
            SetWindowPlacement(*self, lpwndpl);
        };
    }

    fn set_window_pos(
        &self,
        handle_insert_after: HWND,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: SET_WINDOW_POS_FLAGS,
    ) {
        unsafe { SetWindowPos(*self, handle_insert_after, x, y, cx, cy, flags) };
    }

    fn set_window_text(&self, string: &str) {
        let string = string.as_wide();
        unsafe { SetWindowTextW(*self, string.as_pcwstr()) };
    }

    fn move_window(&self, x: i32, y: i32, width: i32, height: i32, repaint: bool) {
        unsafe { MoveWindow(*self, x, y, width, height, repaint) };
    }

    fn get_class_name(&self) -> String {
        const BUFFER_SIZE: usize = 100;
        let mut class_name_raw: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let count = unsafe { GetClassNameW(*self, &mut class_name_raw) as usize };
        let class_name = &class_name_raw[..count];
        String::from_utf16_lossy(class_name)
    }

    fn is_window(&self) -> Result<(), windows::core::Error> {
        let result = unsafe { IsWindow(*self) };
        result.ok()
    }

    // + ещё 15 различных состояний.
    fn minimize(&self) {
        unsafe {
            ShowWindow(*self, SW_MINIMIZE);
        }
    }

    fn maximize(&self) {
        unsafe {
            ShowWindow(*self, SW_MAXIMIZE);
        }
    }

    fn restore(&self) {
        unsafe {
            ShowWindow(*self, SW_RESTORE);
        }
    }
}

// get current instance
pub fn get_current_instance() -> HMODULE {
    unsafe { GetModuleHandleW(None).unwrap() }
}

pub trait IntoLParam {
    fn into_l_param(self) -> *const std::ffi::c_void;
}

impl<T> IntoLParam for T {
    fn into_l_param(self) -> *const std::ffi::c_void {
        let boxed_text = Box::new(self);
        Box::into_raw(boxed_text) as *const std::ffi::c_void
    }
}
