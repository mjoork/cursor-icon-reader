use std::{convert::TryInto, fs::File, io::Read};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[derive(Debug)]
struct IconDirEntry {
    nth: usize,
    width: u8,
    height: u8,
    color_count: u8,
    reserved_byte: u8,
    x_hotspot: u16,
    y_hotspot: u16,
    size_of_image: u32,
    offset_of_image_data: u32,
}

struct IconDirEntries(Vec<IconDirEntry>);

impl IconDirEntries {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl std::fmt::Display for IconDirEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for entry in self.0.iter() {
            buffer.push_str(&format!("{}", entry));
        }
        write!(f, "{}", buffer)
    }
}

impl std::fmt::Display for IconDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
┍━━━━┯━━━━━━━━━━━━┑
│ #{} │ Size       : {}x{}px
┕━━━━┥ N Colors   : {}
     │ Res. Byte  : {}
     │ X Hotspot  : {}
     │ Y Hotspot  : {}
     │ Image (Kb) : {}
     │ Img. Offset: {}
     ┕━━━━━━━━━━━━┙",
            self.nth,
            self.width,
            self.height,
            self.color_count,
            self.reserved_byte,
            self.x_hotspot,
            self.y_hotspot,
            self.size_of_image,
            self.offset_of_image_data
        )
    }
}

struct IconDir {
    reserved_bytes: u16,
    type_of_file: u16,
    number_of_images: u16,
    entries: IconDirEntries,
}

impl std::fmt::Display for IconDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "I've read the file, here's the info:

Type of file    : {}
Number of images: {}
{}",
            if self.type_of_file == 1 {
                "ICO"
            } else if self.type_of_file == 2 {
                "CUR"
            } else {
                "UNKNOWN"
            },
            self.number_of_images,
            self.entries
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut cur_file = File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    cur_file.read_to_end(&mut data)?;

    let mut cur = IconDir {
        reserved_bytes: (&data[0..2]).read_u16::<LittleEndian>()?,
        type_of_file: (&data[2..4]).read_u16::<LittleEndian>()?,
        number_of_images: (&data[4..6]).read_u16::<LittleEndian>()?,
        entries: IconDirEntries::with_capacity((&data[4..6]).read_u16::<LittleEndian>()?.into()),
    };

    for nth_image in 0..cur.number_of_images as usize {
        let offset = 6 + nth_image * 16;
        cur.entries.0.push(IconDirEntry {
            nth: nth_image + 1,
            width: data[offset],
            height: data[offset + 1],
            color_count: data[offset + 2],
            reserved_byte: data[offset + 3],
            x_hotspot: (&data[offset + 4..offset + 6]).read_u16::<LittleEndian>()?,
            y_hotspot: (&data[offset + 6..offset + 8]).read_u16::<LittleEndian>()?,
            size_of_image: (&data[offset + 8..offset + 12]).read_u32::<LittleEndian>()?,
            offset_of_image_data: (&data[offset + 12..offset + 16]).read_u32::<LittleEndian>()?,
        })
    }

    println!("{}", cur);

    Ok(())
}
