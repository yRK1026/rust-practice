fn main() {
    let price = 3950;
    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64 = 10;
    // for i500 in 0..11 {
    for i500 in 0..=count500 {
        // for i100 in 0..4 {
        for i100 in 0..=count100 {
            // for i50 in 0..11 {
            for i50 in 0..=count50 {
                let total = i50 * 50 + i100 * 100 + i500 * 500;
                if total == price {
                    println!("500円={}枚, 100円={}枚, 50円={}枚", i500, i100, i50);
                }
            }
        }
    }

    example();
}

fn example() {
    println!("--- 符号あり整数 ---");
    println!("i8: {} ~ {}", i8::MIN, i8::MAX);
    println!("i16: {} ~ {}", i16::MIN, i16::MAX);
    println!("i32: {} ~ {}", i32::MIN, i32::MAX);
    println!("i64: {} ~ {}", i64::MIN, i64::MAX);
    println!("i128: {} ~ {}", i128::MIN, i128::MAX);
    println!("isize: {} ~ {}", isize::MIN, isize::MAX);
    println!("--- 符号なし整数 ---");
    println!("u8: {} ~ {}", u8::MIN, u8::MAX);
    println!("u16: {} ~ {}", u16::MIN, u16::MAX);
    println!("u32: {} ~ {}", u32::MIN, u32::MAX);
    println!("u64: {} ~ {}", u64::MIN, u64::MAX);
    println!("u128: {} ~ {}", u128::MIN, u128::MAX);
    println!("usize: {} ~ {}", usize::MIN, usize::MAX);
    println!("usize=u{}", usize::BITS);
    println!("--- 浮動小数点数 ---");
    println!("f32: {} ~ {}", f32::MIN, f32::MAX);
    println!("f64: {} ~ {}", f64::MIN, f64::MAX);
}
