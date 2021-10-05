use click_derive::*;
use twilight_model::application::component::button::ButtonStyle;

trait ClickCommand<const N: usize> {
	const LABELS: [&'static str; N];
	const STYLES: [ButtonStyle; N];
}

// #[derive(ClickCommand)]
// #[buttons(2)]
// #[styles("Hello, world!", "Goodbye!")]
// struct TestCommand;

fn main() {}
