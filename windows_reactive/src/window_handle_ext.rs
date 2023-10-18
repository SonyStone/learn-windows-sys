use glam::{IVec2, Vec2};
use windows::{
    core::IntoParam,
    Win32::{
        Foundation::{BOOL, HMODULE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
        Graphics::{
            Dwm::{
                DwmExtendFrameIntoClientArea, DwmGetWindowAttribute, DWMWA_CAPTION_BUTTON_BOUNDS,
            },
            Gdi::{BeginPaint, EndPaint, ScreenToClient, UpdateWindow, HDC, PAINTSTRUCT},
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Controls::MARGINS,
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, GetClassNameW, GetClientRect, GetWindowLongPtrW,
                GetWindowPlacement, GetWindowRect, IsWindow, MoveWindow, PostMessageW,
                PostQuitMessage, SetWindowLongPtrW, SetWindowPlacement, SetWindowPos,
                SetWindowTextW, ShowWindow, GWLP_HINSTANCE, GWLP_ID, GWLP_USERDATA, GWLP_WNDPROC,
                GWL_EXSTYLE, GWL_STYLE, GWL_USERDATA, HMENU, SET_WINDOW_POS_FLAGS, SHOW_WINDOW_CMD,
                SW_FORCEMINIMIZE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_NORMAL, SW_RESTORE,
                SW_SHOW, SW_SHOWDEFAULT, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWMINNOACTIVE,
                SW_SHOWNA, SW_SHOWNOACTIVATE, SW_SHOWNORMAL, WINDOWPLACEMENT, WINDOW_EX_STYLE,
                WINDOW_LONG_PTR_INDEX, WINDOW_STYLE,
            },
        },
    },
};

use crate::{
    pcwstr_handler::{AsPCWSTR, AsWide},
    window_handle_getter::WindowHandleGetter,
};

