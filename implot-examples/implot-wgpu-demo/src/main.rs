use imgui::{im_str, Condition, Window};
use implot::Context;

// The actual backend-specific code is in this.
mod support;

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    let mut showing_rust_demo = true;
    let plotcontext = Context::create();
    system.main_loop(move |_, ui| {
        // The context is moved into the closure after creation so plot_ui is valid.
        let plot_ui = plotcontext.get_plot_ui();

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }

        if showing_rust_demo {
            examples_shared::show_demos(ui, &plot_ui);
        }
    });
}
