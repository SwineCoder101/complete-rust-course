fn main() {
    // let x: i8 = 10;
    // println!("{}", x);

    // let _y: u8 = 10;

    // let decimal = 02_55;
    // let hex = 0xff;
    // let octal = 0o377;
    // let binary = 0b1111_1111;

    // println!("{}",decimal);
    // println!("{}",hex);
    // println!("{}",octal);
    // println!("{}",binary);

    // let byte = b'A';
    // println!("{}",byte);

    let x = 2.0; //f64 default
    let y: f32 = 1.0;

    let t = true;
    let f:  bool = false;

    let c = 'c';

    println!("{}",c);

    // + - * / %

    // let a = 10;
    // let b = 4;

    // let remainder = a % b;
    // println!("{}",remainder)

    // let tup = (500, "hi", true);
    // let (x, y, z) = tup;

    // println!("{}", x);
    // println!("{}", y);
    // println!("{}", z);

    // let array = [1,2,3];

    // println!("{}", array[0]);

    // let mut array2: [i32; 3] = [4,5,6];
    // println!("{}", array2[0]);
    // array2[0] = 10;
    // println!("{}", array2[3]);

    // let mut nums = vec![1,2,3];

    // nums.push(4);
    // println!("{:?}", nums);
    // nums.pop();
    // println!("{:?}", nums);

    // let mut vec = Vec::new(); //vec!
    // vec.push("a");
    // vec.push("b");
    // vec.push("c");
    // vec.push("d");
    // vec.pop_if(|index| index.to_ascii_lowercase().starts_with('t'));
    // println!("{:?}", vec);

    // vec.reverse();
    // println!("{:?}", vec);

    // println!("{}", vec.concat());

    // vec.reverse();
    // println!("{:?}", vec);



    // let mut vect = Vec::<i32>::with_capacity(2);
    // println!("{}", vect.capacity());

    // let mut v: Vec<i32> = (0..5).collect();
    // println!("{:?}", v);

    // let sv = &v[2..4];

    // println!("{:?}", sv);

    // let name = String::from("Tyler");
    // let course = "Rust".to_string();
    // let new_name = name.replace("Tyler", "Ty");

    // println!("{}", name);
    // println!("{}", course);
    // println!("{}", new_name);

    // // &str = "string slice" or "stir"
    // let str1 = "hello"; //&str
    // let str2 = str1.to_string();
    // let str3 = &str2;

    // println!("{}", str1.ends_with("o"));
    // println!("{}", str2.to_ascii_lowercase());
    // println!("{}", str3.to_ascii_uppercase());

    // // compare string == != (does not equal)
    // println!("{}", "ONE".to_lowercase() == "one");

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
