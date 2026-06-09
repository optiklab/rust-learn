fn main() {
    // cpp_temperature is the generated Rust crate for the C++ cc_library target
    // named `cpp_temperature` in BUILD.
    let warm_day = cpp_temperature::classroom::recommend_fan(27);

    // C++ namespaces become Rust modules. The original C++ functions keep their
    // names, but they are called through Rust syntax.
    let fahrenheit = cpp_temperature::classroom::celsius_to_fahrenheit(warm_day.celsius);
    let comfortable = cpp_temperature::classroom::is_comfortable(warm_day);

    println!("Rust called C++ through Crubit");
    println!("27 C is {fahrenheit} F");
    println!("fan_on = {}", warm_day.fan_on);
    println!("comfortable = {comfortable}");
}