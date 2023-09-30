use std::arch::asm;

fn add(x: u64, y: u64) -> u64 {
    let result: u64;

    // x86/x86-64 assembly
    unsafe { asm! ("add {}, {}", inout(reg) x => result, in(reg) y) }

    result
}

fn main() {
    dbg!(add(1, 3));
}
