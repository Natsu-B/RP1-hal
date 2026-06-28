use core::cmp;
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};

use rp1_abi::debug::{self, DebugMailbox};

#[repr(transparent)]
pub struct DebugMailboxCell(DebugMailbox);

unsafe impl Sync for DebugMailboxCell {}

#[unsafe(link_section = ".rp1_debug_stub.mailbox")]
#[used]
pub static mut DEBUG_MAILBOX: DebugMailboxCell = DebugMailboxCell(DebugMailbox::new());

static LAST_SEQ: AtomicU32 = AtomicU32::new(0);

const _: () = assert!(core::mem::size_of::<DebugMailbox>() <= debug::MAILBOX_SIZE);

pub fn init() {
    let mailbox = mailbox_mut();
    write_u32(&mut mailbox.magic, debug::MAGIC);
    write_u32(&mut mailbox.version, debug::VERSION);
    write_u32(&mut mailbox.size, core::mem::size_of::<DebugMailbox>() as u32);
    write_u32(&mut mailbox.state, debug::state::RUNNING);
    write_u32(&mut mailbox.stop_reason, debug::stop_reason::NONE);
    write_u32(&mut mailbox.command, debug::command::NONE);
    write_u32(&mut mailbox.status, debug::status::OK);
    LAST_SEQ.store(read_u32(&mailbox.seq), Ordering::Relaxed);
}

pub fn poll() {
    let mailbox = mailbox_mut();
    let seq = read_u32(&mailbox.seq);
    if seq == LAST_SEQ.load(Ordering::Relaxed) {
        return;
    }

    let command = read_u32(&mailbox.command);
    let status = match command {
        debug::command::NONE => debug::status::OK,
        debug::command::PING => debug::status::OK,
        debug::command::GET_REGS => {
            snapshot_core_regs(mailbox);
            debug::status::OK
        }
        debug::command::READ_MEM => read_mem(mailbox),
        debug::command::WRITE_MEM => write_mem(mailbox),
        debug::command::HALT => {
            write_u32(&mut mailbox.state, debug::state::STOPPED);
            write_u32(&mut mailbox.stop_reason, debug::stop_reason::HOST_HALT);
            debug::status::OK
        }
        debug::command::CONTINUE => {
            write_u32(&mut mailbox.state, debug::state::RUNNING);
            write_u32(&mut mailbox.stop_reason, debug::stop_reason::NONE);
            debug::status::OK
        }
        _ => debug::status::BAD_COMMAND,
    };

    write_u32(&mut mailbox.status, status);
    write_u32(&mut mailbox.command, debug::command::NONE);
    write_u32(&mut mailbox.ack, seq);
    LAST_SEQ.store(seq, Ordering::Relaxed);
}

pub fn fault() -> ! {
    let mailbox = mailbox_mut();
    snapshot_core_regs(mailbox);
    write_u32(&mut mailbox.state, debug::state::FAULTED);
    write_u32(&mut mailbox.stop_reason, debug::stop_reason::EXCEPTION);
    loop {
        poll();
        core::hint::spin_loop();
    }
}

pub fn panic() -> ! {
    let mailbox = mailbox_mut();
    snapshot_core_regs(mailbox);
    write_u32(&mut mailbox.state, debug::state::FAULTED);
    write_u32(&mut mailbox.stop_reason, debug::stop_reason::PANIC);
    loop {
        poll();
        core::hint::spin_loop();
    }
}

fn read_mem(mailbox: &mut DebugMailbox) -> u32 {
    let addr = read_u32(&mailbox.arg0) as usize;
    let len = cmp::min(read_u32(&mailbox.arg1) as usize, debug::MAILBOX_DATA_LEN);
    for idx in 0..len {
        let value = unsafe { ptr::read_volatile((addr + idx) as *const u8) };
        unsafe { ptr::write_volatile(mailbox.data.as_mut_ptr().add(idx), value) };
    }
    write_u32(&mut mailbox.data_len, len as u32);
    debug::status::OK
}

fn write_mem(mailbox: &mut DebugMailbox) -> u32 {
    let addr = read_u32(&mailbox.arg0) as usize;
    let len = read_u32(&mailbox.arg1) as usize;
    if len > debug::MAILBOX_DATA_LEN {
        return debug::status::BAD_LENGTH;
    }
    for idx in 0..len {
        let value = unsafe { ptr::read_volatile(mailbox.data.as_ptr().add(idx)) };
        unsafe { ptr::write_volatile((addr + idx) as *mut u8, value) };
    }
    debug::status::OK
}

fn snapshot_core_regs(mailbox: &mut DebugMailbox) {
    let mut regs = [0u32; debug::MAILBOX_REG_COUNT];
    unsafe {
        core::arch::asm!(
            "mov {r0_out}, r0",
            "mov {r1_out}, r1",
            "mov {r2_out}, r2",
            "mov {r3_out}, r3",
            "mov {r4_out}, r4",
            "mov {r5_out}, r5",
            "mov {r6_out}, r6",
            "mov {r7_out}, r7",
            "mov {r8_out}, r8",
            "mov {r9_out}, r9",
            "mov {r10_out}, r10",
            "mov {r11_out}, r11",
            "mov {r12_out}, r12",
            "mov {sp_out}, sp",
            "mov {lr_out}, lr",
            "mrs {xpsr_out}, xpsr",
            r0_out = lateout(reg) regs[0],
            r1_out = lateout(reg) regs[1],
            r2_out = lateout(reg) regs[2],
            r3_out = lateout(reg) regs[3],
            r4_out = lateout(reg) regs[4],
            r5_out = lateout(reg) regs[5],
            r6_out = lateout(reg) regs[6],
            r7_out = lateout(reg) regs[7],
            r8_out = lateout(reg) regs[8],
            r9_out = lateout(reg) regs[9],
            r10_out = lateout(reg) regs[10],
            r11_out = lateout(reg) regs[11],
            r12_out = lateout(reg) regs[12],
            sp_out = lateout(reg) regs[13],
            lr_out = lateout(reg) regs[14],
            xpsr_out = lateout(reg) regs[16],
            options(nomem, preserves_flags),
        );
    }
    regs[15] = snapshot_core_regs as usize as u32;
    for (dst, src) in mailbox.regs.iter_mut().zip(regs) {
        write_u32(dst, src);
    }
}

fn mailbox_mut() -> &'static mut DebugMailbox {
    unsafe {
        let ptr = core::ptr::addr_of_mut!(DEBUG_MAILBOX) as *mut DebugMailbox;
        &mut *ptr
    }
}

fn read_u32(src: &u32) -> u32 {
    unsafe { ptr::read_volatile(src) }
}

fn write_u32(dst: &mut u32, value: u32) {
    unsafe { ptr::write_volatile(dst, value) }
}
