static EMPTY: char = '-';
static PIN: char = 'P';
static SHAPE: char = 'S';
static CRYSTAL: char = 'C';

enum QuarterType {
    Empty,
    Pin,
    Shape,
    Crystal,
}

fn to_char(q: QuarterType) -> char {
    if let QuarterType::Pin = q {
        'P'
    } else if let QuarterType::Shape = q {
        'S'
    } else if let QuarterType::Crystal = q {
        'C'
    } else {
        '-'
    }
}

fn to_type(c: char) -> QuarterType {
    if c == 'P' {
        QuarterType::Pin
    } else if c == 'S' {
        QuarterType::Shape
    } else if c == 'C' {
        QuarterType::Crystal
    } else {
        QuarterType::Empty
    }
}
