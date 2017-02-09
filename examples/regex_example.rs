#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::bytes;

fn main() {
    let raw_buffer: &[u8] = &[
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00,
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x20,0x0A,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x80,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00,
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x80,0x0A,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00,
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0xE0,0x0A,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x80,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00,
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x40,0x0B,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00,
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0xA0,0x0B,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x80,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00,
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x00,0x0C,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00
    ];

    lazy_static! {
        static ref RE: bytes::Regex = bytes::Regex::new(
            "..\x00\x00(\x02)\x00\x00\x00"
        ).unwrap();
    }

    for hit in RE.find_iter(&raw_buffer[..]) {
        println!("Hit at: {}",hit.0);
    }
}
