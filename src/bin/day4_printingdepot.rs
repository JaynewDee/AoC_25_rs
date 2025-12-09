fn main() {
    let content = std::fs::read_to_string("./input/day4_printingdepot.txt").unwrap();
    let rows = content.lines();
    let mut grid: Vec<Vec<char>> = rows.clone().map(|s| s.chars().collect()).collect();

    let mut total_accessible = 0;

    for (y, row) in rows.enumerate() {
        let cs = row.chars();

        for (x, c) in cs.enumerate() {
            if c == '@' {
                // it's a paper roll.
                // check all adjacent positions
                // count how many adjacent rolls
                // if count adjacent < 4, add to total accessible
                if check_adjacent(&mut grid, (y, x)) {
                    total_accessible += 1;
                }
            }
        }
    }

    println!("Total Accessible: {}", total_accessible);
}

fn check_adjacent(grid: &mut Vec<Vec<char>>, roll_pos: (usize, usize)) -> bool {
    let mut count_adjacent_rolls = 0;

    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dr, dc) in dirs {
        let nr = roll_pos.0 as isize + dr;
        let nc = roll_pos.1 as isize + dc;

        // Only return valid positions
        if nr >= 0 && nr < grid.len() as isize && nc >= 0 && nc < grid.len() as isize {
            if grid[nr as usize][nc as usize] == '@' {
                count_adjacent_rolls += 1;
            }
        }
    }

    if count_adjacent_rolls < 4 {
        grid[roll_pos.0][roll_pos.1] = '.';
        true
    } else {
        false
    }
}
