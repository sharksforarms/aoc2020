/*!
Part 1:
Count number of valid passports, `cid` field is optional

Part 2:
Count number of valid passports, more conditions, `cid` field is optional
*/

use std::{
    collections::HashMap,
    io::{self, Read},
};

fn count_valid(input: String) -> usize {
    input
        .split("\n\n")
        .map(|passport_buf| {
            passport_buf
                .lines()
                .flat_map(|line| {
                    line.split(' ').map(|kv| {
                        let mut tok = kv.split(':');
                        (tok.next().unwrap(), tok.next().unwrap())
                    })
                })
                .collect::<HashMap<&str, &str>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .fold(0, |valid_count, passport| {
            // Validate fields

            // Part 1
            //valid_count + ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].
            //iter().map(|f| passport.contains_key(f)).all(|c| c==true) as usize

            // Part 2
            valid_count
                + ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                    .iter()
                    .map(|f| match passport.get(f) {
                        Some(v) => match *f {
                            "byr" => {
                                let byr = v.parse::<u16>().unwrap();
                                byr >= 1920 && byr <= 2002
                            }
                            "iyr" => {
                                let iyr = v.parse::<u16>().unwrap();
                                iyr >= 2010 && iyr <= 2020
                            }
                            "eyr" => {
                                let eyr = v.parse::<u16>().unwrap();
                                eyr >= 2020 && eyr <= 2030
                            }
                            "hgt" => {
                                let unit = &v[v.len() - 2..];
                                let value = v[..v.len() - 2].parse::<u8>().unwrap();
                                match unit {
                                    "cm" => value >= 150 && value <= 193,
                                    "in" => value >= 59 && value <= 76,
                                    _ => false,
                                }
                            }
                            "hcl" => {
                                if v.len() != 7 {
                                    return false;
                                }
                                v.starts_with('#')
                                    && v[1..7].chars().all(|c| {
                                        (c.is_ascii_digit() || c.is_ascii_lowercase())
                                            && c.is_ascii_hexdigit()
                                    })
                            }
                            "ecl" => match *v {
                                "amb" => true,
                                "blu" => true,
                                "brn" => true,
                                "gry" => true,
                                "grn" => true,
                                "hzl" => true,
                                "oth" => true,
                                _ => false,
                            },
                            "pid" => {
                                if v.len() != 9 {
                                    return false;
                                }
                                v.chars().all(|c| c.is_ascii_digit())
                            }
                            _ => todo!(),
                        },
                        None => false,
                    })
                    .all(|c| c) as usize
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let valid_count = count_valid(buffer);
    println!("Valid count: {}", valid_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let invalid_pp = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .to_string();

        assert_eq!(0, count_valid(invalid_pp));

        let valid_pp = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            .to_string();

        assert_eq!(4, count_valid(valid_pp));
    }
}
