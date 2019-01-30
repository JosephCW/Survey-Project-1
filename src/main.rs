use std::io;
use std::cmp;

fn main() {
    main_screen();
}

// This is so we can get it later and loop as many times as we want.
fn main_screen() {
    println!("Enter 0 to run the hardcoded tests against brute force algorithm.");
    println!("Enter 1 to run the hardcoded tests against kadane's algorithm.");
    println!("Enter 2 to enter your own array of numbers to run against both.");
    println!("Enter any other value to exit.");
    let mut chosen_option = String::new();
    io::stdin()
        .read_line(&mut chosen_option)
        .expect("Failed to read from std::in.");
    match chosen_option.trim().parse::<i32>() {
        Ok(i) => {
            println!("Value of i chosen: {}.", i);
            match i {
                0 => {
                    hard_coded_brute_force();
                },
                1 => {
                    hard_coded_kadane();
                }
                2 => {
                    hard_coded_brute_force();
                    hard_coded_kadane();
                    manual_input_array();
                }
                 _ => return,
            }
            continue_program();
        }
        Err(_) => {
            println!("Encountered an error while parsing what you read. Restarting the program.");
            main_screen();
        }
    };
}

fn hard_coded_brute_force() {
    let an_array = vec![1, -1, 2, 4, 1];
    let highest_sum = brute_force(&an_array);
    println!("Highest sum using brute force: {}.", highest_sum);
}

fn hard_coded_kadane() {
    let list = vec![1, -1, 2, 4, 1];
    let largest_sum = kadane(&list);
    println!("Largest sum using kadane: {}.", largest_sum);
}

fn manual_input_array() {
    println!("Doo da doo. Manual array input");
}

fn continue_program() {
    let mut read_continue = String::new();
    println!("Would you like to continue? 1 for yes, 0 for no.");
    io::stdin()
        .read_line(&mut read_continue)
        .expect("Failed to read from std::in.");

    match read_continue.trim().parse::<i32>() {
        Ok(i) => {
            if i == 0 {
                return;
            }
            main_screen();
        }
        Err(_) => {
            println!("Encountered an error while parsing what you read. Restarting the program.");
            main_screen();
        }
    };
}

fn brute_force(array: &Vec<i32>) -> i32 {
    let number_of_elements = array.len();
    let mut max_sum = array[0];
    // from zero to the last element
    for i in 0..=number_of_elements - 1 {
        // from current to last element
        for j in i..=number_of_elements - 1 {
            // sum up the values between i and j
            // set max sum if it is largest.
            let mut total = 0;
            for k in i..=j {
                total += array[k];
            }
            if max_sum < total {
                max_sum = total;
            }
        }
    }
    return max_sum;
}

// Please read more into Kadane's algorithm, but basically, it allows use
// to pass through the list once and keeps track of the "running maximum".
fn kadane(nums: &Vec<i32>) -> i32 {
    let (mut current_max, mut largest_subset) = (nums[0], nums[0]);
    for x in &nums[1..] {
        // keeps track of the largest subset up to that run of x, note that
        // x here must be dereferenced because nums is passed around as a pointer
        largest_subset = cmp::max(*x, largest_subset + x);
        // maximum value is always kept up to date as x advances through the list
        current_max = cmp::max(current_max, largest_subset);
    }
    return current_max;
}
