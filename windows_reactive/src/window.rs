use crate::{pre_settings, window_handle::*};

#[derive(Default, PartialEq, Eq)]
pub struct Window {
    pub handle: WindowHandle,
}

pub struct WindowBuilder<'a> {
    title: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    accept_files: bool,
    center: bool,
    topmost: bool,
    maximized: bool,
    minimized: bool,
}

impl Default for WindowBuilder<'static> {
    fn default() -> Self {
        WindowBuilder {
            title: "New Window",
            size: (500, 500),
            position: (300, 300),
            accept_files: false,
            topmost: false,
            center: false,
            maximized: false,
            minimized: false,
        }
    }
}

impl<'a> WindowBuilder<'a> {
    pub fn build(&self) -> Window {
        pre_settings::init_window_class();

        let handle = create_window_handle()
            .class_name("Custom Name")
            .size(self.size)
            .position(self.position)
            .build();

        if self.minimized {
            handle.minimize();
        }

        if self.maximized {
            handle.maximize();
        }

        Window { handle }
    }

    pub fn minimized(mut self) -> Self {
        self.minimized = true;
        self
    }
}

pub fn window() -> WindowBuilder<'static> {
    WindowBuilder::default()
}
