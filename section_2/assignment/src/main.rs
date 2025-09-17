/*
Create three variables with the names: val1, val2, and ans. We want to perform a simple operation of generating 
the modulo of val1 and val2. Set val1 to 5 and val2 to 2. Assign the answer to the ans variable. Before executing 
your code, what do you think the answer will be?

Create a vector and put in the values "2, 4, 6, 8, 10". Once you have created the vector perform the 
following: print out the current values, remove the value 10, add the value 12, and then print the vector back out to 
confirm your results.

Create a function called "concat_string". Create a string variable and assign the value "Hello" to it. 
The function is going to take one argument that is of type string and is going to return a String. 
Inside this function, concatenate the string " World". Print out the results in main() to confirm your results.

Create a function called control_flow. This is going to take one argument that is an integer. 
Based on this integer, print out the following: "The value is one", "The value is greater than 50", 
"The value is less than 25", or "The value is greater than 25 but less than 50".
*/

fn main() {
    let val1 = 2;
    let val2 = 5;
    let ans = val2 % val1;
    println!("ans: {}", ans);

    let mut vec = Vec::new();
    vec.extend([2, 4, 6, 8, 10]);
    vec.pop();
    vec.push(12);

    println!("{:?}", vec);

    println!("{}", concat_string("Liam, you will be come a great rust developer!"));

    println!("{}", control_flow(1));
    println!("{}", control_flow(51));
    println!("{}", control_flow(24));
    println!("{}", control_flow(29));
    
}

fn control_flow(i: i32) -> String {
    if i == 1 {
        String::from("The value is one")
    } else if i > 50 {
        String::from("The value is greater than 50")
    } else if i < 25 {
        String::from("The value is less than 25")
    } else {
        String::from("The value is greater than 25 but less than 50")
    }
}

fn concat_string(in_str: &str) -> String {
    let mut out_str = String::from("Hello ");
    out_str.push_str(in_str);
    out_str
}