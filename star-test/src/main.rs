use click_derive::*;
use twilight_model::application::component::button::ButtonStyle;

trait ClickCommand<const N: usize> {
	const LABELS: [&'static str; N];
	const STYLES: [ButtonStyle; N];
	const LINKS: &'static [(usize, &'static str)] = &[];
}

#[derive(ClickCommand)]
#[buttons(Danger("Hello, world!"), Link("Testing", "https://github.com"))]
struct TestCommand;

fn main() {}
