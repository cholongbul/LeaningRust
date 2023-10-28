use std::collections::HashMap;

fn main() {
    //벡터
    {
        let v: Vec<i32> = Vec::new();

        let v = vec![1, 2, 3];
    }


    //벡터 값 추가 (값 추가를 위해선 mut로 생성)
    {
        let mut vmut = Vec::new();

        vmut.push(5);
        vmut.push(6);
        vmut.push(7);
        vmut.push(8);
        vmut.push(9);
        println!("{:?}", vmut);
    }

    //백터 해제
    {
        let v = vec![1, 2, 3, 4];
    } // 범위를 벗어나면 drop 메서드가 호출되며 메모리가 해제


    //백터 값 읽기
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2]; //& []이용
        println!("세 번쨰 원소: {}", third);

        match v.get(2) { //get 이용
            Some(third) => println!("세 번쨰 원소:{}", third),
            None => println!("세 번쨰 원소가 없습니다.")
        }

        match v.get(100) { //get 이용
            Some(third) => println!("백 번쨰 원소:{}", third),
            None => println!("백 번쨰 원소가 없습니다.")
        }

        //let dous_not_exist = &v[100]; 패닉
        let dous_not_exist = &v.get(100); //None 리턴
    }

    {
        //가변참조와 불변참조는 동시에 가질 수 없음
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0]; //불변 참조
        // v.push(6); //추가 변화 //에러
        // println!("{}", first);
    }

    //8.1.5 벡터에 저장된 값 순회
    {
        //불변
        let v = vec![1, 2, 3, 4, 5];
        for i in &v {
            print!("{}", i);
        }
    }

    {
        //가변
        let mut v = vec![1, 2, 3, 4, 5];
        for i in &mut v {
            *i += 50; //*역참조 연산자
            println!("{}", i);
        }
    }

    //열거자를 이용해 하나의 벡터에 다른 타입 저장
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),
    ];

    //8.2 String 타입에 UTF-8 형식의 텍스트 저장

    //문자열 생성
    {
        //빈 문자열
        let mut s = String::new();

    }

    {
        //to_string 메서드
        let data = "문자열초깃값";
        let s = data.to_string();

        //let s ="문자열초깃값".to_string(); 형식도 가능
    }

    {
        //String::from
        let s = String::from("문자열초깃값");
    }

    //문자열 수정
    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2: {}", s2);

    }

    {
        let mut s = String::from("lo");
        s.push('l');
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; //이렇게 되면 변수 s1은 메모리가 해제되어 더 이상 사용할 수 없다
        println!("{}", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        //2개 이상 합칠 때 +는 비효율
        //let s = s1+"-"+&s2+"-"+&s3;

        let s = format!("{}-{}-{}", s1,s2,s3);
        println!("{}", s);
    }
    //인덱스

    // {
    //     let s1 = String::from("hello");
    //     let h = s1[0];
    //     println!("{}",h); //에러! 러스트 문자열은 인덱스 사용 안함
    // }

    //대체 1 문자열 슬라이스
    {
        let hello = "안녕하세요";

        let s = &hello[0..3]; // 한글은 한 글자 3byte 차지 [0..1]은 패닉 발생
        println!("{}",s);
    }

    //대체 2 문자열 순회 메서드
    {
        for c in "안녕하세요".chars() {
            println!("{}",c);
        }

        for b in "안녕하세요".bytes() {
            println!("{}",b);
        }
    }

    //해시 맵
    use std::collections::HashMap;

    //생성
    {
     let mut scores = HashMap::new();

        scores.insert(String::from("블루"), 10);
        scores.insert(String::from("옐로"), 50);


    }

    {
        let teams = vec![String::from("블루"), String::from("옐로")];
        let initial_scroes = vec![10,50];
        //리스트를 이용한 생성
        let scores: HashMap<_,_> = teams.iter().zip(initial_scroes.iter()).collect();

    }

    //해시 맵과 소유권
    {
        let filed_name = String::from("Favorite color");
        let field_value = String::from("블루");

        let mut map = HashMap::new();
        map.insert(filed_name, field_value);
        // field_name과 field_value 변수는 이 지점부터 유효하지 않다.
        // 이 값들을 사용하려고 하면 컴파일러가 에러를 발생한다.

    }

    //해시맵의 값에 접근
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"), 10);
        scores.insert(String::from("옐로"), 50);

        let team_name = String::from("블루");
        let score = scores.get(&team_name);
        println!("{:?}",&score);
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"), 10);
        scores.insert(String::from("옐로"), 50);

        for(key, value) in &scores{
            println!("{}: {}" , key, value);
        }

    }

    //해시 맵 수정

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"),10);
        scores.insert(String::from("블루"),20);

        println!("{:?}",scores);

    }
    //값이 없을 때에만 할당
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("블루"),10);

        scores.entry(String::from("옐로")).or_insert(50);
        scores.entry(String::from("블루")).or_insert(20);

        println!("{:?}",scores);

    }

    //기존 값에 따라 수정
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count +=1;
        }

        println!("{:?}",map);
    }








}
