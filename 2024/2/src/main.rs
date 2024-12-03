fn main() {
    let (safe_readings, unsafe_readings) = calculate_total_unsafe_readings();
    println!("Total unsafe readings: {}\nTotal safe readings: {}", unsafe_readings, safe_readings);
}

fn calculate_total_unsafe_readings() -> (i32, i32) {
    const INPUT_STRING: &str = include_str!("../input.txt"); // Pull in input file as string
    let lines = INPUT_STRING.lines();
   
    // Convert all lines to a vector of vectors of i32 
    let mapped_lines = lines.map(|line: &str| {
        line.split_whitespace().map(|reading| reading.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    });
    
    let mut unsafe_readings = 0;
    let mut safe_readings = 0;
    
    for readings in mapped_lines {
        let mut diffs: Vec<i32> = vec![];
        let mut is_unsafe = false;
        
        // Check for unsafe differences between readings
        for i in 0..readings.len() - 1 {
            let diff: i32 = readings[i] - readings[i+1]; // Calculate difference between readings
            
            if diff == 0 || diff > 3 || diff < -3 {
                is_unsafe = true;
                break;
            }
            
            diffs.push(diff);
        }
        
        // Check for all increasing or decreasing readings
        if is_unsafe == false {
            let mut increasing = false;
            let mut decreasing = false;
            
            for diff in diffs.clone() {
                match diff < 0 {
                    true => decreasing = true,
                    false => increasing = true,
                }
                
                if increasing == true && decreasing == true {
                    is_unsafe = true;
                    break;
                }
            }
        }
        
        match is_unsafe {
            true => unsafe_readings += 1,
            false => safe_readings += 1
        }
    }
    
    (unsafe_readings, safe_readings)
}
