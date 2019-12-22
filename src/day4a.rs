fn check(s: String) -> bool {
    let mut dups = 0;

    let mut pc = s.chars().nth(0);

    for i in 1..s.len() {
        let cc = s.chars().nth(i);
        if pc > cc {
            return false;
        }
        if pc == cc {
            dups += 1;
        }
        pc = cc;
    }

    dups > 0
}

fn day4a(s: i32, e: i32) -> i32 {
    let mut n = 0;
    for i in s..(e + 1) {
        if check(i.to_string()) {
            n += 1;
        }
    }
    n
}

fn main() -> std::io::Result<()> {
    let res = day4a(123257, 647015);

    println!("Result of day 4/A = {:?}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1, day4a(111111, 111111));
        assert_eq!(0, day4a(223450, 223450));
        assert_eq!(0, day4a(123789, 123789));
    }
}
