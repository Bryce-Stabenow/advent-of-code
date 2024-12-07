fn main() {
    const INPUT_STRING: &str = include_str!("../input.txt");

    let lines: Vec<Vec<char>> = INPUT_STRING
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    let mut total_occurrences: i32 = 0;

    for line_idx in 0..lines.len() {
        let current_line: &Vec<char> = &lines[line_idx];

        for ch_idx in 0..current_line.len() {
            let mut current_char: char = current_line[ch_idx];

            if current_char != 'X' {
                continue;
            }

            println!("Found X at pos ({},{})", line_idx, ch_idx);

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    let m_line_to_check_idx = line_idx as i32 + i;
                    let m_char_to_check_idx = ch_idx as i32 + j;

                    if m_line_to_check_idx >= lines.len() as i32
                        || m_line_to_check_idx < 0
                        || m_char_to_check_idx >= current_line.len() as i32
                        || m_char_to_check_idx < 0
                    {
                        continue; // Out of bounds
                    }

                    let a_line_to_check_idx = m_line_to_check_idx as i32 + i;
                    let a_char_to_check_idx = m_char_to_check_idx as i32 + j;

                    if a_line_to_check_idx >= lines.len() as i32
                        || a_line_to_check_idx < 0
                        || a_char_to_check_idx >= current_line.len() as i32
                        || a_char_to_check_idx < 0
                    {
                        continue; // Out of bounds
                    }

                    let s_line_to_check_idx = a_line_to_check_idx as i32 + i;
                    let s_char_to_check_idx = a_char_to_check_idx as i32 + j;

                    if s_line_to_check_idx >= lines.len() as i32
                        || s_line_to_check_idx < 0
                        || s_char_to_check_idx >= current_line.len() as i32
                        || s_char_to_check_idx < 0
                    {
                        continue; // Out of bounds
                    }

                    let m_line_to_check = &lines[m_line_to_check_idx as usize];
                    let m_char_to_check: char = m_line_to_check[m_char_to_check_idx as usize];

                    let a_line_to_check = &lines[a_line_to_check_idx as usize];
                    let a_char_to_check: char = a_line_to_check[a_char_to_check_idx as usize];

                    let s_line_to_check = &lines[s_line_to_check_idx as usize];
                    let s_char_to_check: char = s_line_to_check[s_char_to_check_idx as usize];

                    if m_char_to_check != 'M' {
                        continue;
                    }

                    println!(
                        "Found M at pos ({},{})",
                        m_line_to_check_idx, m_char_to_check_idx
                    );

                    if a_char_to_check != 'A' {
                        continue;
                    }

                    println!(
                        "Found A at pos ({},{})",
                        a_line_to_check_idx, a_char_to_check_idx
                    );

                    if s_char_to_check != 'S' {
                        continue;
                    }

                    println!(
                        "Found S at pos ({},{})",
                        s_line_to_check_idx, s_char_to_check_idx
                    );

                    total_occurrences += 1;
                    println!("Found XMAS at pos ({},{})", line_idx, ch_idx);
                }
            }
        }
    }

    println!("Total XMAS occurrences: {}", total_occurrences);
}
