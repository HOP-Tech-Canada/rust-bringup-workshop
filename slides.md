---
marp: true
---

# Rust bring up
---
# Core concepts
---
##  Core concepts: Enums
```rust
/// AND relation
struct PhoneNumber {
    /// Country code, +1 in Canada
    country_code: String,
    /// Regional code, (819) in sherby
    regional_code: String,
    /// The actual number 562-5022
    number: String,
}

/// OR relation
pub enum ContactMethod {
    Phone(PhoneNumber),
    Email(String),
    Mail {
        postal_code: String,
        address: String,
    },
}
```
---
## Core concepts: Option and Result pattern
```rust
/// We maybe have something
enum Option<T> {
    Some(T),
    None
}

/// We have a result, or an error
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```
---
## Core concepts: Result and option usage
```rust
/// We maybe have something
fn get_first_negative(values: &[i32]) -> Option<i32>{
    for val in values {
        if val < 0 {
            return Some(val)
        }
    }
    None
} 

/// We have a result, or an error
fn get_value_from_file(file: File) -> Result<Item, ItemError> {
    let result = file.get();
    match result {
        Ok(val) => Ok(val.convert()),
        Err(e) => Err(e),
    }
}
```
---
## Core concepts: Map and "?" operator
```rust
/// We have a result, or an error
fn get_value_from_file(file: File) -> Result<Item, ItemError> {
    let result = file.get();
    match result {
        Ok(val) => Ok(val.convert()),
        Err(e) => Err(e),
    }
}
/// We can rewrite it as
fn get_value_from_file(file: File) -> Result<Item, ItemError> {
    // Note the question mark here
    let result = file.get().map(|v| v.convert());
    result 
}
/// Or sometimes easier to read
fn get_value_from_file(file: File) -> Result<Item, ItemError> {
    // Note the question mark here, if it's an error, we return the error, 
    // if not we continue with the value
    let result = file.get()?;
    let val = result.convert();
    Ok(val)
}
```
# Setup a new project
- Introduce the user to the cargo cli
    - new
    - run
    - test
    - doc
- Introduce to the Cargo.toml file
- Intoduce to the lib/main files
- Introduce to the tests system
- Enum vs Struct (Or vs And)
- Option and Result
`cargo new --bin hello-world
---

## Create a new project
The `cargo` command is always your friend, if you are not sure how to use it, don't hesitate to run `cargo --help` or `cargo <subcommand> --help`.

To create a new project named `hello-rust` simply run. 
```sh
cargo new hello-rust
```

## Library
To add library code in your project add the file `src/lib.rs` and add the content.
```rs
//! Here is documentation on the library

/// Here is some documentation to the function
fn hello_function() {
    println!("Hello, world!");
}
```
---
## Rustdoc
Now if you want to read the documentation of your library, run `cargo doc --open` and it will open the generated documentation for you library.

### Execise
Change the main to use the `hello_function` and run it using `cargo`.

---
# Basic application
- Cargo.toml
    - Add dependecies
- Concepts
    - Generic
    - Lifetime
    - Traits
- Introduce macros
  - Function like macros
    - {} vs {:?} in print
  - Derive macros
- More complext Match
---
## Cargo.toml example
```toml
[package]
name = "ad4130"
version = "0.1.0"
edition = "2021"

# Features are used for conditional compilation
[features]
default = ["log"]
# Log feature will enable the optional dependency
log = ["defmt"]

# Download from crates.io by default
[dependencies]
embedded-hal = { version = "1.0.0-alpha.9" }
# Optional dependency, creates a feature called "defmt", same as package name
defmt = { version = "0.3", optional = true}
# Can also get directly from git
embassy-executor = { version = "0.1.0", git = "https://github.com/embassy-rs/embassy"}
```
---
## Total trait project
Create a new project called `total` using `cargo new --bin total`. 

---
## Basic generic and trait
```rust
// A trait, think of it like an interface.
// Traits replace inheritance in other languages and are more versatile
trait  Total{
    fn total(&self) -> u32;
}
/// We take a generic type that implements the `Total` trait
fn print_total<T: Total>(val: &T) {
    println!("Total is: {}", val.total());
}

