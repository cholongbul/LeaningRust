fn main() {
    //불변
    let user1 = User {
        email: String::from("someone@exameple.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //가변
    let mut user1 = User {
        email: String::from("someone@exameple.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    //5.1.2 기존 인스턴스로부터 새 인스턴스 생성하기

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
        // active: user1.active, 생략
        // sign_in_count: user1.sign_in_count, 생략
    };

    //5.1.3 이름없는 필드를 가진 튜플 구조체로 다른 타입 생성하기
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //5.1.4 필드가 없는 유사 유닛 구조체
    // struct User {
    // } // 유사 유닛 구조체

    //5.2 구조체를 사용하는 예제
    let width1 = 30;
    let height1 = 50;

    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area(width1, height1)
    ); //가로와 높이의 관계성이 안 드러남

    let rectl = (30, 50);

    println!(
        "사각형의 면적2: {} 제곱 픽셀",
        area2(rectl)
    );

    //더 많은 의미 반영
    let rectl = Rectangle { width: 30, height: 50 };

    println!(
        "사각형의 면적3 {} 제곱 픽셀",
        area3(&rectl)
    );

    //println!("rectl: {}", rectl); 에러 발생
    //println!("rectl: {:?}", rectl); 디버그 출력 형식 하지만 에러 #[derive(Debug)] 추가

    println!("rectl: {:#?}", rectl);

    //5.3 메서드 문법
    println!("사각형의 면적: {} 제곱 픽셀", rectl.area());

    //5.3.2 더 많은 매개변수를 갖는 메서드
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1은 Rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 Rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    //5.3.3 연관함수
    let square = Rectangle::square(28);
    println!("rect1은 Rect3를 포함하는가? {}", rect1.can_hold(&square));

    //5.3.4 여러 개의 impl 블록 가능

}

#[derive(Debug)] //디버그 명시
struct Rectangle {
    width: u32,
    height: u32,
}

//메서드 정의
impl Rectangle {
    //Rectangle 컨텍스트 내부 함수 정의 위해 impl
    fn square(size: u32) -> Rectangle { // 연관함수 정사각형 생성자
        Rectangle { width: size, height: size }
    }
    fn area(&self) -> u32 { //첫 번째 매개변수는 &self 컨텍스트 내부값
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,  //username: username, 입력변수와 같다면 생략가능
        email,  //email: email, 입력변수와 같다면 생략가능
        sign_in_count: 1,
        active: true,
    }
}
