use bitflags::bitflags;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

macro_rules! write_uart_register {
    ($reg:ident, $val:expr) => {
        unsafe {
            *($reg as *mut u8) = $val;
        }
    };
}

const THR_OFFSET: u8 = 0;
const RHR_OFFSET: u8 = 0;
const IER_OFFSET: u8 = 1;
const FCR_OFFSET: u8 = 2;
const ISR_OFFSET: u8 = 2;
const LCR_OFFSET: u8 = 3;
const MCR_OFFSET: u8 = 5;
const LSR_OFFSET: u8 = 5;
const MSR_OFFSET: u8 = 7;
const SPR_OFFSET: u8 = 0;

bitflags! {
    /// **Transmit Hold Register**
    ///
    /// * Writing to this register (THR) will transfer the contents of data bus
    /// (D7-D0) to the Transmit holding register whenever the transmitter holding
    /// register or transmitter shift register is empty.
    /// * Note that a write operation should be performed when the transmit
    /// holding register empty flag is set.
    /// 
    /// For output bytes.
    pub struct THR: u8 {
        const bit_0 = 1 << 0;
        const bit_1 = 1 << 1;
        const bit_2 = 1 << 2;
        const bit_3 = 1 << 3;
        const bit_4 = 1 << 4;
        const bit_5 = 1 << 5;
        const bit_6 = 1 << 6;
        const bit_7 = 1 << 7;
    }

    /// **Receive Holding Register**
    /// 
    /// For input bytes.
    pub struct RHR: u8 {
        const bit_0 = 1 << 0;
        const bit_1 = 1 << 1;
        const bit_2 = 1 << 2;
        const bit_3 = 1 << 3;
        const bit_4 = 1 << 4;
        const bit_5 = 1 << 5;
        const bit_6 = 1 << 6;
        const bit_7 = 1 << 7;
    }

    /// **Line Status Register**
    pub struct LSR: u8 {
        const receive_data_ready     = 1 << 0;
        const overrun_error          = 1 << 1;
        const parity_error           = 1 << 2;
        const framing_error          = 1 << 3;
        const break_interrupt        = 1 << 4;
        const transmit_holding_empty = 1 << 5;
        const transmit_empty         = 1 << 6;
        const fifo_error             = 1 << 7;
    }

    /// **FIFO control Register**
    ///
    /// This register is used to enable the FIFOs, clear the FIFOs, set the
    /// receiver FIFO trigger level, and select the type of DMA signaling.
    pub struct FCR: u8 {
        const enable = 1 << 0;
        const reset  = 1 << 1;
    }

    /// **Interrupt Enable Register**
    ///
    /// The Interrupt Enable Register (IER) masks the incoming interrupts
    /// from receiver ready, transmitter empty, line status and modem status
    /// registers to the INT output pin.
    ///
    /// * 1 - enable
    /// * 0 - disable
    pub struct IER: u8 {
        const receive_holding_register_interrupt  = 1 << 0;
        const transmit_holding_register_interrupt = 1 << 1;
        const receive_line_status_interrupt 	  = 1 << 2;
        const modem_status_interrupt              = 1 << 3;
        // Other bits is logically set to zero.
    }

    /// **Line Control Register **
    ///
    /// The line Control Register is used to specify the asynchronous data
    /// communication format. The number of the word length, stop bits, and parity
    /// can be selected by writing appropriate bits in this register.
    pub struct LCR: u8 {
        const word_length_bit_0    = 1 << 0;
        const word_length_bit_1    = 1 << 1;
        const stop_bits            = 1 << 2;
        const parity_enable        = 1 << 3;
        const even_parity          = 1 << 4;
        const set_parity           = 1 << 5;
        const set_break            = 1 << 6;
        const divisor_latch_enable = 1 << 7;
    }
}

lazy_static! {
    pub static ref UART_LOCK: Mutex<Uart> = Mutex::new(Uart);
}

pub fn init() {
    UART_LOCK.lock();
}

pub struct Uart(usize);

pub trait Uart16550<const BASE_ADDR: usize> {
    fn thr(&mut self) -> &mut THR;
    fn lsr(&mut self) -> &mut LSR;
    // fn rhr(&mut self) -> &mut RHR;
    fn ier(&mut self) -> &mut IER;
    fn fcr(&mut self) -> &mut FCR;
    // fn isr(&mut self) -> &mut LCR;
    fn lcr(&mut self) -> &mut LCR;
    // fn mcr(&mut self) -> &mut
    // fn msr(&mut self) -> &mut
    // fn spr(&mut self) -> &mut
}

/// UART base address on QEMU RISC-V
pub const UART_BASE_ADDR: usize = 0x1000_0000;

impl Uart16550<UART_BASE_ADDR> for usize {}
