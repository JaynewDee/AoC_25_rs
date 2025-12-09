fn main() {
    let content = std::fs::read_to_string("./input/day3_lobby.txt").unwrap();
    let banks = content.lines().collect::<Vec<&str>>();

    part1(banks);
}

fn part1(banks: Vec<&str>) {
    let mut joltages: Vec<u8> = Vec::with_capacity(banks.len());

    for bank in banks {
        let batteries = bank.chars().collect::<Vec<char>>();
        let mut outer_ptr = 0usize;
        let mut inner_ptr = 1usize;
        let mut highest_joltage = (batteries[outer_ptr], batteries[inner_ptr]);

        while outer_ptr < batteries.len() - 1 {
            let first = batteries[outer_ptr];
            if first > highest_joltage.0 {
                highest_joltage.0 = first;
                highest_joltage.1 = batteries[outer_ptr + 1];
                inner_ptr = outer_ptr + 1;
                continue;
            }
            while inner_ptr < batteries.len() {
                let second = batteries[inner_ptr];
                if second > highest_joltage.1 {
                    highest_joltage.1 = second;
                }

                inner_ptr += 1;
            }

            outer_ptr += 1;
        }

        let mut joltage_str = highest_joltage.0.to_string();
        joltage_str.push(highest_joltage.1);
        joltages.push(joltage_str.parse::<u8>().unwrap());
    }

    let mut total_highest_joltages = 0u32;
    for joltage in joltages {
        total_highest_joltages += joltage as u32;
    }

    println!("{}", total_highest_joltages);
}
