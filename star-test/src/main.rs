use click_derive::*;
use twilight_model::application::component::button::ButtonStyle;

trait ClickCommand<const N: usize> {
	const LABELS: [&'static str; N];
	const STYLES: [ButtonStyle; N];
}

#[derive(ClickCommand)]
#[styles(Success, Danger)]
#[labels("Hello, world!", "Goodbye!")]
struct TestCommand;

fn main() {}
