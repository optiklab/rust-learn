// This is an educational sketch of the kind of Rust API Crubit generates from
// cpp_temperature.h. The exact generated file may differ between Crubit commits.

#![allow(nonstandard_style)]

pub mod classroom {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct TemperatureReading {
        pub celsius: i32,
        pub fan_on: bool,
    }

    pub fn celsius_to_fahrenheit(celsius: i32) -> i32 {
        unsafe { crate::detail::__rust_thunk_celsius_to_fahrenheit(celsius) }
    }

    pub fn recommend_fan(celsius: i32) -> TemperatureReading {
        unsafe { crate::detail::__rust_thunk_recommend_fan(celsius) }
    }

    pub fn is_comfortable(reading: TemperatureReading) -> bool {
        unsafe { crate::detail::__rust_thunk_is_comfortable(reading) }
    }
}

mod detail {
    use super::classroom::TemperatureReading;

    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk_celsius_to_fahrenheit(celsius: i32) -> i32;
        pub(crate) unsafe fn __rust_thunk_recommend_fan(celsius: i32) -> TemperatureReading;
        pub(crate) unsafe fn __rust_thunk_is_comfortable(reading: TemperatureReading) -> bool;
    }
}