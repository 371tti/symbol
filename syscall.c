#include <io.h>
#include <errno.h>
#include <stdarg.h>

long syscall(long number, ...) {
    va_list args;
    va_start(args, number);
    if (number == 1) {  // write syscall
        int fd = va_arg(args, int);
        const void *buf = va_arg(args, const void *);
        unsigned long count = va_arg(args, unsigned long);
        va_end(args);
        return _write(fd, buf, count);
    }
    va_end(args);
    errno = ENOSYS;
    return -1;
}
