use crate::bitmask::Bitmask;

pub struct MemoryAddressDecoder {
    bitmasks: Vec<Bitmask>,
}

impl MemoryAddressDecoder {
    pub fn from_str(mask: &str) -> Self {
        Self {
            bitmasks: mask_strings(mask)
                .into_iter()
                .map(|mask_string| Bitmask::from_str(&mask_string))
                .collect(),
        }
    }

    pub fn decode(&self, memory_address: u64) -> Vec<u64> {
        self.bitmasks
            .iter()
            .map(|bitmask| bitmask.apply(memory_address))
            .collect()
    }
}

impl Default for MemoryAddressDecoder {
    fn default() -> Self {
        Self {
            bitmasks: Vec::with_capacity(0),
        }
    }
}

fn mask_strings(mask: &str) -> Vec<String> {
    let floating_bit_indexes: Vec<(usize, char)> = mask
        .char_indices()
        .filter(|&(_index, character)| character == 'X')
        .collect();
    let num_masks = 2usize.pow(floating_bit_indexes.len() as u32);

    let mut mask_strings: Vec<String> = Vec::with_capacity(num_masks);
    for possibility in 0..num_masks {
        let bits_string = format!("{:b}", possibility);
        let mut bits = bits_string.chars().rev();
        let mask_chars: Vec<char> = mask
            .to_string()
            .chars()
            .map(|b| match b {
                '0' => 'X',
                '1' => '1',
                'X' => bits.next().unwrap_or('0'),
                _ => panic!(),
            })
            .collect();
        mask_strings.push(mask_chars.into_iter().collect())
    }

    mask_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn mask_strings_no_floating_bits() {
        // Arrange
        let input = "101";

        // Act
        let result = mask_strings(input);

        // Assert
        assert_eq!(result, vec!["101".to_string()]);
    }

    #[test]
    fn mask_strings_one_floating_bit() {
        // Arrange
        let input = "1X1";

        // Act
        let result = mask_strings(input);

        // Assert
        assert_eq!(result, vec!["101".to_string(), "111".to_string()]);
    }

    #[test]
    fn mask_strings_two_floating_bits() {
        // Arrange
        let input = "1X1X";

        // Act
        let result = mask_strings(input);

        // Assert
        assert_eq!(
            result,
            vec![
                "1010".to_string(),
                "1110".to_string(),
                "1011".to_string(),
                "1111".to_string(),
            ]
        );
    }

    #[test]
    fn mask_strings_three_floating_bits() {
        // Arrange
        let input = "XXX";

        // Act
        let result = mask_strings(input);

        // Assert
        assert_eq!(
            result,
            vec![
                "000".to_string(),
                "100".to_string(),
                "010".to_string(),
                "110".to_string(),
                "001".to_string(),
                "101".to_string(),
                "011".to_string(),
                "111".to_string(),
            ]
        );
    }

    #[test]
    fn decode_two_floating_bits() {
        // Arrange
        let decoder = MemoryAddressDecoder::from_str("000000000000000000000000000000X1001X");

        // Act
        let result = decoder.decode(42);

        // Assert
        assert_eq!(result, [26, 58, 27, 59]);
    }
}
