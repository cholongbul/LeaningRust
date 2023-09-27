fn main() {
    //if

    let number = 3;
    if number < 5 {
        println!("조건이 일치합니다");
    } else {
        println!("조건이 불일치합니다");
    }
    //반드시 불리언 타입만
    // if number{ 정수 타입의 경우 에러
    //     println!("조건이 일치합니다");
    // } else {
    //     println!("조건이 불일치합니다");
    // }

    // else if
    let number = 6;

    if number % 4 == 0 {
        println!("변수가 4로 나누어 떨어집니다.");
    } else if number % 3 == 0 {
        println!("변수가 3로 나누어 떨어집니다.");
    } else if number % 2 == 0 {
        println!("변수가 2로 나누어 떨어집니다.");
    } else {
        println!("변수가 4, 3, 2로 나누어 떨어지지 않습니다.");
    }

    // let 구문에서 if 표현식 사용
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number의 값: {}", number);
    //if가 리턴하는 값은 각각 같은 값이어야 에러가 없다
    // let number = if condition {
    //     5
    // } else {
    //     "6"
    // };
    // println!("number의 값: {}", number);

    //loop
    // 무한 루프
    // loop{
    //     println!("다시 실행!");
    // }

    let mut counter = 0;
    let result = loop {
        counter +=1;
        // println!("The counter is {}", counter);

        if counter == 10{
            break counter * 2; //break가 리턴의 역할도 한다.
        }
    };
    println!("The result is {}", result);

    //while
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number = number -1;
    }
    println!("발사!");

    //for
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("요쇼의 값: {}" , element);
    }

    for number in (1..4).rev() {//rev 뒤집는 함수
        println!("{}" , number);
    }
    println!("발사!");

}
