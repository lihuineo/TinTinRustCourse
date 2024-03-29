use std::collections::HashMap;

pub struct StudentSystem<'a> {
    map: HashMap<&'a str, &'a Student>,
}

impl<'a> StudentSystem<'a> {
    pub fn new() -> Self {
        StudentSystem {
            map: HashMap::with_capacity(200),
        }
    }

    /*
    append_info() //增
    delete_info() //删
    set_info() //改
    get_info() //查
    */
    pub fn append_info(&mut self, stu: &'a Student) {
        assert!(!stu.id.is_empty());
        self.map.insert(stu.id, &stu);
    }

    pub fn delete_info(&mut self, id: &str) {
        assert!(!id.is_empty());
        self.map.remove(id);
    }

    pub fn set_info(&mut self, stu: &'a Student) {
        assert!(!stu.id.is_empty());
        self.map.insert(&stu.id, &stu);
    }

    pub fn get_info(&self, id: &str) -> Option<&Student> {
        assert!(!self.map.is_empty());
        self.map.get(&id).copied()
    }
}

/*
学生信息
 */
pub struct Student {
    pub id: &'static str,        //学生id
    pub name: &'static str,      //学生姓名
    pub community: &'static str, //社区
    pub class: &'static str,     //班级
    pub subject: &'static str,   //课程
}
