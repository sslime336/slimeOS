use super::*;

pub fn sys_call(syscall_id: usize, args: [usize; 6]) -> isize {
    match syscall_id {
        SYSCALL_GETCWD => {}
        SYSCALL_DUP => {}
        SYSCALL_DUP3 => {}
        SYSCALL_MKDIRAT => {}
        SYSCALL_UNLINKAT => {}
        SYSCALL_UMOUNT2 => {}
        SYSCALL_MOUNT => {}
        SYSCALL_CHDIR => {}
        SYSCALL_OPENAT => {}
        SYSCALL_CLOSE => {}
        SYSCALL_PIPE => {}
        SYSCALL_GETDENTS64 => {}
        SYSCALL_READ => {}
        SYSCALL_WRITE => {}
        SYSCALL_FSTAT => {}
        SYSCALL_EXIT => {}
        SYSCALL_NANOSLEEP => {}
        SYSCALL_YIELD => {}
        SYSCALL_KILL => {}
        SYSCALL_TIMES => {}
        SYSCALL_GETTIMEOFDAY => {}
        SYSCALL_GETPID => {}
        SYSCALL_BRK => {}
        SYSCALL_MUNMAP => {}
        SYSCALL_FORK => {}
        SYSCALL_EXEC => {}
        SYSCALL_MMAP => {}
        SYSCALL_WAITPID => {}
    }
    todo!()
}
