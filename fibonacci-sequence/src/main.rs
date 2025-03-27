fn main() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a + b);
        let tmp = a;
        a = b;
        b = tmp + b;
    }

    example();
}

fn example() {
    let price = 98000.0;
    let a_ship_fee = 1200.0;
    let a_rate = 0.8;
    let b_ship_fee = 0.0;
    let b_rate = 0.9;

    println!("A社={}円", price * a_rate + a_ship_fee);
    println!("B社={}円", price * b_rate + b_ship_fee);
}