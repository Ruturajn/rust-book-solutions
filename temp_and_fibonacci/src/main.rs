/**
 * @file main.rs
 * @brief This program converts celcius to fahrenheit, and vice-versa. It also prints
 * the fibonacci series upto a particular number.
 * @author Ruturaj A. Nanoti
 * @version 0.1.0
 * @date 2022-06-12
 */


fn main() {
    //=================Temperature Conversion=========================
    // Formula for converting fahrenheit to celcius.
    // C = (F - 32) * (5/9) => C * (9/5) + 32 = F
    let temp: f32 = 30.0;

    // Converting fahrenheit to celcius
    let temp_c: f32 = (temp - 32.0) * (5.0/9.0);
    println!("{}°F = {}°C", temp, temp_c);

    // Converting celcius to fahrenheit
    let temp_f: f32 = (temp) * (9.0/5.0) + 32.0;
    println!("{}°C = {}°F", temp, temp_f);

    //=================Fibonacci Series=======================
    // Number of Fibonacci numbers to print
    const N: usize = 10;
    // An array of n elements to store the numbers, intialized to 0.
    let mut fib_arr = [0;N];
    // Iterating n times
    for i in 0..N {
        if i <= 1 {
            fib_arr[i] = i;
            println!("Fibonacci number {} is : {}", i+1, fib_arr[i]);
        } else {
        fib_arr[i] = fib_arr[i-2] + fib_arr[i-1];
        println!("Fibonacci number {} is : {}", i+1, fib_arr[i]); 
        }
    }
}
