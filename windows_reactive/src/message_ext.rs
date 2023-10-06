use windows::Win32::{
    Foundation::BOOL,
    UI::WindowsAndMessaging::{
        DispatchMessageW, GetAncestor, GetMessageW, IsDialogMessageW, TranslateMessage, GA_ROOT,
        MSG,
    },
};

pub trait MessageExt {
    fn get_message(&mut self) -> BOOL;

    fn is_dialog_message(&mut self) -> bool;

    fn translate_message(&self);

    fn dispatch_message(&self);

    /// Dispatch system events in the current thread.
    /// This method will pause the thread until there are events to process.
    fn dispatch_thread_events();
}

impl MessageExt for MSG {
    fn get_message(&mut self) -> BOOL {
        unsafe { GetMessageW(self, None, 0, 0) }
    }

    fn is_dialog_message(&mut self) -> bool {
        unsafe { IsDialogMessageW(GetAncestor(self.hwnd, GA_ROOT), self).as_bool() }
    }

    fn translate_message(&self) {
        unsafe { TranslateMessage(self) };
    }

    fn dispatch_message(&self) {
        unsafe { DispatchMessageW(self) };
    }

    fn dispatch_thread_events() {
        let mut message: MSG = MSG::default();

        while message.get_message().into() {
            if !message.is_dialog_message() {
                // println!("message");
                message.translate_message();
                message.dispatch_message();
            } else {
                // ? game tick should go here?
                // print!("\x1B[2J");
                // tick += 1;
                // println!("Tick {}", tick);
            }
        }
    }
}

pub fn dispatch_thread_events() {
    MSG::dispatch_thread_events()
}
