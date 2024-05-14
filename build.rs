fn main() {
    println!("cargo:rustc-linker=arm-linux-gnueabihf-gcc");
    println!("cargo:rustc-flags=static=link-arg=-march=armv6");
}
