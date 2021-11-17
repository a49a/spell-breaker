fn main() {
    let r = dangle()
}

fn dangle() -> &String {
    let s = String::from("haha");
    &s
}