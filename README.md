第一课作业题目：  
1.添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符  
2.添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符  
3.使用Cargo编译运行此工程  

第一课作业解答：  
1.第一层子模块char_helper, 函数a_to_z()实现循环打印从'a'~'Z' 之间的所有字符    
2.第二层子模块util/data_helper, 函数a_to_z()实现循环打印从'A'~'z'之间的所有字符  
3.main中依次调用两个模块的打印函数，通过cargo run命令运行，运行截图：  
<img width="1426" alt="image" src="https://github.com/lihuineo/TinTinRustCourse/assets/161575076/d58a4bc1-1380-40b1-a058-e37ff9b81bf9">
