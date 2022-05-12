// Lifetime, Ownerships and Borrowing

type Message=String;


#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox
}
impl CubeSat{
    fn recv(&mut self) -> Option<Message>{
        self.mailbox.messages.pop()
    }
}

struct GroundStation;

impl GroundStation{
    fn send(&self, to:&mut CubeSat, msg: Message){
        to.mailbox.messages.push(msg);
    }
}

#[derive(Debug)]
enum StatusMessage{
    Ok,
}

// fn check_status(sat_id: CubeSat) ->StatusMessage {
//     StatusMessage::Ok
// }

fn main() {
    let base = GroundStation{};

    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { 
            messages: vec![] 
        },
    };

    println!("t0 : {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello Satellite A!!"));

    println!("t1 : {:?}", sat_a);

    let sat_a_read_msg = sat_a.recv();
    println!("msg read by sattelite : {:?}", sat_a_read_msg.unwrap());
    println!("t2 : {:?}", sat_a);
    

}
    
