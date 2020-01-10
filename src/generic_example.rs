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