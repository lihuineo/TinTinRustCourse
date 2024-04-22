pub trait Evaluate {
    fn get_credit(&self) -> u8;
}

pub struct Math {}

pub struct Biology {}

pub struct Psychology {}

impl Evaluate for Math {
    fn get_credit(&self) -> u8 {
        15
    }
}

impl Evaluate for Biology {
    fn get_credit(&self) -> u8 {
        18
    }
}

impl Evaluate for Psychology {
    fn get_credit(&self) -> u8 {
        20
    }
}

pub struct Course {}
impl Course {
    pub fn get_credit(a: &dyn Evaluate) -> u8 {
        a.get_credit()
    }
}
