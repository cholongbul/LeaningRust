fn main() {

    // 변수--------------------------------------------------------
    let mut x = 5; // mut 를 붙여야 가변 변수 없으면 불변 변수
    println!("x의 값: {}", x);
    x = 6;
    println!("x의 값: {}", x);

    const MAX_POINTS: u32 = 100_000; // 상수

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x의 값: {}", x); // shadowed

    let spaces = "     ";
    let spaces = spaces.len(); // 타입 변환도 가능

    //let mut spaces = "    ";
    //spaces = spaces.len();  mut 사용시 같은 타입만

    //데이터 타입 -----------------------------------------------
    let guess: u32 = "42".parse().expect("숫자가 아닙니다!"); //타입 애노테이션 필수

    // 스칼라 타입 : 정수(integer) 부동 소수점 숫자(floating point numbers), 불리언(booleans), 문자(characters)

    // 정수 8bit 16bit 32bit 64bit arch
    //Decimal Hex Octal Binary byte

    //부동 소수점 : f64 f32

    let x = 2.0; // f64

    let y: f32 = 3.0; //f32

    // 사칙 연산

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    //나머지
    let remainder = 43 % 5;

    //boolean

    let t = true;

    let f: bool = false;

    //문자타입

    let c = 'z';
    let z = '한';
    let emoji = '😀';

    //컴파운드 타입
    //튜플

    //선택적 타입 애노테이션
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //패턴 매칭을 이용한 해체
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("y의 값: {}", y);

    // // 마침표(.)를 이용한 직접 참조
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    //
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;
    // println!("five_hundred의 값: {}", five_hundred);
    // println!("six_point_four의 값: {}", six_point_four);
    // println!("one의 값: {}", one);
    //
    // //배열 타입
    // //고정된 길이, 스택 메모리, 길이 조정이 가능한 건 벡터
    //
    // let a = [1, 2, 3, 4, 5];
    //
    // //길이가 고정되어 있는 월
    // let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    //
    // // a: [i32; 5]를 통해 길이와 타입 지정
    // let a: [i32; 5] = [1,2,3,4,5];
    // // let a = [3,3,3,3,3] ==  let a = [3; 5];
    // let a = [3; 5];
    //
    // //배열 요소 접근
    //
    // let a = [1,2,3,4,5];
    // let first = a[0];
    // let second = a[1];

    //유효하지 않은 배열 요소에 접근하는 경우 : 런타입 에러 :this operation will panic at runtime
    //러스트 안정성 원리
    // let a = [1,2,3,4,5,];
    // let index = 10;
    //
    // let element = a[index];
    // println!("요소의 값: {}", element);

    

}
