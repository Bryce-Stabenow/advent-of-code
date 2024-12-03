fn main() {
    calculate_total_unsafe_readings();
}

fn calculate_total_unsafe_readings(){
    const INPUT_STRING: &str = include_str!("../input.txt"); // Pull in input file as string
    let lines = INPUT_STRING.lines();
   
    // Convert all lines to a vector of vectors of i32 
    let mapped_lines = lines.map(|line: &str| {
        line.split_whitespace().map(|reading| reading.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    });
    
    let mut unsafe_readings = 0;
    for readings in mapped_lines {
        println!("Readings: {:?}", readings);
        
        // Check for all increasing or decreasing readings
        // TODO
        
        // Check for unsafe differences between readings
        for i in 0..readings.len() - 1 {
            let first: i32 = readings[i];
            let next: i32 = readings[i+1];
            let diff: i32 = (first - next).abs(); // Calculate difference between readings
            
            println!("First: {}, Next: {}, Diff: {}", first, next, diff);
            
            if diff > 3 || diff == 0 {
                unsafe_readings += 1;
                break;
            }
        }
    }
    
    println!("Total unsafe readings: {}", unsafe_readings);
}
