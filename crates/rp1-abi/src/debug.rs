pub const MAILBOX_ADDR: u32 = 0x2000_fc00;
pub const MAILBOX_SIZE: usize = 1024;
pub const MAILBOX_DATA_LEN: usize = 256;
pub const MAILBOX_REG_COUNT: usize = 18;

pub const MAGIC: u32 = u32::from_le_bytes(*b"D1RP");
pub const VERSION: u32 = 1;

pub mod command {
    pub const NONE: u32 = 0;
    pub const PING: u32 = 1;
    pub const GET_REGS: u32 = 2;
    pub const READ_MEM: u32 = 3;
    pub const WRITE_MEM: u32 = 4;
    pub const CONTINUE: u32 = 5;
    pub const HALT: u32 = 6;
}

pub mod state {
    pub const OFFLINE: u32 = 0;
    pub const RUNNING: u32 = 1;
    pub const STOPPED: u32 = 2;
    pub const FAULTED: u32 = 3;
}

pub mod stop_reason {
    pub const NONE: u32 = 0;
    pub const HOST_HALT: u32 = 1;
    pub const EXCEPTION: u32 = 2;
    pub const PANIC: u32 = 3;
}

pub mod status {
    pub const OK: u32 = 0;
    pub const BAD_COMMAND: u32 = 1;
    pub const BAD_LENGTH: u32 = 2;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DebugMailbox {
    pub magic: u32,
    pub version: u32,
    pub size: u32,
    pub flags: u32,
    pub seq: u32,
    pub ack: u32,
    pub state: u32,
    pub stop_reason: u32,
    pub command: u32,
    pub arg0: u32,
    pub arg1: u32,
    pub status: u32,
    pub regs: [u32; MAILBOX_REG_COUNT],
    pub data_len: u32,
    pub data: [u8; MAILBOX_DATA_LEN],
}

impl DebugMailbox {
    pub const fn new() -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            size: core::mem::size_of::<Self>() as u32,
            flags: 0,
            seq: 0,
            ack: 0,
            state: state::RUNNING,
            stop_reason: stop_reason::NONE,
            command: command::NONE,
            arg0: 0,
            arg1: 0,
            status: status::OK,
            regs: [0; MAILBOX_REG_COUNT],
            data_len: 0,
            data: [0; MAILBOX_DATA_LEN],
        }
    }
}
