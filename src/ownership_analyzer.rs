use std::collections::HashMap;
/*
所有权知识点总结

1.Rust语言使用所有权系统对内存进行有效管理。所有权系统包括所有权、不可变引用、可变引用几个部分

2.所有权规则
1）所有权讨论范围只针对在堆上动态创建的对象类型，而原始类型是在栈上进行分配和管理的，因此不适用所有权规则
2）每一个内存空间（资源）都有一个变量与之绑定，这个变量拥有这个资源的所有权，即它是这个资源的所有者；
3）任一时刻这个资源只有一个所有者，如果其他变量需要对这个资源进行读写，那么它可以通过转移（move）所有权的方式，或者借用（borrow）所有权的方式获得这个资源的所有权
4）如果变量的生命周期结束，那么这个变量对资源的所有权消失，不允许变量的生命周期大于其绑定的资源的生命周期，否则会产生悬垂指针（dangling pointer）

3.不可变引用
1）使用&type的形式进行引用，引用类型的变量在内存中的大小跟操作系统位数相等，32bit/64bit系统是4字节/8字节
2）不可变引用的变量不改变资源的所有权，但可以对资源进行操作
3）不可变引用作为函数形参，那么只能接收不可变引用类型的变量，并且在函数内部不能修改所引用的资源

4.可变引用
1）使用&mut type的形式进行引用，大小与不可变引用相同
2）可变引用的变量不改变资源的所有权，和不可变引用不同之处在于可以对资源进行读写操作，而不可变引用仅运行对资源进行读操作
3）可变引用的资源必须也是可变的（mut），作为函数的形参和实参都需要标记&mut进行使用
4）如果对一个资源既存在不可变引用，又存在可变引用，那么它们的生命周期不能重叠
 */

#[allow(dead_code)]
pub fn do_analyze() {
    /*
    1.String对象的所有权在作函数change_ownership形参的时候进行了move
    2.change_ownership执行完成之后，String对象被释放
    3.根据所有权规则，当变量绑定的对象释放后，变量s的生命周期也结束了
    4.此时如果执行35行会报错：borrow of moved value: `s`
     */
    let s = String::from("hello");
    change_ownership(s);
    // println!("{:?}", s);

    /*
    1. x默认是原始数据类型，不适用所有权机制
    2. make_copy函数执行之后，不存在所有权转移的问题，因此43行可以正常打印
     */
    let x = 6;
    make_copy(x);
    println!("{}", x);

    /*
    针对上述所有权在函数执行完成后被释放的问题，可以通过三种种方式进行解决：
    1.通过返回对象从而返回资源的所有权
    2.通过将对象进行深拷贝
    3.使用不可变引用
     */
    let s0 = String::from("tintin");
    let s1 = give_ownership_by_back(s0);
    give_ownership_by_copy(s1.clone());
    give_ownership_by_ref(&s1);
    println!("{s1}");

    /*
    1.HashMap中同样适用于所有权规则
    2.k作为String类型，在执行65行之后其所有权就转移给了map
    3.此时如果执行66行则会报错：borrow of moved value: `k`
    4.通过&k_string1不可变引用的方式，map1不会获得k_string1的所有权，从而59行可以正常执行
    */
    let (k, v) = (String::from("Sports"), String::from("Basketball"));
    let mut map = HashMap::new();
    map.insert(k, v);
    // println!("{}", k);
    let (k_string1, v_string1) = (String::from("Sports"), String::from("Tennis"));
    let mut map1 = HashMap::new();
    map1.insert(&k_string1, &v_string1);
    println!("{}", k_string1);

    /*
    不可变引用作为函数参数规则
    1.形参和实参都需要使用&类型
    2.函数中只能对引用参数进行读的操作
     */
    let vec1 = vec![1, 2, 4];
    let vec2 = vec![4, 5, 6];
    let ans = sum_vec(&vec1, &vec2);
    println!("{ans}");

    /*
    可变引用作为函数参数参数规则
    1.被引用变量本身是mut类型
    2.形参和实参都需要使用&mut类型
    3.函数中可对引用参数进行读写的操作
     */
    let mut vec = Vec::new();
    push_bigger_vec(&mut vec, 1);
    push_bigger_vec(&mut vec, 2);
    push_bigger_vec(&mut vec, 2);
    push_bigger_vec(&mut vec, 2);
    push_bigger_vec(&mut vec, 5);
    println!("{:?}", vec);

    /*
    同时存在不可变引用和可变引用规则
    1.引用的作用域是从其定义到最后一次使用
    2.一个时刻对资源的拥有所有权只能是一个变量，在引用释放前不能访问资源所有者。
    x是资源的所有权变量，y是可变引用，由于一个时刻只允许对资源的引用y其作用域是110行结束，如果此时执行113或114，那么将报错：cannot borrow `x` as immutable because it is also borrowed as mutable
    这是因为此时可变引用y还没有释放，必须先使用y，当y作用域结束被释放后，即110行结束，将所有权归还之后才能使用资源x
    3.可变引用和不可能的作用域不能重合，即不能同时存在可变引用和不可变引用
    如果执行117行，则会报错cannot borrow `x` as immutable because it is also borrowed as mutable
    因为此时同时存在可变引用y和不可变引用z、t
    4.可以同时存在多个不可变引用，z、t
    */
    let mut x = 27u32;
    let y = &mut x;
    *y += 1;
    println!("{y}");
    // println!("{x}");
    // println!("{}, {}", x, y);
    // println!("{}, {}", y, x);
    let z = &x;
    let t = &x;
    println!("{} {} {}", x, z, t);
    // println!("{} {} {}", y, z, t);
}

fn change_ownership(str: String) {
    println!("{:?}", str);
}

fn make_copy(int: i32) {
    println!("{}", int);
}

fn give_ownership_by_back(s: String) -> String {
    let str = String::from(s);
    println!("{str}");
    str
}

fn give_ownership_by_copy(s: String) {
    let str = String::from(s);
    println!("{str}");
}

fn give_ownership_by_ref(s: &String) {
    let str = String::from(s);
    println!("{str}");
}

fn sum_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let sum1: i32 = v1.iter().sum();
    let sum2: i32 = v2.iter().sum();
    sum1 + sum2
}

fn push_bigger_vec(v: &mut Vec<i32>, value: i32) {
    if v.is_empty() || v.get(v.len() - 1).unwrap() < &value {
        v.push(value);
    }
}
