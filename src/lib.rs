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
