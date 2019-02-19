# Project 1: Maximum Subarray

## Installing Rust on Linux / MacOS

Just run this:
```
curl https://sh.rustup.rs -sSf | sh
```

## Installing Rust on Windows

Download the windows installer found [here](https://www.rust-lang.org/tools/install), and install with default configuration settings.

## Building and Running the Project
Once rust (with Cargo) is installed navigate to the base directory of the project.
Compile the program with:
```
cargo build --release
```
This will put the compiled executable in the ./target/release/ directory.
You can execute this program with the command .\target\release\project-1-rusty-bois.exe (on Windows) or simply type:
```
cargo run --release
```
Be sure to add the `--release` option, otherwise cargo will build and run the debug version, which is very slow.
Following the instructions from the running program to get the different desired outputs.

## 44-349: Survey of Algorithms

### The Problem

Given an array `A` of `n` numerical values (positive and negative) find the values `i` and `j` (`0 <= i <= j <= n-1`) such that the sum of the values from index `i` through index `j` is maximized.
In other words, you want to find `start` and `end` such that the total is the largest for a given array:

```python
total = 0
for i = start..end:
	total += A[i]
```

For example: if `A=[-2, 1, -3, 4, -1, 2, 1, -5, 4]` you sum the values from index `3` through index `6` (`[4, -1, 2, 1]`) the result is 6; no other subarray in the array will be larger.

This problem has applications in data mining, computer vision, and genomic sequence analysis.  For more information see https://en.wikipedia.org/wiki/Maximum_subarray_problem

### Your Task

You must implement (in a language of your choosing) two separate ways: using brute force and Kadane's algorithm.
After implementing these algorithms you are to time the execution of the algorithms and compare them.

#### Brute Force

The brute force algorithm is simple: generate all possible start/end index pairs, calculate the sum for each, and determine which returns the largest sum.
Essentially:

```python
# n is number of elements, array is 0 indexed, n-1 is max index
maxsum = A[0]
for i = 0..n-1:
	for j = i..n-1: # only start at i; the max could be a single value
		total = 0
		for k = i..j:
			total += A[k]
		if maxsum < total:
			maxsum = total
return maxsum
```

Depending on your use case it may be appropriate to return the start and end index instead of the max value; for our purposes returning the value is appropriate.

#### Kadane's Algorithm

See https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane%27s_algorithm for more information about this algorithm, including pseudocode.

### Testing Your Code

To demonstrate your code works you must write some tests; this can include using hand-generated examples to show you get the correct answer or showing that the simple brute-force algorithm gives you the same ansewr as Kadane's Algorithm (you should do both of these, at least).
Your code submission must contain a way to run your code and verify it works (perhaps unit tests?)

### Timing Your Code

When you have verified that your code works you must write code to time the execution of your algorithms.
Remember: when timing you should average the runtime over many iterations on the _same array size_.
You should be able to generate and report timings of your code for a variety of values for `n` (array size).

How you generate your arrays is up to you; remember that the arrays can contain both negative and positive values.  You may also want to use different arrays for the same array size; you should decide how the parameters of your timing.

It is unacceptable to ask the grader to modify/recompile your code to generate timings for various array sizes; you must either read in this parameter from a file, the keyboard, or the command line (runtime parameter) so it is easy to generate timings.
Alternately you may want to decide all values of `n` you will want to test, and output all timings in a table.
Example output is provided for both scenarios

When you write your report you will be expected to state (and possibly explain) the following:

* The number of iterations you averaged over (large is good!)
* How you generated your input data for timing
* Special considerations (are you comparing special cases? doing anything to your data?)

### Deliverables

Your submission must contain:

* Instructions for running your verification code
* Instructions for running your timing code for various values of `n`

### Sample I/O
```bash
# Output all timings for paper as a table
squishy@balmung ~/Dropbox/msub $ python3 ./time_maxsubarray.py
Number of iterations: 100
	5	10	100	250	500
brute	1.2044e-05	3.4805e-05	0.0047	0.0518	0.3468
kadane	2.4909e-06	4.2882e-06	4.0644e-05	0.0001	0.0002

#Output timings for a single n
squishy@balmung ~/Dropbox/msub $ python3 ./time_maxsubarray.py 10
Number of iterations: 100
Array size: 10
Brute force (s); 3.75412404537201e-05
Kadane (s): 4.855040460824967e-06

# Running tests
squishy@balmung ~/Dropbox/msub $ python3 ./max_subarray.py
Same for wiki problem: True
Same for random (size 15) True
Same for random (size 100) True
```
