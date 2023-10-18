use glam::IVec2;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    System::SystemServices::{
        MK_CONTROL, MK_LBUTTON, MK_MBUTTON, MK_RBUTTON, MK_SHIFT, MK_XBUTTON1, MK_XBUTTON2,
        MODIFIERKEYS_FLAGS,
    },
    UI::{
        Input::Pointer::{
            GetPointerFramePenInfo, GetPointerFrameTouchInfo, GetPointerInfo, GetPointerPenInfo,
            GetPointerTouchInfo, GetPointerType, POINTER_FLAGS, POINTER_FLAG_CANCELED,
            POINTER_FLAG_CAPTURECHANGED, POINTER_FLAG_CONFIDENCE, POINTER_FLAG_DOWN,
            POINTER_FLAG_FIFTHBUTTON, POINTER_FLAG_FIRSTBUTTON, POINTER_FLAG_FOURTHBUTTON,
            POINTER_FLAG_HASTRANSFORM, POINTER_FLAG_HWHEEL, POINTER_FLAG_INCONTACT,
            POINTER_FLAG_INRANGE, POINTER_FLAG_NEW, POINTER_FLAG_NONE, POINTER_FLAG_PRIMARY,
            POINTER_FLAG_SECONDBUTTON, POINTER_FLAG_THIRDBUTTON, POINTER_FLAG_UP,
            POINTER_FLAG_UPDATE, POINTER_FLAG_WHEEL, POINTER_INFO, POINTER_PEN_INFO,
            POINTER_TOUCH_INFO,
        },
        WindowsAndMessaging::{
            BN_CLICKED, BN_DBLCLK, BN_PUSHED, CREATESTRUCTW, EN_CHANGE, POINTER_INPUT_TYPE,
            PT_MOUSE, PT_PEN, PT_POINTER, PT_TOUCH, PT_TOUCHPAD, WM_ACTIVATE, WM_CLOSE, WM_COMMAND,
            WM_CREATE, WM_DESTROY, WM_DISPLAYCHANGE, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN,
            WM_MOUSEWHEEL, WM_PAINT, WM_POINTERDOWN, WM_POINTERENTER, WM_POINTERHWHEEL,
            WM_POINTERLEAVE, WM_POINTERUP, WM_POINTERUPDATE, WM_POINTERWHEEL, WM_QUIT,
            WM_RBUTTONDOWN, WM_SIZE, WM_USER,
        },
    },
};

use crate::{
    errors::last_error,
    param_ext::{LParamExt, ParamExt},
    window_handle_ext::WindowHandleExt,
};

/// First messages on window creation
/// - (36) - WM_GETMINMAXINFO
/// - (129) - WM_NCCREATE
/// - (131) - WM_NCCALCSIZE -- first that goes to user to handle
/// - (1) - WM_CREATE
/// - (24) - WM_SHOWWINDOW
/// - (70) - WM_WINDOWPOSCHANGING
/// - (70) - WM_WINDOWPOSCHANGING
/// - (28) - WM_ACTIVATEAPP
/// - (134) - WM_NCACTIVATE
/// - (127) - WM_GETICON
/// - (127) - WM_GETICON
/// - (127) - WM_GETICON
/// - (6) - WM_ACTIVATE
/// - (641) - WM_IME_SETCONTEXT
/// - (642) - WM_IME_NOTIFY
/// - (61) - WM_GETOBJECT
/// - (61) - WM_GETOBJECT
/// - (7) - WM_SETFOCUS
/// - (133) - WM_NCPAINT
/// - (20) - WM_ERASEBKGND
/// - (71) - WM_WINDOWPOSCHANGED
/// - (5) - WM_SIZE
/// - (3) - WM_MOVE
/// - (127) - WM_GETICON
///  
/// - (799) - WM_DWMNCRENDERINGCHANGED --- first thats gose into PeekMessageW
/// - (49422) - ????
/// - (127) - WM_GETICON
/// - (18) - WM_QUIT
///
#[derive(Debug)]
pub enum Message {
    /// WM_CREATE
    /// (1)
    ///
    /// _wParam_ is not used
    ///
    /// _lParam_ pointer to a CREATESTRUCT
    Create(CREATESTRUCTW),