struct SomeStruct{
    val: u32,
    other: u32,
} 
impl Total for SomeStruct {
    fn total(&self) -> u32 {
        self.val + self.other
    }
}

struct AnotherStruct(u32);
impl Total for AnotherStruct {
    fn total(&self) -> u32 {
        self.0
    }
}

fn main() {
    let some = SomeStruct{
        val: 69,
        other: 420
    };
    let other = AnotherStruct(42);
    // Note that they have different types!
    print_total(&some);
    print_total(&other);
}
```
## Lifetime project
Create a new project called `lifetime` using `cargo new --bin lifetime`.
We will explore the concept of lifetime and the borrow checker. 

---
## Borrow checker
Rust enforce only **ONE** owner of a memory segment and **ONE** can mutate a memory segment.
- We can own using `Type`. Only ONE owner can exist
- We can borrow asing `&Type`
- We can mutably borrow using `&mut Type`
```rust 
// We can only read it 
fn borrow_vec(vec: &Vec) {}
// We can modify it
fn borrow_mut_vec(vec: &mut Vec) {}
// We own it, it will be destroyed at the end of the function, no garbage collection
fn own_vec(vec: Vec) {}
```
The next example contains commented lines of code, try to uncomment them and understand why it cannot compile.

---
## Basic lifetime example
```rust
// Equivalent to `fn borrow_vec<'a>(vec: &'a Vec)
fn borrow_vec(vec: &Vec<u8>) {
    println!("Vec borrowed: {:?}", vec);
}
fn borrow_mut_vec(vec: &mut Vec<u8>) {
    vec.push(3);
    println!("Vec borrowed mutably: {:?}", vec);
}
fn own_vec(vec: Vec<u8>) {
    println!("Vec owned: {:?}", vec);
}

pub struct BorrowedVec<'a> {
    vec: &'a Vec<u8>
}
// We implement functions here if we want to add some
impl<'a> BorrowedVec<'a>{
    fn print_borrowed_vec(&self) {
        println!("Struct vec borrowed is: {:?}", self.vec);
    }
}

fn main() {
    let mut vec = vec![0,1,2];
    borrow_vec(&vec);
    borrow_mut_vec(&mut vec);
    borrow_vec(&vec);
    own_vec(vec);
    // borrow_vec(&vec); // This would not work since vec was moved

    // Create ANOTHER vec
    let vec = vec![0,1,2];
    let borrower = BorrowedVec { vec: &vec };
    borrow_vec(&vec);
    // borrow_mut_vec(&mut vec); // Cannot borrow mutably and immutably
    // own_vec(vec); // Borrrowed, cannot own

    borrower.print_borrowed_vec();
}
```

---
## Lifetime project
Create a new project called `clap-example` using `cargo new --bin clap-example`. Let's explore macros.

---
## Basic macro usage from library
```rust 
/// Imports
use clap::Parser;
use clap::ValueEnum;

/// Define the arguments for our cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// An argument
    #[arg(short, long)]
    name: String,

    /// An optional "DataType" to pass
    #[arg(short, long)]
    data_type: Option<DataType>,
}

/// We define an enum, Note that enum can contain data
#[derive(ValueEnum, Debug, Copy, Clone)]
enum DataType {
    HeartRate,
    SkinTemp,
    Eda,
    Accel,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}, using argument {:?}", args.name, args.data_type);
    match args.data_type {
        Some(DataType::HeartRate) => println!("We got HR {:?}", args.data_type),
        None => println!("We got nothing"),
        _ => println!("We don't care!"),
    }
}
```
---
# Basic hello world embedded
- Create a new project
- Add basic dependencies for running
- Add .config
- Setup runner

---
## Required dependencies
We need to install the compiler for this target. 
`rustup` is Rust tool to mange toolchains
```sh
rustup target add thumbv7em-none-eabi
```

---
## nrf52840dk-hello project
Create a new project called `nrf52840dk-hello` using `cargo new --bin nrf52840dk-hello`. Make sure your dk is plugged and working properly. 

---
## Add the project dependencies
```toml
[dependencies]
# Cortex m register and peripheral access
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"]}

