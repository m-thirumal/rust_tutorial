To Create project

    cargo new helloworld

To build 

    cargo build

    target/debug

To run

    cargo run

To Release

* This is same as build with optimization for production
  
    ```
    cargo build --release

    target/release
    ```

### ---------------Rules of Ownership-----------

 1. Each value in Rust has a varaiable that's called it's owner
 2. There can only be one owner at a time
 3. When the owner goes out of scope, the value will be dropped


Dependencies

* Dependencies are maintained in `Cargo.toml`