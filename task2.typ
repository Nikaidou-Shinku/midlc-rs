#import "@preview/codelst:2.0.1": sourcecode

#align(center, text(17pt)[*任务二：语义分析与代码生成*])

#grid(
  columns: (1fr, 1fr),
  align(center)[*学号*：2021302926],
  align(center)[*姓名*：张逸飞],
)

#linebreak()

== 语义分析

命名冲突测例为 `testcases/task2-1.idl`，编译器成功拒绝编译并报错：

#sourcecode[```plain
Compile Error!
Duplicate definition of `num` in struct `A`
```]

#image("assets/Screenshot_20240607_215642.png")

#pagebreak()

未定义即使用测例为 `testcases/task2-4.idl`，编译器成功拒绝编译并报错：

#sourcecode[```plain
Compile Error!
No definition of `B` in struct `A`
```]

#image("assets/Screenshot_20240607_220719.png")

#pagebreak()

字面量类型检查测例为 `testcases/task2-6.idl`，编译器成功拒绝编译并报错：

#sourcecode[```plain
Compile Error!
Literal `a` can not be assigned to type `short`
```]

#image("assets/Screenshot_20240607_224400.png")

#pagebreak()

== 代码生成

对于测例 `testcases/struct_nest.idl`，编译生成的代码如下：

#sourcecode[```cpp
namespace space {
struct A {
  short i1;
};
struct B {
  long i2;
  long foo;
  A i3;
};
}
```]

其余测例见 `testcases` 文件夹。
