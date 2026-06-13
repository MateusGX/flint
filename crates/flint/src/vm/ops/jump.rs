//! Branching — `jmp` always; `je`/`jne`/`jl`/`jg`/`jle`/`jge` read the
//! [`Flags`](super::cmp::Flags) the last `cmp`/`cmp_imm` left behind. All
//! seven share this file: same shape (an instruction-pointer target plus,
//! for the conditionals, a flag check), differing only in *which* flags they
//! look at — splitting them further would just spread that one difference
//! across seven near-identical files.
//!
//! Each function returns the instruction pointer to continue at — `target`
//! if the branch is taken, `fallthrough` (the address right after this
//! instruction) otherwise — which the dispatch loop assigns to `next_pc`.

use crate::vm::Vm;

pub(crate) fn jmp(target: usize) -> usize {
    target
}

pub(crate) fn je(vm: &Vm, target: usize, fallthrough: usize) -> usize {
    if vm.flags().eq {
        target
    } else {
        fallthrough
    }
}

pub(crate) fn jne(vm: &Vm, target: usize, fallthrough: usize) -> usize {
    if !vm.flags().eq {
        target
    } else {
        fallthrough
    }
}

pub(crate) fn jl(vm: &Vm, target: usize, fallthrough: usize) -> usize {
    if vm.flags().lt {
        target
    } else {
        fallthrough
    }
}

pub(crate) fn jg(vm: &Vm, target: usize, fallthrough: usize) -> usize {
    if vm.flags().gt {
        target
    } else {
        fallthrough
    }
}

pub(crate) fn jle(vm: &Vm, target: usize, fallthrough: usize) -> usize {
    let flags = vm.flags();
    if flags.lt || flags.eq {
        target
    } else {
        fallthrough
    }
}

pub(crate) fn jge(vm: &Vm, target: usize, fallthrough: usize) -> usize {
    let flags = vm.flags();
    if flags.gt || flags.eq {
        target
    } else {
        fallthrough
    }
}
