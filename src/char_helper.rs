/*
循环打印从’a’~’Z’ 之间的所有字符
 */
pub fn a_to_z() {
    println!("循环打印从a~Z之间的所有字符:");
    let mut chars = vec![];
    for ch in 'a'..='z' {
        chars.push(ch);
    }
    for ch in 'A'..='Z' {
        chars.push(ch);
    }
    for ch in chars.iter() {
        print!("{} ", ch);
    }
    println!("\n");
}
