use std::io;

fn main() {
    main_screen();
}

// This is so we can get it later and loop as many times as we want.
fn main_screen() {
    println!("Enter 0 to run the hardcoded tests against brute force algorithm");
    println!("Enter 1 to run the hardcoded tests against kadane's algorithm");
    println!("Enter 2 to enter your own array of numbers to run against both");
    println!("Enter -1 to exit");
    let mut chosen_option = String::new();
    io::stdin()
        .read_line(&mut chosen_option)
        .expect("Failed to read from std::in.");
    match chosen_option.trim().parse::<i32>() {
        Ok(i) => {
            println!("Value of i chosen: {}", i);
            if i == 0 {
               hard_coded_brute_force();
               continue_program();
            } else if i == 1 {
                hard_coded_kadane();
                continue_program();
            } else if i == 2 {
                manual_input_array();
                continue_program();
            } else if i == -1 {
                return;
            } else {
                main_screen();
            }
        },
        Err(_) => {
            println!("Encountered an error while parsing what you read. Restarting the program.");
            main_screen();
        }
    };
}

fn hard_coded_brute_force() {
    let an_array: [i32; 5] = [1, -1, 2, 4, 1];
    let highest_sum = brute_force(&an_array);
    println!("Highest sum in our array: {}", highest_sum);
}

fn hard_coded_kadane() {
    println!("Doo da doo. Kadane's hard coded");
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
        },
        Err(_) => {
            println!("Encountered an error while parsing what you read. Restarting the program.");
            main_screen();
        }
    };
}

fn brute_force(array: &[i32]) -> i32 {
    let number_of_elements = array.len();
    let mut max_sum = array[0];
    // from zero to the last element
    for i in 0 ..= number_of_elements - 1{
        // from current to last element
        for j in i ..= number_of_elements - 1 {
            // sum up the values between i and j
            // set max sum if it is largest.
            let mut total = 0;
            for k in i ..= j {
                total += array[k];
            }
            if max_sum < total {
                max_sum = total;
            }
        }
    }
    return max_sum;
}

