use std::net::{Ipv4Addr, Ipv6Addr};
use std::option;

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;//열거값 표현 인스턴스

    route(IpAddrKind::V4);//함수 정의
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 열거자를 구조체에 넣지 않고 열거자만 이용하는 간편한 방법
    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));
    let home = IpAddr3::V4(127, 0, 0, 1);

    //메서드도 생성 가능 구조체와의 공통점
    let m = Message::Write(String::from("hello"));
    m.call();

    //6.1.2. Option 열거자
    //러스트는 null 값 개념 없음 어떤 값의 존재 여부를 표현하는 열거자는 존재 그게 바로 Option<T>


    let some_number = Some(5);
    let some_string = Some("a string");

    // let absent_number: Option<i32> = None; //러스트의 null 대체 사용법

    //6.2 match 흐름 제어 연산자
    enum Coin {
        penny,
        Nickle,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::penny =>{println!("행운의 페니!");
                            1}
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    //6.2.1 값을 바인딩하는 패턴
    #[derive(Debug)]
    enum UsState {
        Alabama, Alaska,
    }

    enum Coin2 {
        penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin: Coin2) -> u32 {
        match coin {
            Coin2::penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {println!("State quarter from {:?}!",state); 25},
        }
    }

    value_in_cents2(Coin2::Quarter(UsState::Alaska));

    fn plus_one (x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) =>Some(i +1),
        }
    }

    let some_u8_value = Some(0u8);

    //6.3 if let을 이용한 간결한 흐름제어
    if let Some(3) = some_u8_value {
        println!("three");


}


enum IpAddrKind {
    //열거자 enums
    V4,
    //열거값 variants
    V6,
}

struct IpAddr {
    //구조체
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    //열거자 enums
    V4(String),
    //열거값 variants
    V6(String),
}

enum IpAddr3 {
    //열거자 enums
    V4(u8, u8, u8, u8),
    //열거값 variants
    V6(String),
}

enum IpAddr4 {
    V4(Ipv4Addr),
    //어떤 데이터도 저장 가능
    V6(Ipv6Addr),
}

//6-2 개별 값을 각기 다른 타입으로 정의한 Message 열거자
enum Message {
    Quit,
    //연관 데이터 없음
    Move { x: i32, y: i32 },
    //익명 구조체 포함
    Write(String),
    //String 값 포함
    ChangeColor(i32, i32, i32), //i32값 세개 포함
}

impl Message {
    fn call(&self) {
        //메서드 생성
    }
}

fn route(ip_type: IpAddrKind) {}