// Part 1
fn main() {
    const INPUT_STRING: &str = include_str!("../input.txt"); // Pull in input file as string
    let lines = INPUT_STRING.lines(); // Iterator over lines

    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect(); // Split at 3 spaces
        left.push(parts[0]);
        right.push(parts[1]);
    }
    
    left.sort();
    right.sort();
    
    let total: i32 = left.into_iter().zip(right.into_iter())
    .map(|(l, r)| (l.parse::<i32>().unwrap() - r.parse::<i32>().unwrap()).abs())
    .sum();

    println!("{:?}", total);
}
