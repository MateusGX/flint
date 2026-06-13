//! End-to-end tests: compile Flint source to bytecode, run it on the VM,
//! and check the results it left behind in registers/memory. These exercise
//! the full pipeline (lexer -> parser -> compiler -> VM) the way a real
//! program would be run.

use flint::lang::compile_source;
use flint::vm::{NativeRegistry, Value, Vm};

fn natives() -> NativeRegistry {
    let mut registry = NativeRegistry::new();
    flint::stdlib::register_all(&mut registry);
    registry
}

fn run(source: &str) -> Vm {
    let program = compile_source(source).expect("program should compile");
    let mut vm = Vm::new(natives());
    vm.run(&program).expect("program should run without errors");
    vm
}

fn reg(vm: &Vm, register: u8) -> &Value {
    vm.register(register).expect("register should exist")
}

#[test]
fn inspecting_an_invalid_register_returns_none() {
    let vm = Vm::new(natives());
    assert!(vm.register(16).is_none());
}

const FIB_ITERATIVE: &str = r#"
    mov r0, 10        ; n
    mov r1, 0         ; a = fib(0)
    mov r2, 1         ; b = fib(1)
    mov r3, 0         ; i = 0
loop:
    cmp r3, r0
    jge done
    mov r4, r2        ; tmp = b
    add r2, r1, r2    ; b = a + b
    mov r1, r4        ; a = tmp
    add r3, r3, 1     ; i += 1
    jmp loop
done:
    hlt
"#;

#[test]
fn iterative_fibonacci_computes_fib_10() {
    let vm = run(FIB_ITERATIVE);
    assert_eq!(reg(&vm, 1), &Value::Int(55));
}

const FIB_RECURSIVE: &str = r#"
main:
    mov r0, 10
    call fib
    hlt

; fib(n): argument and result both in r0. r1 is used as scratch and is
; explicitly saved/restored across the second recursive call via the stack —
; a textbook hand-written calling convention.
fib:
    cmp r0, 2
    jl base
    push r0
    sub r0, r0, 1
    call fib
    mov r1, r0        ; r1 = fib(n-1)
    pop r0            ; r0 = n
    push r1
    sub r0, r0, 2
    call fib          ; r0 = fib(n-2)
    pop r1            ; r1 = fib(n-1)
    add r0, r0, r1    ; r0 = fib(n-1) + fib(n-2)
    ret
base:
    ret
"#;

#[test]
fn recursive_fibonacci_computes_fib_10() {
    let vm = run(FIB_RECURSIVE);
    assert_eq!(reg(&vm, 0), &Value::Int(55));
}

const MEMORY_ROUNDTRIP: &str = r#"
main:
    mov r0, 42
    mov r1, 7
    store [r1], r0
    load r2, [r1]
    hlt
"#;

#[test]
fn store_and_load_round_trip_through_linear_memory() {
    let vm = run(MEMORY_ROUNDTRIP);
    assert_eq!(reg(&vm, 2), &Value::Int(42));
}

const STATIC_MEMORY: &str = r#"
section .data
counter:
    data 41

section .bss
buffer:
    res 4

section .text
main:
    mov r0, counter
    load r1, [r0]
    add r1, r1, 1
    store [r0], r1
    load r2, [r0]
    mov r3, buffer
    mov r4, 7
    store [r3], r4
    load r5, [r3]
    hlt
"#;

#[test]
fn static_data_and_bss_sections_are_addressable() {
    let vm = run(STATIC_MEMORY);
    assert_eq!(reg(&vm, 2), &Value::Int(42));
    assert_eq!(reg(&vm, 5), &Value::Int(7));
}

const HELLO_WORLD: &str = r#"
main:
    mov r0, "Hello from Flint!"
    ncall debug.print, r0
    hlt
"#;

#[test]
fn ncall_invokes_native_functions_registered_with_the_vm() {
    // `debug.print` has no return value, so this also checks that `ncall`
    // (as opposed to `ncallr`) doesn't require one.
    run(HELLO_WORLD);
}

const NCALLR_WITH_UNKNOWN_NATIVE: &str = r#"
main:
    ncallr r0, totally.unknown, r1
    hlt
"#;

