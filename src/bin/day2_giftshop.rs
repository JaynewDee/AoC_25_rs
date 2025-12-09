fn main() {
    let input = std::fs::read_to_string("./input/day2_giftshop.txt").unwrap();
    let ranges = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    println!("{:#?}", &ranges);
    // part1(ranges.clone());
    part2(ranges);
}

fn part1(ranges: Vec<&str>) {
    let mut total_invalids: Vec<u64> = Vec::new();

    for range in ranges {
        let mut invalids_in_range: Vec<u64> = Vec::new();

        let mut pieces = range.split("-");
        let start = pieces.next().unwrap().parse::<u64>().unwrap();
        let end = pieces.next().unwrap().parse::<u64>().unwrap();

        for num in start..=end {
            if !has_even_digits(num) {
                continue;
            }

            let s = num.to_string();
            let mid = s.len() / 2;
            let first_half = &s[0..mid];
            let second_half = &s[mid..];
            if first_half == second_half {
                invalids_in_range.push(num);
            }
        }

        total_invalids.extend(invalids_in_range);
    }
    let total: u64 = total_invalids.iter().sum();

    println!("{}", total);
}

fn has_even_digits(n: u64) -> bool {
    n.to_string().len() % 2 == 0
}

fn part2(ranges: Vec<&str>) {
    let mut total_invalids: Vec<u64> = Vec::new();

    for range in ranges {
        let mut invalids_in_range: Vec<u64> = Vec::new();

        let mut parts = range.split("-");
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();

        for num in start..=end {
            let s = num.to_string();
            let num_digits = s.len();

            for i in 1..num_digits {
                if num_digits % i == 0 {
                    let pieces: Vec<&str> = s
                        .as_bytes()
                        .chunks(i)
                        .map(|c| std::str::from_utf8(c).unwrap())
                        .collect();
                    let identical_pieces: bool = pieces.iter().all(|c| *c == pieces[0]);
                    if identical_pieces && pieces.len() > 1 {
                        println!("{}", num);
                        invalids_in_range.push(num);
                        break;
                    }
                }
            }
        }

        total_invalids.extend(invalids_in_range);
    }

    let total: u64 = total_invalids.iter().sum();

    println!("{}", total);
}
