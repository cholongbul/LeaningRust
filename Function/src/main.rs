fn main() {

    //함수 및 매개변수
    println!("Hello, world!");

    another_function(5,6);

    //구문 : 동작 실행하나 값 리턴 없음
    let y = 6; //변수 선언도 하나의 구문
//    let x = (let y = 6); --컴파일 에러 구문을 다른 변수에 대입 불가

    //    표현식은 최종 결과 값으로 평가됨
    // 5 + 6
    let x = 5;
    let y = {//코드 불록 4라는 값으로 평가
        let x =3;
        x +1//세미콜론으로 끝나지 않음 표현식은 세미콜론 없음
    };

    println!("y의 값 : {}", y);
    //값을 리턴하는 함수
    let x = five();
    println!("x의 값 : {}" , x);

    //다른 예제
    let x = plus_one(5);
    println!("plus_one의 값 : {}" , x);

}

//+1
fn plus_one(x: i32) -> i32 {
    x + 1
}

//값을 리턴하는 함수
fn five() -> i32 {//리턴타입 정의
    5
}
fn another_function(x: i32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}


