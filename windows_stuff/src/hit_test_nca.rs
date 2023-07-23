use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
    UI::WindowsAndMessaging::*,
};

use super::{BOTTOMEXTENDWIDTH, LEFTEXTENDWIDTH, RIGHTEXTENDWIDTH, TOPEXTENDWIDTH};

pub unsafe fn hit_test_nca(window: HWND, _: WPARAM, lparam: LPARAM) -> LRESULT {
    // Get the point coordinates for the hit test.
    let pt_mouse = POINT {
        x: get_x_lparam(lparam),
        y: get_y_lparam(lparam),
    };

    // Get the window rectangle.
    let mut rc_window = RECT::default();
    GetWindowRect(window, &mut rc_window).unwrap();

    // Get the frame rectangle, adjusted for the style without a caption.
    let mut rc_frame = RECT::default();
    AdjustWindowRectEx(
        &mut rc_frame,
        WS_OVERLAPPEDWINDOW & !WS_CAPTION,
        false,
        WINDOW_EX_STYLE::default(),
    )
    .unwrap();

    let hit = HitTest::default();
    let hit = hit.determine_poitnt(pt_mouse, rc_window, rc_frame);

    LRESULT(hit.hit_test() as isize)
}

fn get_x_lparam(lparam: LPARAM) -> i32 {
    (lparam.0 & 0xffff) as i32
}

fn get_y_lparam(lparam: LPARAM) -> i32 {
    (lparam.0 >> 16) as i32
}

/// Determine if the hit test is for resizing. Default middle (1,1).
#[derive(Debug, Default)]
struct HitTest {
    row: WmNchittest,
    column: WmNchittest,
    on_resize_border: bool,
}

impl HitTest {
    fn determine_poitnt(mut self, mouse_point: POINT, window_rect: RECT, frame_rect: RECT) -> Self {
        // Determine if the point is at the top or bottom of the window.

        if mouse_point.y >= window_rect.top && mouse_point.y < window_rect.top + TOPEXTENDWIDTH {
            self.on_resize_border = mouse_point.y < (window_rect.top - frame_rect.top);
            self.row = WmNchittest::Top;
        } else if mouse_point.y < window_rect.bottom
            && mouse_point.y >= window_rect.bottom - BOTTOMEXTENDWIDTH
        {
            self.row = WmNchittest::Bottom;
        }

        // Determine if the point is at the left or right of the window.
        if mouse_point.x >= window_rect.left && mouse_point.x < window_rect.left + LEFTEXTENDWIDTH {
            self.column = WmNchittest::Left; // left side
        } else if mouse_point.x < window_rect.right
            && mouse_point.x >= window_rect.right - RIGHTEXTENDWIDTH
        {
            self.column = WmNchittest::Right; // right side
        }

        self
    }

    /// Hit test (HTTOPLEFT, ... HTBOTTOMRIGHT)
    fn hit_test(&self) -> WmNchittest {
        match (self.row, self.column) {
            (WmNchittest::Top, WmNchittest::Left) => WmNchittest::TopLeft,
            (WmNchittest::Top, WmNchittest::NoWhere) => {
                if self.on_resize_border {
                    WmNchittest::Top
                } else {
                    WmNchittest::Caption
                }
            }
            (WmNchittest::Top, WmNchittest::Right) => WmNchittest::TopRight,
            (WmNchittest::NoWhere, WmNchittest::Left) => WmNchittest::Left,
            (WmNchittest::NoWhere, WmNchittest::NoWhere) => WmNchittest::NoWhere,
            (WmNchittest::NoWhere, WmNchittest::Right) => WmNchittest::Right,
            (WmNchittest::Bottom, WmNchittest::Left) => WmNchittest::BottomLeft,
            (WmNchittest::Bottom, WmNchittest::NoWhere) => WmNchittest::Bottom,
            (WmNchittest::Bottom, WmNchittest::Right) => WmNchittest::BottomRight,
            _ => unreachable!(), // This case should never happen
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
enum WmNchittest {
    #[default]
    NoWhere = HTNOWHERE as isize,
    Caption = HTCAPTION as isize,
    Left = HTLEFT as isize,
    Right = HTRIGHT as isize,
    Top = HTTOP as isize,
    TopLeft = HTTOPLEFT as isize,
    TopRight = HTTOPRIGHT as isize,
    Bottom = HTBOTTOM as isize,
    BottomLeft = HTBOTTOMLEFT as isize,
    BottomRight = HTBOTTOMRIGHT as isize,
}
