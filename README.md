<h1 align="center">📔 note-rocket</h1>

<p align="center">
<a target="_blank" href="https://github.com/zhouboyi1998/note-rocket"> 
<img src="https://img.shields.io/github/stars/zhouboyi1998/note-rocket?logo=github">
</a>
<a target="_blank" href="https://opensource.org/licenses/MIT"> 
<img src="https://img.shields.io/badge/license-MIT-red"> 
</a> 
<img src="https://img.shields.io/badge/Rust-1.61.0-orange">
<img src="https://img.shields.io/badge/Rocket-0.5.0 rc.2-red">
<img src="https://img.shields.io/badge/Diesel-1.4.8-red">
</p>

### 📖 语言

简体中文 | [English](./README.en.md)

### ⌛ 开始

#### Windows 安装 diesel_cli

* 安装 `diesel_cli` (只安装 `SQLite` 数据库的 `diesel_cli`)

```bash
cargo install diesel_cli --no-default-features --features sqlite-bundled
```

#### 打包 sqlite3.lib
* 在 `Visual Studio` 安装目录下找到类似以下路径的目录

```
D:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.29.30133\bin\Hostx64\x64
```

* 复制 `x64` 文件夹到任意目录下
* 将 `SQLite` 安装目录下的 `sqlite3.def` 文件复制到新的 `x64` 文件夹中
* 在 `x64` 目录下执行以下命令
    * 打包 `sqlite3.lib`

```bash
lib /DEF:sqlite3.def /MACHINE:X64
```

* 将生成的 `sqlite3.lib` 文件和 `sqlite3.exp` 文件复制到 `SQLite` 安装目录下
* 在 `SQLite` 安装目录下，使用 `PowerShell` 执行以下命令
    * 将 `sqlite3.dll` 文件和 `sqlite3.lib` 文件复制到 `.rustup` 目录下
    * 根据使用的 `Rust` 工具链版本，复制到 `stable` 或 `nightly` 对应的目录下

```bash
# use rust toolchain stable version
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\sqlite3.dll

# use rust toolchain nightly version
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\sqlite3.dll
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

* `up.sql` 中写创建表的操作，项目启动时执行

```sql
CREATE TABLE note_card (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    tip VARCHAR NOT NULL,
    extra VARCHAR NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
```

* `down.sql` 中写删除表的操作，项目停止时执行

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

#### 构建

```bash
cargo build
```

### 📜 开源协议

[MIT License](https://opensource.org/licenses/MIT) Copyright (c) 2022 周博义
