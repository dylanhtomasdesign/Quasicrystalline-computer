//! Build script for Senemosìa Kernel
//! 
//! Configures the build process for bare-metal x86_64 target

fn main() {
    // Tell cargo to link with our custom linker script
    println!("cargo:rerun-if-changed=linker.ld");
    
    // Verify target
    println!("cargo:rustc-env=TARGET=x86_64-unknown-none");
}
