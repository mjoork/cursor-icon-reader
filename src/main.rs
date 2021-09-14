use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Write;

enum FileType {
    Icon,
    Cursor,
    Unknown,
}

impl From<u16> for FileType {
    fn from(number: u16) -> Self {
        match number {
            1 => Self::Icon,
            2 => Self::Cursor,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FileType::Cursor => "Cursor",
            FileType::Icon => "Icon",
            FileType::Unknown => "Unknown",
        })
    }
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);

    let file_path = args.next().unwrap_or_else(|| {
        eprintln!("You must specify the path of the file to read.");
        std::process::exit(1);
    });

    let mut file = std::fs::File::open(file_path)?;

    // println! locks stdout for each invocation, so pre-locking stdout for the whole
    // program execution is better for performance.
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());

    // Reserved byte
    let _ = file.read_u16::<LittleEndian>()?;

    let filetype = FileType::from(file.read_u16::<LittleEndian>()?);
    writeln!(stdout, "File type: {}", filetype)?;

    let images_count = file.read_u16::<LittleEndian>()?;
    writeln!(stdout, "Images count: {}", images_count)?;

    for _ in 0..images_count {
        writeln!(stdout)?;
        writeln!(stdout, "Width: {}", file.read_u8()?)?;
        writeln!(stdout, "Height: {}", file.read_u8()?)?;
        writeln!(stdout, "Colors in palette: {}", file.read_u8()?)?;
        // Reserved byte
        let _ = file.read_u8()?;

        match filetype {
            FileType::Icon => {
                writeln!(stdout, "Color planes: {}", file.read_u16::<LittleEndian>()?)?;
                writeln!(stdout, "Bits per pixel: {}", file.read_u16::<LittleEndian>()?)?;
            }
            FileType::Cursor => {
                writeln!(stdout, "X Hotspot: {} px", file.read_u16::<LittleEndian>()?)?;
                writeln!(stdout, "Y Hotspot: {} px", file.read_u16::<LittleEndian>()?)?;
            }
            FileType::Unknown => {
                writeln!(stdout, "Color planes or X Hotspot: {} [#/px]", file.read_u16::<LittleEndian>()?)?;
                writeln!(stdout, "Bits per pixel or Y Hotspot: {} [#/px]", file.read_u16::<LittleEndian>()?)?;
            }
        }

        writeln!(stdout, "Image size: {} bytes", file.read_u32::<LittleEndian>()?)?;
        writeln!(stdout, "Image offset: {} bytes", file.read_u32::<LittleEndian>()?)?;
    }

    Ok(())
}
