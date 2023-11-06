fn main() {
    // //10.1함수로부터 중복 제거
    //
    // //숫자의 리스트로부터 가장 큰 숫자 찾기
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let mut largest = number_list[0];
    //
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //
    // println!("가장 큰 숫자: {}", largest);
    //
    // //두개의 숫자 리스트에서 가장 큰 숫자찾기(중복발생)
    // let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    //
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //
    // println!("가장 큰 숫자: {}", largest);
    //
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let mut largest = number_list[0];
    //
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("가장 큰 숫자: {}", largest);

    //함수 활용
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);


    println!("가장 큰 숫자: {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);

    println!("가장 큰 숫자: {}", result);


    let number_list = vec![34,50,25,100,65];

    let result = largest_i32(&number_list);
    println!
}


fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//시그너처의 이름과 타입만 다르고 동작은 같은 두 함수

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
