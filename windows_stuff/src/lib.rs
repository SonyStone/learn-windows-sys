mod window_state;

use window_state::WindowFlags;
use windows::{
    core::*,
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{BeginPaint, EndPaint, FillRect, ValidateRect, HBRUSH, PAINTSTRUCT},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

pub fn run() {
    unsafe {
        let mut window_flags = WindowFlags::empty();
        window_flags.set(WindowFlags::RESIZABLE, true);

        let (style, ex_style) = window_flags.to_window_styles();

        let (class_name, instance) = register_window_class();

        let handle = CreateWindowExW(
            ex_style,
            class_name,
            w!("TLearn to Program Windows"),
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
            DispatchMessageA(&message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None);

                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);

                FillRect(hdc, &mut ps.rcPaint, HBRUSH(0));

                EndPaint(window, &mut ps);

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
