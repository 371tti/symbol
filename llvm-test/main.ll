@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1

define i32 @main() {
entry:
    %handle = add i64 0, -11
    %msg = getelementptr [14 x i8], [14 x i8]* @.str, i64 0, i64 0
    %len = add i64 13, 0
    %sysnum = add i32 0, 89
    %ret = call i64 asm sideeffect "movq r10, rcx\nmovq rcx, $0\nmovq rdx, $1\nmovq r8, $2\nmovq r9, $3\nmovl eax, $4\nsyscall", "={rax},r,r,r,r,r"(i64 %handle, i64 0, i8* %msg, i64 %len, i32 %sysnum)
    ret i32 0
}
