use std::fs::File;
use std::io::{self, BufWriter, Write};

pub fn write_matches_to_file(matches: &[String], output_file: &str) -> io::Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    for match_ in matches {
        writeln!(writer, "{}", match_)?;
    }

    writer.flush()?;
    Ok(())
}