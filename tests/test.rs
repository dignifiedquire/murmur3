// Copyright (c) 2016 Stu Small
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate byteorder;
extern crate murmur3;

use std::hash::Hasher;
use std::io::Cursor;

use byteorder::{ByteOrder, LittleEndian};

use murmur3::murmur3_32::MurmurHasher as MurmurHasher32;

struct Result {
    string: &'static str,
    hash_32: u32,
    hash_128_x86: [u8; 16],
    hash_128_x64: [u8; 16],
}

#[test]
fn test_static_strings() {
    let tests = [
        Result {
            string: "Lorem ipsum dolor sit amet, consectetur adipisicing \
                     elit",
            hash_32: 0x3bf7e870,
            hash_128_x86: [
                0xAB, 0x53, 0x3F, 0x57, 0xAD, 0x3B, 0xBA, 0x56, 0xFE, 0xA4, 0x9F, 0x73, 0x48, 0x88,
                0x91, 0x10,
            ],
            hash_128_x64: [
                0x6F, 0x5C, 0xB0, 0x2C, 0xFD, 0x5E, 0xDC, 0x6F, 0xE6, 0x9D, 0xF0, 0xFF, 0x60, 0x41,
                0x70, 0x46,
            ],
        },
        Result {
            string: "Hello, world!",
            hash_32: 0xc0363e43,
            hash_128_x86: [
                0xA7, 0xDB, 0xAC, 0x26, 0xFC, 0x8D, 0x63, 0xF0, 0x63, 0x42, 0x2B, 0x40, 0xC3, 0xD4,
                0xFD, 0x0A,
            ],
            hash_128_x64: [
                0xDF, 0x65, 0xD6, 0xD2, 0xD1, 0x2D, 0x51, 0xF1, 0x64, 0xC5, 0xF3, 0xA8, 0x50, 0x66,
                0x32, 0x2C,
            ],
        },
        Result {
            string: "",
            hash_32: 0000000000,
            hash_128_x86: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
            hash_128_x64: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        },
        Result {
            string: "1",
            hash_32: 0x9416ac93,
            hash_128_x86: [
                0xFB, 0xF4, 0xF2, 0xE0, 0xD5, 0xB1, 0x6B, 0xD9, 0xD5, 0xB1, 0x6B, 0xD9, 0xD5, 0xB1,
                0x6B, 0xD9,
            ],
            hash_128_x64: [
                0x71, 0x7C, 0x7B, 0x8A, 0xFE, 0xBB, 0xFB, 0x71, 0x37, 0xF6, 0xF0, 0xF9, 0x9B, 0xEB,
                0x2A, 0x94,
            ],
        },
        Result {
            string: "12",
            hash_32: 0xf9d2ef15,
            hash_128_x86: [
                0x9F, 0x1F, 0x9B, 0xC0, 0x1E, 0x49, 0x4F, 0xAF, 0x1E, 0x49, 0x4F, 0xAF, 0x1E, 0x49,
                0x4F, 0xAF,
            ],
            hash_128_x64: [
                0x95, 0xFD, 0xE3, 0x09, 0x62, 0x3C, 0x53, 0x4A, 0x1D, 0x31, 0x0B, 0x5E, 0x69, 0x2C,
                0xC7, 0x88,
            ],
        },
        Result {
            string: "123",
            hash_32: 0x9eb471eb,
            hash_128_x86: [
                0x2F, 0x2B, 0x4D, 0x51, 0x08, 0x6F, 0x40, 0x7B, 0x08, 0x6F, 0x40, 0x7B, 0x08, 0x6F,
                0x40, 0x7B,
            ],
            hash_128_x64: [
                0x6A, 0x7F, 0x66, 0x0D, 0x1B, 0x2D, 0x5B, 0x98, 0x69, 0xCF, 0x0E, 0xCE, 0xE3, 0xA1,
                0x7E, 0x42,
            ],
        },
        Result {
            string: "1234",
            hash_32: 0x721c5dc3,
            hash_128_x86: [
                0x4D, 0x64, 0xDF, 0x4A, 0xCE, 0x32, 0xAA, 0x2E, 0xCE, 0x32, 0xAA, 0x2E, 0xCE, 0x32,
                0xAA, 0x2E,
            ],
            hash_128_x64: [
                0xB4, 0xE7, 0x8F, 0x21, 0x4D, 0x36, 0x97, 0x08, 0xA5, 0xFD, 0x37, 0x24, 0xD9, 0x8B,
                0x1E, 0x34,
            ],
        },
        Result {
            string: "12345",
            hash_32: 0x13a51193,
            hash_128_x86: [
                0x7C, 0xF8, 0x48, 0xD0, 0x30, 0xD9, 0x59, 0x0A, 0x94, 0xED, 0x4F, 0xD4, 0x94, 0xED,
                0x4F, 0xD4,
            ],
            hash_128_x64: [
                0xCB, 0xDF, 0x21, 0x6B, 0x17, 0x3A, 0xF8, 0x20, 0xF4, 0xA9, 0x5C, 0x32, 0x41, 0x5C,
                0x3C, 0xF1,
            ],
        },
        Result {
            string: "123456",
            hash_32: 0xbf60eab8,
            hash_128_x86: [
                0xB9, 0xF9, 0x06, 0x7D, 0xBF, 0x7E, 0xEB, 0x69, 0xED, 0xEF, 0x32, 0x88, 0xED, 0xEF,
                0x32, 0x88,
            ],
            hash_128_x64: [
                0xD6, 0xD0, 0xBB, 0x0B, 0x05, 0xCF, 0x17, 0xE4, 0xFE, 0x31, 0x25, 0x00, 0x91, 0x80,
                0xA4, 0x51,
            ],
        },
        Result {
            string: "1234567",
            hash_32: 0xb7ef82f7,
            hash_128_x86: [
                0x71, 0xFE, 0x6B, 0x74, 0x54, 0x40, 0xC1, 0xEA, 0x92, 0xB3, 0x83, 0x8C, 0x92, 0xB3,
                0x83, 0x8C,
            ],
            hash_128_x64: [
                0xA2, 0x23, 0xC6, 0xF2, 0xF7, 0xC5, 0xDA, 0x2C, 0x55, 0xD9, 0xE1, 0xCA, 0x8B, 0x51,
                0xDC, 0x37,
            ],
        },
        Result {
            string: "12345678",
            hash_32: 0x91b313ce,
            hash_128_x86: [
                0x93, 0xD3, 0xAD, 0x65, 0x87, 0x73, 0x0B, 0x56, 0x3A, 0xEE, 0xC8, 0x6A, 0x3A, 0xEE,
                0xC8, 0x6A,
            ],
            hash_128_x64: [
                0x9C, 0x41, 0xB1, 0x38, 0x06, 0x64, 0x4A, 0x3B, 0x57, 0x25, 0xD4, 0x6B, 0x67, 0x0E,
                0x3B, 0x91,
            ],
        },
        Result {
            string: "123456789",
            hash_32: 0xb4fef382,
            hash_128_x86: [
                0xBB, 0x76, 0x58, 0xC6, 0x52, 0x15, 0x9A, 0x11, 0xD7, 0xE5, 0xE3, 0xC5, 0xA4, 0x8C,
                0x16, 0xA9,
            ],
            hash_128_x64: [
                0xA4, 0xCC, 0x66, 0xDB, 0x5E, 0x64, 0x84, 0x3C, 0x05, 0xA1, 0x1E, 0x3A, 0xC7, 0xFA,
                0xF8, 0x99,
            ],
        },
        Result {
            string: "1234567890",
            hash_32: 0x3204634d,
            hash_128_x86: [
                0x34, 0x0E, 0xAD, 0x47, 0x37, 0xA5, 0x10, 0x92, 0xAB, 0x4F, 0x1A, 0xE5, 0x40, 0xDA,
                0xE7, 0xAB,
            ],
            hash_128_x64: [
                0x0A, 0x87, 0x79, 0x80, 0xE6, 0x4A, 0xFA, 0xEC, 0x2B, 0xD2, 0xEB, 0x20, 0xC8, 0x17,
                0xD0, 0xC1,
            ],
        },
        Result {
            string: "12345678901",
            hash_32: 0x3ca173d0,
            hash_128_x86: [
                0xE7, 0xA8, 0xB1, 0x1F, 0x7D, 0x7F, 0xCD, 0xFD, 0x1F, 0x44, 0xB5, 0x93, 0x8F, 0x0A,
                0x14, 0x04,
            ],
            hash_128_x64: [
                0xD3, 0x27, 0xB3, 0x85, 0x13, 0xFB, 0x84, 0x2A, 0xC1, 0xDF, 0xE0, 0x7D, 0x85, 0x95,
                0xEB, 0xDA,
            ],
        },
        Result {
            string: "123456789012",
            hash_32: 0x6c75e419,
            hash_128_x86: [
                0x99, 0x7E, 0x6F, 0x80, 0x1C, 0x4F, 0x20, 0x74, 0xCB, 0x0E, 0x11, 0xFB, 0xF1, 0xE4,
                0x3F, 0x41,
            ],
            hash_128_x64: [
                0x14, 0x29, 0x02, 0x7C, 0x8B, 0xE3, 0xA6, 0xDD, 0x1E, 0x9D, 0x71, 0xFD, 0x83, 0x39,
                0xA2, 0x75,
            ],
        },
        Result {
            string: "1234567890123",
            hash_32: 0xcaf7e549,
            hash_128_x86: [
                0x83, 0xA6, 0x21, 0xBE, 0xBB, 0x3E, 0xF4, 0x38, 0x22, 0xC8, 0x85, 0x19, 0x3D, 0x7D,
                0xED, 0xBB,
            ],
            hash_128_x64: [
                0x49, 0xDF, 0x72, 0x37, 0x85, 0xF2, 0xDD, 0xE3, 0x97, 0x24, 0xEF, 0x5E, 0xF0, 0x21,
                0xC5, 0x1B,
            ],
        },
        Result {
            string: "12345678901234",
            hash_32: 0x57ae5bd1,
            hash_128_x86: [
                0xD8, 0xC7, 0x1E, 0x23, 0x6A, 0x9B, 0x8D, 0xAC, 0x8D, 0xC4, 0xA2, 0xA0, 0x96, 0xAF,
                0x97, 0x3F,
            ],
            hash_128_x64: [
                0x91, 0xCC, 0x3C, 0xE8, 0x70, 0xE1, 0x51, 0x7D, 0xD0, 0x5A, 0xF8, 0xFA, 0xBE, 0x6C,
                0x3D, 0xC6,
            ],
        },
        Result {
            string: "123456789012345",
            hash_32: 0x09bb660c,
            hash_128_x86: [
                0x78, 0x0A, 0x91, 0x78, 0x23, 0xC9, 0x84, 0xD4, 0x03, 0xF9, 0x0A, 0xC3, 0xDB, 0x30,
                0x55, 0x12,
            ],
            hash_128_x64: [
                0xD6, 0xCF, 0xAF, 0xA2, 0xAE, 0x01, 0x70, 0x88, 0xB3, 0x01, 0x08, 0x4F, 0x36, 0x26,
                0xC3, 0x1E,
            ],
        },
        Result {
            string: "1234567890123456",
            hash_32: 0x06b2ff24,
            hash_128_x86: [
                0xB8, 0xDF, 0x3D, 0x20, 0x83, 0x3D, 0xF4, 0x19, 0x4C, 0xB4, 0x40, 0x3F, 0xBD, 0xA0,
                0xD7, 0xD2,
            ],
            hash_128_x64: [
                0xF8, 0x2C, 0xE3, 0xC0, 0xC5, 0x5D, 0xBE, 0x4F, 0xC1, 0x22, 0xC3, 0x60, 0x6B, 0xE9,
                0xC8, 0xC0,
            ],
        },
        Result {
            string: "12345678901234567",
            hash_32: 0xc50a5d2b,
            hash_128_x86: [
                0x6F, 0x8B, 0x41, 0x43, 0x9E, 0x40, 0xE2, 0x6C, 0x42, 0x31, 0x9E, 0x9D, 0x7C, 0x80,
                0x9A, 0x40,
            ],
            hash_128_x64: [
                0x7E, 0xB7, 0x26, 0x80, 0x96, 0x17, 0x86, 0x74, 0x03, 0x71, 0x3F, 0x47, 0x86, 0x63,
                0x1E, 0x29,
            ],
        },
        Result {
            string: "123456789012345678",
            hash_32: 0xe970a44f,
            hash_128_x86: [
                0xC6, 0x05, 0xE9, 0x5C, 0x08, 0xD2, 0x0D, 0x92, 0x89, 0x2C, 0x38, 0x68, 0x3F, 0x8A,
                0x96, 0x4F,
            ],
            hash_128_x64: [
                0xAF, 0x61, 0xA9, 0xCF, 0x1C, 0xE5, 0xEA, 0xEA, 0x69, 0x04, 0xCC, 0x52, 0x7D, 0x65,
                0x4C, 0x75,
            ],
        },
        Result {
            string: "1234567890123456789",
            hash_32: 0xf7c5400e,
            hash_128_x86: [
                0x7C, 0x88, 0x97, 0x5A, 0xA2, 0xCE, 0x7F, 0x0C, 0xD8, 0xE6, 0xA2, 0x29, 0xE0, 0x61,
                0x05, 0x26,
            ],
            hash_128_x64: [
                0x59, 0x99, 0x47, 0x0A, 0xBA, 0x2F, 0x72, 0x0C, 0x2A, 0x8A, 0x21, 0x12, 0x69, 0xCD,
                0xBB, 0x4E,
            ],
        },
        Result {
            string: "12345678901234567890",
            hash_32: 0x45e28067,
            hash_128_x86: [
                0xE1, 0x93, 0x21, 0x22, 0x7E, 0xAA, 0x15, 0x07, 0xA6, 0x22, 0x87, 0xCD, 0x54, 0x55,
                0x9E, 0x1E,
            ],
            hash_128_x64: [
                0x3A, 0x8C, 0xDC, 0x25, 0x19, 0xD8, 0x1C, 0xB1, 0x7D, 0x36, 0xF1, 0xE8, 0x3C, 0x60,
                0x9F, 0x71,
            ],
        },
        Result {
            string: "123456789012345678901",
            hash_32: 0x7b4f3da6,
            hash_128_x86: [
                0xE3, 0x50, 0x2D, 0xFB, 0x0F, 0x03, 0x6F, 0x39, 0xA3, 0x83, 0x11, 0xEA, 0x5E, 0xB6,
                0x85, 0x9A,
            ],
            hash_128_x64: [
                0x55, 0x68, 0xEE, 0x16, 0x3C, 0xF2, 0xD7, 0xA2, 0xD3, 0x3D, 0xF5, 0xA5, 0x02, 0x37,
                0xE6, 0xFE,
            ],
        },
        Result {
            string: "1234567890123456789010",
            hash_32: 0x1e4a77ff,
            hash_128_x86: [
                0xFE, 0x78, 0x3B, 0x50, 0x3D, 0x5A, 0x45, 0x33, 0xA7, 0xCB, 0x0F, 0x53, 0xB9, 0xF0,
                0xAB, 0x36,
            ],
            hash_128_x64: [
                0x76, 0x70, 0x7E, 0xAE, 0xC7, 0x8B, 0x20, 0x37, 0xAF, 0xB8, 0xAB, 0x7A, 0x58, 0x79,
                0xA9, 0xEF,
            ],
        },
        Result {
            string: "€",
            hash_32: 0x5b43fca5,
            hash_128_x86: [
                0x4C, 0xF5, 0x4D, 0xC, 0x9F, 0x57, 0x9A, 0xBA, 0x9F, 0x57, 0x9A, 0xBA, 0x9F, 0x57,
                0x9A, 0xBA,
            ],
            hash_128_x64: [
                0x55, 0x95, 0xDD, 0x2F, 0x3A, 0x30, 0xE3, 0x59, 0x64, 0x31, 0xBC, 0xE4, 0xB3, 0x8B,
                0x9D, 0x4F,
            ],
        },
        Result {
            string: "€€€€€€€€€€",
            hash_32: 0xda3c1253,
            hash_128_x86: [
                0xE5, 0xBB, 0xF1, 0xB2, 0x53, 0xDC, 0xB5, 0x92, 0x78, 0x4E, 0x71, 0xDE, 0x44, 0x38,
                0x31, 0xC5,
            ],
            hash_128_x64: [
                0x6F, 0xEF, 0x5E, 0x37, 0x77, 0xEB, 0xCF, 0xCE, 0xC6, 0xE2, 0x69, 0x68, 0xC2, 0xB,
                0x83, 0xE9,
            ],
        },
    ];

    for test in &tests {
        let mut out: [u8; 16] = [0; 16];
        let mut hasher = MurmurHasher32::new(0);
        hasher.write(test.string.as_bytes());
        assert!(
            hasher.build_murmur_hash() == test.hash_32,
            "Failed on string {}",
            test.string
        );
        murmur3::murmur3_x86_128(&mut Cursor::new(test.string.as_bytes()), 0, &mut out);
        assert!(out == test.hash_128_x86, "Failed on string {}", test.string);
        let hash =
            murmur3::murmur3_x64_128::murmur3_x64_128(&mut Cursor::new(test.string.as_bytes()), 0)
                .unwrap();
        let expected = LittleEndian::read_u128(&test.hash_128_x64);
        assert!(hash == expected, "Failed on string {}", test.string);
    }
}
