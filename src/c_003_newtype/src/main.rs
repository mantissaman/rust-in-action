
struct Hostname(String);

fn connect(h: Hostname) {
    println!(" connected to {}", h.0)
}

fn main() {
    let ordinary_string = "localhost".to_owned();
    let host = Hostname(ordinary_string.clone());
    connect(host);
}
