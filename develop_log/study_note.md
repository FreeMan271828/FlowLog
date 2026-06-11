## 单例模式

对于一些占用资源较大的业务组件，需要确保只初始化一次，因此需要单例模式。

- OnceLock

OnceLock通过

## 分支操作
在远程仓库有新分支，本地进行开发
1. 本地git checkout -b 新分支
2. 本地提交到远程
3. 远程合并
4. 本地切换回main分支并拉取

## 切片操作
对于一个Vec<usize>，通过*Vec<usize>可以获得实际上的数据结构：一个在栈上的结构体，包含三个字段：ptr（指向堆内存的指针）、capacity（容量）、len（当前长度）。
*Vec<usize>相当于调用Deref::deref(&Vec<usize>)，类型返回成[usize]
最后通过&获得一个胖指针，类型为 &[usize]。这个胖指针包含两部分信息：
- 数据指针：指向堆内存的起始地址。
- 长度（Length）
相比较，Vec多一个
- 容量（Capacity）
切片的数据结构就是一个胖指针。
通过as_slice就可以把一个Vec转化成一个切片（胖指针）

## Cow

Cow是一种Copy-On-Write，内部只是一种枚举，有借用状态和所有权状态。

```rust
pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

在内部的to_mut方法中，如果当前状态是借用，会创建一个新的对象再返回可变指针，如果是所有权则直接返回。
**所以这里的修改并不是全局的。**
Cow可以被使用在无法在编译期确定数据到底是静态的还是动态的，或者绝大多数情况是只读，极少数情况需要修改的场景。
