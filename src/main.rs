mod fibonacci;
mod loops;
mod lyrics;
mod math_functions;
mod temp_conversion;

fn main() {
    println!("Hello, world!");

    let sum_test = math_functions::sum(1, 4);
    let rest_test: i32 = math_functions::rest(1, 4);

    println!("sum_test: {}", sum_test);

    println!("rest_test: {}", rest_test);

    let fibonacci_result = fibonacci::fibonacci_number(1000);

    println!("fibonacci_result: {:?}", fibonacci_result);

    let celcius_test = temp_conversion::fahrenheit_to_celcius(100);
    let farhenheit_test = temp_conversion::celsius_to_farhenheit(100);

    println!("celcius_test: {}", celcius_test);
    println!("farhenheit_test: {}", farhenheit_test);

    let lyrics_test = lyrics::twelve_days_of_christmas();
    println!("{}", lyrics_test);

    // before_function();
    // after_function();
    // statements_vs_expressions();

    // let expression_value = expressions();
    // let statements_value = statements();

    // println!("expression_value: {}", expression_value);
    // println!("statements_value: {}", statements_value);

    loops::countdown();
    loops::faster_safe_loop();
    loops::loop_labels();
    loops::loop_labels();
    loops::return_values_from_loops();
    loops::return_values_from_loops();
}
