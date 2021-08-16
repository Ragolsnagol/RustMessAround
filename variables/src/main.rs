fn main() {
    let mut x = five();
    print_value_x(x);
    x = 6;
    print_value_x(x);

    // Using shadowing for variables instead of mut
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    print_two_values(x, y);

    another_function();

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if in a let statement
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);

    // loop with while
    println!("Loop with while");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // loop with for
    println!("Loop with for");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // loop with range
    println!("Loop with range");
    // rev here reverses the range for the loop, so goes 3,2,1
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function printing!");
}

fn print_value_x(x: i32) {
    println!("The vale of x is: {}", x);
}

fn print_two_values(x: i32, y: i32) {
    println!("The vale of x is: {}", x);
    println!("The vale of y is: {}", y);
}

// function returning an expression
fn five() -> i32 {
    5
}
