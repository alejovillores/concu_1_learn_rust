/* 
Consideremos que para fumar un cigarrillo se necesitan 3
ingredientes: tabaco, papel y fósforos.
Hay 3 fumadores alrededor de una mesa. Cada uno tiene una
cantidad infinita de uno solo de los ingredientes y necesita los
otros dos para fumar. Cada fumador posee un ingrediente
distinto y le faltan los otros dos.
Existe además un agente que pone aleatoriamente dos
ingredientes en la mesa. El fumador que los necesita los
tomará para hacer su cigarrillo y fumará un rato. Cuando el
fumador termina, el agente pone otros dos ingredientes.


Voy a tener 3 semaforos, uno por ingrediente
Voy a tener otros 3 mas, uno por cada fumador
Voy a tener un semaforo que despierte al proovedor
Voy a tener un pusher, que basicamente lo podemos pensar como 
la entidad que dado los ingredientes despierta al fumador correcto

*/

use std::{sync::{Arc, RwLock, RwLockWriteGuard}, thread};





fn main() {

    #[derive(Debug, Clone)]
    enum Ingredientes {
        Tabaco,
        Papel,
        Fosforos
    }

    const INGREDIENTES: [Ingredientes; 3] = [Ingredientes::Tabaco,Ingredientes::Papel,Ingredientes::Fosforos];

    let semaforo_agente = Arc::new(Semaphore::new(1)); 
    // 1 porque cuando el agente haga acquire, significa que puede poner en la mesa ingrediente
    let semaforos_ingredientes = Arc::new((0..3)
                                                                    .map(|_| {Semaphore::new(0)})
                                                                    .collect());

    // 0 porque cuando le hago un release, significa que hay 1 de ese ingrediente


    let mesa = Arc::new(RwLock::new(Vec::with_capacity(1)));




    let agente_semaforo_agente = semaforo_agente.clone();
    let agente_semaforo_ingredientes = semaforos_ingredientes.clone();

    // El agente espera a que le avisen que puede colocar 
    // y prende el semaforo del ingrediente que coloca.
    let agente = thread::spawn(move || loop {

        println!("[agente] - esperando para poder poner ingredientes");
        agente_semaforo_agente.acquire();
        
        println!("[agente] - poniendo ingredientes en la mesa");

        let mut ingredientes = INGREDIENTES.to_vec();

        ingredientes.swap(1, 2);
        let selected = &ingredientes[1..2];
        for s in selected {
            println!("[agente] - Colocando {:?} ", s);
            agente_semaforo_ingredientes.[*s as usize].release();
        }
    });

    let semaforos_fumadores = Arc::new((0..3).map(|_| {Semaphore::new(0)}).collect());


    // Los threads de acompañamiento por recurso se denominan pushers

    let pushers = (0..3).map(|i| {
        let pusher_semaforo_ingredientes = semaforos_ingredientes.clone();
        let pusher_semaforo_fumadores = semaforos_fumadores.clone();
        let pusher_mesa = mesa.clone();

        thread::spawn(move || loop {
            
            // Que ingrediente soy?
            let ingrediente = INGREDIENTES.get(i);

        
            pusher_semaforo_ingredientes[ingrediente].acquire();
            println!("[ingrediente {:?}] - Esta en la mesa" , ingrediente);

            // como estoy en la mesa tengo que modificar la mesa

            if let Ok( mut mesa) = pusher_mesa.write() {
                mesa[i] = true;

                // me fijo si tengo 2 true, para notificar al fumador
                notificar_fumador(&mut mesa, &pusher_semaforo_fumadores );

            }
        })
    });

    // quedaria hacer los threads para los fumadores y que cada fumador, imprima que esta fumando
    // y libere al agente.


    fn notificar_fumador(mesa: &mut RwLockWriteGuard<Vec<bool>>>, semaforos :&Arc<Unknow>) {

        if mesa[Ingredientes::Tabaco] {
            if mesa[Ingredientes::Papel]{
                semaforos[Ingredientes::Fosforos].release(); // despierto al fumador
                mesa[Ingredientes::Tabaco] = false;
                mesa[Ingredientes::Papel] = false;
            }

            if mesa[Ingredientes::Fosforos]{
                semaforos[Ingredientes::Papel].release(); // despierto al fumador
                mesa[Ingredientes::Tabaco] = false;
                mesa[Ingredientes::Fosforos] = false;

            }
        
        }
        else if mesa[Ingredientes::Papel] {
            if mesa[Ingredientes::Fosforos]{
                semaforos[Ingredientes::Tabaco].release(); // despierto al fumador
                mesa[Ingredientes::Fosforos] = false;
                mesa[Ingredientes::Papel] = false;

            }        
        }
    
    }


}

