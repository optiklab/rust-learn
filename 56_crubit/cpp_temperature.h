#ifndef RUST_LEARN_56_CRUBIT_CPP_TEMPERATURE_H_
#define RUST_LEARN_56_CRUBIT_CPP_TEMPERATURE_H_

#include <stdint.h>

namespace classroom {

// A trivial C++ struct: simple public fields, no custom destructor, no virtual
// methods. This kind of type is intentionally beginner-friendly for FFI tools.
struct TemperatureReading {
  int32_t celsius;
  bool fan_on;
};

// Inline functions are enough for this toy example. Crubit still generates a
// Rust function wrapper, so Rust code can call this like an ordinary function.
inline int32_t celsius_to_fahrenheit(int32_t celsius) {
  return celsius * 9 / 5 + 32;
}

inline TemperatureReading recommend_fan(int32_t celsius) {
  return TemperatureReading{celsius, celsius >= 25};
}

inline bool is_comfortable(TemperatureReading reading) {
  return reading.celsius >= 20 && reading.celsius <= 24 && !reading.fan_on;
}

}  // namespace classroom

#endif  // RUST_LEARN_56_CRUBIT_CPP_TEMPERATURE_H_