use std::mem::take;

fn main() {

    //scope
    {// 변수 s를 아직 선언하지는 않았으므로 변수가 유효하지 않다
        let s = "hello"; // 변수 s는 이 지점부터 유효하다.
     // 변수 s를 이용해 필요한 동작을 수행한다
    }// 여기서 범위를 벗어나므로 변수 s 는 이제 유효하지 않다.a

    //String 타입 : 힙에 할당됨 컴파일 시점에 알 수 없는 크기의 문자열 저장. 리터럴은 불변
    let s = String::from("hello");

    //변경 가능 문자열
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()은 String 인스턴스에 리터럴을 결합한다.
    println!("{}", s);

    //러스트의 메모리 할당 해제 방식 == 변수를 소유한 범위를 벗어나는 순간 자동 해제


    //변수와 데이터가 상호작용하는 방식 : 이동(Move)
    //정수대입
    let x = 5;
    let y = x;

    //String 대입시 포인터가 같은 위치를 가리킴 :: 이중 해제 에러 qkftod rksmdtjd
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 재참조 시 에러 대입 시 move 하여 s1은 무효화
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    //변수와 데이터가 상호작용하는 방식 : 복제(Clone)

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {} s2 = {}, world!", s1, s2);

    // 스택 전용 데이터: 복사(Copy)
    let x = 5;
    let y = x;
    //Copy 트레이트 적용된 경우
    println!("x = {}  y= {}" , x, y);


    //소유권과 함수
    let s = String::from("hellop"); //변수 s가 범위 내에 생성된다.

    takes_ownership(s); //변수 s의 값이 함수 내로 이동한다.
                                    // 그리고 이 시점부터 변수 s는 더이상 유효하지 않다.

    let x = 5;                  //변수 x가 범위 내에 생성된다.

    makes_copy(x);        // 변수 x의 값이 함수 내로 이동한다.
                                        //하지만 i32 타입은 복사를 수행하므로
                                        // 변수 x는 이 시점 이후로도 여전히 유효하다.


    //4.1.6 리턴값과 범위
    //리턴값의 소유권 이전
    let s1 = gives_ownership(); //리턴값이 변수 s1으로 옮겨짐

    let s2= String::from("hello"); // 변수 s2가 범위 내에 생성

    let s3 = takes_and_gives_back(s2); //변수 s2가 takes and gives bakc 함수로 옮겨간 후 리턴

    //매개 변수의 소유권을 다시 리턴하는 함수
    let s1 = String::from("hello");
    let (s2 , len) = calculate_length(s1);
    println!("'{}'의 길이는 {}입니다.", s2, len);


    //4.2참조와 대여



} // 이 지점에서 변수 x가 범위를 벗어난 후, 다음으로 변수 s도 범위를 벗어난다
//하지만 변수 s의 값은 함수 내로 이동했기 때문에 아무런 일도 일어나지 않는다.
fn takes_ownership(some_string: String){  // some_string변수가 범위 내에서 생성된다.
    println!("{}", some_string);
} //이 지점에서 some_string 변수가 범위를 벗어나며 drop 함수가 호출된다.
  // 따라서 some_string 변수가 사용 중이던 메모리가 해제된다.

fn makes_copy(some_integer: i32){//some_integer 변수가 범위 내에 생성된다.
    println!("{}", some_integer);

} //이 지점에서 some_integer 변수가 범위를 벗어난다. 하지만 아무런 일도 일어나지 않는다.

fn gives_ownership() -> String { //gives_ownership 함수의 리턴값은 호출한 함수로 옮겨진다.
    let some_string = String::from("hello"); //변수 some_string이 범위 내에 생성된다.

    some_string //리턴되어 호출한 함수로 옮겨진다.
}

fn takes_and_gives_back(a_string: String) -> String{ // 변수 a_string이 범위내에 생성된다

    a_string // 그대로 리턴
}

fn calculate_length(s: String)->(String, usize){
    let length = s.len(); //len() 함수는 문자열 길이를 리턴

    (s, length)
}
