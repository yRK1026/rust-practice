// Title: FizzBuzz
/*
3と5で割り切れる場合はFizzBuzz
3で割り切れる場合はFizz
5で割り切れる場合はBuzz
それ以外は数字を表示する
*/
fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    example();
}

fn example() {
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 || i > 30 && i < 40 {
            println!("A");
        } else {
            println!("{}", i);
        }
    }
}