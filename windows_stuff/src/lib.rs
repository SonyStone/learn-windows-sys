mod caption_title;
mod window_state;

use window_state::WindowFlags;
use windows::{
    core::*,
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
        Graphics::{
            Dwm::{DwmDefWindowProc, DwmExtendFrameIntoClientArea, DwmIsCompositionEnabled},
            Gdi::{BeginPaint, EndPaint, GetStockObject, DKGRAY_BRUSH, HBRUSH, PAINTSTRUCT},
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::{Controls::MARGINS, WindowsAndMessaging::*},
    },
};

const LEFTEXTENDWIDTH: i32 = 8;
const RIGHTEXTENDWIDTH: i32 = 8;
const BOTTOMEXTENDWIDTH: i32 = 20;
const TOPEXTENDWIDTH: i32 = 27;

pub fn run() {
    unsafe {
        let mut window_flags = WindowFlags::empty();
        window_flags.set(WindowFlags::RESIZABLE, true);
        window_flags.set(WindowFlags::MINIMIZABLE, true);
        window_flags.set(WindowFlags::MAXIMIZABLE, true);

        let (style, ex_style) = window_flags.to_window_styles();

        let (class_name, instance) = register_window_class();

        let handle = CreateWindowExW(
            ex_style,
            class_name,
            w!("Learn to Program Windows"),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        ShowWindow(handle, SW_SHOWNORMAL);

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            // println!("GetMessageA");
            match message.message {
                WM_LBUTTONDOWN => {
                    println!("inner WM_LBUTTONDOWN");
                }
                _ => {}
            }
            DispatchMessageA(&message);
        }
    }
}

// window procedure
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        let mut pf_call_dwp = false;
        let mut l_ret = LRESULT(0);

        // Winproc worker for custom frame issues.
        if DwmIsCompositionEnabled().is_ok() {
            (l_ret, pf_call_dwp) = custom_caption_proc(window, message, wparam, lparam)
        }

        // Winproc worker for the rest of the application.
        if pf_call_dwp {
            l_ret = app_win_proc(window, message, wparam, lparam);
        }

        l_ret
    }
}

