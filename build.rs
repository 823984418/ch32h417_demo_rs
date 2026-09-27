fn main() {
    println!("cargo:rustc-env=RISCV_RT_BASE_ISA={}", "rv32i");
}
