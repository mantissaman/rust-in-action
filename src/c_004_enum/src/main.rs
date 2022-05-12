#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (Event::Unknown, line.to_owned());
    }
    let event = parts[0];
    let rest = parts[1].to_owned();

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, line.to_owned()),
    }
}

fn main() {
    let log = "BEGIN Transaction XYZ
UPDATE S1 Site
DELETE S1 Site";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
