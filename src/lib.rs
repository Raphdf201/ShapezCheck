static PIN: char = 'P';
static SHAPE: char = 'S';
static CRYSTAL: char = 'C';
static EMPTY: char = '-';

enum QuarterType {
    Empty,
    Pin,
    Shape,
    Crystal,
}

impl QuarterType {
    fn from_u8(p0: u8) -> QuarterType {
        todo!()
    }
}

struct Shape {
    layer: usize,
    quarter: usize
}

impl Shape {

}

fn repeat(val: i32, width: usize, count: usize) -> i32 {
    let mut result = 0;
    for _ in 0..count {
        result <<= width;
        result |= val;
    }
    result
}

fn to_char(q: QuarterType) -> char {
    if let QuarterType::Pin = q {
        PIN
    } else if let QuarterType::Crystal = q {
        CRYSTAL
    } else if let QuarterType::Empty = q {
        EMPTY
    } else {
        SHAPE
    }
}

fn to_type(c: char) -> QuarterType {
    if c == PIN {
        QuarterType::Pin
    } else if c == CRYSTAL {
        QuarterType::Crystal
    } else if c == EMPTY {
        QuarterType::Empty
    } else {
        QuarterType::Shape
    }
}
