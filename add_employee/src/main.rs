/*
 * @file main.rs
 * @brief This program creates a text interface to let a user add employee
 *        names to a department in a company.
 * @author Ruturaj A. Nanoti
 * @version 0.1.0
 * @date 2022-06-30
 */

// Use a HashMap that has the name of the department as the key, and
// the value would be a vector that stores the names of the employees in
// that department.

#![warn(dead_code)]
#[allow(unused)]
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // Create the hash map named company.
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("Enter the name and the department to which the Employee needs to be added [q to quit] : ");
        let _ = io::stdout().flush();
        let mut user_input = String::new();

        // Read the input from the user.
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to get the command!");

        // Remove '\r' and '\n'.
        user_input.pop();
        user_input.pop();

        if user_input == "q".to_string() {
            break;
        } else {
            // Print out the command.
            println!("You entered : {}", user_input);
            // Convert the user input into a vector of strings, and use the 2nd
            // and 4th word in it, which specifies name of the employee and department.
            let words_arr: Vec<&str> = user_input.split(" ").collect();

            if words_arr.len() == 4 {
                let emp_name: String = words_arr[1].to_string();
                let emp_dep: String = words_arr[3].to_string();

                // Insert the employee name into the department, if the department does
                // not exist, create it and then add the employee name to it.
                company.entry(emp_dep).or_insert(Vec::new()).push(emp_name);
            } else {
                break;
            }
        }
    }
    
    // Get the input for the user for retrieving the company or department list.
    print!("Details for [c]ompany or [d]epartment : ");
    let _ = io::stdout().flush();
    let mut details_input = String::new();

    io::stdin()
        .read_line(&mut details_input)
        .expect("Failed to get command!");

    // Remove '\r' and '\n'.
    details_input.pop();
    details_input.pop();

    if details_input == "c".to_string() {
        for (key_val, mut val) in company {
            val.sort();
            println!("People in {} department are : {:#?}", key_val, val);
        }
    } else if details_input == "d".to_string() {
        
        print!("Enter the name of the department : ");
        let _ = io::stdout().flush();
        let mut dep_name = String::new();

        io::stdin()
            .read_line(&mut dep_name)
            .expect("Failed to get command!");

        // Remove '\r' and '\n'.
        dep_name.pop();
        dep_name.pop();
        
        let dep_emp_list = company.get_mut(&dep_name);

        if let Some(list) = dep_emp_list {
    
            list.sort();
            println!("People in {} department are : {:#?}", dep_name, list);
        } 
        else {
            println!("No department named : {}", dep_name);
        };
    } else {
        println!("Not a valid entry!!");
    }
}
