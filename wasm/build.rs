fn main() {
    println!("cargo::rustc-link-arg=--export=x401000");
    println!("cargo::rustc-link-arg=--import-memory");
}
