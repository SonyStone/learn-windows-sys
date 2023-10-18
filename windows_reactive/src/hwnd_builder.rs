use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::Pointer::EnableMouseInPointer,
        WindowsAndMessaging::{
            BS_FLAT, BS_PUSHBUTTON, CW_USEDEFAULT, WINDOW_EX_STYLE, WINDOW_STYLE, WM_NCCREATE,
            WS_BORDER, WS_CAPTION, WS_CHILD, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES,
            WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOPMOST,
            WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MAXIMIZE, WS_MAXIMIZEBOX, WS_MINIMIZE,
            WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,
        },
    },
};

use crate::{param_ext::LParamExt, pre_settings, window_handle_ext::WindowHandleExt};

pub type OnCLick = Box<dyn Fn(HWND)>;

pub type OnMessage = Box<dyn FnMut(HWND, u32, WPARAM, LPARAM) -> LRESULT>;

#[derive(Default)]
pub struct HwndBuilder {
    handle: HWND,
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

impl Drop for HwndBuilder {
    fn drop(&mut self) {
        println!("🚮 HwndBuilder dropped here")
    }
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
        self.click_callback = Some(Box::new(f));
        self
    }

    pub fn on_right_click(mut self, f: impl Fn(HWND) + 'static) -> Self {
        self.right_click_callback = Some(Box::new(f));
        self
    }

    pub fn on_message(
        mut self,
        f: impl FnMut(HWND, u32, WPARAM, LPARAM) -> LRESULT + 'static,
    ) -> Self {
        self.on_message_callback = Some(Box::new(f));
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

    pub fn build(&mut self) -> HWND {
        let class_name = &self.class_name.clone();
        let (width, height) = self.size.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let (x, y) = self.pos.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
        let window_name = &self.text.clone().unwrap_or("".to_string());
        let parent = self.parent.unwrap_or(HWND::default());

        unsafe {
            EnableMouseInPointer(true);
        }

        pre_settings::init_window_class(class_name, Some(Self::window_proc));

        let handle = HWND::create_window_2(
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
            Some(self as *mut _ as _),
        );

        debug_assert!(handle.0 != 0);
        debug_assert!(handle == self.handle);

        if self.parent.is_some() {
            handle.set_user_data(self as *mut _ as _);
        };

        handle
    }

    fn message_handler(
        &mut self,
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        self.on_message_callback.as_mut().unwrap()(window, message, wparam, lparam)
    }

    extern "system" fn window_proc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_NCCREATE => {
                let this = lparam.get_create_struct().lpCreateParams as *mut Self;
                window.set_user_data(this);
                unsafe {
                    (*this).handle = window;
                }
                window.default_window_procedure(message, wparam, lparam)
            }
            _ => window.get_user_data::<HwndBuilder>().map_or_else(
                || window.default_window_procedure(message, wparam, lparam),
                |this| {
                    this.on_message_callback.as_mut().map_or_else(
                        || window.default_window_procedure(message, wparam, lparam),
                        |callback| callback(window, message, wparam, lparam),
                    )
                },
            ),
        }
    }
}

pub fn create_window_handle() -> HwndBuilder {
    HwndBuilder::default()
}

#[cfg(test)]
mod tests {
    use crate::droppable::Droppable;

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
