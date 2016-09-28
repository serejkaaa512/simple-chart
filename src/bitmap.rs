use byteorder::{LittleEndian, WriteBytesExt};

const HEADER_LENGHT: u32 = 14;
const INFO_LENGHT: u32 = 124;
const COLOR_SIZE: u32 = 4;
const RESERVED: u8 = 0;

#[derive(Debug)]
pub struct BitMap {
    header: BitMapHeader,
    info: BitMapInfo,
    color_table: ColorTable,
    array: Vec<u8>,
}

impl Default for BitMap {
    fn default() -> Self {
        Self::new(740, 480)
    }
}

impl BitMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut b = BitMap {
            header: BitMapHeader::new(),
            info: BitMapInfo::new(),
            color_table: ColorTable::new(),
            array: vec![],
        };

        b.info.set_width(width as i32);
        b.info.set_height(height as i32);

        b
    }

    pub fn add_pixels(&mut self, pic: &[u8]) {
        self.array.extend_from_slice(pic);
    }

    pub fn add_color<C>(&mut self, color: C) -> u8
        where C: Into<Color>
    {
        let color = color.into();
        self.color_table.add_color(color);

        self.info.clr_used += 1;
        (self.info.clr_used - 1) as u8
    }

    pub fn as_vec(&mut self) -> Vec<u8> {
        let mut bitmap = vec![]; // V5

        let pixels_data_offset = HEADER_LENGHT + INFO_LENGHT + self.color_table.get_size();
        self.header.set_data_offset(pixels_data_offset);

        let file_lenght = pixels_data_offset + self.array.len() as u32;
        self.header.set_lenght(file_lenght);

        bitmap.extend_from_slice(&self.header.to_vec());
        bitmap.extend_from_slice(&self.info.to_vec());
        bitmap.extend_from_slice(&self.color_table.to_vec());
        bitmap.extend_from_slice(&self.array);
        bitmap
    }
}

#[derive(Debug)]
struct BitMapHeader {
    little_indian: u16,
    file_length: u32,
    reserved: u32,
    f_off_bitsfield: u32,
}

impl BitMapHeader {
    fn new() -> Self {
        BitMapHeader {
            little_indian: 0x4d42,
            file_length: 0u32,
            reserved: 0u32,
            f_off_bitsfield: 0u32,
        }
    }

    fn set_lenght(&mut self, length: u32) {
        self.file_length = length;
    }


    fn set_data_offset(&mut self, offset: u32) {
        self.f_off_bitsfield = offset;
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut header = vec![]; // V5
        header.write_u16::<LittleEndian>(self.little_indian).unwrap();
        header.write_u32::<LittleEndian>(self.file_length).unwrap();
        header.write_u32::<LittleEndian>(self.reserved).unwrap();
        header.write_u32::<LittleEndian>(self.f_off_bitsfield).unwrap();
        header
    }
}

#[derive(Debug)]
struct BitMapInfo {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bitcount: u16,
    compression: u32,
    sizeimage: u32,
    xpels_per_meter: i32,
    ypels_per_meter: i32,
    clr_used: u32,
    clr_important: u32,

    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    alpha_mask: u32,
    c_stype: u32,

    c_iexyztriple: Vec<u8>,

    gamma_red: u32,
    gamma_green: u32,
    gamma_blue: u32,

    intent: u32,
    profile_data: u32,
    profile_size: u32,
    reserved: u32,
}

