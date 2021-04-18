## 实现println 宏

1. volatile 高版本问题

volatile = "0.4.4" 版本不支持进行直接write进行操作，
缓存字符写入后续可以调整兼容到0.4.4 +

2. 使用 lazy_static，必须要在项目其实进行导入

```rust
#[macro_use]
extern crate lazy_static;
```
这个后续可以通过创建单独的lib进行解决

## 内容补充

- [通俗易懂的自旋锁和互斥锁](https://www.zhihu.com/question/66733477)



