fn main() {
    println!("|{:>8}| #{:06x}", "red", 0xff0000);
    println!("|{:>8}| #{:06x}", "green", 0x00ff00);
    println!("|{:>8}| #{:06x}", "blue", 0x0000ff);
    println!("|{:>8}| RGB{:?}", "yellow", (255, 255, 0));
}
