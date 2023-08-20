use std::{any::Any, mem::size_of};

use windows::{
    w,
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{BeginPaint, EndPaint, TextOutW, COLOR_WINDOW, HBRUSH, PAINTSTRUCT},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, GetClientRect, LoadCursorW, LoadIconW, MoveWindow,
            PostQuitMessage, RegisterClassExW, ShowWindow, CW_USEDEFAULT, IDC_ARROW,
            IDI_APPLICATION, MSG, SW_NORMAL, SW_SHOW, WINDOW_EX_STYLE, WM_CREATE, WM_DESTROY,
            WM_PAINT, WM_SIZE, WNDCLASSEXW, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};
use windows_reactive::{
    pcwstr_handler::{AsPCWSTR, AsWide},
    window_handle_ext::WindowHandleExt,
};

static mut CHILD_HWND: Option<HWND> = None;
static mut _HWND: Option<HWND> = None;
static mut H_INSTANCE: Option<HMODULE> = None;

fn main() {
    unsafe {
        H_INSTANCE = Some(GetModuleHandleW(None).unwrap());
        let class_name = "Win32DesktopApp";
        let class_name_w = class_name.as_wide();

        let window_class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: H_INSTANCE.unwrap(),
            lpszClassName: class_name_w.as_pcwstr(),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize),
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };

        let class_token = RegisterClassExW(&window_class);

        // window_class.hIconSm = LoadIconW(H_INSTANCE.unwrap(), IDI_APPLICATION).unwrap();

        let hwnd = HWND::create_window(
            WINDOW_EX_STYLE(0),
            class_name,
            "Windows c++ Win32 Desktop App",
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            Some(1),
        );

        _HWND = Some(hwnd);

        // А дальше ничего нет, так как Winui 3 для Rust пока нет.

        hwnd.show_window(SW_NORMAL);
        hwnd.update_window();

        windows_reactive::message_ext::dispatch_thread_events();
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            let greeting = "Hello World in Win32!".as_wide();
            TextOutW(hdc, 300, 5, &greeting);
            EndPaint(hwnd, &ps);

            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }

        // Create main window
        WM_CREATE => {
            CHILD_HWND = Some(CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("ChildWClass"),
                None,
                WS_CHILD | WS_BORDER,
                0,
                0,
                0,
                0,
                hwnd,
                None,
                H_INSTANCE.unwrap(),
                None,
            ));
            LRESULT(0)
        }

        // Main window changed size
        WM_SIZE => {
            // Get the dimensions of the main window's client
            // area, and enumerate the child windows. Pass the
            // dimensions to the child windows during enumeration.
            let mut rc_client = RECT::default();
            GetClientRect(hwnd, &mut rc_client);
            MoveWindow(CHILD_HWND.unwrap(), 200, 200, 400, 500, true);
            ShowWindow(CHILD_HWND.unwrap(), SW_SHOW);

            LRESULT(0)
        }

        // Process other messages.
        _ => DefWindowProcW(hwnd, msg, w, l),
    }
}
