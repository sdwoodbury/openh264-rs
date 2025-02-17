#![feature(test)]

extern crate test;

use openh264::decoder::{Decoder, DecoderConfig};
use openh264::formats::{YUVBuffer, YUVSource};
use test::Bencher;

#[bench]
fn convert_rgb_to_yuv_512x512(b: &mut Bencher) {
    let source = &include_bytes!("../tests/data/lenna_512x512.rgb")[..];

    let mut yuv = YUVBuffer::new(512, 512);

    b.iter(|| {
        yuv.read_rgb(source);
    });
}

#[bench]
fn convert_rgb_to_yuv_1920x1080(b: &mut Bencher) {
    let source = include_bytes!("../tests/data/single_1920x1080_cabac.h264");

    let config = DecoderConfig::default();
    let mut decoder = Decoder::with_config(config).unwrap();
    let yuv = decoder.decode(&source[..]).unwrap().unwrap();
    let mut rgb = vec![0u8; (yuv.width() * yuv.height() * 3) as usize];
    yuv.write_rgb8(&mut rgb);
    let mut converter = YUVBuffer::new(1920, 1080);

    b.iter(|| {
        converter.read_rgb(&rgb);
    });
}
