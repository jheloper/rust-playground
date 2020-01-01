use std::fs::File;

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
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}