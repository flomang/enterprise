
#[derive(Debug, Copy, Clone)]
struct Number {
    num: i32
}

fn main() {
    let zed = Number{ num: 25 };
    let me = zed;

    //me.num = 8;

    println!("zed: {:?}", zed);
    println!("me: {:?}", me);
}
