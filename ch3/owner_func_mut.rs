fn add_quote(msg: &mut String) {
    msg.insert_str(0, "「");
    msg.push_str("」");
}

fn main() {
    let mut g1 = String::from("強い心は病気の支えとなる");
    add_quote(&mut g1);
    println!("g1: {}", g1);
}