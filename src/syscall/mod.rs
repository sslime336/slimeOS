mod sys_call;

const SYSCALL_GETCWD:       usize = 17;
const SYSCALL_DUP:          usize = 23;
const SYSCALL_DUP3:         usize = 24;
const SYSCALL_MKDIRAT:      usize = 34;
const SYSCALL_UNLINKAT:     usize = 35;
const SYSCALL_UMOUNT2:      usize = 39;
const SYSCALL_MOUNT:        usize = 40;
const SYSCALL_CHDIR:        usize = 49;
const SYSCALL_OPENAT:       usize = 56;
const SYSCALL_CLOSE:        usize = 57;
const SYSCALL_PIPE:         usize = 59;
const SYSCALL_GETDENTS64:   usize = 61;
const SYSCALL_READ:         usize = 63;
const SYSCALL_WRITE:        usize = 64;
const SYSCALL_FSTAT:        usize = 80;
const SYSCALL_EXIT:         usize = 93;
const SYSCALL_NANOSLEEP:    usize = 101;
const SYSCALL_YIELD:        usize = 124;
const SYSCALL_KILL:         usize = 129;
const SYSCALL_TIMES:        usize = 153;
const SYSCALL_GETTIMEOFDAY: usize = 169;
const SYSCALL_GETPID:       usize = 172;
const SYSCALL_BRK:          usize = 214;
const SYSCALL_MUNMAP:       usize = 215;
const SYSCALL_FORK:         usize = 220;
const SYSCALL_EXEC:         usize = 221;
const SYSCALL_MMAP:         usize = 222;
const SYSCALL_WAITPID:      usize = 260;