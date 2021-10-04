use click_derive::*;

trait ClickCommand<const N: usize> {
	const BUTTONS: [u64; N];
}

#[derive(ClickCommand)]
#[buttons(2)]
// #[styles[
//     ["Hello, world!", Success],
//     ("Goodbye!", Danger)
// ]]
#[styles("Hello, world!", "Goodbye!")]
struct TestCommand;

fn main() {}
