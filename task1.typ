#import "@preview/codelst:2.0.1": sourcecode

#align(center, text(17pt)[*任务一：词法与语法分析*])

#grid(
  columns: (1fr, 1fr),
  align(center)[*学号*：2021302926],
  align(center)[*姓名*：张逸飞],
)

#linebreak()

本项目使用 Rust 编程语言构建了 MIDL 的语法与词法分析器，下为 AST 定义：

#sourcecode[```rust
struct MidlAst(Vec<MidlDefinition>);

enum MidlDefinition {
  Struct(MidlStruct),
  Module(MidlModule),
}

struct MidlStruct {
  identifier: String,
  members: Vec<MidlStructMember>,
}

enum MidlStructMember {
  Simple(MidlStructSimpleMember),
  Array(MidlStructArrayMember),
}

struct MidlStructSimpleMember {
  r#type: String,
  identifier: String,
  initial_value: Option<String>,
}

struct MidlStructArrayMember {
  r#type: String,
  identifier: String,
  length: String,
  initial_value: Option<Vec<String>>,
}

struct MidlModule {
  identifier: String,
  members: Vec<MidlDefinition>,
}
```]

测试用例见 `testcases` 文件夹，测试用例结果见 `results` 文件夹。
