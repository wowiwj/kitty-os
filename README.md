# Rust OS 实践

本项目基于 https://os.phil-opp.com/ 教程，学习开发一个简单的OS
并记录一下过程中的心得

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