    /// [WM_DESTROY](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
    /// (2)
    ///
    /// _wParam_ and _lParam_ are not used
    Destroy,

    /// WM_CLOSE
    /// (16)
    ///
    /// _wParam_ and _lParam_ are not used
    Close,

    /// WM_PAINT
    /// (15)
    ///
    /// _wParam_ and _lParam_ are not used
    Paint,

    /// [WM_POINTERDOWN](https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/inputmsg/wm-pointerdown.md)
    /// (582)
    ///
    /// _wParam_ is a lot of stuff
    ///
    /// _lParam_ is contains the point location of the pointer
    ///
    /// Posted when a pointer makes contact over the client area of a window.
    PointerDown(PointerEvent),

    /// [WM_POINTERUPDATE](https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/inputmsg/wm-pointerupdate.md)
    /// (581)
    PointerUpdate(PointerEvent),

    /// [WM_POINTERUP](https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/inputmsg/wm-pointerup.md)
    /// (583)
    PointerUp(PointerEvent),

    /// [WM_POINTERENTER](https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/inputmsg/wm-pointerenter.md)
    ///
    /// Sent to a window when a new pointer enters detection range over the window (hover) or when an existing pointer
    /// moves within the boundaries of the window.
    PointerEnter(PointerEvent),

    /// [WM_POINTERLEAVE](https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/inputmsg/wm-pointerleave.md)
    ///
    /// Sent to a window when a pointer leaves detection range over the window (hover) or when a pointer moves
    /// outside the boundaries of the window.
    PointerLeave(PointerEvent),

    /// [WM_LBUTTONDOWN](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
    ///
    /// _wParam_ is indicates whether various virtual keys are down.
    ///
    /// _lParam_ is contains the point location of the pointer
    LeftButtonDown,

    /// WM_RBUTTONDOWN
    RightButtonDown,

    /// WM_LBUTTONDBLCLK
    LeftButtondDoubleClick,

    /// WM_COMMAND
    /// (273)
    Command(CommandInfo),

    /// [WM_USER](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-user)
    /// (1024)
    ///
    /// user-defined Windows messages
    /// Used to define private messages for use by private window classes,
    /// usually of the form WM_USER+x, where x is an integer value.
    User,

    /// [WM_ACTIVATE](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
    /// (6)
    ///
    /// Sent to both the window being activated and the window being deactivated.
    Activate,

    /// [WM_DISPLAYCHANGE](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-displaychange)
    /// (126)
    ///
    /// message is sent to all windows when the display resolution has changed.
    DisplayChange,

    /// [WM_SIZE](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size)
    /// (5)
    ///
    /// Sent to a window after its size has changed.
    Size,

    /// [WM_MOUSEWHEEL](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel)
    /// (522)
    ///
    /// Note: Pinch zoom gestures also triggers the MouseWheel event.
    MouseWheel(MouseWheelInfo),

    /// [WM_QUIT](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-quit)
    /// (18)
    ///
    /// Indicates a request to terminate an application, and is generated when the application
    /// calls the PostQuitMessage function. This message causes the GetMessage function to
    /// return zero.
    Quit,

    Other,
}

