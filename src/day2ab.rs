fn day2a(noon: u32, verb: u32, src: Vec<u32>) -> u32 {
    let mut v = src;
    v[1] = noon;
    v[2] = verb;

    let n = v.len() / 4;
    for i in 0..n {
        let base = (i * 4) as usize;
        let command = v[base];
        if 99 == command {
            break;
        }
        let arg1 = v[v[base + 1] as usize];
        let arg2 = v[v[base + 2] as usize];
        let tidx = v[base + 3] as usize;
        let t = &mut v[tidx];

        match command {
            1 => {
                *t = arg1 + arg2;
            }
            2 => {
                *t = arg1 * arg2;
            }
            _ => println!("error"),
        }
    }
    //println!("{:?}", v);

    v[0]
}

fn main() -> std::io::Result<()> {
    let src = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,23,13,27,1,10,27,31,2,31,6,35,1,5,35,39,1,39,10,43,2,9,43,47,1,47,5,51,2,51,9,55,1,13,55,59,1,13,59,63,1,6,63,67,2,13,67,71,1,10,71,75,2,13,75,79,1,5,79,83,2,83,9,87,2,87,13,91,1,91,5,95,2,9,95,99,1,99,5,103,1,2,103,107,1,10,107,0,99,2,14,0,0";
    let v: Vec<u32> = src
        .split(',')
        .map(|item| item.parse::<u32>().unwrap())
        .collect();

    let res_2_a = day2a(12, 2, v.clone());
    println!("Result of day 2/A = {}", res_2_a);

    for noon in 0..99 {
        for verb in 0..99 {
            let res = day2a(noon, verb, v.clone());
            if res == 19690720u32 {
                println!(
                    "Result of day 2/B = noon: {}; verb: {}; -> {}",
                    noon,
                    verb,
                    100 * noon + verb
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let src = "1,9,10,3,2,3,11,0,99,30,40,50";
        let v: Vec<u32> = src
            .split(',')
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        assert_eq!(day2a(9, 10, v), 3500);
    }
}
