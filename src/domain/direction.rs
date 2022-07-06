use {serde::Serialize, thiserror::Error};
#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Left = -1,
    Right = 1,
    Stay = 0,
}

#[derive(Error, Serialize, Debug, PartialEq)]
pub enum ValidateError {
    #[error("left and right cannot be true consecutively.")]
    ConsecutiveTrueError,
}

impl Direction {
    //     fn value(self) -> i32 {
    //         match self {
    //             Direction::Left => -1,
    //             Direction::Right => 1,
    //             Direction::Stay => 0,
    //         }
    //     }
    fn validate(left: bool, right: bool) -> Result<(), ValidateError> {
        match (left, right) {
            (true, true) => Err(ValidateError::ConsecutiveTrueError),
            _ => Ok(()),
        }
    }

    fn of(left: bool, right: bool) -> Result<Direction, ValidateError> {
        Self::validate(left, right)?;
        let direction = match (left, right) {
            (true, _) => Direction::Left,
            (_, true) => Direction::Right,
            _ => Direction::Stay,
        };

        Ok(direction)
    }

    fn first(current: bool) -> Result<Direction, ValidateError> {
        Self::of(false, current)
    }

    fn next(self, current: bool) -> Result<Direction, ValidateError> {
        match self {
            Self::Right => Self::of(true, current),
            _ => Self::of(false, current),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::domain::direction::Direction;

    use super::ValidateError;

    #[test]
    fn binary_operation() {
        let actual = 2 + Direction::Left as i32;
        assert_eq!(actual, 1);
        let actual = 2 + Direction::Right as i32;
        assert_eq!(actual, 3);
        let actual = 2 + Direction::Stay as i32;
        assert_eq!(actual, 2);
    }

    #[test]
    fn direction() -> Result<(), Box<dyn Error>> {
        assert_eq!(Direction::first(true), Ok(Direction::Right));
        assert_eq!(Direction::first(true)?.next(false), Ok(Direction::Left));
        assert_eq!(Direction::first(false)?.next(true), Ok(Direction::Right));
        assert_eq!(
            Direction::first(true)?.next(true),
            Err(ValidateError::ConsecutiveTrueError)
        );
        Ok(())
    }
}
