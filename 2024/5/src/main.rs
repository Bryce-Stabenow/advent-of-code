use std::collections::HashMap;

fn main() {
    const INPUT_STRING: &str = include_str!("../input.txt");
    let parts = INPUT_STRING.split("\n\n").collect::<Vec<&str>>();
    let mut total_of_middle_pages: usize = 0;

    // Get the rules and print orders from the input string
    let rules_map: HashMap<usize, Vec<usize>> = get_rules_map(parts[0]);
    let print_orders: Vec<Vec<usize>> = get_print_orders(parts[1]);

    // Loop over each order and check if it is valid
    for order in print_orders {
        let mut is_valid: bool = true;

        for page in &order {
            // Check each page to make sure it is not violating the rule
            match rules_map.get(&page) {
                Some(rules) => {
                    // For each rule, check if the order is following the rules
                    match is_order_valid_for_page(page, &order, rules) {
                        true => (),
                        false => {
                            is_valid = false;
                            break;
                        }
                    }
                }
                None => continue, // If the page has no rules, it is valid
            }
        }

        // If the order is valid, print the middle page value
        if is_valid {
            total_of_middle_pages += get_middle_page_value(&order);
        }
    }

    println!("Total of middle pages: {}", total_of_middle_pages);
}

fn get_rules_map(input_string: &str) -> HashMap<usize, Vec<usize>> {
    // Map rules and print orders to vectors of integers from the original strings
    let rules: Vec<Vec<usize>> = input_string
        .split("\n")
        .map(|r| {
            r.split("|")
                .map(|n: &str| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    // To get the best list of rules, we'll index the rules by the first number in the rule
    // and set the dependant pages as a vector for each index
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for rule in rules.iter() {
        match rules_map.contains_key(&rule[0]) {
            true => rules_map
                .get_mut(&rule[0])
                .unwrap()
                .extend(rule[1..].to_vec()),
            false => {
                rules_map.insert(rule[0], rule[1..].to_vec());
                ()
            }
        }
    }

    rules_map
}

fn get_print_orders(input_string: &str) -> Vec<Vec<usize>> {
    input_string
        .split("\n")
        .map(|o| {
            o.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn is_order_valid_for_page(page: &usize, order: &Vec<usize>, rules: &Vec<usize>) -> bool {
    // Check if the order is valid for the page
    let idx_of_page_number = order.iter().position(|n| n == page).unwrap();
    println!("\n\nPage: {}, Order: {:?}, Rules: {:?}", page, order, rules);

    for rule in rules {
        let idx_of_rule = order.iter().position(|n| n == rule);

        println!(
            "Rule: {}, Index of Rule: {:?}, Index of Page: {}",
            rule, idx_of_rule, idx_of_page_number
        );

        match idx_of_rule {
            Some(idx) => {
                if idx_of_page_number > idx {
                    return false;
                }
            }
            None => (), // If the other page is not in the print order, it is valid
        }
    }

    true
}

fn get_middle_page_value(page_order: &Vec<usize>) -> usize {
    let middle: usize = page_order.len() / 2;
    page_order[middle]
}
