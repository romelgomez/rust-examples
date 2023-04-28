pub fn statements() {
    // statement does not return values
    let y = 6;
}

pub fn statements_vs_expressions() {
    let x = 2 + 6;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}, y: {}", x, y);
}
