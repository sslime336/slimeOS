macro_rules! stext { () => {{ $crate::macros::kernel_segment::stext as usize }}; }
macro_rules! etext { () => {{ $crate::macros::kernel_segment::etext as usize }}; }
macro_rules! srodata { () => {{ $crate::macros::kernel_segment::srodata as usize }}; }
macro_rules! erodata { () => {{ $crate::macros::kernel_segment::erodata as usize }}; }
macro_rules! sdata { () => {{ $crate::macros::kernel_segment::sdata as usize }}; }
macro_rules! edata { () => {{ $crate::macros::kernel_segment::edata as usize }}; }
macro_rules! sbss { () => {{ $crate::macros::kernel_segment::sbss as usize }}; }
macro_rules! ebss { () => {{ $crate::macros::kernel_segment::ebss as usize }}; }
macro_rules! ekernel { () => {{ $crate::macros::kernel_segment::ekernel as usize }}; }
macro_rules! strampoline { () => {{ $crate::macros::kernel_segment::strampoline as usize }}; }

extern "C" {
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss();
    pub fn ebss();

    pub fn ekernel(); // 0x80209000
    pub fn strampoline();
}
