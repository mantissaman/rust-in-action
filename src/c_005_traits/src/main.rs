#![allow(unused_variables)]


trait Readable {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug)]
struct File;

impl Readable for File {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>{
        Ok(0)
    }
}


fn main() {
    let f = File{};
    let mut buf =vec![];
    let read_bytes = f.read(&mut buf).unwrap();
    println!("{} byte(s) read from {:?}", read_bytes, f);
}
