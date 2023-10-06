use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    time::{Duration, SystemTime},
};

use glam::{IVec2, Vec2};
use windows_reactive::messages::{EventType, PointerEvent, PointerId, PointerType};

#[derive(Debug)]
pub struct PointerEventInput {
    pointers: HashMap<PointerId, PointerEvent>,
    first_input: Option<HammerInput>,
    first_multiple: Option<HammerInput>,
    offset_delta: Vec2,
    prev_delta: Vec2,
    prev_input: Option<HammerInput>,
}

impl PointerEventInput {
    pub fn new() -> Self {
        PointerEventInput {
            pointers: HashMap::new(),
            first_input: None,
            first_multiple: None,
            offset_delta: Vec2::ZERO,
            prev_delta: Vec2::ZERO,
            prev_input: None,
        }
    }

    pub fn handler(&mut self, event: PointerEvent) -> HammerInput {
        let pointer_type = match event.pointer_type {
            PointerType::Pointer => InputType::Touch,
            PointerType::Touch(_) => InputType::Touch,
            PointerType::Pen(_) => InputType::Pen,
            PointerType::Mouse(_) => InputType::Mouse,
            PointerType::Touchpad => InputType::Touch,
        };

        let event_type = match event.event_type {
            EventType::PointerDown => InputEventType::Start,
            EventType::PointerUpdate => InputEventType::Move,
            EventType::PointerUp => InputEventType::End,
            EventType::PointerEnter => InputEventType::Cancel,
            EventType::PointerLeave => InputEventType::Cancel,
            EventType::PointerOver => InputEventType::Cancel,
        };

        let is_touch = match event.pointer_type {
            PointerType::Touch(_) => true,
            _ => false,
        };

        let remove_pointer = match event_type {
            InputEventType::Start => {
                self.pointers.clear();
                self.pointers.insert(event.pointer_id, event);
                false
            }
            InputEventType::Move => {
                self.pointers.insert(event.pointer_id, event);
                false
            }
            InputEventType::End | InputEventType::Cancel => true,
            _ => false,
        };

        let changed_pointers = vec![event];
        let pointers_len = self.pointers.len() as i32;
        let changed_pointers_len = changed_pointers.len() as i32;

        let (is_first, is_final) = if changed_pointers_len - pointers_len == 0 {
            match event_type {
                InputEventType::Start => (true, false),
                InputEventType::End => (false, true),
                InputEventType::Cancel => (false, true),
                InputEventType::Move => (false, false),
            }
        } else {
            (false, false)
        };

        if is_first {
            self.first_input = None;
            self.first_multiple = None;
            self.offset_delta = Vec2::ZERO;
            self.prev_delta = Vec2::ZERO;
            self.prev_input = None;
        }

        let pointers = self
            .pointers
            .clone()
            .into_values()
            .collect::<Vec<PointerEvent>>();

        let pointers_vec = pointers
            .iter()
            .map(|p| p.local_position)
            .collect::<Vec<IVec2>>();

        let time_stamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let center = get_center(&pointers);

        let mut input = HammerInput {
            event_type,
            is_first,
            is_final,
            time_stamp,
            center,
            pointers,
            pointers_vec: pointers_vec.clone(),
            changed_pointers,
            pointer_type,
            ..Default::default()
        };

        // store the first input to calculate the distance and direction
        if self.first_input.is_none() {
            self.first_input = Some(input.clone());
        }

        // to compute scale and rotation we need to store the multiple touches
        if pointers_len > 1 && self.first_multiple.is_none() {
            self.first_multiple = Some(input.clone());
        } else if pointers_len == 1 {
            self.first_multiple = None;
        }

        let offset_center = if self.first_multiple.is_some() {
            self.first_multiple.as_ref().unwrap().center
        } else {
            self.first_input.as_ref().unwrap().center
        };

        input.delta_time = input.time_stamp - self.first_input.as_ref().unwrap().time_stamp;
        input.angle = get_angle(offset_center, input.center);
        input.distance = get_distance(offset_center, input.center);

        input.delta = compute_delta(self, &mut input);

        input.scale = if self.first_multiple.is_some() {
            get_scale(
                &self.first_multiple.as_ref().unwrap().pointers_vec,
                &pointers_vec,
            )
        } else {
            1.0
        };
        input.rotation = if self.first_multiple.is_some() {
            get_rotation(
                &self.first_multiple.as_ref().unwrap().pointers_vec,
                &pointers_vec,
            )
        } else {
            0.0
        };

        if remove_pointer {
            self.pointers.remove(&event.pointer_id);
        };

        // println!("----------------");
        // println!("🔴 HammerInput {:?} {:?}", input.scale, input.rotation,);

        self.prev_input = Some(input.clone());

        input
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum InputType {
    Touch,
    Pen,
    #[default]
    Mouse,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum InputEventType {
    Start,
    Move,
    End,
    #[default]
    Cancel,
}

#[derive(Debug, Clone, Default)]
pub struct HammerInput {
    /// Name of the event. Like panstart.
    // input_type: str,

    /// Movement of the X and Y axises.
    pub delta: Vec2,

    /// Total time in ms since the first input.
    pub delta_time: Duration,

    /// Distance moved.
    pub distance: f32,

    /// Angle moved.
    pub angle: f32,

    /// Velocity on the X and Y axis, in px/ms.
    pub velocity: i32,

    /// Highest velocityX/Y value.
    pub velocity_vec: Vec2,

    pub overall_velocity: i32,

    pub overall_velocity_vec: Vec2,

    /// Direction moved. Matches the DIRECTION constants.
    pub direction: i32,

    /// Direction moved from it's starting point. Matches the DIRECTION constants.
    pub offset_direction: i32,

    /// Scaling that has been done when multi-touch. 1 on a single touch.
    pub scale: f32,

    /// Rotation that has been done when multi-touch. 0 on a single touch.
    pub rotation: f32,

    /// Center position for multi-touch, or just the single pointer.
    pub center: IVec2,

    /// Source event object, type TouchEvent, MouseEvent or PointerEvent.
    pub src_event: PointerEvent,

    // /// Target that received the event.
    // target: HTMLElement;
    /// Primary pointer type, could be touch, mouse, pen or kinect.
    pointer_type: InputType,

    // Event type, matches the INPUT constants.
    event_type: InputEventType,

    /// true when the first input.
    pub is_first: bool,

    /// true when the final (last) input.
    pub is_final: bool,

    /// Array with all pointers, including the ended pointers (touchend, mouseup).
    pub pointers: Vec<PointerEvent>,

    pub pointers_vec: Vec<IVec2>,

    /// Array with all new/moved/lost pointers.
    pub changed_pointers: Vec<PointerEvent>,

    /// Maximum number of pointers detected in the gesture
    pub max_pointers: i32,

    /// Timestamp of a gesture
    pub time_stamp: Duration,
    // /// Reference to the srcEvent.preventDefault() method. Only for experts!
    // preventDefault: Function;
}

/// calculate the scale factor between two pointersets
/// no scale is 1, and goes down to 0 when pinched together, and bigger when pinched out
///
/// # Arguments
///
/// * `start` - array of pointers
/// * `end` - array of pointers
///
/// # Returns
///
/// * `scale`
///
pub fn get_scale(start: &Vec<IVec2>, end: &Vec<IVec2>) -> f32 {
    (IVec2::distance_squared(end[0], end[1]) as f32).sqrt()
        / (IVec2::distance_squared(start[0], start[1]) as f32).sqrt()
}

/// calculate the rotation degrees between two pointersets
/// @param {Array} start array of pointers
/// @param {Array} end array of pointers
/// @return {Number} rotation
///
pub fn get_rotation(start: &Vec<IVec2>, end: &Vec<IVec2>) -> f32 {
    get_angle(end[1], end[0]) + get_angle(start[1], start[0])
}

/**
 * @private
 * get the center of all the pointers
 * @param {Array} pointers
 * @return {Object} center contains `x` and `y` properties
 */
pub fn get_center(pointers: &Vec<PointerEvent>) -> IVec2 {
    if pointers.is_empty() {
        return IVec2::ZERO;
    }

    let pointers_length = pointers.len() as i32;

    // no need to loop when only one touch
    if pointers_length == 1 {
        return IVec2 {
            x: pointers.first().unwrap().local_position.x,
            y: pointers.first().unwrap().local_position.y,
        };
    }

    let mut x = 0;
    let mut y = 0;

    for point in pointers {
        x += point.local_position.x;
        y += point.local_position.y;
    }

    IVec2 {
        x: x / pointers_length,
        y: y / pointers_length,
    }
}

///
/// calculate the angle between two coordinates
/// # Arguments
/// * p1
/// * p2
/// # Returns
/// * angle
///
pub fn get_angle(p1: IVec2, p2: IVec2) -> f32 {
    let p = p2 - p1;
    f32::atan2(p.y as f32, p.x as f32) //* 180.0 / PI
}

///
/// calculate the absolute distance between two points
/// # Arguments
/// * p1 {x, y}
/// * p2 {x, y}
/// # Returns
/// * distance
///
pub fn get_distance(p1: IVec2, p2: IVec2) -> f32 {
    (p1.distance_squared(p2) as f32).sqrt()
}

pub fn compute_delta(session: &mut PointerEventInput, input: &mut HammerInput) -> Vec2 {
    if input.event_type == InputEventType::Start
        || session
            .prev_input
            .as_ref()
            .map_or(false, |v| v.event_type == InputEventType::End)
    {
        session.prev_delta = session.prev_input.as_ref().map_or(Vec2::ZERO, |v| v.delta);
        session.offset_delta = input.center.clone().as_vec2();
    }

    session.prev_delta + (input.center.as_vec2() - session.offset_delta)
}
