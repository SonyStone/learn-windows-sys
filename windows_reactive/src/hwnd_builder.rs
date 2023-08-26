use std::fmt::Display;

use windows::Win32::{
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

use crate::{
    device_context_ext::DeviceContextExt,
    errors::last_error,
    param_ext::{LParamExt, ParamExt, WParamExt},
    rect_ext::RectExt,
    window_handle_ext::WindowHandleExt,
};

pub struct Callback(pub Box<dyn FnMut(HWND, u32, WPARAM, LPARAM) -> LRESULT>);

impl Callback {
    pub fn new(closure: impl FnMut(HWND, u32, WPARAM, LPARAM) -> LRESULT + 'static) -> Self {
        Self(Box::new(closure))
    }
}

pub struct OnCLick(pub Box<dyn Fn(HWND)>);

pub struct OnMessage(pub Box<dyn FnMut(HWND, u32, WPARAM, LPARAM) -> LRESULT>);

#[derive(Default)]
pub struct HwndBuilder {
    class_name: String,
    text: Option<String>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
    on_message_callback: Option<OnMessage>,
    click_callback: Option<OnCLick>,
    right_click_callback: Option<OnCLick>,
    parent: Option<HWND>,
    style: WINDOW_STYLE,
    ex_style: WINDOW_EX_STYLE,
}

impl HwndBuilder {
    pub fn parent(mut self, parent: &HWND) -> Self {
        self.parent = Some(*parent);
        self
    }

    pub fn class_name(mut self, name: &str) -> Self {
        self.class_name = name.to_string();
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> Self {
        self.size = Some(size);
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn on_click(mut self, f: impl Fn(HWND) + 'static) -> Self {
        self.click_callback = Some(OnCLick(Box::new(f)));
        self
    }

    pub fn on_right_click(mut self, f: impl Fn(HWND) + 'static) -> Self {
        self.right_click_callback = Some(OnCLick(Box::new(f)));
        self
    }

    pub fn on_message(
        mut self,
        f: impl FnMut(HWND, u32, WPARAM, LPARAM) -> LRESULT + 'static,
    ) -> Self {
        self.on_message_callback = Some(OnMessage(Box::new(f)));
        self
    }

    pub fn resizable(mut self) -> Self {
        self.style |= WS_SIZEBOX;
        self
    }

    pub fn minimizable(mut self) -> Self {
        self.style |= WS_MINIMIZEBOX;
        self
    }

    pub fn maximizable(mut self) -> Self {
        self.style |= WS_MAXIMIZEBOX;
        self
    }

    pub fn visible(mut self) -> Self {
        self.style |= WS_VISIBLE;
        self
    }

    pub fn push_button(mut self) -> Self {
        self.style |= WINDOW_STYLE(BS_PUSHBUTTON as u32);
        self
    }

    pub fn flat(mut self) -> Self {
        self.style |= WINDOW_STYLE(BS_FLAT as u32);
        self
    }

    pub fn child(mut self) -> Self {
        self.style |= WS_CHILD;
        self
    }

    pub fn on_taskbar(mut self) -> Self {
        self.ex_style |= WS_EX_APPWINDOW;
        self
    }

    pub fn always_on_top(mut self) -> Self {
        self.ex_style |= WS_EX_TOPMOST;
        self
    }

    pub fn no_back_buffer(mut self) -> Self {
        self.ex_style |= WS_EX_NOREDIRECTIONBITMAP;
        self
    }

    pub fn popup(mut self) -> Self {
        self.style |= WS_POPUP;
        self
    }

    pub fn minimized(mut self) -> Self {
        self.style |= WS_MINIMIZE;
        self
    }

    pub fn maximized(mut self) -> Self {
        self.style |= WS_MAXIMIZE;
        self
    }

    pub fn ignore_cursor_event(mut self) -> Self {
        self.ex_style |= WS_EX_TRANSPARENT | WS_EX_LAYERED;
        self
    }

    pub fn fullscreen(mut self) -> Self {
        self.style &= !WS_OVERLAPPEDWINDOW;
        self
    }

    ///
    /// Should go at the beginning
    /// * WS_CAPTION - The window has a title bar (includes the WS_BORDER style).
    /// * WS_BORDER - The window has a thin-line border
    /// * WS_CLIPSIBLINGS - Clips child windows relative to each other
    /// * WS_CLIPCHILDREN - Excludes the area occupied by child windows when drawing occurs within the parent window.
    /// * WS_SYSMENU - The window has a window menu on its title bar.
    ///
    /// * WS_EX_WINDOWEDGE - The window has a border with a raised edge.
    /// * WS_EX_ACCEPTFILES - The window accepts drag-drop files.
    pub fn window(mut self) -> Self {
        self.style = WS_CAPTION | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
        self.ex_style = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;
        self
    }

    pub fn build(self) -> HWND {
        let class_name = &self.class_name;
        let (width, height) = self.size.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let (x, y) = self.pos.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let window_name = &self.text.clone().unwrap_or("".to_string());
        let parent = self.parent.unwrap_or(HWND::default());

        let mut click_callback = None;

        unsafe {
            EnableMouseInPointer(true);
        }

        if self.parent.is_some() && self.click_callback.is_some() {
            click_callback = Some(Box::into_raw(Box::new(self.click_callback.unwrap())));
        }

        let handel = HWND::create_window(
            self.ex_style,
            class_name,
            window_name,
            self.style,
            x,
            y,
            width,
            height,
            parent,
            None,
            self.on_message_callback,
        );

        if self.parent.is_some() {
            if let Some(click_callback) = click_callback {
                handel.set_user_data(click_callback);
            };
        };

        handel
    }
}

pub fn create_window_handle() -> HwndBuilder {
    HwndBuilder::default()
}

#[cfg(test)]
mod tests {
    use super::Droppable;

    #[derive(Default)]
    pub struct StructWithCallback {
        callback: Option<Box<dyn Fn(i32)>>,
        droppable: Droppable,
    }

    impl StructWithCallback {
        fn callback(mut self, callback: impl Fn(i32) + 'static) -> Self {
            self.callback = Some(Box::new(callback));
            self
        }

        fn build(&self) -> Box<dyn Fn(bool) + '_> {
            let handler = move |value: bool| {
                if value {
                    if let Some(callback) = &self.callback {
                        callback(7);
                    }
                }
            };

            Box::new(handler)
        }
    }

    #[test]
    fn it_works() {
        let binding = StructWithCallback::default().callback(|value| {
            println!("value is {}", value);
        });

        let some = binding.build();

        some(true);
    }
}

pub struct Droppable {
    name: String,
}

impl Droppable {
    fn new(name: &str) -> Self {
        Droppable {
            name: name.to_string(),
        }
    }
}

impl Default for Droppable {
    fn default() -> Self {
        Droppable {
            name: "default".to_string(),
        }
    }
}

impl Display for Droppable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(> Still here {})", self.name)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("(> Dropping {})", self.name);
    }
}

