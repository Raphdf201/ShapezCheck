static PIN: char = 'P';
static SHAPE: char = 'S';
static CRYSTAL: char = 'C';
static EMPTY: char = '-';
static LAYER: char = ':';

enum QuarterType {
    Empty,
    Pin,
    Shape,
    Crystal,
}

struct Shape {
    layer: usize,
    quarter: usize,
    value: usize
}

impl Shape {
    fn new(layer: usize, quarter: usize, value: usize) -> Self {
        Self {
            layer,
            quarter,
            value,
        }
    }
    fn get(&self, layer: usize, part: usize) -> QuarterType {
        let index = layer * Self{quarter} + part;
        let mask = 3 << (index * 2);
        let value = (self.value & mask) >> (index * 2);
        from_u8(value as u8)
    }
}

fn repeat(val: i32, width: usize, count: usize) -> i32 {
    let mut result = 0;
    for _ in 0..count {
        result <<= width;
        result |= val;
    }

    result
}

fn from_type(q: QuarterType) -> char {
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

fn from_char(c: char) -> QuarterType {
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

fn from_u8(c: u8) -> QuarterType {
    if c as char == PIN {
        QuarterType::Pin
    } else if c as char == CRYSTAL {
        QuarterType::Crystal
    } else if c as char == EMPTY {
        QuarterType::Empty
    } else {
        QuarterType::Shape
    }
}
