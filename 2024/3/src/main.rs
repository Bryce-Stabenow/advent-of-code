use regex::Regex;

fn main() {
    const INPUT_STRING: &str = include_str!("../input.txt"); // Pull in input file as string
    let mut total_sum = 0;
    
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(INPUT_STRING);
    
    for (_, [num_1, num_2]) in matches.map(|captures| captures.extract()) {
        total_sum += num_1.parse::<i32>().unwrap() * num_2.parse::<i32>().unwrap();
    }
    
    println!("{:?}", total_sum);
}
