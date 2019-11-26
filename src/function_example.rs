pub fn example_function() {
    another_function(10, 15);
    block_expression();

    let function_result = return_ten();
    println!("function result is: {}", function_result);
}

fn another_function(x: i32, y: i32) {
    println!(
        "This is another function! argument x is: {}, y is: {}",
        x, y
    );
}

fn block_expression() {
    let x = 10;

    let y = {
        let x = 5;
        x + 20
    };

    println!("The value of y is: {}", y);
}

fn return_ten() -> i32 {
    10
}