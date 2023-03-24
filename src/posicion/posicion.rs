use std::fmt::Error;

pub struct Posicion {x: i32, y:i32}

impl Posicion {
    pub fn new(x: i32, y:i32) -> Self {
        Self {
            x,y
        }
    }

    pub fn donde_estoy(&self) -> Result<(i32,i32), Error> {
        println!("estoy en {} ,{} ", self.x, self.y);

        //Ok((self.x, self.y))
        Err(Error)
    }
}