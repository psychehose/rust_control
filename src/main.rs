fn main() {

    // if else

    // let condition = true;
    // let number = if condition {
    //   5
    // } else {
    //   6
    //     /*
    //         if, else에서 같은 값을 return 해야함 six로 바꾸면 에러 발생
    //     */
    //
    // };
    //
    // println!("The value of number is {}", number);

    // 무한루프
    loop {
        println!("again");

        break;
    }

    // 조건부 반복 while

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number -1;
    }
    println!("LIFEOFF!!!");


    // for와 함께하는 콜렉션 반복하기

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

        // 배열은 크기가 고정되어 있고 배열의 6번째 값을 접근하기 전에 반복은 중지되어야함
        // 반복할 때마다, 런타임 코드를 추가하기 때문에 비효율 초래
        // 이런 경우에는 for 문 사용
    while index < 5 {
        println!("The value is {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
