use core::num;

fn main() {
    // data_types_exercises();
    
    // let output = sample_with_output(2, 3);
    // println!("{}", output);

    // // Read user input
    // let mut n: String = String::new();
    // std::io::stdin()
    // .read_line(&mut n)
    // .expect("failed to read input");

    // let n: f64 = n.trim().parse().expect("Error while parsing");
    // println!("the number you entered is: {}", n);


    // RUST OWNERSHIP
    //  1. Each value in Rust has a variable, which is called it's owner
    //  2. There can only be 1 owner at a time
    //  3. Once the owner goes out of scope, the value will be dropped. 

    let x: f64 = 32.5;
    let y: f64 = x; // RUST creates a copy of x in y. Both are stored in different memmory locations

    println!("x: {}, y: {}", x, y);

    // Primitive data types are stored in stack and non-primitive types are stored in Heap

    let s1: String = String::from("Hey");
    // let s2: String = s1; // RUST doesn't create a copy rather points s2 to s1's memory locations and s1 gets idle and removed. Ownership of memory location of s1 changed from s1 to s2.  

    // println!("S1: {}, S2: {}", s1, s2); // value borrowed here after move

    // EXPLAINATION 

//     let s1: String = String::from("Hey");
//    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 28 |     let s2: String = s1;
//    |                      -- value moved here
// 29 |
// 30 |     println!("S1: {}, S2: {}", s1, s2);
//    |                                ^^ value borrowed here after move

    let s2: &String = &s1;  // Now s2 contains a reference of s1 and doesn't actually move the ownership
    println!("S1: {}, S2: {}", s1, s2);

    let s2: String = s1.clone(); // creates a copy of s1 and assigns it to s2
    println!("S1: {}, S2: {}", s1, s2);

    let mut vec: Vec<i32> = vec![4,5,6];
    // let mut ref1: &Vec<i32> = &vec; //  ^^^^ `ref1` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    let ref1: &mut Vec<i32> = &mut vec;

    ref1.push(20);
    // vec.push(10); // In Rust, you can either have many immutable references, or one mutable reference

    // println!("{:?} {:?}", vec, ref1);
    println!("{:?}", ref1);


    // let v1: Vec<i32> = vec![2,3];
    // v1.push(19); // cannot borrow as mutable
    // let ref1: &mut Vec<i32> = &mut v1; // cannot borrow as mutable

    let mut var: Vec<i32> = vec![1,2,3];
    // test(&mut var); // cannot borrow as mutable
    test(&mut var);
    println!("outside funciton scope: {:?}", var);


    // REFERENCE RULES 
    //  1. One mutable reference in a scope 
    //  2. Many immutable references 
    //  3. Mutable and immutable references cannot coexist
    //  4. Scope of a reference 
    //  5. Data should not change when immutable references are in scope 

    let mut vec: Vec<i32> = vec![12,3,4];
    let ref1: &Vec<i32> = &vec;
    let ref2: &Vec<i32> = &vec;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    let ref3: &mut Vec<i32> = &mut vec; // works here because the scope of immutable references started when they were defined and ended when they were used. 
    ref3.push(10);
    println!("ref3: {:?}", ref3);


    // Dereferencing 
    let mut some_number: i32 = 42;
    let ref1: &mut i32 = &mut some_number;
    let another_number: i32 = *ref1;

    *ref1 = 13;
    println!("some_number: {}, another_number: {}", some_number, another_number);


    let mut vec: Vec<i32> = vec![1,2,3];
    let ref1 = &mut vec;
    // let another_vec: Vec<i32> = *ref1; // move occurs because `*ref1` has type `Vec<i32>`, which does not implement the `Copy` trait
    // vec.push(10); // ^^^ second mutable borrow occurs here
    let another_vec = ref1.clone();
    println!("ref1: {:?}", ref1);

    let ref2 = ref1; // ---- move occurs because `ref1` has type `&mut Vec<i32>`, which does not implement the `Copy` trait. Value moved here. 
    // let ref3 = ref1; //  value used here after move

    let a = 12; 
    if a > 10 {
        println!("Number is greater than 10");
    } else {
        println!("Number is less than 10");
    }

    // & - used for reference 
    // && - AND operator 

    let some_number = 9; 
    match some_number {
        1 | 2 => println!("Number is 1"),
        3..=9 => println!("Number is between 3 to 9"),
        10  => println!("Number is 10"),
        _ => println!("Number is something else")
    }


    // Loops 

    // let some_number: u8 = 10; 
    // let mut guess = false; 

    // println!("Please guess a number:");

    // while guess != true {
    //     let mut nummber: String = String::new();
    //     std::io::stdin()
    //     .read_line(&mut nummber)
    //     .expect("Failed to read number");

    //     let number: u8 = nummber.trim().parse().expect("Failed to parse number");

    //     if number == some_number {
    //         println!("You guessed correctly");
    //         guess = true;
    //     } else {
    //         println!("Incorrect guess. Please try again!");
    //     }
    // }

    let mut some_vec = vec![43,23, 23, 1234, 41];

    // for index in 0..5 {
    //     println!("{}", some_vec[index]);
    // }

    // for i in some_vec { // value of some_vec is moved to i here. 
    //     println!("{}", i);
    // }

    // println!("{:?}", some_vec); // borrow of moved value because value of some_vec is moved to i 


    // To avoid moving value, we can use the some_vec.iter() or &some_vec which doesnt' move ownership. For mutable we can use some_vec.iter_mut() or &mut some_vec

    for i in some_vec.iter() {
        println!("{}", i);
    }

    for i in some_vec.iter_mut() {
        *i += 5;
        println!("{}", i);
    }

    println!("{:?}", some_vec);

}

