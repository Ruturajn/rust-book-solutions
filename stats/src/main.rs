/*
 * @file main.rs
 * @brief This program calculates the median and mode for a given list of
 *        integers using vectors, and hash maps.
 * @author Ruturaj A. Nanoti
 * @version 0.1.0
 * @date 2022-06-29
 */

#![warn(dead_code)]
#[allow(unused)]

use std::collections::HashMap;

fn main() {
    // => Calculation of Median
    // Define the integers
    let mut int_arr = vec![10, 12 , 13, 14, 14, 14, 15, 16, 18, 18, 19, 20];
    
    // Sort the vector.
    // int_arr.sort();
    bubble_sort(&mut int_arr);

    // Print Sorted vector
    println!("Sorted List : ");
    println!("{:?}", int_arr);

    // Calculate the position in the middle
    if int_arr.len() % 2 == 0 {
        let ind_1 = int_arr.len() / 2;
        let ind_2 = int_arr.len() / 2 + 1;
        println!("The median is : {}", (&int_arr[ind_1] + &int_arr[ind_2]) / 2)
    } else {
        let ind_1 = int_arr.len() / 2 + 1;
        println!("The median is : {}", &int_arr[ind_1])
    }

    // Calculation of Mode
    let mut count_map = HashMap::new();

    for item in &int_arr {
        let count = count_map.entry(item).or_insert(0);
        *count += 1;
    }

    // Print the count_map with values denoting number of times each key
    // appears in the list.
    println!("HashMap with frequency of each element : ");
    println!("{:?}", count_map);

    // Look for the key with the highest value.
    let mut max_val = 0;
    let mut key_max_val = 0;
    for (key, value) in &count_map {
        if max_val < *value {
            max_val = *value;
            key_max_val = **key;
        }
    }
    
    // Print the mode, i.e. the key with the highest value.
    println!("The mode is : {}", key_max_val);
}

fn bubble_sort(in_vec: &mut Vec<i32>) {
    for i in 0..in_vec.len() {
        for j in 0..in_vec.len() {
            if in_vec[i] < in_vec[j] {
                let temp = in_vec[i];
                in_vec[i] = in_vec[j];
                in_vec[j] = temp;
           }
        }
    }
}
