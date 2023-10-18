pub mod com_initialized;
pub mod device_context_ext;
pub mod direct_2d;
pub mod droppable;
pub mod errors;
pub mod game;
mod graphics;
pub mod hwnd_builder;
pub mod message_ext;
pub mod messages;
pub mod param_ext;
pub mod pcwstr_handler;
pub mod pre_settings;
pub mod rect_ext;
pub mod test;
pub mod window_handle;
pub mod window_handle_ext;
mod window_handle_getter;

// Winodws api without wrappers
pub use windows::Win32::{
    Foundation::{
        GetLastError, SetLastError, COLORREF, HWND, LPARAM, LRESULT, POINT, RECT, WIN32_ERROR,
        WPARAM,
    },
    Graphics::Gdi::{
        BeginPaint, Ellipse, EndPaint, GetDC, LineTo, MoveToEx, ReleaseDC, ScreenToClient,
        SetPixel, HDC, PAINTSTRUCT,
    },
    UI::{
        Input::Pointer::{
            EnableMouseInPointer, GetPointerFramePenInfo, GetPointerFrameTouchInfo, GetPointerInfo,
            GetPointerPenInfo, GetPointerTouchInfo, GetPointerType, POINTER_INFO, POINTER_PEN_INFO,
            POINTER_TOUCH_INFO,
        },
        WindowsAndMessaging::{
            PostQuitMessage, BN_CLICKED, BN_DBLCLK, BN_PUSHED, BS_FLAT, BS_PUSHBUTTON, CS_DBLCLKS,
            CW_USEDEFAULT, EN_CHANGE, POINTER_INPUT_TYPE, PT_MOUSE, PT_PEN, PT_POINTER, PT_TOUCH,
            PT_TOUCHPAD, WINDOW_EX_STYLE, WINDOW_STYLE, WM_CLOSE, WM_COMMAND, WM_CREATE,
            WM_DESTROY, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_PAINT, WM_POINTERDOWN, WM_POINTERUP,
            WM_POINTERUPDATE, WM_RBUTTONDOWN, WS_BORDER, WS_CAPTION, WS_CHILD, WS_CLIPCHILDREN,
            WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_EX_LAYERED,
            WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE,
            WS_MAXIMIZE, WS_MAXIMIZEBOX, WS_MINIMIZE, WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW,
            WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,
        },
    },
};