fn test(var: &mut Vec<i32>){
    var.push(10);
    println!("inside funciton scope: {:?}", var);
}

// fn sample_with_output(num1: i32, num2: i32) -> i32 {
//     num1 * num2 
// }


// fn data_types_exercises(){
//     println!("The value is {}", 10);

//     let x: u8 = 15;

//     println!("The max value of i8 is : {}", std::i8::MAX);
//     println!("The max value of u8 is : {}", std::u8::MAX);
//     println!("The max value of f32 is : {}", std::f32::MAX);

//     let y: i8 = -5;
//     let z: f32 = 4.65;
//     let status: bool = true;
//     let a: char = 't';

//     println!("The values of our varaibles are {:?}", (x,y,z,status, a));

//     let (a_number, b_number) = (5, 5.5);
//     println!("{:?}", (a_number, b_number));

//     let b: u8 = 255;
//     println!("The value of variable in octal is {:o}, in hexadecimal is {:X} and in binary is {:b}", b,b,b);

//     let b = 11; // Can declare the value twice = shadowing
//     // b = 12; // cannot assign twice to immutable variable
//     println!("The value of b is {}", b);

//     const X_TEST: u32 = 6;
//     println!("The value of X_TEST is {}", X_TEST);


//     let test_string: &str = "This is a string";
//     // test_string = "test"; // cannot assign twice to immutable variable

//     let mut test_string: &str = "This is a string";
//     test_string = "test";

//     // test_string.push('s'); // method not found in `&str`

//     println!("{}", test_string);

//     let another_string: String = String::from("This is a growing string");
//     // another_string = String::from("test1"); // cannot assign twice to immutable variable

//     let mut another_string: String = String::from("This is a growing string");
//     another_string = String::from("test1");
    
//     another_string.push('s');
//     println!("{}", another_string);
//     another_string.pop();
//     println!("{}", another_string);

//     another_string.push_str("+test");
//     println!("{}", another_string);

//     println!("Is Empty: {}", another_string.is_empty());
//     println!("Length: {}", another_string.len());
//     println!("Bytes: {}", another_string.capacity());
//     println!("Contains another string: {}", another_string.contains("use"));

//     let concat_string: String = format!("{}{}", test_string, another_string);
//     println!("{}", concat_string);

//     // TUPLE - can contain elements of different types

//     let my_info: (&str, i32) = ("Salary", 1000);
//     println!("This is how we read first element of Tuple {} and second element {}", my_info.0, my_info.1);
//     println!("Another way to pirnt it is {:?}", my_info);

//     // Tuple destructuring
//     let (salary, number) = my_info;
//     println!("Salary is {} and number is {}", salary, number);

//     //nested tuple
//     let nested_tuple = (1,2.09, (3, 4.2), "Jelo");
//     println!("Thjis is how we fetahc nested tuple value: {}", nested_tuple.2.0);

//     // empty tuple
//     let empty_tuple: () = ();

//     // ARRAYS - can contain elements of the same type, lenght must be known at compile time 
//     // notice the semi-colon during declaration instead of comma
    
//     let mut number_array: [i32;5] = [1,2,3,4,5];
//     println!("{}", number_array[0]);
//     println!("{:?}", number_array);

//     number_array[4] = 2;
//     println!("{:?}", number_array);

//     // number_array.push(3); // method not found - arrays sizes cannot be updated during runtime
//     // println!("{:?}", number_array);

//     let array_with_same_elements: [i32;10] = [0; 10];
//     println!("{:?}", array_with_same_elements);

//     let subset_array: &[i32] = &number_array[0..3];
//     println!("{:?}", subset_array);

//     let subset_array = &number_array[0..=3];
//     println!("{:?}", subset_array);

//     println!("length: {}", number_array.len());
//     println!("Number of bytes: {}", std::mem::size_of_val(&number_array));

//     let test: Option<&i32> = number_array.get(100);
//     // println!("{}", test); //Option<&i32> cannot be formatted with the default formatter
//     println!("{:?}", test);

//     let test: Option<&i32> = number_array.get(2);
//     println!("{:?}", test);


//     // VECTORS - resizable, rest is very similar to arrays

//     let mut number_vector: Vec<i32> = vec![12,3,34,5,6,31,2];
//     println!("{}", number_vector[2]);
//     println!("{:?}", number_vector);

//     let subset_array: &[i32] = &number_vector[0..3];
//     println!("{:?}", subset_array);

//     number_vector.push(30);
//     println!("{:?}", number_vector);

//     number_vector.remove(2);
//     println!("{:?}", number_vector);

//     // println!("The vector contains a number 10 or not: {}", number_vector.contains(10)); //expected `&i32`, found integer
//     println!("The vector contains a number 10 or not: {}", number_vector.contains(&10));
// }


//tets21