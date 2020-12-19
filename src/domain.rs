use std::convert::TryFrom;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mark {
    X,
    O,
}

#[derive(Default)]
pub struct Field {
    mark: Option<Mark>,
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

impl Field {
    pub fn is_marked(&self) -> bool {
        self.mark.is_some()
    }

    pub fn is_marked_with(&self, mark: Mark) -> bool {
        if let Some(m) = self.mark {
            mark == m
        } else {
            false
        }
    }

    pub fn get_mark(&self) -> Option<&Mark> {
        self.mark.as_ref()
    }
}

pub type Pixel = usize;

const DIMENSION: Pixel = 3;

#[derive(Default)]
pub struct Playground {
    fields: [Field; DIMENSION * DIMENSION],
}

#[derive(Debug, PartialEq, Eq)]
pub struct PixelCoord {
    pub x: Pixel,
    pub y: Pixel,
}

impl PixelCoord {
    pub fn index(&self) -> Pixel {
        self.y * DIMENSION + self.x
    }
}

#[derive(Debug)]
pub enum FieldError {
    NoFieldAt(Pixel),
    Occupied(Pixel),
}

const WINNING_SCORES: [usize; 8] = [7, 56, 73, 84, 146, 273, 292, 448];
const WINNING_SCORE_BASE: usize = 2;

impl Playground {
    pub fn get_fields(&self) -> &[Field] {
        self.fields.as_ref()
    }

    pub fn is_field_occupied(&self, coord: &PixelCoord) -> bool {
        if let Some(field) = self.fields.get(coord.index()) {
            field.is_marked()
        } else {
            false
        }
    }

    pub fn mark_field_with(&mut self, coord: &PixelCoord, mark: Mark) -> Result<(), FieldError> {
        let index = coord.index();
        if let Some(field) = self.fields.get_mut(index) {
            if field.is_marked() {
                Err(FieldError::Occupied(index))
            } else {
                field.mark = Some(mark);
                Ok(())
            }
        } else {
            Err(FieldError::NoFieldAt(index))
        }
    }

    pub fn has_won(&self, mark: Mark) -> bool {
        let score: usize = self
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                if field.is_marked_with(mark) {
                    WINNING_SCORE_BASE.pow(index as u32)
                } else {
                    0
                }
            })
            .sum();

        WINNING_SCORES
            .iter()
            .any(|winning_score| (winning_score & score) == *winning_score)
    }

    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|field| field.is_marked())
    }
}

#[derive(Debug)]
pub enum NameError {
    TooLong(usize),
    TooShort(usize),
}

const MIN_NAME_LEN: usize = 3;
const MAX_NAME_LEN: usize = 60;

pub struct Name(String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Name {
    type Error = NameError;

    fn try_from(s: &str) -> Result<Self, NameError> {
        let s = s.trim();
        let len = s.len();

        if len < MIN_NAME_LEN {
            Err(NameError::TooShort(len))
        } else if len > MAX_NAME_LEN {
            Err(NameError::TooLong(len))
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

pub enum Player {
    Human { name: Name },
    KI,
}

pub trait Marker {
    fn get_mark(&self) -> Mark;
}

pub struct X {
    player: Player,
}

impl X {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}

impl Marker for X {
    fn get_mark(&self) -> Mark {
        Mark::X
    }
}

pub struct O {
    player: Player,
}

impl O {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}

impl Marker for O {
    fn get_mark(&self) -> Mark {
        Mark::O
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DirectionError {
    TooManyParts,
    UnknownDirection,
    UnknownRow,
    UnknownColumn,
}

pub enum Row {
    Left,
    Right,
    Center,
}

impl Row {
    pub fn parse_str(s: &str) -> Result<Self, DirectionError> {
        match s.trim().to_ascii_lowercase().as_ref() {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "center" => Ok(Self::Center),
            _ => Err(DirectionError::UnknownRow),
        }
    }

    pub fn as_pixel(&self) -> Pixel {
        match self {
            Self::Left => 0,
            Self::Center => 1,
            Self::Right => 2,
        }
    }
}

pub enum Column {
    Top,
    Bottom,
    Center,
}

impl Column {
    pub fn parse_str(s: &str) -> Result<Self, DirectionError> {
        match s.trim().to_ascii_lowercase().as_ref() {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            "center" => Ok(Self::Center),
            _ => Err(DirectionError::UnknownColumn),
        }
    }

    pub fn as_pixel(&self) -> Pixel {
        match self {
            Self::Top => 0,
            Self::Center => 1,
            Self::Bottom => 2,
        }
    }
}

fn parse_direction(s: &str) -> Result<(Row, Column), DirectionError> {
    let s = s.trim().to_ascii_lowercase();
    if s.contains('-') {
        let mut parts = s.split('-');
        match (parts.next(), parts.next(), parts.next()) {
            // if you enter e.g. "center-left"
            (Some(first_part), Some(second_part), None) => {
                match (Row::parse_str(second_part), Column::parse_str(first_part)) {
                    (Ok(row), Ok(column)) => Ok((row, column)),
                    // if you enter e.g. "left-center"
                    _ => Ok((Row::parse_str(first_part)?, Column::parse_str(second_part)?)),
                }
            }
            _ => Err(DirectionError::TooManyParts),
        }
    } else if s == "center" {
        Ok((Row::Center, Column::Center))
    } else {
        Err(DirectionError::UnknownDirection)
    }
}

impl TryFrom<&str> for PixelCoord {
    type Error = DirectionError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (row, column) = parse_direction(s)?;

        Ok(PixelCoord {
            x: row.as_pixel(),
            y: column.as_pixel(),
        })
    }
}

pub trait FieldFormatter {
    fn format(&self, field: &Field) -> String;
}

pub trait PlaygroundDisplay {
    fn display(&self, playground: &Playground);
}

pub trait Writer {
    fn writeln(&self, s: &str);
}

pub trait Reader {
    fn readln(&self) -> String;
}

#[cfg(test)]
mod tests {
    use crate::domain::PixelCoord;
    use std::convert::TryFrom;

    #[test]
    fn it_parses_directions() {
        assert_eq!(
            Ok(PixelCoord { x: 0, y: 0 }),
            PixelCoord::try_from("top-left")
        );
    }
}
