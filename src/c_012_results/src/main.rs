use std::{path::Path, fs::File, io::Read, string::FromUtf8Error};

fn main() {
    let path = Path::new("data.txt");
    let file = File::open(&path);
    

    let mut file= match file {
        Ok(f) => f,
        Err(err) => panic!("Error while opening file: {}", err)
    };

    let mut s = String::new();
    let mut size=0;
    let _read=file.read_to_string(&mut s);
    if let Ok(b) = _read {
        size=b;
    }

    println!("Content: {} : {} bytes", s, size);


    let v = vec!["1", "2", "3", "4"];
    let x= v.get(5).ok_or("Value not found");


    early_return();
}


fn early_return() ->() {
    let invalid_str_bytes = vec![197, 198];
    let data = str_upper_match(invalid_str_bytes);
    println!("String: {:?}", data);
}

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {

    let data = String::from_utf8(str);
    let ret = data?;

    println!("Converted: {}", ret);
    Ok(ret)
}