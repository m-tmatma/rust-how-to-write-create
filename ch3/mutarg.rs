fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main() {
    let mut x = 16;
    x2(&mut x);
    println!("x: {}", x);
}
