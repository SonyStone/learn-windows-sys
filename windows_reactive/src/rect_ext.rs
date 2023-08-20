use windows::Win32::{
    Foundation::RECT,
    UI::WindowsAndMessaging::{
        AdjustWindowRectEx, WINDOW_EX_STYLE, WS_CAPTION, WS_OVERLAPPEDWINDOW,
    },
};

pub trait RectExt {
    fn rect(&self) -> &RECT;

    fn adjust_window_rect() -> RECT {
        let mut rc_frame = RECT::default();
        unsafe {
            AdjustWindowRectEx(
                &mut rc_frame,
                WS_OVERLAPPEDWINDOW & !WS_CAPTION,
                false,
                WINDOW_EX_STYLE::default(),
            )
            .unwrap();
        }

        rc_frame
    }

    fn width(&self) -> i32 {
        self.rect().right - self.rect().left
    }

    fn height(&self) -> i32 {
        self.rect().bottom - self.rect().top
    }
}

impl RectExt for RECT {
    fn rect(&self) -> &RECT {
        self
    }
}
