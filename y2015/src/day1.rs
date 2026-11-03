pub(crate) fn solve() {
    println!("Day 1");
    let input = include_str!("day-1-input");

    let mut sum = 0;
    let mut invalid = 0;
    for (i,c) in input.chars().enumerate() {
        
        if c == '(' { sum = sum + 1; }
        else if c == ')' { sum = sum - 1; }
        else { invalid = i+1; }
        
        println!("{} {}", sum, i+1);
        if sum == -1 { println!("First time at -1: {}", i+1 - invalid); }
    }
    println!("{}", sum);
}