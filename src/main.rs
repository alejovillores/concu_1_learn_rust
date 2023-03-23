use std::thread;


const AMIGOS: i32 = 5;

fn main() {
    let  total = 0;
    let  _saldo: f64 = 100_000.0;
    let mut vec = Vec::with_capacity(AMIGOS as usize);

    for id in 1..AMIGOS  {
        let thread = thread::spawn(move || {
            println!("this is thread n: {} and the total is: {}", id, total);
        });
        vec.push(thread);
    }

    for v in vec {

        if let Ok(_) = v.join() {
            println!("joined")
        }
    }

    println!("Main thread")

}
