use std::fs::File;
use std::io::ErrorKind;

pub fn example_error() {
    // panic! 매크로 사용
    // panic!("crash and burn!");

    // RUST_BACKTRACE=1 환경변수 설정으로 백트레이스를 볼 수 있음
    // let v = vec![1, 2, 3];
    // v[99];

    // File::open 함수는 Result 열거형 반환
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    // unwrap 메서드를 사용
    // let f2 = File::open("hello2.txt").unwrap();

    // expect 메서드를 사용하여 panic! 에러 메시지를 선택할 수 있음
    let f2 = File::open("hello2.txt").expect("Failed to open file");
}
