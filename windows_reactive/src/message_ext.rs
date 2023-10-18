use windows::Win32::{
    Foundation::BOOL,
    UI::WindowsAndMessaging::{
        DispatchMessageW, GetAncestor, GetMessageW, IsDialogMessageW, PeekMessageA, PeekMessageW,
        TranslateMessage, GA_ROOT, MSG, PM_REMOVE, WM_QUIT,
    },
};

pub trait MessageExt {
    fn get_message(&mut self) -> bool;

    /// [PeekMessageW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
    ///
    /// Dispatches incoming nonqueued messages, checks the thread message queue for a posted message, and retrieves the message (if any exist).
    fn peek_message(&mut self) -> bool;

    fn is_dialog_message(&mut self) -> bool;

    fn is_quit(&self) -> bool;

    fn translate_message(&self);

    fn dispatch_message(&self);

    /// Dispatch system events in the current thread.
    /// This method will pause the thread until there are events to process.
    fn dispatch_thread_events();
}

impl MessageExt for MSG {
    fn get_message(&mut self) -> bool {
        unsafe { GetMessageW(self, None, 0, 0) }.as_bool()
    }

    fn peek_message(&mut self) -> bool {
        unsafe { PeekMessageW(self, None, 0, 0, PM_REMOVE) }.as_bool()
    }

    fn is_dialog_message(&mut self) -> bool {
        unsafe { IsDialogMessageW(GetAncestor(self.hwnd, GA_ROOT), self).as_bool() }
    }

    fn is_quit(&self) -> bool {
        self.message == WM_QUIT
    }

    fn translate_message(&self) {
        unsafe { TranslateMessage(self) };
    }

    fn dispatch_message(&self) {
        unsafe { DispatchMessageW(self) };
    }

    fn dispatch_thread_events() {
        let mut message: MSG = MSG::default();

        loop {
            if true {
                while message.peek_message() {
                    if message.is_quit() {
                        return;
                    }
                    message.translate_message();
                    message.dispatch_message();
                }
            } else {
                while message.get_message() {
                    if message.is_quit() {
                        return;
                    }

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
    }
}

pub fn dispatch_thread_events() {
    MSG::dispatch_thread_events()
}
