
/* 
Create a function that takes one argument called val that is of type Vec with the values: 1,3,5,7. 
Inside of this function check the first value of the vector and see if it is equal to one. If the value is equal to one, 
then return true, otherwise return false. Next add the value 15 to the vector. Print out the vector to confirm your results.

Create a function called "add_two". This function is going to take one parameter that is i8 and add two to it. 
For the function, do you have to pass the value by reference in order for you to maintain ownership of it inside of main?
*/



fn main() {

    let mut vec = vec![1,3,5,7];
    let num = 5;
    let checked = check_vec(&vec);

    println!("{}", checked);

    vec.push(15);

    println!("{:?}", vec);
}

fn add_two(num: i32) -> i32 {
    num +2
}


fn check_vec(vec: & Vec<i32>) -> bool {
    vec[0] == 1
}