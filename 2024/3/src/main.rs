use regex::Regex;

fn main() {
    const INPUT_STRING: &str = include_str!("../input.txt"); // Pull in input file as string
    let mut total_sum = 0;

    // Remove all text between don't() and do() or don't() and newline
    let removal_re = Regex::new(r"don't\(\).+?(do\(\)|\n)").unwrap();
    let enabled_input = removal_re.replace_all(INPUT_STRING, "don't()do()").to_string();
    
    // Find all instances of mul() and extract the two numbers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(&enabled_input);
    
    // Sum the product of the two numbers
    for (_, [num_1, num_2]) in matches.map(|captures| captures.extract()) {
        total_sum += num_1.parse::<i32>().unwrap() * num_2.parse::<i32>().unwrap();
    }
    
    // Print the total sum
    println!("Total sum of mul(): {:?}", total_sum);
}
