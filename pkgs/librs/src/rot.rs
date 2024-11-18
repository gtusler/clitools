use std::{error::Error, fmt::Display};

use crate::math::wrap::wrap_u16;

pub enum Charset {
    AtoZ,
    AlphaNumeric,
}

impl Charset {
    pub fn from_string(input: String) -> Result<Charset, CharsetError> {
        match input.as_str() {
            "az" => Ok(Charset::AtoZ),
            "a9" => Ok(Charset::AlphaNumeric),
            _ => Err(CharsetError::FailedParse(input)),
        }
    }
}

impl Display for Charset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Charset::AtoZ => "az",
            Charset::AlphaNumeric => "a9",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub enum CharsetError {
    FailedParse(String),
}

impl Error for CharsetError {}

impl Display for CharsetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CharsetError::FailedParse(input) => {
                format!("Failed to parse string to supported charset: {}", input)
            }
        };
        write!(f, "{}", msg)
    }
}

pub const CHARSET_ALPHA_NUMERIC: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
pub const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn rot(rotation: u8, mut input: String, charset: Charset) -> Result<String, RotError> {
    input.make_ascii_lowercase();
    let chrset = match charset {
        Charset::AtoZ => CHARSET_LETTERS,
        Charset::AlphaNumeric => CHARSET_ALPHA_NUMERIC,
    };
    let charset_len: u16 = u16::try_from(chrset.len()).expect("charset is too long");
    let range_mod = 1;
    let mut output = String::new();

    for chr in input.chars() {
        let char_idx = chrset.find(chr);

        if char_idx.is_none() {
            return Err(RotError::InvalidChar(chr, input.find(chr).unwrap()));
        }

        let char_idx: u16 = u16::try_from(char_idx.unwrap()).expect("char out of charset range");
        let rotated_idx_modded = wrap_u16(
            char_idx + range_mod,
            rotation.into(),
            range_mod,
            charset_len - 1 + range_mod,
        );
        let rotated_idx = rotated_idx_modded - range_mod;
        let rotated_char = chrset.chars().nth(usize::from(rotated_idx)).expect("huhh");
        output.push(rotated_char);
    }

    Ok(output)
}

/// A rot encoding, based on the given bounds
// pub fn wrap_u16_cryptii(input: u16, amount: u16, min: u16, max: u16) -> u16 {
//     // only rotate if input is within bounds
//     if input >= min && input <= max {
//         let count = max - min + 1;
//         let output = input + (count / 2);
//
//         if output > max {
//             return output - count;
//         }
//     }
//     input
// }

#[derive(Debug, PartialEq, Eq)]
pub enum RotError {
    InvalidChar(char, usize),
}

impl Error for RotError {}

