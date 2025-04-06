use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // First I wrote it with ifs - the compiler suggested a
        // match statement with .cmp
        match value.cmp(&0) {
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }

    fn simpler_new(value: i64) -> Result<Self, CreationError> {
        // But it's also possible to do plain comparison in the patterns,
        // so no reliance on Ordering
        match value {
            0 => Err(CreationError::Zero),
            value if value < 0 => Err(CreationError::Negative),
            value => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
