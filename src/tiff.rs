// A simple implementation of TIFF Baseline encoder, no compression, single strip

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

struct FieldType;

impl FieldType {
    const SHORT: u16 = 3;
    const LONG: u16 = 4;
    const RATIONAL: u16 = 5;
}

struct Tag;

impl Tag {
    const ImageWidth: u16 = 0x100;
    const ImageLength: u16 = 0x101;
    const BitsPerSample: u16 = 0x102;
    const Compression: u16 = 0x103;
    const PhotometricInterpretation: u16 = 0x106;
    const StripOffsets: u16 = 0x111;
    const SamplesPerPixel: u16 = 0x115;
    const RowsPerStrip: u16 = 0x116;
    const StripByteCounts: u16 = 0x117;
    const XResolution: u16 = 0x11A;
    const YResolution: u16 = 0x11B;
}

pub struct TiffFile {
    f: BufWriter<File>,
    img_width: u32,
    img_height: u32,
}

impl TiffFile {
    pub fn new(path: &str, img_width: u32, img_height: u32) -> TiffFile {
        let path = Path::new(path);
        let f = BufWriter::new(File::create(&path).unwrap());
        let mut tiff_file = TiffFile {
            f,
            img_width,
            img_height,
        };
        tiff_file.write_header();
        tiff_file
    }

    pub fn write(&mut self, buf: &[u8]) {
        self.f.write(buf).unwrap();
    }

    fn write_field(&mut self, tag: u16, field_type: u16, count: u32, value: u32) {
        self.write(&tag.to_le_bytes());
        self.write(&field_type.to_le_bytes());
        self.write(&count.to_le_bytes());
        self.write(&value.to_le_bytes());
    }

    fn write_header(&mut self) {
        self.write(b"II");
        self.write(&[42, 0, 8, 0, 0, 0]);

        let n_fields = 11;
        let offset = 8 + 2 + n_fields as u32 * 12 + 4;
        let resolution = 300u32;

        self.write(&[n_fields, 0]);
        self.write_field(Tag::ImageWidth, FieldType::SHORT, 1, self.img_width);
        self.write_field(Tag::ImageLength, FieldType::SHORT, 1, self.img_height);
        self.write_field(Tag::BitsPerSample, FieldType::SHORT, 3, offset);
        self.write_field(Tag::Compression, FieldType::SHORT, 1, 1);
        self.write_field(Tag::PhotometricInterpretation, FieldType::SHORT, 1, 2);
        self.write_field(Tag::StripOffsets, FieldType::SHORT, 1, offset + 6 + 8 + 8);
        self.write_field(Tag::SamplesPerPixel, FieldType::SHORT, 1, 3);
        self.write_field(Tag::RowsPerStrip, FieldType::SHORT, 1, self.img_height);
        self.write_field(
            Tag::StripByteCounts,
            FieldType::LONG,
            1,
            self.img_width * self.img_height * 3,
        );
        self.write_field(Tag::XResolution, FieldType::RATIONAL, 1, offset + 6);
        self.write_field(Tag::YResolution, FieldType::RATIONAL, 1, offset + 6 + 8);
        self.write(&[0, 0, 0, 0]);

        self.write(&[8, 0, 8, 0, 8, 0]);
        self.write(&resolution.to_le_bytes());
        self.write(&[1, 0, 0, 0]);
        self.write(&resolution.to_le_bytes());
        self.write(&[1, 0, 0, 0]);
    }
}
