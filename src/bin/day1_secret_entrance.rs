use core::num;

fn main() {
    let content = std::fs::read_to_string("input/day1_secretentrance.txt").unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    part1(&lines);
    part2(&lines);
}

fn part1(input_lines: &Vec<&str>) {
    let mut current = 50u32;
    let mut num_zeroes = 0u32;

    for line in input_lines {
        let pieces = line.split_at(1);
        let direction = pieces.0;
        let moves = pieces.1.parse::<u32>().unwrap();

        match direction {
            "L" => {
                for _ in 1..=moves {
                    if current == 0 {
                        current = 99;
                    } else {
                        current -= 1;
                    }
                }
            }
            "R" => {
                for _ in 1..=moves {
                    if current == 99 {
                        current = 0;
                    } else {
                        current += 1;
                    }
                }
            }
            _ => panic!("Unknown direction: {}", direction),
        }

        if current == 0 {
            num_zeroes += 1;
        }
    }
    println!("{}", num_zeroes);
}

fn part2(input_lines: &Vec<&str>) {
    let mut current = 50i64;
    let mut num_zeroes = 0i64;

    for line in input_lines {
        let pieces = line.split_at(1);
        let direction = pieces.0;
        let rotations = pieces.1.parse::<i64>().unwrap();

        let (new_pos, passes) = rotate(direction, rotations, current);

        num_zeroes += passes;
        current = new_pos;
    }

    println!("{}", num_zeroes);
}

fn rotate(direction: &str, rotations: i64, pos: i64) -> (i64, i64) {
    match direction {
        "L" => {
            let end = pos as i64 - rotations as i64;
            let wrapped = ((end % 100) + 100) % 100;
            let wraps = end.div_euclid(100) - (pos as i64).div_euclid(100);
            // println!("Wrapped: {}\nWraps: {}", wrapped, wraps);
            (wrapped, wraps.abs())
        }
        "R" => {
            let end = pos as i64 + rotations as i64;
            let wrapped = ((end % 100) + 100) % 100;
            let wraps = end.div_euclid(100) - (pos as i64).div_euclid(100);
            // println!("Wrapped: {}\nWraps: {}", wrapped, wraps);
            (wrapped, wraps.abs())
        }
        _ => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let direction = "R";
        let rotations: i64 = 1000;
        let current: i64 = 50;

        let (pos, passes) = rotate(direction, rotations, current);

        assert!(pos == 50);
        assert!(passes == 10);
    }
}
