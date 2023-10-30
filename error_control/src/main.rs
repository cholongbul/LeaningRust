use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    //9.1 panic 매크로를 이용한 회복 불가능한 에러 처리
    //  panic!("crash and burn");

    //panic! 역추적 사용
    let v = vec![1, 2, 3];
    // v[99];//실행 중단

    //9.2 Result 타입으로 에러 처리
    {
        enum Result<T, E> {
            //Result 열거자
            Ok(T),
            Err(E),
        }
    }

    {
        //let f:u32 = File::open("hello.text"); 반환 타입: Result<File, std::io::Error>`
    }

    // {
    //     let f = File::open("hello.txt");
    //
    //     let f = match f { 매치 표현식으로 Result 에러 제어
    //         Ok(file) => file,
    //         Err(error) => {
    //             panic!("파일 열기 실패: {:?}", error);
    //         }
    //     };
    //}
    // {//에러 종류에 따라 다르게 처리
    //     let f = File::open("hello.txt");
    //
    //     let f = match f{
    //         Ok(file) =>file,
    //         Err(ref error) =>match error.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt"){
    //                 Ok(fc) =>fc,
    //                 Err(e) => panic!("파일을 열지 못했습니다: {:?}", e),
    //             },
    //             other_error=> panic!("파일을 열지 못했습니다: {:?}", other_error),
    //
    //         },
    //     };
    //
    // }

    //에러 빠르게 발생 시키는 방법 unwrap expect
    {
        // let f = File::open("hello.txt").unwrap();
        //let f = File::open("hello.txt").expect("파일 못염");
    }

    //에러 전파
    fn read_username_from_file() -> Result < String, io::Error >//리턴타입
    {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s){
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    //?연산자를 이용하여 더 쉽게 전파
    fn read_username_from_file2() -> Result < String, io::Error >//리턴타입
    {
        let mut f = File::open("hello.txt")?;

        let mut s = String::new();

        f.read_to_string(&mut s)?;
        //더 짧게 File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}

