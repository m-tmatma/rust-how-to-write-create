fn gen_message() -> &str {
    let msg = String::from("過ちを見過ごし人は美しい");
    return &msg;
}

fn main() {
    let msg = gen_message();
    println!("msg: {}", msg);
}
