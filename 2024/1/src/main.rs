fn main(){
    calculate_total_distance();
    calculate_similarity_score();
}

fn generate_left_and_right_lists_from_input() -> (Vec<&'static str>, Vec<&'static str>) {
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
    
    (left, right)
}

// Part 1
fn calculate_total_distance() {
    let (left, right) = generate_left_and_right_lists_from_input();
    
    let total: i32 = left.into_iter().zip(right.into_iter())
    .map(|(l, r)| (l.parse::<i32>().unwrap() - r.parse::<i32>().unwrap()).abs())
    .sum();

    println!("Total Distance: {:?}", total);
}

// Part 2
fn calculate_similarity_score(){
    let mut similarity_score = 0;
    let (left, right) = generate_left_and_right_lists_from_input();
    
    for l in left.into_iter() {
        let mut score = 0;
        
        for r in right.clone().into_iter(){ // Clone is bad and I should feel bad for using it
            if l == r {
                score += 1;
            }
        }
        
        let score: i32 = score * l.parse::<i32>().unwrap();
        similarity_score += score;
    }
    
    println!("Total Similarity: {:?}", similarity_score);
}
