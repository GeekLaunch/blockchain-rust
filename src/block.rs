use std::fmt::{ self, Debug, Formatter };

pub struct Block {
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block")
    }
}

impl Block {
}
