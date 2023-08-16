use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWLP_USERDATA},
};

pub struct Callback {
    closure: Box<dyn FnMut(HWND)>,
}

impl Callback {
    pub fn new(closure: impl FnMut(HWND) + 'static) -> Self {
        Self {
            closure: Box::new(closure),
        }
    }

    pub fn call(&mut self, handle: HWND) {
        (self.closure)(handle);
    }

    pub fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(ptr: isize) -> &'static mut Self {
        let callback = ptr as *mut Callback;
        unsafe { &mut *callback }
    }
}

pub trait UserDataExt {
    fn add(self, callback: Callback);

    fn add_raw(self, callback: &mut Callback);

    fn get(self) -> Option<&'static mut Callback>;
}

impl UserDataExt for HWND {
    fn add(self, callback: Callback) {
        let ptr = callback.into_raw() as _;
        println!("add_user_data");
        unsafe {
            SetWindowLongPtrW(self, GWLP_USERDATA, ptr);
        };
    }

    fn add_raw(self, callback: &mut Callback) {
        let ptr = callback as *const _ as _;
        unsafe {
            SetWindowLongPtrW(self, GWLP_USERDATA, ptr);
        };
    }

    fn get(self) -> Option<&'static mut Callback> {
        let ptr = unsafe { GetWindowLongPtrW(self, GWLP_USERDATA) };
        if ptr != 0 {
            let callback = Callback::from_raw(ptr);
            Some(callback)
        } else {
            None
        }
    }
}
