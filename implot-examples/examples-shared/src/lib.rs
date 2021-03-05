pub mod bar_plots;
pub mod heatmaps;
pub mod line_plots;
pub mod scatter_plots;
pub mod stairs_plots;
mod stem_plots;
pub mod text_plots;

use imgui::{im_str, Condition, Ui, Window};
use implot::PlotUi;

pub fn show_demos(ui: &Ui, plot_ui: &PlotUi) {
    Window::new(im_str!("implot-rs demo"))
        .size([430.0, 450.0], Condition::FirstUseEver)
        .build(ui, || {
            // ui.text(im_str!("Bar plots:")); // uncomment this to make it crash
            bar_plots::show_demo_headers(ui, plot_ui);
        });
}
