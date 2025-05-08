define i64 @systemcall(i64 %num, i64 %fd, i8* %buf, i64 %count) {
entry:
    ; %num が 1 でない場合は失敗
    %cmp = icmp eq i64 %num, 1
    br i1 %cmp, label %do_syscall, label %fail

do_syscall:
    ; Linux x86_64 の write システムコール
    ; rdi = fd, rsi = buf, rdx = count, rax = 1
    %ret = call i64 asm sideeffect "movq $0, %%rdi; movq $1, %%rsi; movq $2, %%rdx; movq $3, %%rax; syscall", "={rax},r,r,r,r,r"(i64 %fd, i8* %buf, i64 %count, i64 1)
    ret i64 %ret

fail:
    ret i64 -1
}
