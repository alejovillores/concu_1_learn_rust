/*
Un viejo banquero retirado se mudó a vivir al campo lejos de toda la tecnología.
Para vivir, invierte la plata que hizo durante sus años de trabajo mediante los amigos que tiene en
diversos fondos de inversión.
Al inicio de cada semana les envía por correo el dinero para invertir a sus amigos; luego espera
hasta el final de la semana a que le envíen a su buzón el resultado de esas inversiones.
Modelar la situación planteada en Rust, considerando que:
 ◦ A todos los amigos les envía el mismo monto
 ◦ Que el dinero resultante lo vuelve a invertir la próxima semana
 ◦ Que las inversiones pueden dar una ganancia entre -50% y 50% de lo invertido

Han habido algunos problemas de organización, y los hijos del señor banquero, si
bien desean que los inversores sigan trabajando de forma autonóma, deben
tomar el dinero de la cuenta y devolverlo al final de la semana, analogamente a
como se hacía cuando el señor banquero se encontraba vivo. Recordamos cada
inversor debe tomar exáctamente el mismo dinero.

*/

// TODO: para poder correrlo hay que agregar rand al proyecto
// cargo add rand

use std::thread;
use std::sync::Barrier;
use std::sync::Arc;
use std::sync::RwLock;
use rand::{Rng, thread_rng};

fn inversor(friend: i32, account: Arc<RwLock<i32>>,begin_week_barrier: Arc<Barrier>, account_barrier: Arc<Barrier>){
    loop {
        begin_week_barrier.wait();
        let mut base_inversion = 0;

        { 
            if let Ok(acc) = account.read() {
                base_inversion += *acc/FRIENDS;
            }
        }

        println!("[Friend {friend}] has {base_inversion} to invert initially");
        account_barrier.wait();

        let inversion_rate = thread_rng().gen_range(0.5.. 1.5);
        let earned_inversion = base_inversion as f32 * inversion_rate;
        println!("[Friend {friend}] earned {earned_inversion}");

        {
            if let Ok(mut acc) = account.write() {
                *acc += (-base_inversion) + earned_inversion as i32;
            }
        }

        println!("[Friend {friend}] finished inverting for this week");

    }
}


const INITIAL_BALANCE:i32 = 10_000;
const FRIENDS:i32 = 5;
fn main(){

    let account = Arc::new(RwLock::new(INITIAL_BALANCE));
    let begin_week_barrier = Arc::new(Barrier::new(FRIENDS as usize));
    let account_barrier = Arc::new(Barrier::new(FRIENDS as usize));
    let mut handles_vector = Vec::with_capacity(FRIENDS as usize);

    for i in 0..FRIENDS {
        let account_clone = account.clone();
        let begin_week_barrier_clone = begin_week_barrier.clone();
        let account_barrier_clone = account_barrier.clone();
        let handle = thread::spawn(move || {
            inversor(i, account_clone, begin_week_barrier_clone, account_barrier_clone);
        });
        handles_vector.push(handle);
    }

    for h in handles_vector.into_iter() {
        h.join().expect("Ok");
    }
}