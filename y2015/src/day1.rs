pub(crate) fn solve() {
    println!("Day 1");
    let input = include_str!("day-1-input");

    let mut sum = 0;

    for c in input.chars() {
        if c == '(' { sum = sum + 1; }
        else if c == ')' { sum = sum - 1; }
    }

    println!("{}", sum);
}