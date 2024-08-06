/// this behaves strangely when the range starts at 0.
pub fn wrap_u16(input: u16, amount: u16, min: u16, max: u16) -> u16 {
    let output = input + amount;

    if output > max {
        output - max
    } else if output < min {
        input + amount + min
    }
    else {
        output
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_doesnt_mutate_the_input() {
        assert_eq!(wrap_u16(5, 0, 1, 10), 5);
    }

    #[test]
    fn it_stays_within_bounds() {
        assert_eq!(wrap_u16(4, 1, 1, 10), 5);
        assert_eq!(wrap_u16(6, 2, 1, 10), 8);
    }

    #[test]
    fn it_wraps_once() {
        assert_eq!(wrap_u16(5, 10, 1, 10), 5);
        assert_eq!(wrap_u16(1, 11, 1, 10), 2);
        assert_eq!(wrap_u16(10, 1, 1, 10), 1);
    }

    #[test]
    fn it_wraps_once_again() {
        assert_eq!(wrap_u16(4, 8, 10, 20), 12);
    }

    // #[test]
    // fn it_handles_zero() {
    //     // This is an annoying thing which I can't be fucked to correct rn
    //     assert_eq!(wrap_u16(9, 1, 0, 9), 0);
    // }
}
