pub trait Operation {
    fn add_with(&mut self, num: i32);
}

pub struct RealNum {
    pub val: i32,
}

pub struct ComplexNum {
    pub re: i32,
    pub im: i32,
}

impl Operation for RealNum {
    fn add_with(&mut self, num: i32) {
        self.val += num;
    }
}

impl Operation for ComplexNum {
    fn add_with(&mut self, num: i32) {
        self.re += num;
        self.im += num;
    }
}

#[allow(dead_code)]
pub fn make_add(a: &mut dyn Operation, num: i32) {
    a.add_with(num);
}
