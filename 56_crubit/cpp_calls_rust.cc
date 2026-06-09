#include <cstdint>
#include <iostream>

// Crubit generates this header from the rust_library target named rust_rules.
// The include path is based on the Bazel label, not on the cc_bindings target.
#include "examples/56_crubit/rust_rules.h"

int main() {
  std::cout << "C++ called Rust through Crubit\n";

  rust_rules::SensorScore score = rust_rules::score_reading(27, true);
  bool open_window = rust_rules::should_open_window(27);

  std::cout << "score.points = " << score.points << "\n";
  std::cout << "score.needs_attention = " << score.needs_attention << "\n";
  std::cout << "should_open_window = " << open_window << "\n";

  return 0;
}