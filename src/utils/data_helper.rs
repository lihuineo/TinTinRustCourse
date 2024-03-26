/*
循环打印从’A’~’z’ 之间的所有字符
 */
pub fn a_to_z() {
    println!("循环打印从A~z之间的所有字符:");
    for ch in 'A'..='z' {
        if ch.is_alphabetic() {
            print!("{} ", ch);
        }
    }
    println!("\n");
}
