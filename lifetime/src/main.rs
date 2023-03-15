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