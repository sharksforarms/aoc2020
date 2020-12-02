/*!
Part 1:
Count all valid passwords, example:
For the input "1-3 a: aabbcc"
`a` must appear `1` to `3` time in the password `aabbcc`
For this input, the result is false.

Part 2:
Count all valid passwords, example:
For the input "1-3 a: aabbcc"
`a` must appear in index `1` or `3` but not both of the password `aabbcc`
Note: "Arrays start at 1"
*/

use std::io::{self, Read};

/// Returns true if the password is valid
/// if `vchar` appears `min` to `max` times in `password`
fn validate_password1(min: usize, max: usize, vchar: char, password: &str) -> bool {
    (min..=max).contains(&password.matches(vchar).count())
}

/// Returns true if the password is valid
/// if `vchar` appears in index `i1` or `i2` of `password`
fn validate_password2(i1: usize, i2: usize, vchar: char, password: &str) -> bool {
    match (
        password.chars().nth(i1 - 1).unwrap() == vchar,
        password.chars().nth(i2 - 1).unwrap() == vchar,
    ) {
        (true, false) => true,
        (false, true) => true,
        _ => false,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut count1 = 0;
    let mut count2 = 0;

    for line in buffer.lines() {
        let mut toks = line.split(' ');

        let (i1, i2) = toks
            .next()
            .map(|v| {
                let mut range = v.split('-');
                let i1 = range.next().unwrap().parse::<usize>().unwrap();
                let i2 = range.next().unwrap().parse::<usize>().unwrap();

                (i1, i2)
            })
            .unwrap();

        let validatation_char = toks.next().unwrap().chars().next().unwrap();

        let password = toks.next().unwrap();

        if validate_password1(i1, i2, validatation_char, password) {
            count1 += 1;
        }
        if validate_password2(i1, i2, validatation_char, password) {
            count2 += 1;
        }
    }

    println!("Part 1: # of valid passwords: {}", count1);
    println!("Part 2: # of valid passwords: {}", count2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate1() {
        assert!(validate_password1(0, 2, 'a', "b"));
        assert!(validate_password1(0, 2, 'a', "ab"));
        assert!(validate_password1(0, 2, 'a', "aab"));
        assert!(!validate_password1(0, 2, 'a', "aaab"));
    }

    #[test]
    fn test_validate2() {
        assert!(validate_password2(1, 3, 'a', "abcde"));
        assert!(!validate_password2(1, 3, 'b', "cdefg"));
        assert!(!validate_password2(2, 9, 'c', "ccccccccc"));
    }
}
