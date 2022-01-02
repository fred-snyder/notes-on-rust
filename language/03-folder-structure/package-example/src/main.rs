// example of importing functions from other files

// import the example.rs file
mod example;

// import files in a sub-directory
#[path = "topics/print.rs"] mod print;

fn main() {
    // call functions from imports
    example::example();
    print::print();
}
