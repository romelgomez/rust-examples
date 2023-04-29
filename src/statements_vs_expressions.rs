pub fn statements() {
    let y = 6;

    println!("statement does not return values");
    println!("The value of y is: {}", y);
}

pub fn expressions() -> (i32, i32) {
    let x = 2 + 6;

    let y = {
        let x = 3;
        x + 1
    };

    (x, y)
}
