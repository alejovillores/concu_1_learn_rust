use rust_examples::posicion::posicion::Posicion;


fn main() {
    let vec1 = vec![1,2,3];

    let _vec2 = &vec1;

    let pos = Posicion::new(1,2);
    // mal pos.donde_estoy().unwrap();
    // mal pos.donde_estoy().expect("...");


    // try / catch 
    match pos.donde_estoy() {
        Ok(_) => {
            println!("me llego un ok")
            

        },
        Err(_) => {

            println!("Me llego un error")
            
        }
    }

    while let Ok(_) = pos.donde_estoy() {
        println!("me llego un ok");
    }

    if let Err(_) = pos.donde_estoy() {
        println!("Me LLEGO UN ERROR !!")
    }

    let x = 10;

    match x {
        10 => {println!("x es 10!")},
        0 ..= 9 => {println!("es menor a 10")}
        _ => {println!("no cumple con nada")}
    }

    println!("{:?}",vec1);
}