# Cortex m runtime  (what happens before the main fucntion)
cortex-m-rt = "0.7.3"

# Logging library
defmt = "0.3"

# We log via rtt (we could log via uart, usb, to memory, etc)
defmt-rtt = "0.3"

# Hal for the nrf52840, use spi, i2c, etc.
nrf52840-hal = "0.16.0"

# Panic behavior, use defmt
panic-probe = { version = "0.3", features = ["print-defmt"] } 
```
---

## Write the main application
``` rust
// We don't have an actual main function, we just jump to a function address.
#[no_main]
// We don't have the standard library since we are on baremetal
#[no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _; // panic behavior
 
#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, World!");
    // Pause the execution
    cortex_m::asm::bkpt();
}
```
---
# But linker script?
We need to add a linker script to the build, to do so, simply add a file called 
`memory.x` with the following content. 
```ld
/* Linker script for the nRF52 - WITHOUT SOFT DEVICE */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
```

---
## Make the linker script available
But currently the linker script exists but is not used, we need a wait to link it at build time. Rust does support build scripts. It does so by compiling and running a file called `build.rs` at the **root** of the project. This file is executed **at build time**, thus we can write file on the host, generate code, do checks and more.

---
## Build script example
```rs
//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Linker arguments for defmt, the logger that we use
    // You can ignore the magic here
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=--nmagic");

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
```
--- 
## But how?
Remember the `cortex-m-rt` dependency we added to our `Cargo.toml`? Well it tries to find this file at **build** time and use it to generate a more complete linker script with interrupt vector table and stuff like that. Then the complete linker script is used by the linker and we can now have a valid `elf` file!

---

# Run it
Try `cargo run`. It won't work since it's not for your x86 computer.

## How to run it?
Use `probe-run`, it's like `openocd` or `JLinkCommander` but integrates with the rust ecosystem. It flashes the device, run your code and print to console.
To install we can use cargo package manage to install binaries.
```sh
cargo install probe-run
```

To run, use the command
```sh
cargo build --target=thumbv7em-none-eabihf
probe-run --chip=nRF52840_xxAA ./target/thumbv7em-none-eabihf/debug/nrf52840dk-hello
```
But that sucks and we cannot use the usual `cargo run`. Let's see how to fix this.

---
# Setup runner
Add this content in .config/config.toml in your cargo project.

```toml
# For all arm targets let use probe-run
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-run --chip nRF52840_xxAA"

# Default build target will be architecture of the nrf52840
# We can override it using `carog run --target=XXXX` if we want to
[build]
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)

# We want the info level for logging (defaults to Warn)
[env]
DEFMT_LOG = "info"
```
We can now use `cargo build` without target flags and `cargo run` normally!


Debugging with breakpoints will be for another time since it's IDE specific and out of scope.

---

## Lets make the program panic

Read to an index out of bound or unwrap an error and see what happens.
``` rust
// We don't have an actual main function
#[no_main]
// We don't have the standard library
#[no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _; // panic behavior
 
#[cortex_m_rt::entry]
fn main() -> ! {
    let some_array = [0,1,2,3];
    // Out of bound access, will panic!
    defmt::info!("Hello, World! {}", some_array[99]);
    // Pause the execution
    cortex_m::asm::bkpt();
}
```
 --- 
## Unwrap
Try it again, but this time by using the `unwrap()` function on an `Option` or `Error` result. Why is the unwrap discouraged in library code? 
``` rust
fn none() -> Option<u32> {
    None
}
fn some() -> Option<u32> {
    Some(42)
}
fn err() -> Error<u32, ()>{
    Err(())
}
fn ok() -> Error<u32, ()>{
    Ok(42)
}
```
