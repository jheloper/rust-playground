pub fn example_slices() {
    let mut s1 = String::from("hello world");
    let word_index = first_word(&s1);
    println!("word index is {}", word_index);

    s1.clear();

    // 슬라이스를 사용한 예제 코드
    let mut s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("first word is {}", hello);
    println!("second word is {}", world);

    s2.clear();

    // 문자열에서 첫번째 단어의 슬라이스를 반환하는 함수 예제 코드
    let mut s3 = String::from("hello world");
    let first_word = first_word_slices(&s3);
    println!("first word is {}", first_word);

    // 두번째 단어의 슬라이스를 반환하는 함수 예제 코드
    let second_word = second_word_slices(&s3);
    // 아래 clear 함수 호출 코드는 컴파일 오류가 발생한다.
    // 슬라이스로 불변 참조자를 만든 상태이고, clear 함수 내부에서 가변 참조자를 얻기 위한 시도를 하기 때문.
    // s3.clear();
    println!("second word is {}", second_word);

    s3.clear();

    // 스트링 리터럴은 바이너리의 특정 지점을 가리키고 있는 슬라이스, 아래 string_literal의 타입은 &str.
    let string_literal = "hello world";
    let first_word_by_literal = first_word_slices(string_literal);
    println!("first word is {}", first_word_by_literal);

    let second_word_by_literal = second_word_slices(string_literal);
    println!("second word is {}", second_word_by_literal);

    // 일반적인 배열에 대한 슬라이스 예제 코드.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("array slice is {:?}", slice);
}

fn first_word(s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slices(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word_slices(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..s.len()];
        }
    }

    &s[..]
}