pub trait Add {
    fn add_with(&mut self, num: i32);
}

pub struct RealNum {
    pub val: i32,
}

pub struct ComplexNum {
    pub re: i32,
    pub im: i32,
}

impl Add for RealNum {
    fn add_with(&mut self, num: i32) {
        self.val += num;
    }
}

impl Add for ComplexNum {
    fn add_with(&mut self, num: i32) {
        self.re += num;
        self.im += num;
    }
}

#[allow(dead_code)]
pub fn make_add(a: &mut dyn Add, num: i32) {
    a.add_with(num);
}
