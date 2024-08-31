fn main() {
    let g1 = String::from("過ちを見過ごし人は美しい");
    show_message(&g1);
    println!("g1: {}", g1);
}

fn show_message(msg: &String) {
    println!("msg: {}", msg);
}
