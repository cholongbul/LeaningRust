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
    let s1 = String::from("hello");
    let len = calculate_length2(&s1); //참조를 이용하도록 작성 &Stirng 소유권은 유지

    println!("'{}'의 길이는 {}입니다.", s1, len);

    //4.2.1 가변 참조
    let mut s = String::from("hello)");

    change(&mut s);

    //특정범위 내에 특정 데이터에 대한 가변 참조는 오직 한 개만 존재해야함 ::아래 예제는 에러 발생
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{},{}",r1, r2);

    //아래와 같이 사용할 때 두 가지의 가변 참조 활용
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

    //불변참조와 가변 참조 혼합 사용시 에러 발생
    // let mut s = String::from("hello");
    //
    // let r1 = &s; //아무 문제 없음
    // let r2 = &s; //아무 문제 없음
    // let r3 = &mut s; //문제 발생
    // println!("{},{},and {}", r1, r2, r3);

    //4.2.2 죽은 참조
    //에러 missing lifetime specifier : 이 리턴에 대여한 값을 리턴하고자 하나 실제 대여해올 값이 존재하지 않음
    // let reference_to_nothing = dangle();

    //4.3 슬라이스 타입
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); //문자열을 비워 빈 문자열을 만든다.
    //변수 word 에는 여전히 5라는 값이 할당되었지만 이 값을 적용할 문자열이 더는 없다.
    //그래서 변수 word는 아무런 쓸모가 없게 된다. -> 문자열 슬라이스를 통해 해결

    //4.3.1 문자열 슬라이스
    let s = String::from("hello world");

    let hello = &s[0..5]; //&s[..5] 0 생략 가능
    let world = &s[6..11];//&s[6..] 마지막 생략 가능







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

fn calculate_length2(s: &String)->(usize){ //참조를 이용하도록 작성 &Stirng
    let length = s.len(); //len() 함수는 문자열 길이를 리턴
    // 대여한 변수는 변환 불가 etc) s.push_str(",wolrd");

    length
}

fn change(some_sting: &mut String){
    some_sting.push_str(", world"); //가변 참조시 변환 가능
}

// fn dangle() -> &String{ // dangle 함수는 String에 대한 참조를 리턴한다.
//     let s = String::from("hello");//새 변수 s
//
//     &s // s에 대한 참조를 리턴 하지만 이 지점에서 변수 s가 범위를 벗어나므로 drop 함수 호출 에러
// }

//first_word 함수는 전달된 문자열 매개변수에서 바이트의 인덱스값 리턴

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}