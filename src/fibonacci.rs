// generate fibonacci numbers up to limit

pub fn fibonacci_number(limit: i32) -> Vec<i32> {
    let mut a = 0;
    let mut b = 1;
    let mut numbers = vec![];

    while b < limit {
        numbers.push(b);

        let c = a + b;

        a = b;
        b = c;
    }

    numbers
}
