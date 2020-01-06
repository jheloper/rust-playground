use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

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

    // unwrap 메서드를 사용하여 위의 match 구문과 비슷한 처리를 할 수 있음
    // let f2 = File::open("hello2.txt").unwrap();

    // expect 메서드를 사용하여 panic! 에러 메시지를 선택할 수 있음
    // let f2 = File::open("hello2.txt").expect("Failed to open file");

    let result = read_username_from_file();
    println!("result: {:?}", result);

    let result2 = read_username_from_file2();
    println!("result2: {:?}", result2);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // ?연산자를 통해 에러를 전파하는 위의 match 표현식을 대체할 수 있음
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}