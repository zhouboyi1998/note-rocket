<h1 align="center">📔 note-rocket</h1>

<p align="center">
<a target="_blank" href="https://github.com/zhouboyi1998/note-rocket"> 
<img src="https://img.shields.io/github/stars/zhouboyi1998/note-rocket?logo=github" alt="">
</a>
<a target="_blank" href="https://opensource.org/licenses/MIT"> 
<img src="https://img.shields.io/badge/license-MIT-red" alt=""> 
</a> 
<img src="https://img.shields.io/badge/Rust-1.61.0-orange" alt="">
<img src="https://img.shields.io/badge/Rocket-0.5.0 rc.2-red" alt="">
<img src="https://img.shields.io/badge/Diesel-1.4.8-red" alt="">
</p>

### 📖 语言

简体中文 | [English](./README.en.md)

### ⌛ 开始

#### 安装 diesel_cli

* 安装 `diesel_cli` (只安装 `SQLite` 数据库的 `diesel_cli`)

```bash
cargo install diesel_cli --no-default-features --features sqlite-bundled
```

#### Windows 编译 sqlite3.lib

* 在 `Visual Studio` 安装目录下找到类似以下路径的目录

```
D:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.29.30133\bin\Hostx64\x64
```

* 复制 `x64` 文件夹到任意目录下
* 将 `SQLite` 安装目录下的 `sqlite3.def` 文件复制到新的 `x64` 文件夹中
* 在 `x64` 目录下执行命令编译 `sqlite3.lib`

```bash
lib /DEF:sqlite3.def /MACHINE:X64
```

* 将编译后的 `sqlite3.lib` 文件和 `sqlite3.exp` 文件复制回 `SQLite` 安装目录下
* 在 `SQLite` 安装目录下，使用 `PowerShell` 执行以下命令
    * 将 `sqlite3.dll` 文件和 `sqlite3.lib` 文件复制到 `.rustup` 目录下
    * 可以根据自己使用的 `Rust` 工具链版本
    * 将 `SQLite` 文件复制到对应的工具链目录下

###### 使用本机安装的最新稳定版本工具链 `stable`

```bash
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
```

```bash
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\sqlite3.dll
```

###### 使用本机安装的最新不稳定版本工具链 `nightly`

```bash
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
```

```bash
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\sqlite3.dll
```

###### 使用指定版本的工具链 `1.61.0`

```bash
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\1.61-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
```

```bash
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\1.61-x86_64-pc-windows-msvc\bin\sqlite3.dll
```

#### 生成 SQLite 数据库文件

* 在项目根目录下执行以下命令
* 使用 `diesel` 创建项目对应的 `SQLite` 数据库文件

```bash
diesel setup --database-url=database.sqlite
```

* 创建存放 `diesel SQL` 的文件夹

```bash
diesel migration generate create_card
```

* `up.sql` 中写创建表的操作

```sql
CREATE TABLE note_card
(
    id          BIGINT PRIMARY KEY NOT NULL,
    title       VARCHAR            NOT NULL,
    content     VARCHAR            NOT NULL,
    tip         VARCHAR            NOT NULL,
    extra       VARCHAR            NOT NULL,
    create_time TIMESTAMP          NOT NULL
)
```

* `down.sql` 中写删除表的操作

```sql
DROP TABLE note_card
```

* 创建 `schema.rs` 文件

```bash
diesel migration run --database-url=database.sqlite
```

#### 运行

```bash
cargo run
```

#### 调试构建

```bash
cargo build
```

#### 发布构建

```bash
cargo build --release
```

### 📜 开源协议

[MIT License](https://opensource.org/licenses/MIT) Copyright (c) 2022 周博义
