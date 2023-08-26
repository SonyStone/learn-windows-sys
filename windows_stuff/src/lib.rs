mod caption_title;
mod hit_test_nca;
pub mod register_window_class;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::{
        Dwm::{DwmDefWindowProc, DwmIsCompositionEnabled},
        Gdi::{BeginPaint, EndPaint, PAINTSTRUCT},
    },
    UI::{Controls::MARGINS, WindowsAndMessaging::*},
};
use windows_reactive::{
    hwnd_builder::create_window_handle, message_ext::dispatch_thread_events, rect_ext::RectExt,
    window_handle_ext::WindowHandleExt,
};

pub const LEFTEXTENDWIDTH: i32 = 8;
pub const RIGHTEXTENDWIDTH: i32 = 8;
pub const BOTTOMEXTENDWIDTH: i32 = 20;
pub const TOPEXTENDWIDTH: i32 = 27;

pub fn run() {
    unsafe {
        register_window_class::register_window_class();

        let window = create_window_handle()
            .class_name("Sample Window Class")
            .size((500, 500))
            .position((300, 300))
            .text("Learn to Program Windows")
            .window()
            .resizable()
            .maximizable()
            .minimizable()
            .always_on_top()
            .visible()
            .build();

        create_window_handle()
            .class_name("BUTTON")
            .text("maximize")
            .position((0, 10))
            .size((80, 20))
            .parent(&window)
            .push_button()
            .flat()
            .child()
            .visible()
            .on_click(move |button| {
                if window.is_maximized() {
                    window.restore();
                    button.set_window_text("maximize");
                } else {
                    window.maximize();
                    button.set_window_text("resotre");
                }
            })
            .build();

        dispatch_thread_events();
    }
}

// window procedure
pub extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
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
            _ => window.default_window_procedure(message, wparam, lparam),
        }
    }
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
        let rc_client = window.get_window_rect();

        // Inform application of the frame change.
        SetWindowPos(
            window,
            None,
            rc_client.left,
            rc_client.top,
            rc_client.width(),
            rc_client.height(),
            SWP_FRAMECHANGED,
        )
        .unwrap();

        f_call_dwp = true;
        l_ret = LRESULT(0);
    }

    // Handle window activation.
    if message == WM_ACTIVATE {
        window.extend_frame_into_client_area(MARGINS {
            cxLeftWidth: LEFTEXTENDWIDTH,
            cxRightWidth: RIGHTEXTENDWIDTH,
            cyBottomHeight: BOTTOMEXTENDWIDTH,
            cyTopHeight: TOPEXTENDWIDTH,
        });

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
        l_ret = hit_test_nca::hit_test_nca(window, wparam, lparam);

        if l_ret != LRESULT(0) {
            f_call_dwp = false;
        }
    }

    (l_ret, f_call_dwp)
}
