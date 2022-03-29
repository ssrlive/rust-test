#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn demo(self) {
        let Rectangle { width, height } = self;

        println!("width = {}, height = {}", width, height);
    }
}

fn main() {
    let a = Rectangle {
        width: 09,
        height: 977,
    };
    a.demo();
}
