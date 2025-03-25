mod label;
mod button;
mod window;

pub use label::Label;
pub use button::Button;
pub use window::Window;

pub trait Widget {
    /// Ширина self.
    fn width(&self) -> usize;

    /// Прорисовка виджета в буфер.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Прорисовка виджета.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}