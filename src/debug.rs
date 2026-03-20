/// A no-op Debug struct that replaces the removed `iced_runtime::Debug`.
///
/// In iced 0.14, the `Debug` struct was removed from `iced_runtime`.
/// This provides a compatible no-op replacement.
pub struct Debug;

impl Debug {
    pub fn new() -> Self {
        Self
    }

    pub fn startup_started(&mut self) {}
    pub fn startup_finished(&mut self) {}

    pub fn event_processing_started(&mut self) {}
    pub fn event_processing_finished(&mut self) {}

    pub fn draw_started(&mut self) {}
    pub fn draw_finished(&mut self) {}

    pub fn render_started(&mut self) {}
    pub fn render_finished(&mut self) {}

    pub fn layout_started(&mut self) {}
    pub fn layout_finished(&mut self) {}

    pub fn view_started(&mut self) {}
    pub fn view_finished(&mut self) {}

    pub fn update_started(&mut self) {}
    pub fn update_finished(&mut self) {}

    pub fn log_message<T: std::fmt::Debug>(&self, _message: &T) {}

    pub fn toggle(&mut self) {}

    pub fn overlay(&self) -> Vec<String> {
        Vec::new()
    }
}