#[derive(Debug)]
pub struct PointerInfo {
    pub pointerId: PointerId,
    pub frameId: u32,
    pub location: POINT,
    pub time: u32,
    pub pointerType: PointerType,
}

/// hover state?
/// in hover state Pen don't have pressure, tilt and rotation
/// I don't know
#[derive(Debug)]
pub struct PenEvent {
    pub pressure: u32,
    pub tilt: (i32, i32),
    pub rotation: u32,
}

#[derive(Debug)]
pub struct MouseEvent {}

#[derive(Debug)]
pub struct TouchEvent {}

#[derive(Debug)]
pub enum PointerType {
    Pointer,
    Touch(TouchEvent),
    Pen(PenEvent),
    Mouse(MouseEvent),
    Touchpad,
}

#[derive(Debug)]
pub struct PointerId(u32);

impl PointerId {
    pub fn new(w: WPARAM) -> Self {
        PointerId(w.get_loword())
    }

    pub fn get_pointer_type(&self) -> POINTER_INPUT_TYPE {
        let mut pointer_input_type = POINTER_INPUT_TYPE::default();

        unsafe { GetPointerType(self.0, &mut pointer_input_type) };

        pointer_input_type
    }

    pub fn point_type(pointer_input_type: POINTER_INPUT_TYPE) -> &'static str {
        match pointer_input_type {
            PT_POINTER => "pointer",
            PT_TOUCH => "touch",
            PT_PEN => "pen",
            PT_MOUSE => "mouse",
            PT_TOUCHPAD => "touchpad",
            _ => "unknown",
        }
    }

    pub fn get_pointer_info(&self) -> POINTER_INFO {
        let mut pointer_info = POINTER_INFO::default();

        unsafe { GetPointerInfo(self.0, &mut pointer_info) };

        pointer_info
    }

    pub fn get_pointer_pen_info(&self) -> POINTER_PEN_INFO {
        let mut pointer_pen_info = POINTER_PEN_INFO::default();

        let succeeds = unsafe { GetPointerPenInfo(self.0, &mut pointer_pen_info) };

        if !succeeds.as_bool() {
            println!("GetPointerPenInfo {:?}", last_error());
            panic!();
        }

        pointer_pen_info
    }

    pub fn get_pointer_frame_pen_info(&self) -> (POINTER_PEN_INFO, u32) {
        let mut pointer_pen_info = POINTER_PEN_INFO::default();
        let mut pointer_count = 0;

        let result = unsafe {
            GetPointerFramePenInfo(self.0, &mut pointer_count, Some(&mut pointer_pen_info))
        };

        (pointer_pen_info, pointer_count)
    }

    pub fn get_pointer_touch_info(&self) -> POINTER_TOUCH_INFO {
        let mut pointer_touch_info = POINTER_TOUCH_INFO::default();

        let succeeds = unsafe { GetPointerTouchInfo(self.0, &mut pointer_touch_info) };

        if !succeeds.as_bool() {
            println!("GetPointerTouchInfo {:?}", last_error());
            panic!();
        }

        pointer_touch_info
    }

    pub fn get_pointer_frame_touch_info(&self) -> (POINTER_TOUCH_INFO, u32) {
        let mut pointer_touch_info = POINTER_TOUCH_INFO::default();
        let mut pointer_count = 0;

        let result = unsafe {
            GetPointerFrameTouchInfo(self.0, &mut pointer_count, Some(&mut pointer_touch_info))
        };

        println!("PT_TOUCH GetPointerFrameTouchInfo {:?}", result);

        println!("PT_TOUCH GetLastError {:?}", unsafe { GetLastError() });

        (pointer_touch_info, pointer_count)
    }
}

trait PointerInputTypeEx {
    fn get_pointer_type(pointerid: u32) -> POINTER_INPUT_TYPE;
}

impl PointerInputTypeEx for POINTER_INPUT_TYPE {
    fn get_pointer_type(pointerid: u32) -> POINTER_INPUT_TYPE {
        let mut pointer_input_type = POINTER_INPUT_TYPE::default();

        unsafe {
            GetPointerType(pointerid, &mut pointer_input_type);
        }

        pointer_input_type
    }
}
