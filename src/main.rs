use std::sync::mpsc;
use std::thread;

fn main() {
    let (send, recv) = mpsc::channel();
    let thread = thread::spawn(move || {
        let v = vec![1, 2, 3];
        println!("\tsending {:?}", v);
        send.send(v).unwrap();
        println!("\tsent");
    });


    println!("receiving");
    
    let v= recv.recv().unwrap();

    println!("received {:?}", v);

    thread.join().unwrap();
}