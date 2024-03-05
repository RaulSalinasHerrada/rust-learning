    //concurrent: different executes independently
    //parallel: different at the same time

    //good solutions :D

    //faster through complexity, no order in which it completes
    //- race condition
    //- deadlock: each waiting for a resource the other has
    //- probabilistic bugs

    //one-to-one threads. Maps to OS thread
    //green threads (program threads): M2N OS thread
    //runtime support!
    //only 1-1 on std. green threads are used with other

use std::{rc::Rc, sync::{mpsc, Arc, Mutex}, thread, time::Duration};


fn main() {
{
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{} from main", i);
        thread::sleep(Duration::from_millis(1));
    }

    //if main thread ends, the other ends as well
    //handle join waits for execution 
    handle.join().unwrap();
}

    let v = vec![1,2,3];

    // as rust doesn't know how much it will run, it doens't know if v is
    //going to be a valid reference
    //don't infer it they are borrowed, explicitly move and take ownership
    let spa = thread::spawn(move ||{
        println!("here's v: {:?}", v);
    });

    spa.join().unwrap();

    // don't share memory to communicate.
    // Instead, communicate to share memory
    
    //channel. Transmitter (upstream) and receiver (downstream)
    // methods put data into the channel. Other received 

    {
        let (tx, rx) = mpsc::channel();
        // tx tranmitter, rx receiver
        let tx2 = tx.clone();
        thread::spawn(move || {

            let vals = vec! [
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
            let msg = String::from("other message");
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
            // println!("msg is {}", msg); //cant
        });

        thread::spawn(move || {

            let vals = vec! [
                String::from("2 more"),
                String::from("2 msg"),
                String::from("2 for"),
                String::from("2 you"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
            let msg = String::from("it's other thread");
            tx2.send(msg).unwrap();
            
            // println!("msg is {}", msg); //cant
        });
        
        for received in rx {
            println!("Got: {}", received);
        }

        //recv: Blocks the thread. try_recv. doens't block
        // println!("Got: {} ",received)
    

    // ownership rule prevents to mess up

    }

    //other way to pass data is through a shared state
    // locks are a pain, but ownership ensure lock rules
    {
        // let m = Mutex::new(5);
        // {
        //     let mut num = m.lock().unwrap();
        //     *num = 6;
        // }

        // println!("m = {:?}", m);
        // Arc: rc for threads
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10{
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                
                *num += 1;
                
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("result: {}", counter.lock().unwrap());

    }

}
