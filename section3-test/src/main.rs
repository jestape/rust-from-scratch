const X_VALUE: u32 = 10;

fn main() { 
   
    // Variables

    let mut x = X_VALUE;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowning concept: difference between shadowing and mut

    let x = X_VALUE;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    
    // Tuples

    let tup = (500_000, 3.14, 666);
    let (x,_y,_z) = tup;
    println!("The value of tup.x is: {}", x);

    let tup2: (i32, f64, u32) = (500_000, 3.14, 666);
    println!("The value of tup2.y is: {}", tup2.1);

    // Arrays
   
    let arr: [i32;3] = [1, 2, 3];
    println!("The value of arr[1] is: {}", arr[1]);

    // Functions

    another_function(2);
    println!("The value returned by the function is: {}", ma_function());

    // Conditionals
    
    let x = 5;
    if x > 5 {
        println!("Condition was true [{} > 5].", x);
    } else if x == 5 {
        println!("Condition was false [{} == 5].", x);
    } else {
        println!("Condition was false [{} < 5].", x);
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Loops
   
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Loops + Conditionals (While)
    
    let mut number = 6;
    while number != 0 {
        println!("{}!", number);
        number -= 2;
    }

    // Loops + Arrays (For)
    
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    // Range + for

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_function(x: u32) {
    println!("I'm function number {}.", x);
}

fn ma_function() -> u32 {
   312 + 2
}
