pub const CHAR_SPACE: &str = "   ";
pub const CHAR_DOWN: &str = "│  ";
pub const CHAR_DOWN_JOIN: &str = "├─ ";
pub const CHAR_DOWN_END: &str = "└─ ";

pub fn make_stripy_prefix(indent_level: u8, stripes: &[u8]) -> String {
    let mut output = String::new();

    for i in 0..indent_level {
        let is_line_kept = stripes.contains(&i);

        if is_line_kept {
            output.push_str(CHAR_DOWN);
        } else {
            output.push_str(CHAR_SPACE);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    mod stripy_prefix {
        use super::*;

        #[test]
        fn nothing() {
            assert_eq!(make_stripy_prefix(0, &mut Vec::new()), String::from(""));
        }

        #[test]
        fn top_level() {
            assert_eq!(make_stripy_prefix(1, &mut Vec::new()), String::from("   "));
            assert_eq!(make_stripy_prefix(1, &mut vec![0]), String::from("│  "));
        }

        #[test]
        fn two_levels() {
            assert_eq!(
                make_stripy_prefix(2, &mut Vec::new()),
                String::from("      ")
            );
            assert_eq!(make_stripy_prefix(2, &mut vec![0]), String::from("│     "));
            assert_eq!(
                make_stripy_prefix(2, &mut vec![0, 1]),
                String::from("│  │  ")
            );
        }

        #[test]
        fn three_levels() {
            assert_eq!(
                make_stripy_prefix(3, &mut vec![]),
                String::from("         ")
            );
            assert_eq!(
                make_stripy_prefix(3, &mut vec![0]),
                String::from("│        ")
            );
            assert_eq!(
                make_stripy_prefix(3, &mut vec![0, 1]),
                String::from("│  │     ")
            );
            assert_eq!(
                make_stripy_prefix(3, &mut vec![0, 1, 2]),
                String::from("│  │  │  ")
            );
        }
    }
}
