---
marp: true
---

# Rust bring up
---

## Setup a new project
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
# Rust enums
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
# Classic enum usage
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
# Result and option usage
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
# Question mark operator "?"
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
/// Or even easier to read
fn get_value_from_file(file: File) -> Result<Item, ItemError> {
    // Note the question mark here
    let result = file.get()?;
    let val = result.convert();
    Ok(val)
}

```

---
# Basic application
- Add `clap` as a dependecy
- Introduce macros
  - Function like macros
    - {} vs {:?}
  - Derive macros
- More complext Match
---
# Cargo.toml example
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
# Basic generic and trait
```rust

trait  Total{
    fn total(&self) -> u32;
}

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

---
# Lifetimes
Rust enforce only **ONE** owner of a memory segment and **ONE** can mutate a memory segment.
- We can own using `Type`. Only ONE owner can exist
- We can borrow asing `&Type`
- We can mutably borrow using `&mut Type`
```rust 
fn borrow_vec(vec: &Vec) {
    // We can only read it 
    println!("Vec borrowed: {:?}", vec);
}
fn borrow_mut_vec(vec: &mut Vec) {
    vec.push(3); // We can modify it
    println!("Vec borrowed mutably: {:?}", vec);
}
fn own_vec(vec: Vec) {
    // We own it, it will be destroyed at the end of the function
    println!("Vec owned: {:?}", vec);
    // Destroyed and memory released here, no garbage collection needed
}
```

---
# Basic lifetime example
```rust
// Equivalent to `fn borrow_vec<'a>(vec: &'a Vec)
fn borrow_vec(vec: &Vec) {
    println!("Vec borrowed: {:?}", vec);
}
fn borrow_mut_vec(vec: &mut Vec) {
    vec.push(3);
    println!("Vec borrowed mutably: {:?}", vec);
}
fn own_vec(vec: Vec) {
    println!("Vec owned: {:?}", vec);
}

pub struct BorrowedVec<'a> {
    vec: &'a Vec
}
// We implement functions here if we want to add some
impl<'a> BorrowedVec<'a>{
    fn print_borrowed_vec(&self) {
        println!("Struct vec borrowed is: {:?}", self.vec);
    }
}

fn main() {
    let vec = vec![0,1,2];
    borrow_vec(&vec);
    borrow_mut_vec(&mut vec);
    borrow_vec(&vec);
    own_vec(vec);
    // borrow_vec(&vec); // This would not work since vec was moved

    // Create ANOTHER vec
    let vec = vec![0,1,2];
    let borrower = BorrowedVec { vec }; // Same as BorrowedVec { vec: vec };
    borrow_vec(&vec);
    // borrow_mut_vec(&mut vec); // Cannot borrow mutably and immutably
    // own_vec(vec); // Borrrowed, cannot own

    borrower.print_borrowed_vec();

}
```

---
# Basic macro usage from library
```rust 
/// Imports
use clap::Parser;
use clap::{Command, ValueEnum};

/// Define the arguments for our cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// An argument
   #[arg(short, long)]
   name: String,
   
   /// An optional "DataType" to pass
   #[arg(short, long)]
   type: Option<Type>,
}

/// We define an enum, Note that enum can contain data
#[derive(ValueEnum, Debug, Copy, Clone)]
enum Type {
    HeartRate,
    SkinTemp,
    Eda,
    Accel,
}

fn main() {
   let args = Args::parse();
   println!("Hello {}, using argument {:?}", args.name, args.type);
   match args.type {
       Some(Type::HeartRate) => println!("We got HR {:?}", args.type);
       None => println!("We got nothing");
       _ => println!("We don't care!");
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
# Required dependencies
```toml
[dependencies]
# Cortex m register and peripheral access
cortex-m = "0.6.3"

# Cortex m runtime  (what happens before the main fucntion)
cortex-m-rt = "0.6.12"

# Logging library
defmt = "0.1"

# We log via rtt (we could log via uart, usb, to memory, etc)
defmt-rtt = "0.1"

# Hal for the nrf52840, use spi, i2c, etc.
nrf52840-hal = "0.11.0"

# Panic behavior, use defmt
panic-probe = { version = "0.3", features = ["print-defmt"] } 
```
---

# Write a main application
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
    defmt::info!("Hello, World!");
    // Pause the execution
    cortex_m::asm::bkpt();
}
```

---
# Run it
Try `cargo run`. It won't work since it's not for your x86 computer.

## How to run it?
Use `probe-run`, it's like `openocd` or `JLinkCommander`, but integrates with the rust ecosystem. It flashes the device, run your code and print to console.
Use the command
```sh
probe-run --chip=nRF52840_xxAA ./target/thumbv7em-none-eabihf/debug/main
```
But that sucks and we cannot use the usual `cargo run`

---
# Setup runner
Add the file in .config/config.toml.

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
We can now use `cargo run` normally!


Debugging with breakpoints will be for another time since it's IDE specific and out of scope for now.

---

# Lets make the program panic!

Let's make the code panic and see what happens! 
Read to an index out of bound or unwrap an error.

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
Checkout https://github.com/knurling-rs/app-template.