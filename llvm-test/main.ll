@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1

; Windows 用 systemcall() の定義（NtWriteFile 呼出し例）
define i32 @systemcall(i8* %handle, i64 %event, i8* %buf, i32 %len, i32 %sysnum) {
entry:
    ; Windows x64 のシステムコール規約:
    ;   rcx = FileHandle, rdx = Event, r8 = Buffer, r9 = Length
    ;   ※ 呼出し前に r10 に rcx を移す必要がある
    ;   eax にシステムコール番号をセットし、syscall 命令を実行する
    %ret = call i32 asm sideeffect "
        mov r10, rcx;
        mov rcx, $0;
        mov rdx, $1;
        mov r8,  $2;
        mov r9,  $3;
        mov eax, $4;
        syscall
    ", "={eax},r,r,r,r"(i8* %handle, i64 %event, i8* %buf, i32 %len, i32 %sysnum)
    ret i32 %ret
}

define i32 @main() {
entry:
    ; STD_OUTPUT_HANDLE は Windows では (HANDLE)-11
    %handle = inttoptr i64 0xFFFFFFFFFFFFFFF5 to i8*
    %msg = getelementptr [14 x i8], [14 x i8]* @.str, i64 0, i64 0
    %len64 = add i64 13, 0
    %len32 = trunc i64 %len64 to i32
    ; ここでは例として NtWriteFile のシステムコール番号を 0x0059（仮）とする
    %res = call i32 @systemcall(%handle, i64 0, %msg, %len32, i32 0x0059)
    ret i32 0
}
