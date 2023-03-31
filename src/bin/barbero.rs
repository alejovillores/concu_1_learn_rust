/* 
Una barberia tiene una sala de espera con sillas.
Si la barberia esta vacia, el barbero se pone a dormir.
Si un cliente entra y el barbero esta atendiendo, se sienta
en una de las sillas y espera su turno.
Si esta durmiendo, lo despierta.
El cliente espera a que le corten el pelo
*/



/* 

Tendriamos que tener 3 semaforos, donde
    * semaforo si hay clientes esperando
    * semaforo barbero dormido
    * semaforo cliente siendo atendido

    La idea es ir sumando 1 cada vez que viene un cliente y restar cuando se va
    Debemos a su vez crear una especia de espacio sincronico en el que el barbero
    le esta cortando al cliente
*/

use std::{sync::{Arc, atomic::AtomicI32}, thread, time::Duration};



fn main() {

    const N:usize = 5;

    let barbero_listo = Arc::new(Semaphore::new(0)); 
    // 0 porque cuando este 1 significa que esta despierto
    let cliente_esperando = Arc::new(Semaphore::new(0));
    // comenzamos con 0 clientes esperando
    let corte_listo = Arc::new(Semaphore::new(0));
    // 0 porque el corte todavia no esta listo

    let id_cliente = Arc::new(AtomicI32::new(0));



    let barbero_listo_bar = barbero_listo.clone();
    let cliente_esperando_bar = cliente_esperando.clone();
    let corte_listo_bar = corte_listo.clone();


    // Thread del barbero
    let barbero = thread::spawn(move || loop {
        println!("[barbero] - Esta esperando a un cliente");
        cliente_esperando_bar.acquire(); // cuando haya un 1 puedo adquirir el recurso

        /* Aca es donde hacemos la sincronizacion entre el cliente y el barbero */
        barbero_listo_bar.release(); // Al hacerle realease sumo 1 al contador, entonces se despierta

        println!("[barbero] - Cortando el pelo");

        thread::sleep(Duration::from_secs(2));

        corte_listo_bar.release(); // le sumo 1 al corte indicando que ya termino.

        println!("[barbero] - Termino con su cliente");

    });


    // Thread de los clientes

    let clientes = (0..(N+1)).map(|_| {

        let barbero_listo_cli = barbero_listo.clone();
        let cliente_esperando_cli = cliente_esperando.clone();
        let corte_listo_cli = corte_listo.clone();
        let id_cliente_cli = id_cliente.clone();

        thread::spawn(move || {

            let me = id_cliente_cli.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            println!("[cliente {} ] - Entro a la barberia", me);
            cliente_esperando_cli.release(); // sumo 1 a los clientes esperando

            println!("[cliente {} ] - Esperando al barbero", me);
            barbero_listo_cli.acquire(); // le resto 1 al barbero despierto (como si fuera el ultimo yo)

            println!("[cliente {} ] - Cortandose el pelo", me);

            corte_listo_cli.acquire(); // Si esta en 1, puedo decir que termino mi corte

            println!("[cliente {} ] - Me terminaron de cortar", me);
        });
    });


}