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
    let mut chosen_option = String::new();
    io::stdin()
        .read_line(&mut chosen_option)
        .expect("Failed to read from std::in.");
    // println!("Value of i chosen: {}.", chosen_option.trim());
    match chosen_option.trim() {
        "0" => hard_coded_brute_force(),
        "1" => hard_coded_kadane(),
        "2" => benchmark_functions(),
        _  => return,
    };
    continue_program();
}

fn benchmark_functions() {
    println!("Enter the number of items to benchmark against, or enter -1 for defaults");
    println!("Defaults are [5, 10, 100, 250, 500, 1000, 2500, 5000]");
    let mut benchmark_count = String::new();
    io::stdin()
        .read_line(&mut benchmark_count)
        .expect("Failed to read from std::in.");
    println!("Size of array to benchmark against: {}", benchmark_count);
    let mut benchmarks: Vec<i32> = Vec::new(); 
    match benchmark_count.trim() {
        "-1" => {
            benchmarks.push(5); 
            benchmarks.push(10);
            benchmarks.push(100);
            benchmarks.push(250);
            benchmarks.push(500);
            benchmarks.push(1000);
            benchmarks.push(2500);
            benchmarks.push(5000);
        },
        _ => {
            // Have to call a string.pop() to remove the newline character
            benchmark_count.pop();
            // we then parse it and conver it to an i32
            benchmarks.push(benchmark_count.parse().unwrap());
        }, 
    };

    println!("\nLargest subarray sum found by both algorithms:");
    println!("n\tForced\tKadane");
    let mut forced_benchmarks: Vec<std::time::Duration> = Vec::new();
    let mut kadane_benchmarks: Vec<std::time::Duration> = Vec::new();
    let mut random = rand::thread_rng();
    for bench in &benchmarks {
        let mut list: Vec<i32> = Vec::new();
        for _ in 0..*bench{
            list.push(random.gen_range(-5, 5));
        }
        let sys_time = SystemTime::now();
        let forced_sum = brute_force(&list);
        forced_benchmarks.push(sys_time.elapsed().unwrap());
        let sys_time = SystemTime::now();
        let kadane_sum = kadane(&list);
        kadane_benchmarks.push(sys_time.elapsed().unwrap());
        println!("{}:  \t{}\t{}",
                 bench, forced_sum, kadane_sum);
    }
    println!("\nTiming both algorithms (in secs):");
    println!("n\tForced\t\tKadane");
    for (i, bench) in benchmarks.iter().enumerate() {
        println!("{}:\t{:.9}\t{:.9}", bench,
                 forced_benchmarks[i].as_secs() as f64
                 + forced_benchmarks[i].subsec_nanos() as f64 * 1e-9,
                 kadane_benchmarks[i].as_secs() as f64
                 + kadane_benchmarks[i].subsec_nanos() as f64 * 1e-9);
    }
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
