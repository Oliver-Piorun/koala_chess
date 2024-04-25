use logger::*;
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

#[derive(Default)]
pub struct Bitmap {
    #[allow(dead_code)]
    pub file_header: FileHeader,
    #[allow(dead_code)]
    pub information_header: InformationHeader,
    pub data: Vec<u8>,
}

impl Bitmap {
    pub fn new(
        file_header: FileHeader,
        information_header: InformationHeader,
        data: Vec<u8>,
    ) -> Bitmap {
        Bitmap {
            file_header,
            information_header,
            data,
        }
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct FileHeader {
    pub r#type: u16,
    pub size: u32,
    pub reserved_1: u16,
    pub reserved_2: u16,
    pub offset: u32,
}

impl FileHeader {
    fn new(r#type: u16, size: u32, reserved_1: u16, reserved_2: u16, offset: u32) -> FileHeader {
        FileHeader {
            r#type,
            size,
            reserved_1,
            reserved_2,
            offset,
        }
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct InformationHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub number_of_color_planes: u16,
    pub number_of_bits_per_pixel: u16,
}

impl InformationHeader {
    fn new(
        size: u32,
        width: i32,
        height: i32,
        number_of_color_planes: u16,
        number_of_bits_per_pixel: u16,
    ) -> InformationHeader {
        InformationHeader {
            size,
            width,
            height,
            number_of_color_planes,
            number_of_bits_per_pixel,
        }
    }
}

pub fn load_bitmap(path: &str) -> io::Result<Bitmap> {
    let mut file = File::open(path)?;
    let r#type = read_u16(&file)?;
    let file_size = read_u32(&file)?;
    let reserved_1 = read_u16(&file)?;
    let reserved_2 = read_u16(&file)?;
    let data_offset = read_u32(&file)?;

    let file_header = FileHeader::new(r#type, file_size, reserved_1, reserved_2, data_offset);

    let information_header_size = read_u32(&file)?;
    let width = read_i32(&file)?;
    let height = read_i32(&file)?;
    let number_of_color_planes = read_u16(&file)?;
    let number_of_bits_per_pixel = read_u16(&file)?;

    // We are currently using bitmaps with a BITMAPV5HEADER (124 bytes)
    // But we support all information headers which are based on the BITMAPINFOHEADER (40 bytes)
    assert!(information_header_size >= 40);
    assert!(width > 0);
    assert!(height > 0);
    assert_eq!(number_of_color_planes, 1);

    // We are currently expecting that we are dealing with BGRA (8 bits per channel)
    assert_eq!(number_of_bits_per_pixel, 32);

    let information_header = InformationHeader::new(
        information_header_size,
        width,
        height,
        number_of_color_planes,
        number_of_bits_per_pixel,
    );

    let mut data = vec![0; (width * height * (number_of_bits_per_pixel / 8) as i32) as usize];
    file.seek(SeekFrom::Start(data_offset.into()))?;
    file.read_exact(&mut data)?;

    info!(
        "Loaded bitmap: {} / width: {} / height: {}",
        path, width, height
    );

    Ok(Bitmap::new(file_header, information_header, data))
}

fn read_u16(mut file: &File) -> io::Result<u16> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;

    Ok(u16::from_le_bytes(buffer))
}

fn read_u32(mut file: &File) -> io::Result<u32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;

    Ok(u32::from_le_bytes(buffer))
}

fn read_i32(mut file: &File) -> io::Result<i32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;

    Ok(i32::from_le_bytes(buffer))
}