pub fn message_handler(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> Message {
    match msg {
        WM_CREATE => Message::Create(*l.get_create_struct()),
        WM_CLOSE => Message::Close,
        WM_PAINT => Message::Paint,
        WM_POINTERDOWN => Message::PointerDown(pointer_message_handler(
            hwnd,
            msg,
            w,
            l,
            EventType::PointerDown,
        )),
        WM_POINTERUPDATE => Message::PointerUpdate(pointer_message_handler(
            hwnd,
            msg,
            w,
            l,
            EventType::PointerUpdate,
        )),
        WM_POINTERUP => Message::PointerUp(pointer_message_handler(
            hwnd,
            msg,
            w,
            l,
            EventType::PointerUp,
        )),
        WM_POINTERENTER => Message::PointerLeave(pointer_message_handler(
            hwnd,
            msg,
            w,
            l,
            EventType::PointerEnter,
        )),
        WM_POINTERLEAVE => Message::PointerLeave(pointer_message_handler(
            hwnd,
            msg,
            w,
            l,
            EventType::PointerLeave,
        )),
        #[cfg(todo)]
        WM_POINTERHWHEEL => Message::Other,
        #[cfg(todo)]
        WM_POINTERWHEEL => Message::Other,
        WM_DESTROY => Message::Destroy,
        WM_LBUTTONDOWN => Message::LeftButtonDown,
        WM_RBUTTONDOWN => Message::RightButtonDown,
        WM_LBUTTONDBLCLK => Message::LeftButtondDoubleClick,
        WM_COMMAND => Message::Command(command_message_handler(hwnd, msg, w, l)),
        WM_USER => Message::User,
        WM_ACTIVATE => Message::Activate,
        WM_DISPLAYCHANGE => Message::DisplayChange,
        WM_SIZE => Message::Size,
        WM_MOUSEWHEEL => Message::MouseWheel(mouse_wheel_message_handler(hwnd, msg, w, l)),
        WM_QUIT => Message::Quit,
        _ => Message::Other,
    }
}

#[derive(Debug)]
pub enum VirtualKeys {
    /// MK_CONTROL
    /// 0x0008
    /// The CTRL key is down.
    Ctrl,

    /// MK_LBUTTON
    /// 0x0001
    /// The left mouse button is down.
    LeftMouseButton,

    /// MK_MBUTTON
    /// 0x0010
    /// The middle mouse button is down.
    MiddleMouseButton,

    /// MK_RBUTTON
    /// 0x0002
    /// The right mouse button is down.
    RightMouseButton,

    /// MK_SHIFT
    /// 0x0004
    /// The SHIFT key is down.
    Shift,

    /// MK_XBUTTON1
    /// 0x0020
    /// The first X button is down.
    FirstXButton,

    /// MK_XBUTTON2
    /// 0x0040
    /// The second X button is down.
    SecondXButton,
}

#[derive(Debug)]
pub struct MouseWheelInfo {
    pub keys: Option<VirtualKeys>,
    pub z_delta: i16,
    pub position: IVec2,
    pub local_position: IVec2,
}

fn mouse_wheel_message_handler(hwnd: HWND, _msg: u32, w: WPARAM, l: LPARAM) -> MouseWheelInfo {
    let keys = w.get_loword();
    let z_delta = w.get_hiword() as i16;

    let keys = match MODIFIERKEYS_FLAGS(keys) {
        MK_CONTROL => Some(VirtualKeys::Ctrl),
        MK_LBUTTON => Some(VirtualKeys::LeftMouseButton),
        MK_MBUTTON => Some(VirtualKeys::MiddleMouseButton),
        MK_RBUTTON => Some(VirtualKeys::RightMouseButton),
        MK_SHIFT => Some(VirtualKeys::Shift),
        MK_XBUTTON1 => Some(VirtualKeys::FirstXButton),
        MK_XBUTTON2 => Some(VirtualKeys::SecondXButton),
        _ => None,
    };

    let position = l.get_point();

    MouseWheelInfo {
        keys,
        z_delta,
        position,
        local_position: hwnd.screen_to_client(&position),
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct PointerFlags: u32 {
        /// Default
        const POINTER_FLAG_NONE = POINTER_FLAG_NONE.0;
        /// Indicates the arrival of a new pointer.
        const POINTER_FLAG_NEW = POINTER_FLAG_NEW.0;
        /// Indicates that this pointer continues to exist. When this flag is not set, it indicates the
        /// pointer has left detection range.
        const POINTER_FLAG_INRANGE = POINTER_FLAG_INRANGE.0;
        /// Indicates that this pointer is in contact with the window surface. When this flag is not
        /// set, it indicates a hovering pointer.
        const POINTER_FLAG_INCONTACT = POINTER_FLAG_INCONTACT.0;
        /// Indicates a primary action, analogous to a mouse left button down.
        ///
        /// A touch pointer has this flag set when it is in contact with the digitizer surface.
        ///
        /// A pen pointer has this flag set when it is in contact with the digitizer surface with no
        /// buttons pressed.
        ///
        /// A mouse pointer has this flag set when the mouse left button is down.
        const POINTER_FLAG_FIRSTBUTTON = POINTER_FLAG_FIRSTBUTTON.0;
        ///Indicates a secondary action, analogous to a mouse right button down.
        ///
        /// A touch pointer does not use this flag.
        ///
        /// A pen pointer has this flag set when it is in contact with the digitizer surface with the
        /// pen barrel button pressed.
        ///
        /// A mouse pointer has this flag set when the mouse right button is down.
        const POINTER_FLAG_SECONDBUTTON = POINTER_FLAG_SECONDBUTTON.0;
        /// Indicates a secondary action, analogous to a mouse right button down.
        ///
        /// A touch pointer does not use this flag.
        ///
        /// A pen pointer does not use this flag.
        ///
        /// A mouse pointer has this flag set when the mouse middle button is down.
        const POINTER_FLAG_THIRDBUTTON = POINTER_FLAG_THIRDBUTTON.0;
        const POINTER_FLAG_FOURTHBUTTON = POINTER_FLAG_FOURTHBUTTON.0;
        const POINTER_FLAG_FIFTHBUTTON = POINTER_FLAG_FIFTHBUTTON.0;
        /// Indicates that this pointer has been designated as primary. A primary pointer may
        /// perform actions beyond those available to non-primary pointers. For example, when
        /// a primary pointer makes contact with a window’s surface, it may provide the window
        /// an opportunity to activate by sending it a WM_POINTERACTIVATE message.
        const POINTER_FLAG_PRIMARY = POINTER_FLAG_PRIMARY.0;
        const POINTER_FLAG_CONFIDENCE = POINTER_FLAG_CONFIDENCE.0;
        const POINTER_FLAG_CANCELED = POINTER_FLAG_CANCELED.0;
        /// Indicates that this pointer just transitioned to a “down” state; that is, it made contact
        /// with the window surface.
        const POINTER_FLAG_DOWN = POINTER_FLAG_DOWN.0;
        /// Indicates that this information provides a simple update that does not include
        /// pointer state changes.
        const POINTER_FLAG_UPDATE = POINTER_FLAG_UPDATE.0;
        /// Indicates that this pointer just transitioned to an “up” state; that is, it broke contact
        /// with the window surface.
        const POINTER_FLAG_UP = POINTER_FLAG_UP.0;
        /// Indicates input associated with a pointer wheel. For mouse pointers, this is
        /// equivalent to the action of the mouse scroll wheel (WM_MOUSEWHEEL).
        const POINTER_FLAG_WHEEL = POINTER_FLAG_WHEEL.0;
        /// Indicates input associated with a pointer h-wheel. For mouse pointers, this is
        /// equivalent to the action of the mouse horizontal scroll wheel
        /// (WM_MOUSEHWHEEL).
        const POINTER_FLAG_HWHEEL = POINTER_FLAG_HWHEEL.0;
        const POINTER_FLAG_CAPTURECHANGED = POINTER_FLAG_CAPTURECHANGED.0;
        const POINTER_FLAG_HASTRANSFORM = POINTER_FLAG_HASTRANSFORM.0;
    }
}

fn pointer_message_handler(
    hwnd: HWND,
    _msg: u32,
    w: WPARAM,
    l: LPARAM,
    event_type: EventType,
) -> PointerEvent {
    let pointer_id: PointerId = PointerId::new(w);

    let pointer_input_type = pointer_id.get_pointer_type();
    let pointer_info = pointer_id.get_pointer_info();

    let location = IVec2 {
        x: pointer_info.ptPixelLocation.x,
        y: pointer_info.ptPixelLocation.y,
    };

    let flags = PointerFlags::from_bits(pointer_info.pointerFlags.0).unwrap();

    let pointer_info = match pointer_input_type {
        PT_POINTER => PointerEvent {
            pointer_id,
            position: l.get_point(),
            local_position: hwnd.screen_to_client(&location),
            time: pointer_info.dwTime,
            frame_id: pointer_info.frameId,
            pointer_type: PointerType::Pointer,
            event_type,
        },
        PT_TOUCH => {
            let pointer_touch_info = pointer_id.get_pointer_touch_info();

            PointerEvent {
                pointer_id,
                position: l.get_point(),
                local_position: hwnd.screen_to_client(&location),
                time: pointer_info.dwTime,
                frame_id: pointer_info.frameId,
                pointer_type: PointerType::Touch(TouchInfo {}),
                event_type,
            }
        }
        PT_PEN => {
            let pointer_pen_info = pointer_id.get_pointer_pen_info();

            PointerEvent {
                pointer_id,
                position: l.get_point(),
                local_position: hwnd.screen_to_client(&location),
                time: pointer_info.dwTime,
                frame_id: pointer_info.frameId,
                pointer_type: PointerType::Pen(PenInfo {
                    pressure: pointer_pen_info.pressure,
                    tilt: (pointer_pen_info.tiltX, pointer_pen_info.tiltY),
                    rotation: pointer_pen_info.rotation,
                }),
                event_type,
            }
        }
        PT_MOUSE => PointerEvent {
            pointer_id,
            position: l.get_point(),
            local_position: hwnd.screen_to_client(&location),
            time: pointer_info.dwTime,
            frame_id: pointer_info.frameId,
            pointer_type: PointerType::Mouse(MouseInfo {}),
            event_type,
        },
        PT_TOUCHPAD => PointerEvent {
            pointer_id,
            position: l.get_point(),
            local_position: hwnd.screen_to_client(&location),
            time: pointer_info.dwTime,
            frame_id: pointer_info.frameId,
            pointer_type: PointerType::Touchpad,
            event_type,
        },
        _ => {
            panic!()
        }
    };

    pointer_info
}

fn command_message_handler(_hwnd: HWND, _msg: u32, w: WPARAM, l: LPARAM) -> CommandInfo {
    let child_handle = l.get_child_handle();
    let message = w.get_hiword();

    let class_name = child_handle.get_class_name();

    let type_command = match &class_name as &str {
        "Button" => TypeCommand::Button,
        "Edit" => TypeCommand::Edit,
        _ => TypeCommand::Other,
    };

    let command = match type_command {
        TypeCommand::Button => match message {
            BN_CLICKED => Command::Clicked,
            BN_DBLCLK => Command::DoubleClick,
            BN_PUSHED => Command::Pushed,
            _ => Command::Other,
        },
        TypeCommand::Edit => match message {
            EN_CHANGE => Command::Change,
            _ => Command::Other,
        },
        _ => Command::Other,
    };

    CommandInfo {
        handle: child_handle,
        message,
        command,
        type_command,
    }
}

#[derive(Debug)]
pub struct CommandInfo {
    /// Childe HWND
    pub handle: HWND,
    pub message: u32,
    pub command: Command,
    pub type_command: TypeCommand,
}

#[derive(Debug)]
pub enum TypeCommand {
    Button,
    Edit,
    Other,
}

#[derive(Debug)]
pub enum Command {
    Clicked,
    DoubleClick,
    Pushed,
    Change,
    Other,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum EventType {
    PointerDown,
    PointerUpdate,
    PointerUp,
    PointerEnter,
    PointerLeave,
    #[default]
    PointerOver,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PointerEvent {
    pub pointer_id: PointerId,
    pub position: IVec2,
    pub local_position: IVec2,
    pub frame_id: u32,
    pub time: u32,
    pub pointer_type: PointerType,
    pub event_type: EventType,
}

/// hover state?
/// in hover state Pen don't have pressure, tilt and rotation
/// I don't know
#[derive(Debug, Clone, Copy)]
pub struct PenInfo {
    pub pressure: u32,
    pub tilt: (i32, i32),
    pub rotation: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MouseInfo {}

#[derive(Debug, Clone, Copy)]
pub struct TouchInfo {}

#[derive(Debug, Clone, Copy, Default)]
pub enum PointerType {
    #[default]
    Pointer,
    Touch(TouchInfo),
    Pen(PenInfo),
    Mouse(MouseInfo),
    Touchpad,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
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

        println!("PT_TOUCH GetLastError {:?}", last_error());

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
