# HW3

作业3实现了hash map的宏定义，直接在 `main.rs` 中实现，又采用泛型实现了栈结构，在 `lifo.rs` 中实现，智能指针在 `myrc.rs` 中实现。

输入 `cargo run` 可以直接看到输出的测试结果：

```
Test macro hash_mao: 
{"one": 1, "two": 2, "three": 3}

Test lifo: 
Poped value Some(5)
Poped value Some(4)
Poped value Some(6)
Poped value Some(3)
Poped value Some(2)
Poped value Some(1)
Poped value None

Test myrc: 
rc1: 42
rc1 ref count: 3
rc2 ref count: 3
rc3 ref count: 3
rc1 ref count: 2
rc3 ref count: 2
```

