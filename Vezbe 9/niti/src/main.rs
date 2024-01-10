mod channel;
mod mutex;
use std::thread;
use std::time::Duration;

fn main() {

   let thread = thread::spawn(|| {
        for i in 1..10{
            println!("{} - Hi, from thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // thread.join().unwrap();

    for i in 1..5{
        println!("{} - Hi, from main", i);
        thread::sleep(Duration::from_millis(1));
    }


    thread.join().unwrap();

    // move i closure

    let v = vec![1, 2, 3, 4, 5];
    let thread2 = thread::spawn(move || {
        println!("v = {:?}", v);
    });


    // v.push(4);

    // drop(v); //PROBLEM!

    thread2.join().unwrap();

    channel::channel();

    channel::channel_1();

    channel::multiply_senders();


    // mutex
    mutex::example_mutex();
    mutex::mutex_shared();
}
