pub trait Evaluate {
    fn credit(&self) -> u8;
}

pub struct Math {}

pub struct Biology {}

pub struct Psychology {}

impl Evaluate for Math {
    fn credit(&self) -> u8 {
        15
    }
}

impl Evaluate for Biology {
    fn credit(&self) -> u8 {
        18
    }
}

impl Evaluate for Psychology {
    fn credit(&self) -> u8 {
        20
    }
}
pub enum Course {
    Psychology(Psychology),
    Math(Math),
    Biology(Biology),
}

impl Course {
    pub fn get_credit(&self) -> u8 {
        match *self {
            Course::Math(ref math) => math.credit(),
            Course::Biology(ref biology) => biology.credit(),
            Course::Psychology(ref psychology) => psychology.credit(),
        }
    }
}
