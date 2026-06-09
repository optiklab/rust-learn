// This is an educational sketch of the kind of C++ API Crubit generates from
// rust_rules.rs. The exact generated file may differ between Crubit commits.

#ifndef RUST_LEARN_56_CRUBIT_RUST_RULES_GENERATED_H_
#define RUST_LEARN_56_CRUBIT_RUST_RULES_GENERATED_H_

#include <cstdint>

namespace rust_rules {

struct SensorScore final {
  std::int32_t points;
  bool needs_attention;
};

SensorScore score_reading(std::int32_t celsius, bool fan_on);
bool should_open_window(std::int32_t celsius);

}  // namespace rust_rules

#endif  // RUST_LEARN_56_CRUBIT_RUST_RULES_GENERATED_H_