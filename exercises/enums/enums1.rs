// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
