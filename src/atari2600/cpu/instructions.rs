use super::super::clocks;
use super::super::memory::memory;
use super::super::memory::addressing;
use super::super::ports;
use super::pc_state;
use super::instruction_set;

pub struct Instruction {}

// There's likely a better way to specify the memory types, but this achieves the intent.
const ADDR_IMM:addressing::AddressingIMM = addressing::AddressingIMM::new();
const ADDR_ZP:addressing::AddressingZP = addressing::AddressingZP::new();
const ADDR_ZPX:addressing::AddressingZPX = addressing::AddressingZPX::new();
const ADDR_IZX:addressing::AddressingIZX = addressing::AddressingIZX::new();
const ADDR_IZY:addressing::AddressingIZY = addressing::AddressingIZY::new();

const NULL_READ:addressing::NullRead = addressing::NullRead::new();
const MEMORY_READ:addressing::MemoryRead = addressing::MemoryRead::new();
const MEMORY_WRITE:addressing::MemoryWrite = addressing::MemoryWrite::new();
const REG_WRITE:addressing::RegisterWrite = addressing::RegisterWrite::new();
const MEMORY_NULL:addressing::MemoryNull = addressing::MemoryNull::new();

const READ_NULL: pc_state::ReadNull = pc_state::ReadNull::new();
const READ_REG_X: pc_state::ReadX = pc_state::ReadX::new();
const WRITE_NULL: pc_state::WriteNull = pc_state::WriteNull::new();
const WRITE_REG_X: pc_state::WriteX = pc_state::WriteX::new();

impl Instruction {


    pub fn execute(
        op_code: u8,
        clock: &mut clocks::Clock,
        memory: &mut memory::Memory,
        pc_state: &mut pc_state::PcState,
        ports: &mut ports::Ports) {
        match op_code {

            0xA2 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::ldx); }

            0xA9 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IMM, MEMORY_READ, MEMORY_NULL, instruction_set::lda); }

            //STA
            0x81 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZX, NULL_READ, REG_WRITE, instruction_set::sta); }
            0x85 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZP,  NULL_READ, REG_WRITE, instruction_set::sta); }
            0x95 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_ZPX, NULL_READ, REG_WRITE, instruction_set::sta); }
            0x91 => { instruction_set::read_write_instruction(clock, pc_state, memory, &ADDR_IZY, NULL_READ, REG_WRITE, instruction_set::sta); }


            0x18 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::clc); }
            0xD8 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::cld); }
            0x58 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::cli); }
            0xB8 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::clv); }

            0x38 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::sec); }
            0x78 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::sei); }
            0xF8 => { instruction_set::single_byte_instruction(clock, pc_state, READ_NULL, WRITE_NULL, instruction_set::sed); }
            _ => {
                panic!("Opcode not implemented: 0x{:x}", op_code);
            }
        }
    }
}
