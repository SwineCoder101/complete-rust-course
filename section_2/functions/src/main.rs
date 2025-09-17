fn main() {
    print_phase("Print my argument");
    println!("{}",gcd(20, 4));
    println!("{}", multiple_return_values(false));
}

fn print_phase(input: &str){
    println!("{}", input);
}

fn gcd(mut a: u8, mut b:u8) -> u8 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a %= b;
    }
    b
}

fn multiple_return_values (flag : bool) -> bool {
    if flag == true {
        true
    }
    else {
        false
    }
}