// This Rust file is compiled as a rust_library target. Crubit can generate a
// C++ header for it, so C++ can call these functions without hand-written
// extern "C" declarations.

#[derive(Default, Clone)]
pub struct SensorScore {
    pub points: i32,
    pub needs_attention: bool,
}

pub fn score_reading(celsius: i32, fan_on: bool) -> SensorScore {
    let mut points = 100;

    if celsius < 18 || celsius > 26 {
        points -= 35;
    }

    if fan_on {
        points -= 10;
    }

    SensorScore {
        points,
        needs_attention: points < 80,
    }
}

pub fn should_open_window(celsius: i32) -> bool {
    celsius >= 25
}