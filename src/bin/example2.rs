use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};


fn main() {
    
    let mut vec:Vec<JoinHandle<()>> = Vec::with_capacity(10);
    let vec2 = vec![1,2,3,4,5,6,7,8,9,10];
    
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    {
        for i in 0..10 {
            let sender_clone = sender.clone();
    
            let handler = thread::spawn(move || {
               println!("thread n: {}" ,{i}); 
               {
                   sender_clone.send(i).expect("fallo el sender");
                   println!("murio el sender_clone")
               }
               // sender_clone se desconecta
            });
            vec.push(handler); // --> no hacer
        }

        drop(sender)

    }


    while let Ok(n) = receiver.recv() {

        let numero = vec2[n as usize];
        println!("vino el numero {}",n);
        println!("en el vector estaba {}",numero);
    
    }

    for v in vec {

        if let Ok(_) = v.join() {
            println!("joined")
        }
    }
    
}


