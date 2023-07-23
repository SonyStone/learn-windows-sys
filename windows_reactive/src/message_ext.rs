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
}
