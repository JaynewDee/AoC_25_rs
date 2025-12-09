use std::str::Lines;

struct Problem {
    operands: Vec<u64>,
    operator: char,
}

fn main() {
    let content = std::fs::read_to_string("./input/day6_trashcompactor.txt").unwrap();
    let mut lines = content.lines();

    let operand1_list = collect_list(&mut lines);
    let operand2_list = collect_list(&mut lines);
    let operand3_list = collect_list(&mut lines);
    let operand4_list = collect_list(&mut lines);
    let operators_list = collect_list(&mut lines);

    let mut problems: Vec<Problem> = Vec::new();

    for i in 0..operand1_list.len() {
        let mut operands: Vec<u64> = Vec::new();
        operands.push(operand1_list[i].parse::<u64>().unwrap());
        operands.push(operand2_list[i].parse::<u64>().unwrap());
        operands.push(operand3_list[i].parse::<u64>().unwrap());
        operands.push(operand4_list[i].parse::<u64>().unwrap());

        if let Some(operator) = operators_list[i].chars().nth(0) {
            problems.push(Problem { operands, operator })
        }
    }

    let mut problems_total = 0;

    for problem in problems {
        match problem.operator {
            '+' => problems_total += problem.operands.iter().sum::<u64>(),
            '*' => problems_total += problem.operands.iter().product::<u64>(),
            _ => {}
        }
    }
    println!("{}", problems_total);
}

fn collect_list<'a>(lines: &mut Lines<'a>) -> Vec<&'a str> {
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
}
