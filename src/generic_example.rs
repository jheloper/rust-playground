pub fn example_generic() {
    let numbers1 = vec![34, 50, 25, 100, 65];

    let result1 = largest(&numbers1);
    println!("The largest number is {}", result1);

    let numbers2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result2 = largest(&numbers2);
    println!("The largest number is {}", result2);

    let result3 = largest_i32(&numbers2);
    println!("The largest number is {}", result3);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result4 = largest_char(&chars);
    println!("The largest char is {}", result4);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 4.0 };
    println!("integer point is {:?}", integer);
    println!("float point is {:?}", float);

    // 아래 구조체는 하나의 제네릭 타입을 사용하기 때문에 동일한 타입이어야 함
    // 따라서 아래 구조체는 오류 발생
    // let wont_work = Point { x: 5, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("both integer point is {:?}", both_integer);
    println!("both float point is {:?}", both_float);
    println!("integer and float point is {:?}", integer_and_float);

    println!("integer point x is {}", integer.x());
    println!("float point x is {}", float.x());
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}