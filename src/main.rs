extern crate alloc;

mod includes {
// for lsp support
#[path ="memory.rs"] mod memory; 
}
core::include!{"includes/memory.rs"}


fn main() {}
