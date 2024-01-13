use super::*;

/* pub(super) const LINEDEFS: [Linedef; 25] = [
    Linedef {
        a: 0,
        b: 1,
        flags: LinedefFlags::from_bits_retain(1),
        special: 11,
        tag: 0,
        front: 0,
        back: None,
    },
    Linedef {
        a: 1,
        b: 2,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 1,
        back: None,
    },
    Linedef {
        a: 3,
        b: 4,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 2,
        back: None,
    },
    Linedef {
        a: 4,
        b: 5,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 3,
        back: None,
    },
    Linedef {
        a: 2,
        b: 6,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 4,
        back: None,
    },
    Linedef {
        a: 7,
        b: 0,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 5,
        back: None,
    },
    Linedef {
        a: 8,
        b: 7,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 6,
        back: None,
    },
    Linedef {
        a: 6,
        b: 9,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 7,
        back: None,
    },
    Linedef {
        a: 10,
        b: 11,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 8,
        back: None,
    },
    Linedef {
        a: 11,
        b: 12,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 9,
        back: None,
    },
    Linedef {
        a: 12,
        b: 13,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 10,
        back: None,
    },
    Linedef {
        a: 13,
        b: 10,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 11,
        back: None,
    },
    Linedef {
        a: 5,
        b: 8,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 16,
        back: None,
    },
    Linedef {
        a: 12,
        b: 14,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 17,
        back: Some(18),
    },
    Linedef {
        a: 15,
        b: 16,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 19,
        back: None,
    },
    Linedef {
        a: 16,
        b: 17,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 20,
        back: None,
    },
    Linedef {
        a: 17,
        b: 18,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 21,
        back: None,
    },
    Linedef {
        a: 18,
        b: 15,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 15,
        back: None,
    },
    Linedef {
        a: 9,
        b: 3,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 12,
        back: None,
    },
    Linedef {
        a: 17,
        b: 19,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 13,
        back: Some(14),
    },
    Linedef {
        a: 14,
        b: 20,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 22,
        back: Some(23),
    },
    Linedef {
        a: 19,
        b: 21,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 24,
        back: Some(25),
    },
    Linedef {
        a: 16,
        b: 13,
        flags: LinedefFlags::from_bits_retain(12),
        special: 0,
        tag: 0,
        front: 26,
        back: Some(27),
    },
    Linedef {
        a: 21,
        b: 9,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 28,
        back: Some(29),
    },
    Linedef {
        a: 20,
        b: 5,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 30,
        back: Some(31),
    },
]; */

pub(super) const LINEDEFS: [Linedef; 25] = [
    Linedef {
        v1: 0,
        v2: 1,
        flags: LinedefFlags::from_bits_retain(1),
        special: 11,
        tag: 0,
        front: 0,
        back: u16::MAX,
    },
    Linedef {
        v1: 1,
        v2: 2,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 1,
        back: u16::MAX,
    },
    Linedef {
        v1: 3,
        v2: 4,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 2,
        back: u16::MAX,
    },
    Linedef {
        v1: 4,
        v2: 5,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 3,
        back: u16::MAX,
    },
    Linedef {
        v1: 2,
        v2: 6,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 4,
        back: u16::MAX,
    },
    Linedef {
        v1: 7,
        v2: 0,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 5,
        back: u16::MAX,
    },
    Linedef {
        v1: 8,
        v2: 7,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 6,
        back: u16::MAX,
    },
    Linedef {
        v1: 6,
        v2: 9,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 7,
        back: u16::MAX,
    },
    Linedef {
        v1: 10,
        v2: 11,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 8,
        back: u16::MAX,
    },
    Linedef {
        v1: 11,
        v2: 12,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 9,
        back: u16::MAX,
    },
    Linedef {
        v1: 12,
        v2: 13,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 10,
        back: u16::MAX,
    },
    Linedef {
        v1: 13,
        v2: 10,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 11,
        back: u16::MAX,
    },
    Linedef {
        v1: 5,
        v2: 8,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 16,
        back: u16::MAX,
    },
    Linedef {
        v1: 12,
        v2: 14,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 17,
        back: 18,
    },
    Linedef {
        v1: 15,
        v2: 16,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 19,
        back: u16::MAX,
    },
    Linedef {
        v1: 16,
        v2: 17,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 20,
        back: u16::MAX,
    },
    Linedef {
        v1: 17,
        v2: 18,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 21,
        back: u16::MAX,
    },
    Linedef {
        v1: 18,
        v2: 15,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 15,
        back: u16::MAX,
    },
    Linedef {
        v1: 9,
        v2: 3,
        flags: LinedefFlags::from_bits_retain(1),
        special: 0,
        tag: 0,
        front: 12,
        back: u16::MAX,
    },
    Linedef {
        v1: 17,
        v2: 19,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 13,
        back: 14,
    },
    Linedef {
        v1: 14,
        v2: 20,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 22,
        back: 23,
    },
    Linedef {
        v1: 19,
        v2: 21,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 24,
        back: 25,
    },
    Linedef {
        v1: 16,
        v2: 13,
        flags: LinedefFlags::from_bits_retain(12),
        special: 0,
        tag: 0,
        front: 26,
        back: 27,
    },
    Linedef {
        v1: 21,
        v2: 9,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 28,
        back: 29,
    },
    Linedef {
        v1: 20,
        v2: 5,
        flags: LinedefFlags::from_bits_retain(29),
        special: 0,
        tag: 0,
        front: 30,
        back: 31,
    },
];

