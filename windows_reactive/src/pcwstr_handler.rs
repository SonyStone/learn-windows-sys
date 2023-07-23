use windows::core::PCWSTR;

pub trait AsPCWSTR {
    fn as_pcwstr(&self) -> PCWSTR;
}

impl AsPCWSTR for WideStr {
    fn as_pcwstr(&self) -> PCWSTR {
        self.ptr
    }
}

impl AsPCWSTR for Vec<u16> {
    fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR(self.as_ptr())
    }
}

pub trait AsWide {
    fn as_wide(&self) -> Vec<u16>;
}

impl AsWide for str {
    fn as_wide(&self) -> Vec<u16> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new(self)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }
}

// -----------------------------------

struct WideStr {
    // this is here to allow it to get dropped at the same time as the PCWSTR
    #[allow(unused)]
    text: Vec<u16>,
    ptr: PCWSTR,
}

impl WideStr {
    fn from_str(text: &str) -> Self {
        text.as_wide().to_wide_str()
    }
}

trait ToWideStr {
    fn to_wide_str(self) -> WideStr;
}

impl ToWideStr for &str {
    fn to_wide_str(self) -> WideStr {
        // do not drop when scope ends, by moving it into struct
        WideStr::from_str(self)
    }
}

impl ToWideStr for Vec<u16> {
    fn to_wide_str(self) -> WideStr {
        WideStr {
            ptr: PCWSTR::from_raw(self.as_ptr()),
            text: self,
        }
    }
}
