use std::io;
use std::cmp;
use rand::Rng;
use std::time::SystemTime;

fn main() {
    main_screen();
}

// This is so we can get it later and loop as many times as we want.
fn main_screen() {
    println!("Enter 0 to run the hardcoded tests against brute force algorithm.");
    println!("Enter 1 to run the hardcoded tests against kadane's algorithm.");
    println!("Enter 2 to benchmark both.");
    println!("Enter any other value or press enter to exit.");
    // Based on user input choose which route to go.
    let mut chosen_option = String::new();
    io::stdin()
        .read_line(&mut chosen_option)
        .expect("Failed to read from std::in.");
    match chosen_option.trim() {
        "0" => hard_coded_brute_force(),
        "1" => hard_coded_kadane(),
        "2" => benchmark_functions(),
        _  => return,
    };
    continue_program();
}

fn benchmark_functions() {
    println!("Enter 0 to enter a custom array.\nEnter any number to test an array of that length, or press return for defaults.");
    println!("Defaults are [1, 5, 10, 100, 250, 500, 1000, 2500, 5000]");
    let mut benchmark_count = String::new();
    io::stdin()
        .read_line(&mut benchmark_count)
        .expect("Failed to read from std::in.");
    // Remove the newline character from the string before converting to i32
    benchmark_count.pop();
    // If the input was 0, then allow the user to input their own array.
    // if the length of the string is zero, then it was just a newline character.
    // else, push that to our vector.
    let trials = 1;
    let default_benchmarks = vec![1, 5, 10, 100, 250, 500, 1000, 2500, 5000, 10000, 25000, 50000, 100000, 100000000];
    let mut benchmarks: Vec<i32> = Vec::new();
    let mut user_array: Vec<i32> = Vec::new();
    if benchmark_count == "0" {
        // run default benchmark with custom array
        println!("Enter your custom array to test against with values separated by spaces");
        let mut user_provided_array = String::new();
        io::stdin()
            .read_line(&mut user_provided_array)
            .expect("Failed to read user array from std::in.");
        for value in user_provided_array.split_whitespace() {
            user_array.push(value.parse::<i32>().expect("Failed to parse user array"));
        }
        benchmarks.push(user_array.len() as i32);
    } else if benchmark_count.len() == 0 {
        // run default benchmarks with random array
        println!("Preparing to run with default benchmark numbers");
        benchmarks.extend(default_benchmarks);
    } else {
        benchmarks.push(benchmark_count.parse().unwrap());
        println!("Size of array to benchmark against: {}", benchmark_count);
    }

    // println!("\nLargest subarray sum found by both algorithms:");
    // println!("n\tForced\tKadane");
    // Each 2d array holds all trial counts for all benchmark lengths
    let mut forced_benchmarks: Vec<Vec<std::time::Duration>>
        = vec![Vec::new(); benchmarks.len()];
    let mut kadane_benchmarks: Vec<Vec<std::time::Duration>>
        = vec![Vec::new(); benchmarks.len()];
    let mut random = rand::thread_rng();
    // Run all of the benchmarks
    for (i, bench) in benchmarks.iter().enumerate() {
        for trial in 0..trials {
            let mut list: Vec<i32> = Vec::new();
            // If the user did not provide an array
            if calculate_length(&user_array) == 0 {
                for _ in 0..*bench {
                    list.push(random.gen_range(-5, 6));
                }
            } else {
                // If the user did provide an array add that to our list.
                list.extend(&user_array);
            }

            // Using a high definition clock SystemTime, please read more into the
            // docs to learn more about it
            let sys_time = SystemTime::now();
            let forced_sum = 0;// brute_force(&list);
            forced_benchmarks[i].push(sys_time.elapsed().unwrap());
            let sys_time = SystemTime::now();
            let kadane_sum = kadane(&list);
            let kadane_bench = sys_time.elapsed().unwrap();
            kadane_benchmarks[i].push(kadane_bench);
            println!("Kadane pushed: {}",
                     kadane_bench.as_secs() as f64
                         + kadane_bench.subsec_nanos() as f64 * 1e-9);
            println!("{}:  \t{}\t{}",
                     bench, forced_sum, kadane_sum);
            println!("Running Trial: #{} for n: {}", trial+1, bench);
        }
    }
    println!("\nAverage time both algorithms (in secs) after {} trials:", trials);
    println!("n\tForced\t\tKadane");
    for (i, bench) in benchmarks.iter().enumerate() {
        let forced_sum: f64 = forced_benchmarks[i].iter().map(
            |&x| x.as_secs() as f64 + x.subsec_nanos() as f64 * 1e-9
        ).sum();
        let kadane_sum: f64 = kadane_benchmarks[i].iter().map(
            |&x| x.as_secs() as f64 + x.subsec_nanos() as f64 * 1e-9
        ).sum();
        println!("{}:\t{:.9}\t{:.9}", bench,
                 forced_sum / trials as f64,
                 kadane_sum / trials as f64);
    }
}

// Get the length of a vector via pointer
fn calculate_length(s: &Vec<i32>) -> usize {
    s.len()
}

// Run the hard coded version of brute force
fn hard_coded_brute_force() {
    let an_array = vec![1, -1, 2, 4, 1];
    let highest_sum = brute_force(&an_array);
    println!("Highest sum using brute force: {}.", highest_sum);
}

// Run the hard coded version of kadane's algorithm
fn hard_coded_kadane() {
    let list = vec![1, -1, 2, 4, 1];
    let largest_sum = kadane(&list);
    println!("Largest sum using kadane: {}.", largest_sum);
}

// Ask if the user would like to continue after each operation
fn continue_program() {
    let mut read_continue = String::new();
    println!("Would you like to continue? (y/N)");
    io::stdin()
        .read_line(&mut read_continue)
        .expect("Failed to read from std::in.");

    match read_continue.trim() {
        "y" | "Y" => main_screen(),
        _ => return
    };
}

// Function to brute force the maximum value
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
