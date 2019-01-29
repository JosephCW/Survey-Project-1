fn main() {
    
    // Brute Force
    let an_array: [i32; 5] = [1, -1, 2, 4, 1];
    let highest_sum = brute_force(&an_array);
    println!("Highest sum in our array: {}", highest_sum);
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

