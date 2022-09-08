use std::io::Read;
use std::io::Stdin;


fn main() {
    let mut buf = String::new();
    let std_in: Stdin = std::io::stdin();
    while std_in.read_line(&mut buf).is_ok() {
        if "q\n" == buf {
            println!("bye");
            break;
        } else {
            println!("{}", buf);
            buf.clear();
        }
    }
}