impl Display for RotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            RotError::InvalidChar(c, pos) => {
                format!("Invalid character at position {}: `{}`", pos, c)
            }
        };
        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_lowercase() {
        assert_eq!(
            rot(0, String::from("SOMETHING"), Charset::AlphaNumeric).unwrap(),
            String::from("something")
        );
    }

    #[test]
    fn it_doesnt_rotate() {
        assert_eq!(
            rot(0, String::from("something"), Charset::AlphaNumeric).unwrap(),
            String::from("something")
        );
    }

    #[test]
    fn it_errors_on_invalid_chars() {
        assert_eq!(
            rot(1, String::from("."), Charset::AlphaNumeric).unwrap_err(),
            RotError::InvalidChar('.', 0)
        );
        assert_eq!(
            rot(1, String::from("-"), Charset::AlphaNumeric).unwrap_err(),
            RotError::InvalidChar('-', 0)
        );
    }

    #[test]
    fn it_rotates_once() {
        assert_eq!(
            rot(1, String::from("a"), Charset::AlphaNumeric).unwrap(),
            String::from("b")
        );
        assert_eq!(
            rot(1, String::from("b"), Charset::AlphaNumeric).unwrap(),
            String::from("c")
        );
        assert_eq!(
            rot(1, String::from("z"), Charset::AlphaNumeric).unwrap(),
            String::from("0")
        );
        assert_eq!(
            rot(1, String::from("anything"), Charset::AlphaNumeric).unwrap(),
            String::from("bozuijoh")
        );
    }

    #[test]
    fn it_wraps() {
        assert_eq!(
            rot(1, String::from("9"), Charset::AlphaNumeric).unwrap(),
            String::from("a")
        );
        assert_eq!(
            rot(10, String::from("8"), Charset::AlphaNumeric).unwrap(),
            String::from("i")
        );
    }

    mod rotations {
        use super::*;
        #[test]
        fn it_rotates_0() {
            assert_eq!(
                rot(0, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("a")
            );
        }

        #[test]
        fn it_rotates_1() {
            assert_eq!(
                rot(1, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("b")
            );
        }

        #[test]
        fn it_rotates_2() {
            assert_eq!(
                rot(2, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("c")
            );
        }

        #[test]
        fn it_rotates_3() {
            assert_eq!(
                rot(3, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("d")
            );
        }

        #[test]
        fn it_rotates_4() {
            assert_eq!(
                rot(4, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("e")
            );
        }

        #[test]
        fn it_rotates_5() {
            assert_eq!(
                rot(5, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("f")
            );
        }

        #[test]
        fn it_rotates_6() {
            assert_eq!(
                rot(6, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("g")
            );
        }

        #[test]
        fn it_rotates_7() {
            assert_eq!(
                rot(7, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("h")
            );
        }

        #[test]
        fn it_rotates_8() {
            assert_eq!(
                rot(8, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("i")
            );
        }

        #[test]
        fn it_rotates_9() {
            assert_eq!(
                rot(9, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("j")
            );
        }

        #[test]
        fn it_rotates_10() {
            assert_eq!(
                rot(10, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("k")
            );
        }

        #[test]
        fn it_rotates_11() {
            assert_eq!(
                rot(11, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("l")
            );
        }

        #[test]
        fn it_rotates_12() {
            assert_eq!(
                rot(12, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("m")
            );
        }

        #[test]
        fn it_rotates_13() {
            assert_eq!(
                rot(13, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("n")
            );
        }

        #[test]
        fn it_rotates_14() {
            assert_eq!(
                rot(14, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("o")
            );
        }

        #[test]
        fn it_rotates_15() {
            assert_eq!(
                rot(15, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("p")
            );
        }

        #[test]
        fn it_rotates_16() {
            assert_eq!(
                rot(16, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("q")
            );
        }

        #[test]
        fn it_rotates_17() {
            assert_eq!(
                rot(17, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("r")
            );
        }

        #[test]
        fn it_rotates_18() {
            assert_eq!(
                rot(18, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("s")
            );
        }

        #[test]
        fn it_rotates_19() {
            assert_eq!(
                rot(19, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("t")
            );
        }

        #[test]
        fn it_rotates_20() {
            assert_eq!(
                rot(20, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("u")
            );
        }

        #[test]
        fn it_rotates_21() {
            assert_eq!(
                rot(21, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("v")
            );
        }

        #[test]
        fn it_rotates_22() {
            assert_eq!(
                rot(22, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("w")
            );
        }

        #[test]
        fn it_rotates_23() {
            assert_eq!(
                rot(23, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("x")
            );
        }

        #[test]
        fn it_rotates_24() {
            assert_eq!(
                rot(24, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("y")
            );
        }

        #[test]
        fn it_rotates_25() {
            assert_eq!(
                rot(25, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("z")
            );
        }

        #[test]
        fn it_rotates_26() {
            assert_eq!(
                rot(26, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("0")
            );
        }

        #[test]
        fn it_rotates_27() {
            assert_eq!(
                rot(27, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("1")
            );
        }

        #[test]
        fn it_rotates_28() {
            assert_eq!(
                rot(28, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("2")
            );
        }

        #[test]
        fn it_rotates_29() {
            assert_eq!(
                rot(29, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("3")
            );
        }

        #[test]
        fn it_rotates_30() {
            assert_eq!(
                rot(30, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("4")
            );
        }

        #[test]
        fn it_rotates_31() {
            assert_eq!(
                rot(31, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("5")
            );
        }

        #[test]
        fn it_rotates_32() {
            assert_eq!(
                rot(32, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("6")
            );
        }

        #[test]
        fn it_rotates_33() {
            assert_eq!(
                rot(33, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("7")
            );
        }

        #[test]
        fn it_rotates_34() {
            assert_eq!(
                rot(34, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("8")
            );
        }

        #[test]
        fn it_rotates_35() {
            assert_eq!(
                rot(35, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("9")
            );
        }

        #[test]
        fn it_rotates_36() {
            assert_eq!(
                rot(36, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("a")
            );
        }

        #[test]
        fn it_rotates_37() {
            assert_eq!(
                rot(37, String::from("a"), Charset::AlphaNumeric).unwrap(),
                String::from("b")
            );
        }
    }
}
