# 学习RUST

## linux环境下安装RUST相关

1. 去 [这个网站 ](https://rust-lang.org/tools/install/) 后复制代码在终端里面输入`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. 去 [rustrover官网](https://www.jetbrains.com/zh-cn/rust/download/?section=linux)下载tar.gz包,在终端输入`cd /home/你的用户名/Downloads`随后进行`tar 你下载的安装包名`,之后`sudo mv 解压后的文件夹名 /opt/rustover`   

## 创建rust项目

在终端要创建的目录地址下输入`cargo new 要创建的项目名称`

## 在环境变量里面加入rustrover

在终端里输入`echo $SHELL`

1. 如果是/bin/bash 修改`~/.bashrc`
2. 如果是/bin/zsh 修改`~/.zshrc`

在终端使用 vim or nano 打开对应文件地址

```
//以zsh举例
nano ~/.zshrc
```

在文件的最后一行输入`export PATH="$PATH:/opt/rustrover/bin"`

然后在终端输入`sourse ~/.zshrc`

之后就可以直接在终端打开RustRover了

## 命令行编译执行rust文件

1. `rustc 你的rust文件.rs` 之后直接 `./main`
2. 进入你的项目路径后直接`cargo run //编译直接运行`或者`cargo bulid //只编译`