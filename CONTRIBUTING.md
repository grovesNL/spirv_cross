# Contributing

`spirv_cross` is linked to the [`SPIRV-Cross`](https://github.com/KhronosGroup/SPIRV-Cross) library through git submodules. To receive changes from the upstream repository, update the submodule to track a different commit.

`spirv_cross` provides a number of C externs to enable automatic bindings generation from [`bindgen`](https://rust-lang-nursery.github.io/rust-bindgen/). To expose additional capabilities, edit [`wrapper.cpp`](https://github.com/grovesNL/spirv_cross/blob/master/spirv_cross/src/wrapper.cpp) and [`wrapper.hpp`](https://github.com/grovesNL/spirv_cross/blob/master/spirv_cross/src/wrapper.hpp). Afterwards, run `cargo run` within the `bindings_generator` directory, which will generate an updated `bindings.rs`. Feel free to update [`bindings_generator/src/main.rs`](https://github.com/grovesNL/spirv_cross/blob/master/bindings_generator/src/main.rs) if changes are necessary to expose additional C++ types that are supported by `bindgen`.
