use bitflags::bitflags;

use windows::Win32::UI::WindowsAndMessaging::{
    WINDOW_EX_STYLE, WINDOW_STYLE, WS_BORDER, WS_CAPTION, WS_CHILD, WS_CLIPCHILDREN,
    WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_NOREDIRECTIONBITMAP,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MAXIMIZE, WS_MAXIMIZEBOX, WS_MINIMIZE,
    WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,
};

bitflags! {
    pub struct WindowFlags: u32 {
        const RESIZABLE         = 1 << 0;
        const MINIMIZABLE       = 1 << 1;
        const MAXIMIZABLE       = 1 << 2;
        const CLOSABLE          = 1 << 3;
        const VISIBLE           = 1 << 4;
        const ON_TASKBAR        = 1 << 5;
        const ALWAYS_ON_TOP     = 1 << 6;
        const ALWAYS_ON_BOTTOM  = 1 << 7;
        const NO_BACK_BUFFER    = 1 << 8;
        const TRANSPARENT       = 1 << 9;
        const CHILD             = 1 << 10;
        const MAXIMIZED         = 1 << 11;
        const POPUP             = 1 << 12;

        /// Marker flag for fullscreen. Should always match `WindowState::fullscreen`, but is
        /// included here to make masking easier.
        const MARKER_EXCLUSIVE_FULLSCREEN = 1 << 13;
        const MARKER_BORDERLESS_FULLSCREEN = 1 << 14;

        /// The `WM_SIZE` event contains some parameters that can effect the state of `WindowFlags`.
        /// In most cases, it's okay to let those parameters change the state. However, when we're
        /// running the `WindowFlags::apply_diff` function, we *don't* want those parameters to
        /// effect our stored state, because the purpose of `apply_diff` is to update the actual
        /// window's state to match our stored state. This controls whether to accept those changes.
        const MARKER_RETAIN_STATE_ON_SIZE = 1 << 15;

        const MARKER_IN_SIZE_MOVE = 1 << 16;

        const MINIMIZED = 1 << 17;

        const IGNORE_CURSOR_EVENT = 1 << 18;

        /// Fully decorated window (incl. caption, border and drop shadow).
        const MARKER_DECORATIONS = 1 << 19;
        /// Drop shadow for undecorated windows.
        const MARKER_UNDECORATED_SHADOW = 1 << 20;

        const MARKER_ACTIVATE = 1 << 21;

        const EXCLUSIVE_FULLSCREEN_OR_MASK = WindowFlags::ALWAYS_ON_TOP.bits();
    }
}

impl WindowFlags {
    pub fn to_window_styles(self) -> (WINDOW_STYLE, WINDOW_EX_STYLE) {
        // Required styles to properly support common window functionality like aero snap.
        let mut style = WS_CAPTION | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
        let mut style_ex = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;

        if self.contains(WindowFlags::RESIZABLE) {
            style |= WS_SIZEBOX;
        }
        if self.contains(WindowFlags::MAXIMIZABLE) {
            style |= WS_MAXIMIZEBOX;
        }
        if self.contains(WindowFlags::MINIMIZABLE) {
            style |= WS_MINIMIZEBOX;
        }
        if self.contains(WindowFlags::VISIBLE) {
            style |= WS_VISIBLE;
        }
        if self.contains(WindowFlags::ON_TASKBAR) {
            style_ex |= WS_EX_APPWINDOW;
        }
        if self.contains(WindowFlags::ALWAYS_ON_TOP) {
            style_ex |= WS_EX_TOPMOST;
        }
        if self.contains(WindowFlags::NO_BACK_BUFFER) {
            style_ex |= WS_EX_NOREDIRECTIONBITMAP;
        }
        if self.contains(WindowFlags::CHILD) {
            style |= WS_CHILD; // This is incompatible with WS_POPUP if that gets added eventually.
        }
        if self.contains(WindowFlags::POPUP) {
            style |= WS_POPUP;
        }
        if self.contains(WindowFlags::MINIMIZED) {
            style |= WS_MINIMIZE;
        }
        if self.contains(WindowFlags::MAXIMIZED) {
            style |= WS_MAXIMIZE;
        }
        if self.contains(WindowFlags::IGNORE_CURSOR_EVENT) {
            style_ex |= WS_EX_TRANSPARENT | WS_EX_LAYERED;
        }

        if self.intersects(
            WindowFlags::MARKER_EXCLUSIVE_FULLSCREEN | WindowFlags::MARKER_BORDERLESS_FULLSCREEN,
        ) {
            style &= !WS_OVERLAPPEDWINDOW;
        }

        (style, style_ex)
    }
}
