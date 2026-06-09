# 56 Crubit

Crubit is a bindings generator for C++ and Rust. The goal is similar to the earlier interop folders in this repo, but the workflow is different: Crubit wants deep build-system integration, especially Bazel.

This folder shows two tiny directions:

1. Rust calls C++ from [cpp_temperature.h](cpp_temperature.h) through [rust_calls_cpp.rs](rust_calls_cpp.rs).
2. C++ calls Rust from [rust_rules.rs](rust_rules.rs) through [cpp_calls_rust.cc](cpp_calls_rust.cc).

The [BUILD](BUILD) file is written in the same style as Crubit's own examples under `D:\my-projects\Rust\crubit\examples`.

## Important Status Note

As of the Crubit checkout used for this example, Crubit's docs say:

- Bazel is the currently supported build system for the full workflow.
- `cc_bindings_from_rs` can be built with Cargo, but `rs_bindings_from_cc` is not a Cargo workflow yet.
- Windows is currently unsupported for the full generated API workflow.

So on this Windows machine, the most reliable way to run the example is WSL/Linux with Bazel, or another Linux environment where your local Crubit checkout builds. You can still read the files here on Windows as a learning example.

## Direction 1: Rust Calls C++

[cpp_temperature.h](cpp_temperature.h) is a normal C++ header:

- `classroom::TemperatureReading` is a simple struct.
- `classroom::celsius_to_fahrenheit` is a simple function.
- `classroom::recommend_fan` returns a C++ struct to Rust.

In [BUILD](BUILD), this C++ target opts into Crubit:

```python
cc_library(
    name = "cpp_temperature",
    hdrs = ["cpp_temperature.h"],
    aspect_hints = ["//features:supported"],
)
```

Then the Rust binary depends on it with `cc_deps`:

```python
rust_binary(
    name = "rust_calls_cpp",
    srcs = ["rust_calls_cpp.rs"],
    cc_deps = [":cpp_temperature"],
)
```

That is the main Crubit idea for C++ to Rust: Rust does not write an FFI block by hand. The generated Rust crate appears from the C++ target.

See [generated_snapshots/cpp_temperature_generated.rs](generated_snapshots/cpp_temperature_generated.rs) for a simplified sketch of what the generated Rust API looks like.

## Direction 2: C++ Calls Rust

[rust_rules.rs](rust_rules.rs) is a normal Rust library with public functions and a public struct.

In [BUILD](BUILD), the Rust library is wrapped with `cc_bindings_from_rust`:

```python
rust_library(
    name = "rust_rules",
    srcs = ["rust_rules.rs"],
)

cc_bindings_from_rust(
    name = "rust_rules_cc_api",
    crate = ":rust_rules",
)
```

Then C++ depends on that generated API:

```python
cc_binary(
    name = "cpp_calls_rust",
    srcs = ["cpp_calls_rust.cc"],
    deps = [":rust_rules_cc_api"],
)
```

The C++ include uses the Rust target name, not the `cc_bindings_from_rust` target name:

```cpp
#include "examples/56_crubit/rust_rules.h"
```

See [generated_snapshots/rust_rules_generated.h](generated_snapshots/rust_rules_generated.h) for a simplified sketch of what the generated C++ API looks like.

## How To Build And Run

Because this repo is not the Crubit Bazel workspace, copy this folder into the local Crubit checkout first.

From PowerShell:

```powershell
$src = "D:\my-projects\Rust\rust-learn\rust-learn\56_crubit"
$dst = "D:\my-projects\Rust\crubit\examples\56_crubit"
New-Item -ItemType Directory -Force $dst
Copy-Item "$src\*" $dst -Recurse -Force
```

Then build from a Linux/WSL shell where Bazel can build Crubit:

```sh
cd /mnt/d/my-projects/Rust/crubit

# Rust calls the C++ header through generated Rust bindings.
bazel run //examples/56_crubit:rust_calls_cpp

# C++ calls the Rust library through a generated C++ header.
bazel run //examples/56_crubit:cpp_calls_rust
```

Expected output will be close to:

```text
Rust called C++ through Crubit
27 C is 80 F
fan_on = true
comfortable = false
```

and:

```text
C++ called Rust through Crubit
score.points = 55
score.needs_attention = true
should_open_window = true
```

To inspect generated files:

```sh
cd /mnt/d/my-projects/Rust/crubit

# Generates and prints paths for Rust bindings for the C++ target.
bazel build --config=crubit-genfiles //examples/56_crubit:cpp_temperature

# Generates and prints paths for C++ bindings for the Rust target.
bazel build --config=crubit-genfiles //examples/56_crubit:rust_rules_cc_api
```

## Comparison With CXX And cbindgen

`cbindgen` is a good fit when C or C++ needs a C ABI header for Rust code. You usually write `extern "C"` Rust functions, generate a `.h` file, then link the Rust library yourself. It is simple and portable, but it works at the C ABI level.

`CXX` is a good fit when you want a Cargo-friendly, explicit bridge. You write a `#[cxx::bridge]` module that describes exactly which Rust and C++ items cross the boundary. It is intentionally limited, but very practical for many projects.

`Crubit` tries to generate bindings from existing Rust and C++ build targets. Rust can depend on C++ with `cc_deps`, and C++ can depend on Rust through `cc_bindings_from_rust`. That can feel more automatic than CXX or cbindgen, but today it depends heavily on Crubit's supported build setup and supported language features.

For learning, the mental model is:

- cbindgen: Rust exports a C-shaped API.
- CXX: Rust and C++ meet at an explicit bridge module.
- Crubit: the build system generates bindings from normal Rust/C++ targets.