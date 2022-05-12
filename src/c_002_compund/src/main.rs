//! Simulating files one step at a time.
use std::fmt;



#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl fmt::Display for FileState{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self{
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}


/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl fmt::Display for File{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    fn new(name: &str) -> File {
        File {
            name: name.to_owned(),
            data: vec![],
            state: FileState::Closed
        }
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read (self: &File, save_to: &mut Vec<u8> ) -> Result<usize, String>{
        if self.state != FileState::Open{
            return Err ("File must be open for reading".to_owned());
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
    
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut data = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("f1.txt", &mut data);

    let mut buf: Vec<u8> = vec![];

    if f1.read(&mut buf).is_err() {
        println!("Error checking is working");
    }

    f1 = open(f1).unwrap();
    let buf_size = f1.read(&mut buf).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buf);

    println!("DEBUG -> {:?}", f1);
    println!("DISPLAY -> {}", f1);
    println!("{} is {} bytes long", &f1.name, buf_size);
    println!("{}", text);

}

