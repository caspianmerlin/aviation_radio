use std::{str::FromStr, num::ParseIntError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RadioFrequency {
    left: u16,
    right: u16,
    is_25_khz_spaced: bool,
}



impl RadioFrequency {
    pub fn new(left: u16, right: u16) -> Result<RadioFrequency, RadioFrequencyError> {
        
        // Validate left
        if !(118..=137).contains(&left) {
            return Err(RadioFrequencyError::InvalidFrequency);
        }
        
        // Validate right
        
        //First digit doesn't matter. Get the last two.
        let last_two = right % 100;
        const VALID_CHANNELS: [u16; 16] = [00, 05, 10, 15, 25, 30, 35, 40, 50, 55, 60, 65, 75, 80, 85, 90];
        if !&VALID_CHANNELS[..].contains(&last_two) {
            return Err(RadioFrequencyError::InvalidFrequency);
        }
        
        Ok(RadioFrequency {
            left,
            right,
            is_25_khz_spaced: [00, 25, 50, 75].contains(&last_two),
        })
        
    }
    pub fn is_8_33_khz_spaced(&self) -> bool {
        !self.is_25_khz_spaced
    }
    pub fn is_25_khz_spaced(&self) -> bool {
        self.is_25_khz_spaced
    }
    
    pub fn frequency(&self) -> (u16, u16) {
        (self.left, self.right)
    }
    
    pub fn left(&self) -> u16 {
        self.left
    }
    
    pub fn right(&self) -> u16 {
        self.right
    }
}

impl std::fmt::Display for RadioFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03}.{:03}", self.left, self.right)
    }
}


impl FromStr for RadioFrequency {
    type Err = RadioFrequencyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let left = parts.next().ok_or(RadioFrequencyError::NotEnoughParts)?.parse::<u16>()?;
        let right = parts.next().ok_or(RadioFrequencyError::NotEnoughParts)?.parse::<u16>()?;
        RadioFrequency::new(left, right)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum RadioFrequencyError {
    InvalidFrequency,
    NotEnoughParts,
    ParseError(ParseIntError),
}

impl From<ParseIntError> for RadioFrequencyError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}

impl std::fmt::Display for RadioFrequencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NotEnoughParts => "Not enough parts",
            Self::InvalidFrequency => "Invalid frequency",
            Self::ParseError(_) => "Int parse error",
        })
    }
}

impl std::error::Error for RadioFrequencyError {}

#[cfg(test)]
mod tests {
    use crate::RadioFrequency;

    #[test]
    fn validate() {
        let valid = RadioFrequency::new(120, 905).unwrap();
        let invalid_a = RadioFrequency::new(110, 300);
        let invalid_b = RadioFrequency::new(118, 3);
        let invalid_c = RadioFrequency::new(138, 005);
        let invalid_d = RadioFrequency::new(121, 012);

        assert!(invalid_a.is_err());
        assert!(invalid_b.is_err());
        assert!(invalid_c.is_err());
        assert!(invalid_d.is_err());

        assert_eq!(valid.left(), 120);
        assert_eq!(valid.right(), 905);

        assert!(valid.is_8_33_khz_spaced())
    }
}