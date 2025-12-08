use std::ops::Index;

fn main() {
    let content =std::fs::read_to_string("input/day1_secretentrance.txt").unwrap() ;
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
                for n in 1..=moves {
                    if current == 0 {
                        current = 99;
                    } else {
                        current -= 1;
                    }

                }
            },
            "R" => {
                for n in 1..=moves {
                    if current == 99 {
                        current = 0;
                    } else {
                        current += 1;
                    }
                }
            }
            _ => panic!("Unknown direction: {}", direction)
        }

        if current == 0 {
            num_zeroes += 1;
        }
    }
    println!("{}", num_zeroes);
}

fn part2(input_lines: &Vec<&str>) {
    let mut current = 50u32;
    let mut num_zeroes = 0u32;

    for line in input_lines {
        let pieces = line.split_at(1);
        let direction = pieces.0;
        let rotations = pieces.1.parse::<u32>().unwrap();

        match direction {
            "L" => {
                for _ in 1..=rotations {
                    if current == 0 {
                        num_zeroes += 1;
                        current = 99;
                    } else {
                        current -= 1;
                    }

                }
            },
            "R" => {
                for _ in 1..=rotations {
                    if current == 99 {
                        current = 0;
                    } else {
                        current += 1;
                    }
                    if current == 0 {
                        num_zeroes += 1;
                    }
                }
            }
            _ => panic!("Unknown direction: {}", direction)
        }
    }

    println!("{}", num_zeroes);
}

fn rotate(direction: &str, rotations: u32, mut current: u32) -> u32 {
    let mut num_passes_over_zero = 0;

    match direction {
        "L" => {
            for _ in 1..=rotations {
                if current == 0 {
                    num_passes_over_zero += 1;
                    current = 99;
                } else {
                    current -= 1;
                }

            }
        },
        "R" => {
            for _ in 1..=rotations {
                if current == 99 {
                    current = 0;
                } else {
                    current += 1;
                }
                if current == 0 {
                    num_passes_over_zero += 1;
                }
            }
        }
        _ => panic!("Unknown direction: {}", direction)
    }

    num_passes_over_zero
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let direction = "R";
        let rotations: u32 = 1000;

        let current: u32 = 50;

        let num_passes_over_zero = rotate(direction, rotations, current);

        assert_eq!(current, 50);
        assert_eq!(num_passes_over_zero, 10);

        let direction = "L";
        let rotations = 60;

        let num_passes_over_zero = rotate(direction, rotations, current);

        assert_eq!(current, 90);
        assert_eq!(num_passes_over_zero, 1);
    }
}