impl WindowHandleExt for HWND {}
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
        mut l_param: Option<&mut P2>,
    ) -> HWND
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>,
        P2: IntoLParam,
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
                l_param.as_mut().map(|l_param| l_param.into_l_param()),
            );

            handle
        }
    }

    fn create_window_2<P0, P1>(
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
        l_param: Option<*const ::core::ffi::c_void>,
    ) -> HWND
    where
        P0: IntoParam<HWND>,
        P1: IntoParam<HMENU>,
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
                l_param,
            );

            handle
        }
    }

    unsafe fn set_window_placement(&self, lpwndpl: *const WINDOWPLACEMENT) {
        unsafe {
            SetWindowPlacement(*self.get_handle(), lpwndpl);
        };
    }

    fn from_l_param(l: LPARAM) -> HWND {
        HWND(l.0)
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

    /// [GetWindowPlacement](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
    fn get_window_placement(&self) -> WINDOWPLACEMENT {
        let mut lpwndpl = WINDOWPLACEMENT::default();
        unsafe { GetWindowPlacement(*self.get_handle(), &mut lpwndpl).unwrap() };
        lpwndpl
    }

    fn is_maximized(&self) -> bool {
        self.get_window_placement().showCmd == SW_SHOWMAXIMIZED
    }

    fn is_minimized(&self) -> bool {
        self.get_window_placement().showCmd == SW_SHOWMINIMIZED
    }

    fn is_normal(&self) -> bool {
        self.get_window_placement().showCmd == SW_SHOWNORMAL
    }

    fn get_window_rect(&self) -> RECT {
        let mut rect = RECT::default();
        unsafe { GetWindowRect(*self.get_handle(), &mut rect).unwrap() };
        rect
    }

    fn get_client_rect(&self) -> RECT {
        let mut rect = RECT::default();
        unsafe { GetClientRect(*self.get_handle(), &mut rect).unwrap() };
        rect
    }

    /// [DwmGetWindowAttribute](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetwindowattribute)
    fn get_caption_button_bounds(&self) -> RECT {
        let mut rect_caption_button_bounds = RECT::default();
        unsafe {
            DwmGetWindowAttribute(
                *self.get_handle(),
                DWMWA_CAPTION_BUTTON_BOUNDS,
                &mut rect_caption_button_bounds as *mut _ as *mut std::os::raw::c_void,
                std::mem::size_of::<RECT>() as u32,
            )
            .unwrap()
        };

        rect_caption_button_bounds
    }

    /// [DwmExtendFrameIntoClientArea](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmextendframeintoclientarea)
    fn extend_frame_into_client_area(&self, margins: MARGINS) {
        unsafe { DwmExtendFrameIntoClientArea(*self.get_handle(), &margins).unwrap() };
    }

    /// [DefWindowProcW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    ///
    /// Calls the default window procedure to provide default processing
    /// for any window messages that an application does not process.
    /// This function ensures that every message is processed.
    fn default_window_procedure<P1, P2>(&self, msg: u32, w: P1, l: P2) -> LRESULT
    where
        P1: IntoParam<WPARAM>,
        P2: IntoParam<LPARAM>,
    {
        unsafe { DefWindowProcW(*self.get_handle(), msg, w, l) }
    }

    fn handled(&self) -> LRESULT {
        LRESULT::default()
    }

    fn post_message<P1, P2>(&self, msg: u32, wparam: P1, lparam: P2)
    where
        P1: IntoParam<WPARAM>,
        P2: IntoParam<LPARAM>,
    {
        unsafe { PostMessageW(*self.get_handle(), msg, wparam, lparam) };
    }

    fn post_quit_message() {
        unsafe { PostQuitMessage(0) };
    }

    fn update_window(&self) {
        unsafe { UpdateWindow(*self.get_handle()) };
    }

    /// set raw pointer.
    /// Mostly used for GWL_USERDATA
    /// [SetWindowLongPtrW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
    fn set_window_pointer<T>(&self, index: WINDOW_LONG_PTR_INDEX, ptr: *mut T) {
        unsafe { SetWindowLongPtrW(*self.get_handle(), index, ptr as _) };
    }

    /// -20 GWL_EXSTYLE
    /// Sets a new [extended window style](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
    fn set_extended_window_style<T>(&self, ptr: *mut T) {
        self.set_window_pointer(GWL_EXSTYLE, ptr);
    }

    /// -6 GWLP_HINSTANCE
    /// Sets a new application instance handle.
    fn set_instance<T>(&self, ptr: *mut T) {
        self.set_window_pointer(GWLP_HINSTANCE, ptr);
    }

    /// -12 GWLP_ID
    /// Sets a new identifier of the child window.
    /// The window cannot be a top-level window.
    fn set_identifier<T>(&self, ptr: *mut T) {
        self.set_window_pointer(GWLP_ID, ptr);
    }

    /// -16 GWL_STYLE
    /// Sets a new [window style](https://learn.microsoft.com/en-us/windows/desktop/winmsg/window-styles).
    fn set_style<T>(&self, ptr: *mut T) {
        self.set_window_pointer(GWL_STYLE, ptr);
    }

    /// -21 GWLP_USERDATA
    /// Sets the user data associated with the window.
    /// This data is intended for use by the application that created the window.
    /// Its value is initially zero.
    fn set_user_data<T>(&self, ptr: *mut T) {
        self.set_window_pointer(GWLP_USERDATA, ptr);
    }

    /// -4 GWLP_WNDPROC
    /// Sets a new address for the window procedure.
    fn set_window_procedure<T>(&self, ptr: *mut T) {
        self.set_window_pointer(GWLP_WNDPROC, ptr);
    }

    /// returns raw pointer
    fn get_window_long_ptr<T>(&self, index: WINDOW_LONG_PTR_INDEX) -> Option<&'static mut T> {
        let ptr = unsafe { GetWindowLongPtrW(*self.get_handle(), index) as *mut T };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&mut *ptr) }
        }
    }

    fn get_user_data<T>(&self) -> Option<&'static mut T> {
        self.get_window_long_ptr(GWL_USERDATA)
    }

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

    fn screen_to_client(&self, point: &IVec2) -> IVec2 {
        let mut point = POINT {
            x: point.x,
            y: point.y,
        };
        unsafe { ScreenToClient(*self.get_handle(), &mut point) };
        IVec2 {
            x: point.x,
            y: point.y,
        }
    }

    fn begin_paint(&self, paint_struct: &mut PAINTSTRUCT) -> HDC {
        unsafe { BeginPaint(*self.get_handle(), paint_struct) }
    }

    fn end_paint(&self, paint_struct: &mut PAINTSTRUCT) -> BOOL {
        unsafe { EndPaint(*self.get_handle(), paint_struct) }
    }
}

// get current instance
pub fn get_current_instance() -> HMODULE {
    unsafe { GetModuleHandleW(None).unwrap() }
}

pub trait IntoLParam {
    fn into_l_param(&mut self) -> *const std::ffi::c_void;
}

impl<T> IntoLParam for T {
    fn into_l_param(&mut self) -> *const std::ffi::c_void {
        self as *mut _ as _
    }
}