impl BitMapInfo {
    fn new() -> Self {
        BitMapInfo {
            size: 124u32,
            width: 0i32,
            height: 0i32,
            planes: 1u16,
            bitcount: 8u16,
            compression: 0u32,
            sizeimage: 0u32,
            xpels_per_meter: 3780i32, // 96 dpi
            ypels_per_meter: 3780i32, // 96 dpi
            clr_used: 0u32,
            clr_important: 0u32,

            red_mask: 0u32,
            green_mask: 0u32,
            blue_mask: 0u32,
            alpha_mask: 0u32,
            c_stype: 0u32,

            c_iexyztriple: vec![0u8; 36],
            gamma_red: 0u32,
            gamma_green: 0u32,
            gamma_blue: 0u32,

            intent: 4u32, // Picture
            profile_data: 0u32,
            profile_size: 0u32,
            reserved: 0u32,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut bmp_info = vec![]; // V5
        bmp_info.write_u32::<LittleEndian>(self.size).unwrap(); // size
        bmp_info.write_i32::<LittleEndian>(self.width).unwrap(); // width
        bmp_info.write_i32::<LittleEndian>(self.height).unwrap(); // height
        bmp_info.write_u16::<LittleEndian>(self.planes).unwrap(); // planes
        bmp_info.write_u16::<LittleEndian>(self.bitcount).unwrap(); // bitcount 32
        bmp_info.write_u32::<LittleEndian>(self.compression).unwrap(); // compression 0 - BI_RGB
        bmp_info.write_u32::<LittleEndian>(self.sizeimage).unwrap(); // sizeimage
        bmp_info.write_i32::<LittleEndian>(self.xpels_per_meter).unwrap(); // XpelsPerMeter
        bmp_info.write_i32::<LittleEndian>(self.ypels_per_meter).unwrap(); // YpelsPerMeter
        bmp_info.write_u32::<LittleEndian>(self.clr_used).unwrap(); // ClrUsed
        bmp_info.write_u32::<LittleEndian>(self.clr_important).unwrap(); // ClrImportant

        bmp_info.write_u32::<LittleEndian>(self.red_mask).unwrap(); // RedMask
        bmp_info.write_u32::<LittleEndian>(self.green_mask).unwrap(); // GreenMask
        bmp_info.write_u32::<LittleEndian>(self.blue_mask).unwrap(); // BlueMask
        bmp_info.write_u32::<LittleEndian>(self.alpha_mask).unwrap(); // AlphaMask
        bmp_info.write_u32::<LittleEndian>(self.c_stype).unwrap(); // CSType

        bmp_info.extend_from_slice(&self.c_iexyztriple);                  // CIEXYZTRIPLE

        bmp_info.write_u32::<LittleEndian>(self.gamma_red).unwrap(); // GammaRed
        bmp_info.write_u32::<LittleEndian>(self.gamma_green).unwrap(); // GammaGreen
        bmp_info.write_u32::<LittleEndian>(self.gamma_blue).unwrap(); // GammaBlue

        bmp_info.write_u32::<LittleEndian>(self.intent).unwrap(); // Intent
        bmp_info.write_u32::<LittleEndian>(self.profile_data).unwrap(); // ProfileData
        bmp_info.write_u32::<LittleEndian>(self.profile_size).unwrap(); // ProfileSize
        bmp_info.write_u32::<LittleEndian>(self.reserved).unwrap(); // Reserved

        bmp_info
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

#[derive(Debug)]
struct ColorTable {
    count: u32,
    table: Vec<u8>,
}

impl ColorTable {
    fn new() -> ColorTable {
        ColorTable {
            count: 0,
            table: vec![],
        }
    }

    fn add_color(&mut self, color: Color) {
        self.count += 1;
        self.table.append(&mut color.get_buffer());
    }

    fn get_size(&self) -> u32 {
        256 * COLOR_SIZE
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        v.extend_from_slice(&self.table);
        for _ in self.count..256 {
            v.write_u32::<LittleEndian>(0).unwrap();
        }
        v
    }
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn get_buffer(&self) -> Vec<u8> {
        vec![self.b, self.g, self.r, RESERVED]
    }
}

// #ffaabb
impl<'a> From<&'a str> for Color {
    fn from(string: &str) -> Color {
        let s = &string.to_lowercase();
        let r = u8::from_str_radix(&s[1..3], 16).unwrap();
        let g = u8::from_str_radix(&s[3..5], 16).unwrap();
        let b = u8::from_str_radix(&s[5..7], 16).unwrap();
        Color { r: r, g: g, b: b }
    }
}
