pub mod add_opt;
pub mod char_helper;
pub mod enum_wrapper;
pub mod ownership_analyzer;
pub mod student_system;
pub mod trait_object;
pub mod utils;

use char_helper as char_helper_1;
use utils::data_helper as char_helper_2;

fn main() {
    char_helper_1::a_to_z();
    char_helper_2::a_to_z();

    ownership_analyzer::do_analyze();
}

#[cfg(test)]
mod tests {
    use crate::add_opt::make_add;

    #[test]
    fn crud_test() {
        use crate::student_system::{Student, StudentSystem};
        let mut mgt = StudentSystem::new();
        let stu1 = Student {
            id: "29-111",
            name: "zhangsan",
            community: "swimming",
            class: "freshman",
            subject: "math",
        };
        let stu2 = Student {
            id: "29-222",
            name: "lisi",
            community: "tennis",
            class: "senior",
            subject: "astronomy",
        };
        let stu3 = Student {
            id: "29-333",
            name: "wangwu",
            community: "cooking",
            class: "junior",
            subject: "biology",
        };

        mgt.append_info(&stu1);
        mgt.append_info(&stu2);
        mgt.append_info(&stu3);

        let val1 = mgt.get_info("29-111").unwrap();
        let val2 = mgt.get_info("29-222").unwrap();
        let val3 = mgt.get_info("29-333").unwrap();

        assert_eq!(val1.name, "zhangsan");
        assert_eq!(val1.community, "swimming");

        assert_eq!(val2.class, "senior");
        assert_eq!(val2.subject, "astronomy");

        assert_eq!(val3.class, "junior");
        assert_eq!(val3.community, "cooking");

        let stu4 = Student {
            subject: "psychology",
            ..stu1
        };
        mgt.set_info(&stu4);
        let val4 = mgt.get_info("29-111").unwrap();
        assert_eq!(val4.name, "zhangsan");
        assert_eq!(val4.subject, "psychology");

        let stu5 = Student {
            community: "basketball",
            class: "junior",
            ..stu2
        };
        mgt.set_info(&stu5);
        let val5 = mgt.get_info("29-222").unwrap();
        assert_eq!(val5.name, "lisi");
        assert_eq!(val5.community, "basketball");
        assert_eq!(val5.class, "junior");

        mgt.delete_info("29-111");
        assert!(mgt.get_info("29-111").is_none());

        mgt.delete_info("29-22");
        assert!(!mgt.get_info("29-222").is_none());
    }

    #[test]
    fn enum_wrapper_test() {
        use crate::enum_wrapper::{Biology, Course, Math, Psychology};

        let math = Course::Math(Math {});
        let biology = Course::Biology(Biology {});
        let psychology = Course::Psychology(Psychology {});
        let crs: Vec<Course> = vec![math, biology, psychology];

        let mut credit_sum = 0;

        for e in crs.iter() {
            let c = e.get_credit();
            println!("credit: {c}");
            credit_sum += c;
        }

        assert_eq!(credit_sum, 53);
    }

    #[test]
    fn trait_object_test() {
        use crate::trait_object::{Biology, Course, Evaluate, Math, Psychology};

        let math = Math {};
        let biology = Biology {};
        let psychology = Psychology {};
        let crs: Vec<&dyn Evaluate> = vec![&math, &biology, &psychology];
        let mut credit_sum = 0;

        for e in crs.iter() {
            let c = Course::get_credit(*e);
            println!("credit: {c}");
            credit_sum += c;
        }

        assert_eq!(credit_sum, 53);
    }

    #[test]
    fn add_opt_test() {
        use crate::add_opt::{ComplexNum, RealNum};

        let mut r = RealNum { val: 7 };
        make_add(&mut r, 2);
        assert_eq!(r.val, 9);

        let mut c = ComplexNum { re: 1, im: 2 };
        make_add(&mut c, 3);
        assert_eq!(c.re, 4);
        assert_eq!(c.im, 5);
    }
}