pub(super) const SIDEDEFS: [Sidedef; 32] = [
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"SW1BRCOM",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 1,
    },
    Sidedef {
        x: 178,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 332,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 192,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 380,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 444,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP3\0\0\0",
        middle: *b"BRNSMALL",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BRNSMALR",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 192,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BRNSMALL",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP3\0\0\0",
        middle: *b"BRNSMALR",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 32,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BROWN96\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BRNSMALC",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP3\0\0\0",
        middle: *b"BRNSMALC",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP3\0\0\0",
        middle: *b"BRNSMALC",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BRNSMALC",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"-\0\0\0\0\0\0\0",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP1\0\0\0",
        middle: *b"-\0\0\0\0\0\0\0",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP3\0\0\0",
        middle: *b"BRNSMALR",
        sec: 1,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BRNSMALL",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"-\0\0\0\0\0\0\0",
        lower: *b"-\0\0\0\0\0\0\0",
        middle: *b"BRNSMALR",
        sec: 0,
    },
    Sidedef {
        x: 0,
        y: 0,
        upper: *b"BROWN96\0",
        lower: *b"STEP3\0\0\0",
        middle: *b"BRNSMALL",
        sec: 1,
    },
];

pub const VERTICES: [Vertex; 22] = [
    Vertex {
        x: -128,
        y: 192
    },
    Vertex {
        x: 0,
        y: 192
    },
    Vertex {
        x: 0,
        y: 128
    },
    Vertex {
        x: 208,
        y: -128
    },
    Vertex {
        x: -336,
        y: -128
    },
    Vertex {
        x: -336,
        y: 64
    },
    Vertex {
        x: 208,
        y: 128
    },
    Vertex {
        x: -128,
        y: 128
    },
    Vertex {
        x: -336,
        y: 128
    },
    Vertex {
        x: 208,
        y: 64
    },
    Vertex {
        x: -128,
        y: 80
    },
    Vertex {
        x: -144,
        y: 80
    },
    Vertex {
        x: -144,
        y: 64
    },
    Vertex {
        x: -128,
        y: 64
    },
    Vertex {
        x: -176,
        y: 64
    },
    Vertex {
        x: 0,
        y: 80
    },
    Vertex {
        x: 0,
        y: 64
    },
    Vertex {
        x: 16,
        y: 64
    },
    Vertex {
        x: 16,
        y: 80
    },
    Vertex {
        x: 48,
        y: 64
    },
    Vertex {
        x: -304,
        y: 64
    },
    Vertex {
        x: 176,
        y: 64
    },
];

pub const SECTORS: [Sector; 2] = [
    Sector {
        florh: 8,
        ceilh: 96,
        flort: *b"CEIL5_2\0",
        ceilt: *b"CEIL5_2\0",
        light: 160,
        special: 0,
        tag: 0,
    },
    Sector {
        florh: 0,
        ceilh: 128,
        flort: *b"CEIL5_2\0",
        ceilt: *b"CEIL5_2\0",
        light: 160,
        special: 0,
        tag: 0,
    },
];

pub const THINGS: [Thing; 7] = [
    Thing {
        x: -64,
        y: -64,
        angle: 90,
        ednum: 1,
        flags: 7,
    },
    Thing {
        x: -320,
        y: 96,
        angle: 0,
        ednum: 86,
        flags: 7,
    },
    Thing {
        x: 192,
        y: 96,
        angle: 180,
        ednum: 86,
        flags: 7,
    },
    Thing {
        x: -112,
        y: 160,
        angle: 0,
        ednum: 86,
        flags: 7,
    },
    Thing {
        x: -16,
        y: 160,
        angle: 0,
        ednum: 86,
        flags: 7,
    },
    Thing {
        x: -64,
        y: 144,
        angle: 270,
        ednum: 2001,
        flags: 7,
    },
    Thing {
        x: -64,
        y: 96,
        angle: 270,
        ednum: 8,
        flags: 7,
    },
];