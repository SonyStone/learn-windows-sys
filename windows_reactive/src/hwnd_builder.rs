use std::fmt::Display;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        PostQuitMessage, BN_CLICKED, BN_DBLCLK, BN_PUSHED, BS_FLAT, BS_PUSHBUTTON, CS_DBLCLKS,
        CW_USEDEFAULT, EN_CHANGE, WINDOW_EX_STYLE, WINDOW_STYLE, WM_CLOSE, WM_COMMAND, WM_CREATE,
        WM_DESTROY, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WS_BORDER, WS_CAPTION,
        WS_CHILD, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES, WS_EX_APPWINDOW,
        WS_EX_LAYERED, WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
        WS_EX_WINDOWEDGE, WS_MAXIMIZE, WS_MAXIMIZEBOX, WS_MINIMIZE, WS_MINIMIZEBOX,
        WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SIZEBOX, WS_SYSMENU, WS_VISIBLE,
    },
};

use crate::{
    param_ext::{LParamExt, ParamExt},
    window_handle_ext::WindowHandleExt,
};

pub struct Callback(pub Box<dyn Fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>);

impl Callback {
    pub fn new(closure: impl Fn(HWND, u32, WPARAM, LPARAM) -> LRESULT + 'static) -> Self {
        Self(Box::new(closure))
    }
}

pub struct OnCLick(pub Box<dyn Fn(HWND)>);

#[derive(Default)]
pub struct HwndBuilder {
    class_name: String,
    text: Option<String>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
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

        let mut messages_handler = None;
        let mut click_callback = None;

        if self.parent.is_some() && self.click_callback.is_some() {
            click_callback = Some(Box::into_raw(Box::new(self.click_callback.unwrap())));
        } else {
            messages_handler = Some(Callback::new(
                move |hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM| -> LRESULT {
                    let handled = match msg {
                        WM_CREATE => {
                            println!("WM_CREATE");
                            true
                        }
                        WM_CLOSE => {
                            println!("WM_CLOSE");
                            hwnd.hide();
                            false
                        }
                        WM_DESTROY => {
                            println!("WM_DESTROY");
                            unsafe {
                                PostQuitMessage(0);
                            }
                            false
                        }
                        WM_LBUTTONDOWN => {
                            println!("WM_LBUTTONDOWN");
                            if let Some(click) = &self.click_callback {
                                click.0(hwnd);
                            }
                            true
                        }
                        WM_RBUTTONDOWN => {
                            println!("WM_RBUTTONDOWN");
                            if let Some(click) = &self.right_click_callback {
                                click.0(hwnd);
                            }
                            true
                        }
                        WM_LBUTTONDBLCLK => {
                            println!("WM_LBUTTONDBLCLK");
                            true
                        }
                        WM_COMMAND => {
                            let child_handle = l.get_child_handle();
                            let message = w.get_hiword();

                            let class_name = child_handle.get_class_name();

                            match &class_name as &str {
                                "Button" => match message {
                                    BN_CLICKED => {
                                        if let Some(click) =
                                            child_handle.get_user_data::<Callback>()
                                        {
                                            click.0(child_handle, msg, w, l)
                                        } else {
                                            child_handle.default_window_proc(msg, w, l)
                                        };
                                    }
                                    BN_DBLCLK => {
                                        println!("Button BN_DBLCLK")
                                    }
                                    BN_PUSHED => {
                                        println!("Button BN_PUSHED")
                                    }
                                    _ => {
                                        println!("Button WUT?")
                                    }
                                },
                                "Edit" => match message {
                                    EN_CHANGE => {
                                        println!("Edit EN_CHANGE")
                                    }
                                    _ => {
                                        println!("Edit WUT?")
                                    }
                                },
                                _ => {
                                    println!("WM_COMMAND WUT? WUT?")
                                }
                            };

                            true
                        }
                        _ => false,
                    };

                    if handled {
                        LRESULT::default()
                    } else {
                        hwnd.default_window_proc(msg, w, l)
                    }
                },
            ));
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
            messages_handler,
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
