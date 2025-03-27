fn main() {
    for i in 1..10 {
        // for j in 1..10 {
        //     if j == 9 {
        //         print!("{:3}", i * j);
        //     } else {
        //         print!("{:3},", i * j);
        //     }
        // }
        let output = (1..10)
            .map(|j| format!("{:3}", i * j))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", output);
    }

    example();
}

fn example() {
    for year in 1926..2027 {
        if year >= 1926 && year <= 1988 {
            let output = year - 1925;
            if output == 1 {
                println!("西暦{}年 = 昭和元年", year);
            } else {
                println!("西暦{}年 = 昭和{}年", year, output);
            }
        } else if year >= 1989 && year <= 2018 {
            let output = year - 1988;
            if output == 1 {
                println!("西暦{}年 = 平成元年", year);
            } else {
                println!("西暦{}年 = 平成{}年", year, output);
            }
        } else {
            let output = year - 2018;
            if output == 1 {
                println!("西暦{}年 = 令和元年", year);
            } else {
                println!("西暦{}年 = 令和{}年", year, output);
            }
        }
    }
}