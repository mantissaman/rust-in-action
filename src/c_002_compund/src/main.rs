#![allow(unused_variables)]


#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File { name:name.to_owned(), data: vec![] }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
}


fn open(f: &File) -> bool {
    true
}

fn close(f: &File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut data = vec![114, 117, 115, 116, 33];
    let f1 = File::new_with_data("f1.txt", &mut data);


    let mut buf:Vec<u8> = vec![];

    open(&f1);
    let buf_size =read(&f1, &mut buf);
    close(&f1);

    let text = String::from_utf8_lossy(&buf);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, buf_size);
    println!("{}", text);
}
