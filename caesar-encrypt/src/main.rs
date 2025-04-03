fn main() {
    let enc = encrypt("I LOVE RUST", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);

    let enc2 = encrypt2("I LOVE RUST", 3);
    let dec2 = encrypt2(&enc2, -3);
    println!("{} => {}", enc2, dec2);
}

fn encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    println!("code_a: {}, code_z: {}", code_a, code_z);
    let mut result = String::new();
    for ch in text.chars() {
        let mut code = ch as i16;
        println!("ch: {}, code: {}", ch, code);
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }

        result.push((code as u8) as char);
    }

    return result;
}

fn encrypt2(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c - a + shift + 26) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    text.chars().map(enc1).collect()
}