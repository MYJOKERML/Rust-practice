# HW4

本次作业实现了最基础的异步Runtime，成功实现了block_on with waker和multitasks，多线程并未实现。

其中 `src/multitask/pollster.rs` 中实现了block_on with waker。这里有个不优雅的做法，因为我在 `multitask.rs` 中引入了 `mod pollster` ，导致编译器一定要在 `src/multitask/` 中去寻找 `pollster.rs` 所以只好新建了 `src/multitask/` 并把 `pollster.rs` 放进去。

输入 `cargo run` 可以得到三个输出

```
Test Waker!
Test multitask! Hello, world!
Test multitask! Hello, world 2!
```