#[test]
fn calling_an_unregistered_native_is_a_runtime_error() {
    let program = compile_source(NCALLR_WITH_UNKNOWN_NATIVE).expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("calling an unknown native should fail");
    assert!(
        err.message
            .contains("unknown native function 'totally.unknown'"),
        "{}",
        err.message
    );
}

#[test]
fn native_calls_reject_extra_arguments() {
    let program = compile_source(
        r#"main:
    mov r0, "hello"
    mov r1, "extra"
    ncallr r2, string.len, r0, r1
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm.run(&program).expect_err("extra native args should fail");
    assert!(
        err.message
            .contains("'string.len' expects exactly 1 argument(s), got 2"),
        "{}",
        err.message
    );
}

const DIVISION_BY_ZERO: &str = r#"
main:
    mov r0, 10
    mov r1, 0
    div r2, r0, r1
    hlt
"#;

#[test]
fn division_by_zero_is_a_runtime_error_not_a_panic() {
    let program = compile_source(DIVISION_BY_ZERO).expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("dividing by zero should fail gracefully");
    assert!(err.message.contains("division by zero"), "{}", err.message);
}

#[test]
fn division_overflow_is_a_runtime_error_not_a_panic() {
    let program = compile_source(
        r#"main:
    mov r0, -9223372036854775807
    mov r1, 1
    sub r0, r0, r1
    mov r2, -1
    div r3, r0, r2
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("overflowing division should fail gracefully");
    assert!(
        err.message.contains("integer overflow in 'div'"),
        "{}",
        err.message
    );
}

#[test]
fn modulo_overflow_is_a_runtime_error_not_a_panic() {
    let program = compile_source(
        r#"main:
    mov r0, -9223372036854775807
    mov r1, 1
    sub r0, r0, r1
    mov r2, -1
    mod r3, r0, r2
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("overflowing remainder should fail gracefully");
    assert!(
        err.message.contains("integer overflow in 'mod'"),
        "{}",
        err.message
    );
}

#[test]
fn float_literals_and_arithmetic() {
    let vm = run("main:\n  mov r0, 3.14\n  mov r1, 2.0\n  addf r2, r0, r1\n  hlt\n");
    // 3.14 + 2.0 = 5.14
    match reg(&vm, 2) {
        flint::vm::Value::Float(f) => assert!((f - 5.14).abs() < 0.001),
        other => panic!("expected Float, got {other:?}"),
    }
}

#[test]
fn bitwise_and_shl() {
    let vm = run("main:\n  mov r0, 12\n  and r1, r0, 10\n  mov r2, 1\n  shl r3, r2, 3\n  hlt\n");
    assert_eq!(reg(&vm, 1), &Value::Int(8)); // 12 & 10 = 8
    assert_eq!(reg(&vm, 3), &Value::Int(8)); // 1 << 3 = 8
}

#[test]
fn typeof_instruction() {
    let vm =
        run("main:\n  mov r0, 42\n  typeof r1, r0\n  mov r2, \"hello\"\n  typeof r3, r2\n  hlt\n");
    assert_eq!(reg(&vm, 1), &Value::Str("int".into()));
    assert_eq!(reg(&vm, 3), &Value::Str("str".into()));
}

#[test]
fn neg_instruction() {
    let vm = run("main:\n  mov r0, 99\n  neg r1, r0\n  hlt\n");
    assert_eq!(reg(&vm, 1), &Value::Int(-99));
}

#[test]
fn string_natives_extended() {
    let vm = run(r#"main:
    mov r0, "hello world"
    mov r1, "world"
    ncallr r2, string.contains, r0, r1
    mov r3, "HELLO"
    ncallr r4, string.to_lower, r3
    mov r5, "  trim me  "
    ncallr r6, string.trim, r5
    mov r7, "<script>alert(\"x\")</script>"
    ncallr r8, string.escape_html, r7
    hlt
"#);
    assert_eq!(reg(&vm, 2), &Value::Int(1));
    assert_eq!(reg(&vm, 4), &Value::Str("hello".into()));
    assert_eq!(reg(&vm, 6), &Value::Str("trim me".into()));
    assert_eq!(
        reg(&vm, 8),
        &Value::Str("&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt;".into())
    );
}

#[test]
fn json_natives_extended() {
    let vm = run(r#"main:
    ncallr r0, json.object
    mov r1, "x"
    mov r2, 42
    ncallr r0, json.set, r0, r1, r2
    ncallr r3, json.len, r0
    ncallr r4, json.has, r0, r1
    ncallr r5, json.null
    ncallr r6, json.type, r5
    hlt
"#);
    assert_eq!(reg(&vm, 3), &Value::Int(1)); // len = 1
    assert_eq!(reg(&vm, 4), &Value::Int(1)); // has "x"
    assert_eq!(reg(&vm, 6), &Value::Str("null".into()));
}

#[test]
fn json_array_indexes_must_be_non_negative() {
    let program = compile_source(
        r#"main:
    ncallr r0, json.array
    mov r1, -1
    mov r2, "nope"
    ncallr r0, json.set, r0, r1, r2
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("negative json index should fail");
    assert!(
        err.message
            .contains("'json.set' expects a non-negative array index"),
        "{}",
        err.message
    );
}

#[test]
fn json_set_refuses_huge_implicit_array_expansion() {
    let program = compile_source(
        r#"main:
    ncallr r0, json.array
    mov r1, 1000001
    mov r2, "nope"
    ncallr r0, json.set, r0, r1, r2
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("huge json expansion should fail");
    assert!(
        err.message
            .contains("'json.set' refuses to expand arrays past index 1000000"),
        "{}",
        err.message
    );
}

#[test]
fn json_get_rejects_negative_array_indexes() {
    let program = compile_source(
        r#"main:
    ncallr r0, json.array
    mov r1, -1
    ncallr r2, json.get, r0, r1
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("negative json index should fail");
    assert!(
        err.message
            .contains("'json.get' expects a non-negative array index"),
        "{}",
        err.message
    );
}

#[test]
fn math_floor_rejects_non_finite_results() {
    let program = compile_source(
        r#"main:
    mov r0, -1
    ncallr r1, math.sqrt, r0
    ncallr r2, math.floor, r1
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("flooring NaN should fail cleanly");
    assert!(
        err.message.contains("'math.floor' result is not finite"),
        "{}",
        err.message
    );
}

#[test]
fn math_abs_rejects_int_min() {
    let program = compile_source(
        r#"main:
    mov r0, -9223372036854775807
    mov r1, 1
    sub r0, r0, r1
    ncallr r2, math.abs, r0
    hlt
"#,
    )
    .expect("program should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("abs(i64::MIN) should fail cleanly");
    assert!(
        err.message
            .contains("'math.abs' result is outside the int range"),
        "{}",
        err.message
    );
}

#[test]
fn call_depth_limit_is_enforced() {
    let program =
        compile_source("infinite:\n  call infinite\n  ret\nmain:\n  call infinite\n  hlt\n")
            .expect("should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run(&program)
        .expect_err("infinite recursion should fail");
    assert!(
        err.message.contains("call stack depth exceeded"),
        "{}",
        err.message
    );
}

#[test]
fn call_depth_is_reset_between_runs_after_an_error() {
    let recursive =
        compile_source("infinite:\n  call infinite\n  ret\nmain:\n  call infinite\n  hlt\n")
            .expect("should compile");
    let mut vm = Vm::new(natives());
    vm.run(&recursive)
        .expect_err("infinite recursion should fail");

    let simple =
        compile_source("main:\n  call target\n  hlt\ntarget:\n  ret\n").expect("should compile");
    vm.run(&simple)
        .expect("a later run should start with clean call depth");
}

#[test]
fn external_call_rejects_invalid_instruction_addresses() {
    let program = compile_source("target:\n  ret\n").expect("should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .call(&program, 999)
        .expect_err("invalid external call address should fail");
    assert!(
        err.message
            .contains("call address 999 is outside the program"),
        "{}",
        err.message
    );

    vm.call(&program, 0)
        .expect("invalid call should not poison later calls");
}

#[test]
fn instruction_limit_is_enforced() {
    let program = compile_source("loop:\n  jmp loop\n").expect("should compile");
    let mut vm = Vm::new(natives());
    let err = vm
        .run_with_instruction_limit(&program, 8)
        .expect_err("infinite jump loop should fail");
    assert!(
        err.message.contains("instruction limit exceeded (8)"),
        "{}",
        err.message
    );
}
