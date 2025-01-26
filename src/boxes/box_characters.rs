const THICK_BORDER_LOOKUP: [char; 16] = [
    ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
];

pub fn thick_border_from_directions(north: bool, east: bool, south: bool, west: bool) -> char {
    let index = (north as usize)
        | ((east as usize) << 1)
        | ((south as usize) << 2)
        | ((west as usize) << 3);
    THICK_BORDER_LOOKUP[index]
}

const THIN_ROUNDED_BORDER_LOOKUP: [char; 16] = [
    ' ', '╵', '╶', '╰', '╷', '│', '╭', '├', '╴', '╯', '─', '┴', '╮', '┤', '┬', '┼',
];

pub fn thin_rounded_border_from_directions(
    north: bool,
    east: bool,
    south: bool,
    west: bool,
) -> char {
    let index = (north as usize)
        | ((east as usize) << 1)
        | ((south as usize) << 2)
        | ((west as usize) << 3);
    THIN_ROUNDED_BORDER_LOOKUP[index]
}

pub enum ArrowDirection {
    North,
    East,
    South,
    West,
}

impl From<ArrowDirection> for char {
    fn from(value: ArrowDirection) -> Self {
        match value {
            ArrowDirection::North => '↑',
            ArrowDirection::East => '→',
            ArrowDirection::South => '↓',
            ArrowDirection::West => '←',
        }
    }
}
