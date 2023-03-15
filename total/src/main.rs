
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