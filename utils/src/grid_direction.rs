#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum GridDirection {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl From<&char> for GridDirection {
    fn from(value: &char) -> Self {
        match *value {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Char {value} unkown."),
        }
    }
}

impl From<&mut char> for GridDirection {
    fn from(value: &mut char) -> Self {
        match *value {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Char {value} unkown."),
        }
    }
}

impl GridDirection {
    pub fn get_directions() -> [GridDirection; 4] {
        [
            GridDirection::Up,
            GridDirection::Right,
            GridDirection::Down,
            GridDirection::Left,
        ]
    }

    pub fn get_int_char(&self) -> char {
        match self {
            GridDirection::Up => '0',
            GridDirection::Right => '1',
            GridDirection::Down => '2',
            GridDirection::Left => '3',
        }
    }

    pub fn from_int_char(c: char) -> Self {
        match c {
            '0' => GridDirection::Up,
            '1' => GridDirection::Right,
            '2' => GridDirection::Down,
            '3' => GridDirection::Left,
            _ => panic!("Not a valid direction {c}"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            GridDirection::Up => '^',
            GridDirection::Right => '>',
            GridDirection::Down => 'v',
            GridDirection::Left => '<',
        }
    }
}
