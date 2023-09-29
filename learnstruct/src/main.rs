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

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //5.1.4 필드가 없는 유사 유닛 구조체


}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username:String) -> User {
    User {
        username,  //username: username, 입력변수와 같다면 생략가능
        email,  //email: email, 입력변수와 같다면 생략가능
        sign_in_count: 1,
        active: true
    }
}
