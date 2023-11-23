use super::super::cpu::pc_state;
use super::memory;

pub trait Address16 {
    fn address16(&self, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16;
    fn get_addressing_size(&self) -> u8;
    fn get_addressing_time(&self) -> u8;
}

pub struct Addressing {
    size:u8,
    cycles:u8,
}

impl Addressing {
    pub const fn new(size:u8, cycles:u8) -> Self {
        Self {
            size: size,
            cycles: cycles,
        }
    }
}

pub struct AddressingIMM {
    addressing:Addressing,
}

impl AddressingIMM {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 0),
        }
    }

    pub fn address(&self, pc_state: &pc_state::PcState, _: &memory::Memory) -> u16 {
        pc_state.get_pc() + 1
    }
}

pub struct AddressingIZX {
    addressing:Addressing,
}

impl AddressingIZX {
    pub const fn new() -> Self {
        Self {
            addressing:Addressing::new(1, 4),
        }
    }

    pub fn address(&self, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
        let tmp8 = memory.read(pc_state.get_pc().wrapping_add(1)).wrapping_add(pc_state.get_x());
        memory.read16(tmp8 as u16)
    }
}

// Common functions associated with 'addressing' types.
// TODO: See if there's a less macro way
macro_rules! impl_addressing {
     ($type:ty)  => {
            impl Address16 for $type {
               fn address16(&self, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) -> u16 {
                   self.address(pc_state, memory)
               }

               fn get_addressing_size(&self) -> u8 { self.addressing.size }
               fn get_addressing_time(&self) -> u8 { self.addressing.cycles  * pc_state::PcState::CYCLES_TO_CLOCK}
               }
    };
}

impl_addressing!(AddressingIMM);
impl_addressing!(AddressingIZX);

pub trait ReadData {
    fn read(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8;
    fn get_reading_time(&self) -> u8;
}

pub struct MemoryRead {
    cycles:u8,
}

impl MemoryRead {
    pub const fn new() -> Self {
        Self {cycles: 2}
    }

    fn read(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
        memory.read(address)
    }
}

macro_rules! impl_read_data {
    ($type:ty)  => {
        impl ReadData for $type {
            fn read(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16) -> u8 {
                self.read(pc_state, memory, address)
            }
        
            fn get_reading_time(&self) -> u8 {
                self.cycles * pc_state::PcState::CYCLES_TO_CLOCK
            }
        }
    };
}

impl_read_data!(MemoryRead);

pub trait WriteData {
    fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8);
    fn get_writing_time(&self) -> u8;
}

pub struct MemoryWrite { cycles:u8}

impl MemoryWrite {
    pub fn new() -> Self {
        Self {cycles:2}
    }

    fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
        memory.write(address, data);
    }
}

macro_rules! impl_write_data {
    ($type:ty)  => {
        impl WriteData for $type {
            fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) {
                self.write(pc_state, memory, address, data);
            }
        
            fn get_writing_time(&self) -> u8 {
                self.cycles * pc_state::PcState::CYCLES_TO_CLOCK
            }
        }
        };
}

impl_write_data!(MemoryWrite); 

// TODO: Fix Null write
pub struct MemoryNull {}

impl MemoryNull {
    pub const fn new() -> Self {
        Self {}
    }
}

impl WriteData for MemoryNull {
    fn write(&self, pc_state: &pc_state::PcState, memory: &mut memory::Memory, address: u16, data: u8) { }
    fn get_writing_time(&self) -> u8 { 0 }
}



