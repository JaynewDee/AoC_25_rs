fn main() {
    let content = std::fs::read_to_string("./input/day5_cafeteria.txt").unwrap();

    let sections = content
        .find("\r\n\r\n")
        .map(|idx| {
            let end = idx + 4;
            (&content[..idx], &content[end..])
        })
        .unwrap();

    let fresh_ranges = sections
        .0
        .lines()
        .map(|s| {
            let mut pieces = s.split('-');
            let start = pieces.next().unwrap().parse::<u64>().unwrap();
            let end = pieces.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    let fresh_ids = sections
        .1
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // part1(&fresh_ranges, &fresh_ids);
    part2(&fresh_ranges);
}

fn within_fresh_range(fresh_ranges: &Vec<(u64, u64)>, id: &u64) -> bool {
    for (low, high) in fresh_ranges {
        if id >= low && id <= high {
            return true;
        }
    }

    false
}

fn part1(fresh_ranges: &Vec<(u64, u64)>, fresh_ids: &Vec<u64>) {
    let mut count_fresh = 0;

    for id in fresh_ids {
        if within_fresh_range(&fresh_ranges, id) {
            count_fresh += 1;
        }
    }

    println!("{:#?}", count_fresh);
}

fn part2(fresh_ranges: &Vec<(u64, u64)>) {
    let fresh_ranges = fresh_ranges.clone();

    let merged_ranges = merge_ranges(fresh_ranges);

    let mut total_fresh = 0;

    for range in merged_ranges {
        total_fresh += (range.1 + 1) - (range.0);
    }

    println!("{}", total_fresh);
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);

    merged
}
