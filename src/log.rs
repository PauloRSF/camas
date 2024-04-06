use std::{error::Error, io::Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn log(tag: &str, message: &String, tag_color: Color) -> Result<(), Box<dyn Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(tag_color)))?;

    write!(&mut stdout, "{tag}: ")?;

    stdout.reset()?;

    writeln!(&mut stdout, "{message:?}")?;

    Ok(())
}
