/*!
Part 1:
Find the two entries that sum to 2020 and then multiply those two numbers together

Part 2:
Find the three entries that sum to 2020 and then multiply those three numbers together
*/

use std::{
    cmp::Ordering,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        let mut input = buffer
            .lines()
            .map(|v| v.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        input.sort_unstable();
        input
    };

    //println!("Input: {:?}", &input);

    println!("Part 1");
    let (i1, i2) = sum_2(&input, 2020).unwrap();
    println!("Result: {} * {} = {}", i1, i2, i1 * i2);

    println!("Part 2");
    let (i1, i2, i3) = sum_3(&input, 2020).unwrap();
    println!("Result: {} * {} * {} = {}", i1, i2, i3, i1 * i2 * i3);

    Ok(())
}

/*
From a sorted input:

299  -- a (increase `a` if sum < search)
366
675
979
1456
1721 -- b (decrease `b` if sum > search)

while a < b:
    let sum = i[a] + i[b];
    if sum == search
        return // Complete
    else if sum > search
        b--
    else if sum < search
        a++
*/
fn sum_2(input: &[u32], search: u32) -> Option<(u32, u32)> {
    let mut a = 0usize;
    let mut b = input.len() - 1;

    while a < b {
        let sum = input[a] + input[b];

        match sum.cmp(&search) {
            Ordering::Greater => b -= 1,
            Ordering::Less => a += 1,
            Ordering::Equal => return Some((input[a], input[b])),
        }
    }

    None
}

/*
More of a bruteforce type of approach + binary search

- Iterate through the array with `a` and `b`
- Binary search for `c` if: (search - (i[a] + i[b])) > -1
*/
fn sum_3(input: &[u32], search: u32) -> Option<(u32, u32, u32)> {
    for a in 0..input.len() {
        for b in 0..input.len() {
            let sum = input[a] + input[b];
            if let Some(rest) = search.checked_sub(sum) {
                if input.binary_search(&rest).is_ok() {
                    return Some((input[a], input[b], rest));
                }
            } else {
                // we can break early because it's sorted so, `b+1` will also underflow
                break;
            }
        }
    }

    None
}
