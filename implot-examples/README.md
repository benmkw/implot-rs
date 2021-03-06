# Examples 

This folder contains examples that demonstrate how to use the Rust bindings. 
Things are structured as follows:

* [examples-shared](examples-shared/) is a library crate that contains the actual usage
  examples. It is used in the backend-specific crates.
* [implot-glium-demo](implot-glium-demo/) is an example for using `implot-rs` in 
conjunction with a [Glium](https://github.com/glium/glium) backend.
* [implot-wgpu-demo](implot-wgpu-demo/) is an example for using `implot-rs` in 
conjunction with a [WebGPU](https://github.com/gfx-rs/wgpu) backend 

If you want to just copy-paste code to start with, copy `examples-shared` along with 
your favourite backend example crate. The glium backend code is largely taken from imgui-rs.
