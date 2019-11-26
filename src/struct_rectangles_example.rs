pub fn example_struct_rectangles() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area(length1, width1)
    );
}

fn calculate_area(length: u32, width: u32) -> u32 {
    length * width
}