pub fn example_struct_rectangles() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area(length1, width1)
    );

    // 위 코드를 튜플을 이용하는 코드로 리팩터링.
    let rect1 = (50, 30);
    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area_by_tuple(rect1)
    );

    // 위 코드를 구조체를 이용하는 코드로 리팩터링.
    let rect2 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area_by_struct(&rect2)
    );

    // Rectangle 구조체 인스턴스 출력.
    println!("rectangle is {:?}", rect2);
    println!("rectangle is {:#?}", rect2);
}

fn calculate_area(length: u32, width: u32) -> u32 {
    length * width
}

fn calculate_area_by_tuple(dimensions: (u32, u32)) -> u32 {
    // 튜플 인덱스가 무엇을 의미하는지 기억해야 한다...
    dimensions.0 * dimensions.1
}

// derive 어노테이션 적용.
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn calculate_area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}