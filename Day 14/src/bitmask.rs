#[derive(Debug)]
pub struct Bitmask {
    zeroes: u64,
    ones: u64,
}

impl Bitmask {
    pub fn from_str(mask: &str) -> Self {
        let padded_mask = format!("{:X>64}", mask);

        Self {
            zeroes: u64::from_str_radix(&padded_mask.replace("X", "1"), 2).unwrap(),
            ones: u64::from_str_radix(&padded_mask.replace("X", "0"), 2).unwrap(),
        }
    }

    pub fn apply(&self, value: u64) -> u64 {
        (value & self.zeroes) | self.ones
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Self {
            zeroes: 0,
            ones: u64::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_bitmask() {
        assert_eq!(
            Bitmask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(11),
            73
        );
    }

    #[test]
    fn apply_bitmask_no_change() {
        assert_eq!(
            Bitmask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(101),
            101
        );
    }

    #[test]
    fn apply_bitmask_to_zero() {
        assert_eq!(
            Bitmask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(0),
            64
        );
    }
}
