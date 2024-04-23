第一课作业题目：  
一、添加一个一层子模块，循环打印从a-Z之间的所有字符  
二、添加一个二层子模块，循环打印从A-z之间的所有字符  
三、使用Cargo编译运行此工程  

第一课作业解答：  
一、第一层子模块char_helper, 函数a_to_z()实现循环打印从a-Z之间的所有字符    
二、第二层子模块util/data_helper, 函数a_to_z()实现循环打印从A-z之间的所有字符  
三、main中依次调用两个模块的打印函数，通过cargo run命令运行，运行截图：  
<img width="1426" alt="image" src="https://github.com/lihuineo/TinTinRustCourse/assets/161575076/d58a4bc1-1380-40b1-a058-e37ff9b81bf9">


第二课作业题目：  
模仿老师的思路，自己对所有权、不可变引用、可变引用这三者的规则或特性做一个集中的总结，写一个笔记列表

第二课作业解答  
ownership_analyzer

第三课作业题目：  
请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作  

第三课作业解答  
核心逻辑：student_system实现了CRUD操作，依次包括append_info、delete_info、set_info、get_info  
测试：crud_test

第四课作业题目：  
1.使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
同时，说明其上两种不同实现方法的区别
2.搜索相关文档，为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用

第四课作业解答  
1.enum_wrapper.rs和trait_object.rs分别定义了三种类型及各自方法。  enum_wrapper.rs枚举类Course定义了Math\Biology\Psychology三种类型，这三个类型都实现credit()方法获取学分。通过Course的get_credit()获取不同课程的学分；  
trait_object.rs中Math\Biology\Psychology通过实现trait Evaluate的credit()方法获取学分。通过get_credit(a: &dyn Evaluate)依次调用其credit();  
enum_wrapper_test()单元测试中，将枚举类型放入Vec中进行遍历并对值进行检验；  
trait_object_test()单元测试中中，将三种类型放入Vec中进行遍历并对值进行检验；  
两种实现方法的区别：枚举的方式需要自定义枚举可能的值，如果需要扩展枚举类型需要修改枚举本身的定义，往往很多时候是不能修改已有的枚举类型的，因此这种方式的扩展性相对较差；而通过Trait Object的方式，如果需要添加有别的类型，只要实现Trait声明的方法即可，扩展性好  
2.add_opt.rs定义trait Operation, add_with用于进行+运算; 自定义RealNum和ComplexNum实现add_with方法；make_add(a: &mut dyn Operation, num)使用Trait Object实现求和方法的调用；  
add_opt_test()单元测试
