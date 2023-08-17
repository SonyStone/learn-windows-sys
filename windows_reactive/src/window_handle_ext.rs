use windows::{
    core::IntoParam,
    Win32::{
        Foundation::{HMODULE, HWND},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetClassNameW, IsWindow, MoveWindow, SetWindowLongPtrW,
            SetWindowPlacement, SetWindowPos, SetWindowTextW, ShowWindow, GWLP_HINSTANCE, GWLP_ID,
            GWLP_USERDATA, GWLP_WNDPROC, GWL_EXSTYLE, GWL_STYLE, HMENU, SET_WINDOW_POS_FLAGS,
            SHOW_WINDOW_CMD, SW_FORCEMINIMIZE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_NORMAL,
            SW_RESTORE, SW_SHOW, SW_SHOWDEFAULT, SW_SHOWMINIMIZED, SW_SHOWMINNOACTIVE, SW_SHOWNA,
            SW_SHOWNOACTIVATE, WINDOWPLACEMENT, WINDOW_EX_STYLE, WINDOW_STYLE,
        },
    },
};

use crate::{
    pcwstr_handler::{AsPCWSTR, AsWide},
    window_handle_getter::WindowHandleGetter,
};

/// [SetWindowLongPtrW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
pub trait WindowHandleExt: WindowHandleGetter {
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
    ) -> HWND
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
    ) -> HWND
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>,
        P2: IntoLParam + std::fmt::Debug,
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

    /// Sets a new [extended window style](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
    fn set_extended_window_style(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), GWL_EXSTYLE, ptr) };
    }

    /// Sets a new application instance handle.
    fn set_instance(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), GWLP_HINSTANCE, ptr) };
    }

    /// Sets a new identifier of the child window.
    /// The window cannot be a top-level window.
    fn set_identifier(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), GWLP_ID, ptr) };
    }

    /// Sets a new [window style](https://learn.microsoft.com/en-us/windows/desktop/winmsg/window-styles).
    fn set_style(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), GWL_STYLE, ptr) };
    }

    /// Sets the user data associated with the window.
    /// This data is intended for use by the application that created the window.
    /// Its value is initially zero.
    fn set_user_data(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), GWLP_USERDATA, ptr) };
    }

    /// Sets a new address for the window procedure.
    fn set_window_procedure(&self, ptr: isize) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), GWLP_WNDPROC, ptr) };
    }

    unsafe fn set_window_placement(&self, lpwndpl: *const WINDOWPLACEMENT) {
        unsafe {
            SetWindowPlacement(*self.get_handle(), lpwndpl);
        };
    }

    /// [SetWindowPos](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
    fn set_window_pos(
        &self,
        handle_insert_after: HWND,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: SET_WINDOW_POS_FLAGS,
    ) {
        unsafe { SetWindowPos(*self.get_handle(), handle_insert_after, x, y, cx, cy, flags) };
    }

    /// [SetWindowTextW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
    /// Changes the text of the specified window's title bar (if it has one).
    fn set_window_text(&self, string: &str) {
        let string = string.as_wide();
        unsafe { SetWindowTextW(*self.get_handle(), string.as_pcwstr()) };
    }

    fn move_window(&self, x: i32, y: i32, width: i32, height: i32, repaint: bool) {
        unsafe { MoveWindow(*self.get_handle(), x, y, width, height, repaint) };
    }

    fn get_class_name(&self) -> String {
        const BUFFER_SIZE: usize = 100;
        let mut class_name_raw: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let count = unsafe { GetClassNameW(*self.get_handle(), &mut class_name_raw) as usize };
        let class_name = &class_name_raw[..count];
        String::from_utf16_lossy(class_name)
    }

    fn is_window(&self) -> Result<(), windows::core::Error> {
        let result = unsafe { IsWindow(*self.get_handle()) };
        result.ok()
    }
}

impl WindowHandleExt for HWND {}

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

pub trait ShowWindowExt: WindowHandleGetter {
    /// [ShowWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    fn show_window(&self, command: SHOW_WINDOW_CMD) {
        unsafe {
            ShowWindow(*self.get_handle(), command);
        }
    }

    /// 0. SW_HIDE
    /// Hides the window and activates another window.
    fn hide(&self) {
        self.show_window(SW_HIDE);
    }

    /// 1. SW_SHOWNORMAL SW_NORMAL
    /// Activates and displays a window.
    /// If the window is minimized, maximized, or arranged, the system restores it
    /// to its original size and position.
    /// An application should specify this flag when displaying the window for the first time.
    fn normal(&self) {
        self.show_window(SW_NORMAL);
    }

    /// 2. SW_SHOWMINIMIZED
    /// Activates the window and displays it as a minimized window.
    fn show_minimized(&self) {
        self.show_window(SW_SHOWMINIMIZED);
    }

    /// 3. SW_SHOWMAXIMIZED SW_MAXIMIZE
    /// Activates the window and displays it as a maximized window.
    fn maximize(&self) {
        self.show_window(SW_MAXIMIZE);
    }

    /// 4. SW_SHOWNOACTIVATE
    /// Displays a window in its most recent size and position.
    /// This value is similar to SW_SHOWNORMAL, except that the window is not activated.
    fn show_not_activate(&self) {
        self.show_window(SW_SHOWNOACTIVATE);
    }

    /// 5. SW_SHOW
    /// Activates the window and displays it in its current size and position.
    fn show(&self) {
        self.show_window(SW_SHOW);
    }

    /// 6. SW_MINIMIZE
    /// Minimizes the specified window and activates the next top-level window in the Z order.
    fn minimize(&self) {
        self.show_window(SW_MINIMIZE);
    }

    /// 7. SW_SHOWMINNOACTIVE
    /// Displays the window as a minimized window.
    /// This value is similar to SW_SHOWMINIMIZED, except the window is not activated.
    fn show_minimized_not_active(&self) {
        self.show_window(SW_SHOWMINNOACTIVE);
    }

    /// 8. SW_SHOWNA
    /// Displays the window in its current size and position.
    /// This value is similar to SW_SHOW, except that the window is not activated.
    fn show_not_active(&self) {
        self.show_window(SW_SHOWNA);
    }

    /// 9. SW_RESTORE
    /// Activates and displays the window.
    /// If the window is minimized, maximized, or arranged,
    /// the system restores it to its original size and position.
    /// An application should specify this flag when restoring a minimized window.
    fn restore(&self) {
        self.show_window(SW_RESTORE);
    }

    /// 10. SW_SHOWDEFAULT
    /// Sets the show state based on the SW_ value specified in the STARTUPINFO
    /// structure passed to the CreateProcess function by the program that started the application.
    fn show_default(&self) {
        self.show_window(SW_SHOWDEFAULT);
    }

    /// 11. SW_FORCEMINIMIZE
    /// Minimizes a window, even if the thread that owns the window is not responding.
    /// This flag should only be used when minimizing windows from a different thread.
    fn force_minimize(&self) {
        self.show_window(SW_FORCEMINIMIZE);
    }
}

impl ShowWindowExt for HWND {}
