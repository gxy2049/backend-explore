# rust注意事项

1. Rust 是一种 预先编译（ahead-of-time compiled）语言
2. `./main # Windows 是 .\main.exe`

## 常用命令

1. `cargo check` 快速检查代码确保其可以编译，但并不产生可执行文件
2. `cargo build --release` 启用优化方式编译项目
3. 运行 cargo doc --open 命令会在本地构建所有依赖提供的文档，并在浏览器中打开

## 语法

1. 变量默认是不可变的，在变量名前使用 mut 来使一个变量可变
2. :: 语法表明 new 是 String 类型的一个 关联函数（associated function）
3. & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝

## 概念

1. 库 crate 可以包含任意能被其他程序使用的代码，但不能独立执行
2. 语义化版本的几种表达?:
3. Cargo.lock 文件确保可重现构建
4. 变量遮蔽（shadowing）
5. 用户必须按下 enter 才能让 read_line 返回并提交他们的猜测，这会在字符串中附加一个换行符（newline）
6. continue，告诉程序进入 loop 的下一次迭代
