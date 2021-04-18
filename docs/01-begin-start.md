## 第一部分 (2021-04-18 00:17:22)

1. 遇到的问题: 使用最新版本的 bootloader (0.10.1),编译会有警告出现

```bash
(If you're using the official bootloader crate, you need at least version 0.5.1)

Caused by:
    The `bootloader` dependency has not the right format: No `package.metadata.bootloader.target` key found in Cargo.toml of bootloader
    
    (If you're using the official bootloader crate, you need at least version 0.5.1)

```
暂时切换到 0.9.3 可以修复此问题


2. error[E0463]: can't find crate for `core`

这个问题可以根据教程下的回复解决
添加一个参数进行编译

```bash
cargo bootimage --verbose -Zbuild-std=core
```

编译完成之后就可以使用

```bash
cargo xrun
```
进行编译在 qeum上运行，后续会尝试在树莓派运行的可能性。

单纯的hello world 程序编译后的大小如下:

```bash
-rwxr-xr-x   1 wangju  staff    62K Apr 18 00:21 bootimage-kitty-os.bin
```
bin 程序带 bootloader，大概有62kb

关于开发中的 0xb8000，这个是开发操作系统中的魔数之一，可以参考这位UP的说明
https://www.bilibili.com/video/BV18K411w7Z2?p=8 (南京话，很地道)

关于引导程序，涉及到汇编，涉及到程序加电自检启动的过程，并且比较繁琐，暂时跳过。rust的 bootloader [https://github.com/rust-osdev/bootloader]已经很优秀了，可以抽空学习下里面的实现。

(end)