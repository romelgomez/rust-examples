pub fn loop_labels() {
    // assign value to count variable
    let mut count = 0;

    // loop_labels = 'counting_up
    'counting_up: loop {
        // print count
        println!("count = {count}");

        // assign value to remaining variable
        let mut remaining = 10;

        loop {
            // print remaining
            println!("remaining = {remaining}");

            // if remaining == 9, break
            if remaining == 9 {
                break;
            }

            // if count == 2, break 'counting_up loop
            if count == 2 {
                break 'counting_up;
            }

            // decrease remaining by 1
            remaining -= 1;
        }

        // increase count by 1
        count += 1;
    }
    println!("End count = {count}");
}

// The safety and conciseness of for loops make them the most commonly used loop construct in Rust
pub fn faster_safe_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Even in situations in which you want to run some code a certain number of times,
//  as in the countdown example that used a while loop in Listing 3-3, most Rustaceans
//  would use a for loop. The way to do that would be to use a Range, provided by the
// standard library, which generates all numbers in sequence starting from one number and ending before another number.
pub fn countdown() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// Expressions do not include ending semicolons.
// If you add a semicolon to the end of an expression,
// you turn it into a statement, and it will then not return a value.
// Keep this in mind as you explore function return values and expressions next.

pub fn return_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("counter: {}", counter);

        if counter == 10 {
            // this return value from loop
            // break counter * 2;
            // break 1
            break 1;
        }
    };

    println!("The result is {result}");
}
