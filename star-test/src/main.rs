use miette::{Diagnostic, SourceSpan, NamedSource, Result};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("oops!")]
#[diagnostic(
	code(oops::my_bad),
	url(docsrs),
    help("try doing it better next time?"),
)]
struct MyBad {
	#[source_code]
	src: NamedSource,
	#[label("This bit here")]
	bad_bit: SourceSpan
}

fn this_fails() -> Result<()> {
    let src = "source\n  text\n    here".to_string();
    // let len = src.len();

    Err(MyBad {
        src: NamedSource::new("bad_file.rs", src),
        bad_bit: (9, 4).into(),
    })?;

    Ok(())
}

fn main() -> Result<()> {
	this_fails()?;

	Ok(())
}