use byteorder::{LittleEndian, WriteBytesExt};

pub struct BitMap {
    header: BitMapHeader,
    info: BitMapInfo,
    array: Vec<u8>,
}

impl BitMap {
    pub fn new() -> Self {
        BitMap {
            header: BitMapHeader::new(),
            info: BitMapInfo::new(),
            array: vec![],
        }
    }

    pub fn add_picture(&self, pic: [u8]) -> Self {
        let header = self.header.set_lenght(pic.lenght);
        let info = self.info.set_size(header.size());
        BitMap {
            header: header,
            info: info,
            array: pic,
        }
    }

    pub fn to_array(&self) -> [u8] {
        let bitmap = vec![]; // V5
        bitmap.append(self.header);
        bitmap.append(self.info);
        bitmap.append(self.array);
        bitmap
    }
}

struct BitMapHeader {
    little_indian: u16,
    file_lenght: u32,
    reserved: u32,
    fOffBitsfield: u32,
}

impl BitMapHeader {
    fn new() -> Self {
        BitMapHeader {
            little_indian: 0x4d42,
            file_lenght: 0u32,
            reserved: 0u32,
            fOffBitsfield: 0u32,
        }
    }

    fn set_lenght(&self, lenght: u32) -> Self {
        BitMapHeader { file_lenght: lenght, ..self }
    }

    fn to_array(&self) -> [u8] {
        let header = vec![]; // V5
        header.write_u16::<LittleEndian>(self.little_indian);
        header.write_u32::<LittleEndian>(self.file_lenght);
        header.write_u32::<LittleEndian>(self.reserved);
        header.write_u32::<LittleEndian>(self.fOffBitsfield);
        header
    }
}


pub struct BitMapInfo {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bitcount: u16,
    compression: u32,
    sizeimage: u32,
    xpelsPerMeter: i32,
    ypelsPerMeter: i32,
    clrUsed: u32,
    clrImportant: u32,

    redMask: u32,
    greenMask: u32,
    blueMask: u32,
    alphaMask: u32,
    cSType: u32,

    cIEXYZTRIPLE: [u8; 36],

    gammaRed: u32,
    gammaGreen: u32,
    gammaBlue: u32,

    intent: u32,
    profileData: u32,
    profileSize: u32,
    reserved: u32,
}

impl BitMapInfo {
    fn new() -> Self {
        BitMapInfo {
            size: 0u32,
            width: 0i32,
            height: 0i32,
            planes: 1u16,
            bitcount: 32u16,
            compression: 0u32,
            sizeimage: 0u32,
            xpelsPerMeter: 0i32,
            ypelsPerMeter: 0i32,
            clrUsed: 0u32,
            clrImportant: 0u32,

            redMask: 0u32,
            greenMask: 0u32,
            blueMask: 0u32,
            alphaMask: 0u32,
            cSType: 0u32,

            cIEXYZTRIPLE: [0u8; 36],

            gammaRed: 0u32,
            gammaGreen: 0u32,
            gammaBlue: 0u32,

            intent: 0u32,
            profileData: 0u32,
            profileSize: 0u32,
            reserved: 0u32,
        }
    }

    fn set_size(&self, size: u32) -> Self {
        BitMapInfo { size: size, ..self }
    }

    fn to_array(&self) -> [u8] {
        let bmp_info = vec![]; // V5
        bmp_info.write_u32::<LittleEndian>(self.size); // size
        bmp_info.write_i32::<LittleEndian>(self.width); // width
        bmp_info.write_i32::<LittleEndian>(self.height); // height
        bmp_info.write_u16::<LittleEndian>(self.planes); // planes
        bmp_info.write_u16::<LittleEndian>(self.bitcount); // bitcount 32
        bmp_info.write_u32::<LittleEndian>(self.compression); // compression 0 - BI_RGB
        bmp_info.write_u32::<LittleEndian>(self.sizeimage); // sizeimage
        bmp_info.write_i32::<LittleEndian>(self.xpelsPerMeter); // XpelsPerMeter
        bmp_info.write_i32::<LittleEndian>(self.ypelsPerMeter); // YpelsPerMeter
        bmp_info.write_u32::<LittleEndian>(self.clrUsed); // ClrUsed
        bmp_info.write_u32::<LittleEndian>(self.clrImportant); // ClrImportant

        bmp_info.write_u32::<LittleEndian>(self.redMask); // RedMask
        bmp_info.write_u32::<LittleEndian>(self.greenMask); // GreenMask
        bmp_info.write_u32::<LittleEndian>(self.blueMask); // BlueMask
        bmp_info.write_u32::<LittleEndian>(self.alphaMask); // AlphaMask
        bmp_info.write_u32::<LittleEndian>(self.cSType); // CSType

        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[0]); // CIEXYZTRIPLE
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[1]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[2]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[3]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[4]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[5]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[6]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[7]); //
        bmp_info.write_u32::<LittleEndian>(self.cIEXYZTRIPLE[8]); //

        bmp_info.write_u32::<LittleEndian>(self.gammaRed); // GammaRed
        bmp_info.write_u32::<LittleEndian>(self.gammaGreen); // GammaGreen
        bmp_info.write_u32::<LittleEndian>(self.gammaBlue); // GammaBlue

        bmp_info.write_u32::<LittleEndian>(self.intent); // Intent
        bmp_info.write_u32::<LittleEndian>(self.profileData); // ProfileData
        bmp_info.write_u32::<LittleEndian>(self.profileSize); // ProfileSize
        bmp_info.write_u32::<LittleEndian>(self.reserved); // Reserved
        bmp_info
    }
}
