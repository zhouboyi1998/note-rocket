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

### 📖 Language

[简体中文](./README.md) | English

### ⌛ Start

#### Install diesel_cli

* Install `diesel_cli` (Only install `diesel_cli` for `SQLite` database)

```bash
cargo install diesel_cli --no-default-features --features sqlite-bundled
```

#### Windows compile sqlite3.lib
* Find a directory similar to the following path in the `Visual Studio` installation directory

```
D:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.29.30133\bin\Hostx64\x64
```

* Copy `x64` folder to any directory
* Copy the `sqlite3.def` file from the `SQLite` installation directory to the new `x64` folder
* Run the command in the 'x64' directory
    * Compile `sqlite3.lib`

```bash
lib /DEF:sqlite3.def /MACHINE:X64
```

* Copy the generated `sqlite3.lib` and `sqlite3.exp` files to the `SQLite` installation directory
* In the `SQLite` installation directory, run the command using `PowerShell`
    * Copy the `sqlite3.dll` and `sqlite3.lib` files to the `.rustup` directory
    * Copy to `stable` or `nightly`, depending on the version of the `Rust` toolchain in use

```bash
# use rust toolchain stable version
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\sqlite3.dll

# use rust toolchain nightly version
cp sqlite3.lib c:\Users\11441\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\sqlite3.lib
cp sqlite3.dll c:\Users\11441\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\sqlite3.dll
```

#### Generate SQLite database file

* Run the command in the project root directory
* Use `diesel` to create the project's corresponding `SQLite` database file

```bash
diesel setup --database-url=database.sqlite
```

* Create the folder for `diesel SQL`

```bash
diesel migration generate create_card
```

* Write the create table operation in `up.sql`

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

* Write the drop table operation in `down.sql`

```sql
DROP TABLE note_card
```

* Create `schema.rs` file

```bash
diesel migration run --database-url=database.sqlite
```

#### Run

```bash
cargo run
```

#### Debug build

```bash
cargo build
```

#### Release build

```bash
cargo build --release
```

### 📜 Licence

[MIT License](https://opensource.org/licenses/MIT) Copyright (c) 2022 周博义
