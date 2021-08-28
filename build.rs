use std::io::Result;

fn main() -> Result<()> {
	built::write_built_file()?;
	Ok(())
}
