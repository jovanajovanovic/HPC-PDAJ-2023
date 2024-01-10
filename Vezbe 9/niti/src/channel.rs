// kanali


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channel(){
    //mpsc = vise proizvodjaca, a jednog potrosaca, vise salje vrednost a jedan prima
    // tx = posiljaoc
    // tr = projemnik
    let (tx, tr) = mpsc::channel();


    thread::spawn(move || {
        let message = String::from("Hi from thread");
        tx.send(message).unwrap();
    });


    let recieved = tr.recv().unwrap();
    println!("Message - {}", recieved);
}

pub fn channel_1(){
    let (tx, tr) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("thread")
        ];

        for m in messages{
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    let receiver = thread::spawn(move || {
        for r in tr {
            println!("Got - {}", r);
        }
    }); 

    receiver.join().unwrap();


}


pub fn multiply_senders(){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    let sender_1 = thread::spawn(move || {
        let messages = vec![
            String::from("Sender 1 - Hi"),
            String::from("Sender 1 - from"),
            String::from("Sender 1 - thread")
        ];

        for m in messages{
            tx1.send(m).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    let sender_2 = thread::spawn(move || {
        let messages = vec![
            String::from("Sender 2 - Hi"),
            String::from("Sender 2 - from"),
            String::from("Sender 2 - thread")
        ];

        for m in messages{
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });

    let reciever = thread::spawn(move || {
        for m in rx{
            println!("{}", m);
        }
    });

    reciever.join().unwrap();
}