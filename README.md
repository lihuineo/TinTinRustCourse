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