fn app_win_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            // WM_PAINT => {
            //     println!("WM_PAINT");
            //     ValidateRect(window, None);

            //     let mut ps = PAINTSTRUCT::default();
            //     let hdc = BeginPaint(window, &mut ps);

            //     FillRect(hdc, &mut ps.rcPaint, HBRUSH(0));

            //     EndPaint(window, &mut ps);

            //     LRESULT(0)
            // }
            WM_LBUTTONDOWN => {
                println!("WM_LBUTTONDOWN");
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

unsafe fn register_window_class() -> (PCWSTR, HMODULE) {
    let instance = GetModuleHandleW(None).unwrap();
    let class_name = w!("Sample Window Class");

    let class = WNDCLASSW {
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hInstance: instance,
        lpszClassName: class_name,
        hbrBackground: HBRUSH(GetStockObject(DKGRAY_BRUSH).0),
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };

    // We ignore errors because registering the same window class twice would trigger
    //  an error, and because errors here are detected during CreateWindowEx anyway.
    // Also since there is no weird element in the struct, there is no reason for this
    //  call to fail.
    let atom = RegisterClassW(&class);
    debug_assert!(atom != 0);

    (class_name, instance)
}

unsafe fn custom_caption_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> (LRESULT, bool) {
    let mut l_ret = LRESULT(0);
    let mut f_call_dwp = !DwmDefWindowProc(window, message, wparam, lparam, &mut l_ret).as_bool();

    // Handle window creation.
    if message == WM_CREATE {
        let mut rc_client = RECT::default();
        GetWindowRect(window, &mut rc_client).unwrap();

        // Inform application of the frame change.
        SetWindowPos(
            window,
            None,
            rc_client.left,
            rc_client.top,
            caption_title::rect_width(rc_client),
            caption_title::rect_height(rc_client),
            SWP_FRAMECHANGED,
        )
        .unwrap();

        f_call_dwp = true;
        l_ret = LRESULT(0);
    }

    // Handle window activation.
    if message == WM_ACTIVATE {
        let margins = MARGINS {
            cxLeftWidth: LEFTEXTENDWIDTH,
            cxRightWidth: RIGHTEXTENDWIDTH,
            cyBottomHeight: BOTTOMEXTENDWIDTH,
            cyTopHeight: TOPEXTENDWIDTH,
        };

        DwmExtendFrameIntoClientArea(window, &margins).unwrap();

        f_call_dwp = true;
        l_ret = LRESULT(0);
    }

    if message == WM_PAINT {
        let mut ps = PAINTSTRUCT::default();
        let hdc = BeginPaint(window, &mut ps);
        caption_title::paint_custom_caption(window, hdc).unwrap();
        EndPaint(window, &mut ps);

        f_call_dwp = true;
        l_ret = LRESULT(0);
    }

    // Handle the non-client size message.
    if (message == WM_NCCALCSIZE) && (wparam == WPARAM(1)) {
        if wparam == WPARAM(1) {
            let mut pncsp = NCCALCSIZE_PARAMS::default();
            pncsp.rgrc[0].left = pncsp.rgrc[0].left + LEFTEXTENDWIDTH;
            pncsp.rgrc[0].top = pncsp.rgrc[0].top + 0;
            pncsp.rgrc[0].right = pncsp.rgrc[0].right - RIGHTEXTENDWIDTH;
            pncsp.rgrc[0].bottom = pncsp.rgrc[0].bottom - 0;
        }

        LRESULT(0);

        // No need to pass the message on to the DefWindowProc.
        f_call_dwp = false;
    }

    // Handle hit testing in the NCA if not handled by DwmDefWindowProc.
    if (message == WM_NCHITTEST) && (l_ret == LRESULT(0)) {
        l_ret = hit_test_nca(window, wparam, lparam);

        if l_ret != LRESULT(0) {
            f_call_dwp = false;
        }
    }

    (l_ret, f_call_dwp)
}

unsafe fn hit_test_nca(window: HWND, _: WPARAM, lparam: LPARAM) -> LRESULT {
    // Get the point coordinates for the hit test.

    // todo get x and y from  lparam (GET_Y_LPARAM)
    let pt_mouse = POINT {
        x: get_x_lparam(lparam.0) as i32,
        y: get_y_lparam(lparam.0) as i32,
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

    // Determine if the hit test is for resizing. Default middle (1,1).
    let mut u_row = 1;
    let mut u_col = 1;
    let mut f_on_resize_border = false;

    // Determine if the point is at the top or bottom of the window.
    if pt_mouse.y >= rc_window.top && pt_mouse.y < rc_window.top + TOPEXTENDWIDTH {
        f_on_resize_border = pt_mouse.y < (rc_window.top - rc_frame.top);
        u_row = 0;
    } else if pt_mouse.y < rc_window.bottom && pt_mouse.y >= rc_window.bottom - BOTTOMEXTENDWIDTH {
        u_row = 2;
    }

    // Determine if the point is at the left or right of the window.
    if pt_mouse.x >= rc_window.left && pt_mouse.x < rc_window.left + LEFTEXTENDWIDTH {
        u_col = 0; // left side
    } else if pt_mouse.x < rc_window.right && pt_mouse.x >= rc_window.right - RIGHTEXTENDWIDTH {
        u_col = 2; // right side
    }

    // Hit test (HTTOPLEFT, ... HTBOTTOMRIGHT)
    let hit_tests: [[LRESULT; 3]; 3] = [
        [
            LRESULT(HTTOPLEFT as isize),
            if f_on_resize_border {
                LRESULT(HTTOP as isize)
            } else {
                LRESULT(HTCAPTION as isize)
            },
            LRESULT(HTTOPRIGHT as isize),
        ],
        [
            LRESULT(HTLEFT as isize),
            LRESULT(HTNOWHERE as isize),
            LRESULT(HTRIGHT as isize),
        ],
        [
            LRESULT(HTBOTTOMLEFT as isize),
            LRESULT(HTBOTTOM as isize),
            LRESULT(HTBOTTOMRIGHT as isize),
        ],
    ];

    return hit_tests[u_row][u_col];
}

fn get_x_lparam(lparam: isize) -> i16 {
    (lparam & 0xffff) as i16
}

fn get_y_lparam(lparam: isize) -> i16 {
    ((lparam >> 16) & 0xffff) as i16
}
