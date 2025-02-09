use super::super::clocks;
use super::super::memory::addressing;
use super::super::memory::addressing::Address16;
use super::super::memory::memory;
use super::pc_state;

pub fn nop(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    data
}

pub fn single_byte_instruction<R, W, I: Fn(&mut clocks::Clock, &mut pc_state::PcState, &mut memory::Memory, u8) -> u8>(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, read: R, write: W, instruction: I)
where
    R: Fn(&pc_state::PcState) -> u8,
    W: Fn(&mut pc_state::PcState, u8),
{
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    let data = read(pc_state);
    let result = instruction(clock, pc_state, memory, data);
    write(pc_state, result);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    pc_state.increment_pc(1);
}

pub fn read_write_instruction<R, W, I: Fn(&mut clocks::Clock, &mut pc_state::PcState, &mut memory::Memory, u8) -> u8>(
    clock: &mut clocks::Clock,
    pc_state: &mut pc_state::PcState,
    memory: &mut memory::Memory,
    address: &addressing::Addressing,
    read: R,
    write: W,
    instruction: I,
) where
    R: addressing::ReadData,
    W: addressing::WriteData,
{
    read_write_instruction_additional_delay(clock, pc_state, memory, address, read, write, instruction, 0);
}

pub fn read_write_instruction_additional_delay<R, W, I: Fn(&mut clocks::Clock, &mut pc_state::PcState, &mut memory::Memory, u8) -> u8>(
    clock: &mut clocks::Clock,
    pc_state: &mut pc_state::PcState,
    memory: &mut memory::Memory,
    address: &addressing::Addressing,
    read: R,
    write: W,
    instruction: I,
    additional_delay: u8,
) where
    R: addressing::ReadData,
    W: addressing::WriteData,
{
    let addr = address.address16(clock, pc_state, memory);
    let mut execute_time = address.get_addressing_time();

    let value = read.read(clock, pc_state, memory, addr);
    execute_time += read.get_reading_time();

    execute_time += write.get_writing_time();

    let data = instruction(clock, pc_state, memory, value);

    clock.increment(execute_time as u32);
    clock.increment(additional_delay as u32);

    write.write(clock, pc_state, memory, addr, data);

    pc_state.increment_pc((address.get_addressing_size() + 1) as i16);
}

pub fn break_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let adl = memory.read(clock, 0xFFFE);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_pch());
    pc_state.increment_s(-1);

    pc_state.increment_pc(1);
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_pcl());
    pc_state.increment_s(-1);

    // The 'B' flag, only alters the value on the stack, not ongoing status.
    pc_state.set_flag_b(true);
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_p());
    pc_state.increment_s(-1);
    pc_state.set_flag_b(false);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let adh = memory.read(clock, 0xFFFF);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.set_pc(adl as u16 + ((adh as u16) << 8));
}

pub fn jump_sub_routine_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let adl = memory.read(clock, pc_state.get_pc());

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    // Increment before store, to catch low to high carry.
    pc_state.increment_pc(1);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_pch());
    pc_state.increment_s(-1);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.write(clock, pc_state.get_s() as u16, pc_state.get_pcl());
    pc_state.increment_s(-1);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let adh = memory.read(clock, pc_state.get_pc());

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.set_pc(adl as u16 + ((adh as u16) << 8));
}

pub fn return_from_interrupt(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_p(memory.read(clock, pc_state.get_s() as u16));

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_pcl(memory.read(clock, pc_state.get_s() as u16));

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_pch(memory.read(clock, pc_state.get_s() as u16));

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.read(clock, pc_state.get_pc());

    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}

pub fn return_from_sub_routine_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    // T1 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
    // T2 - Stack Ptr
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    // T3 - Stack Ptr + 1 -> PCL
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_pcl(memory.read(clock, pc_state.get_s() as u16));
    // T4 - Stack Ptr + 1 -> PCL
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_pch(memory.read(clock, pc_state.get_s() as u16));
    // T5 - discarded
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.read(clock, pc_state.get_pc());
    // T0 - Next instruction
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
}

pub fn jump_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, address: &addressing::Addressing)
{
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    let addr = address.address16(clock, pc_state, memory);
    let execute_time = address.get_addressing_time();
    clock.increment(execute_time as u32);
    pc_state.set_pc(addr);
}

pub fn branch_page_delay(clock: &mut clocks::Clock, start_address: u16, end_address: u16) {
    // If pages don't match, add a cycle.
    if (start_address & 0xF00) != (end_address & 0xF00) {
        clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    }
}

pub fn branch_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, condition_mask: u8, condition: u8) {
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);

    if (pc_state.get_p() & condition_mask) == condition {
        let initial_pc = pc_state.get_pc();
        let delta = memory.read(clock, pc_state.get_pc().wrapping_add(1));
        if delta & 0x80 != 0 {
            pc_state.increment_pc((delta as i8) as i16);
        } else {
            pc_state.increment_pc(delta as i16);
        }
        // If branch to same page add 1, else add 2
        // TODO: Confirm if starting address is 'tmp16' or 'tmp16+2'
        branch_page_delay(clock, initial_pc.wrapping_add(2), pc_state.get_pc().wrapping_add(2));

        clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    }

    pc_state.increment_pc(2);
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}

pub fn asl(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_c(0 != (data >> 7) & 0x1);
    let left_shift = data << 1;
    pc_state::set_status_nz(pc_state, left_shift);
    left_shift
}

pub fn lsr(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_c(0 != data & 0x1);
    let right_shift = data >> 1;
    pc_state::set_status_nz(pc_state, right_shift);
    right_shift
}

pub fn rol(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let t8 = (data << 1) | pc_state.get_flag_c() as u8;
    pc_state.set_flag_c(1 == (data >> 7) & 1);
    pc_state::set_status_nz(pc_state, t8);
    t8
}

pub fn ror(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let t8 = (data >> 1) | ((pc_state.get_flag_c() as u8) << 7);
    pc_state.set_flag_c(1 == data & 1);
    pc_state::set_status_nz(pc_state, t8);
    t8
}

pub fn ldx(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_x(data);
    pc_state::set_status_nz(pc_state, pc_state.get_x());
    0
}

pub fn ldy(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_y(data);
    pc_state::set_status_nz(pc_state, pc_state.get_y());
    0
}

pub fn lda(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_a(data);
    pc_state::set_status_nz(pc_state, data);
    0
}

pub fn and(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_a(pc_state.get_a() & data);
    pc_state::set_status_nz(pc_state, pc_state.get_a());
    0
}

pub fn eor(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_a(pc_state.get_a() ^ data);
    pc_state::set_status_nz(pc_state, pc_state.get_a());
    0
}

pub fn or(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_a(pc_state.get_a() | data);
    pc_state::set_status_nz(pc_state, pc_state.get_a());
    0
}

pub fn add_carry(pc_state: &mut pc_state::PcState, a: u8, b: u8, c: u8) -> u8 {
    if !pc_state.get_flag_d() {
        let r = (a as u16).wrapping_add(b as u16) + c as u16;
        let rc = a.wrapping_add(b).wrapping_add(c);
        pc_state.set_flag_n(0 != rc & 0x80);
        pc_state.set_flag_z(rc == 0x0);
        pc_state.set_flag_v(rc as i8 as u16 != r); // Overflow
        pc_state.set_flag_c(0x100 == (r & 0x100));

        a.wrapping_add(b).wrapping_add(c)
    } else {
        // Decimal Addition
        // FIXME need to fix flags
        let r = (((a >> 4) & 0xF) * 10 + ((a & 0xF) % 10) + ((b >> 4) & 0xF) * 10 + ((b & 0xF) % 10) + c) as u16;
        let rc = a.wrapping_add(b).wrapping_add(c) as u16; // ???? TODO
        pc_state.set_flag_n(false);
        pc_state.set_flag_z(rc == 0x0);
        // TODO: Check not needed        pc_state.set_flag_v(rc != r); // Overflow
        pc_state.set_flag_c(r > 99);
        ((((r / 10 % 10) << 4) & 0xf0) + (r % 10)) as u8
    }
}

pub fn adc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let result = add_carry(pc_state, pc_state.get_a(), data, pc_state.get_flag_c() as u8);
    pc_state.set_a(result);
    0
}

pub fn sbc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let result = sub_carry(pc_state, pc_state.get_a(), data, !pc_state.get_flag_c() as u8);
    pc_state.set_a(result);
    0
}

pub fn sub_carry(pc_state: &mut pc_state::PcState, a: u8, b: u8, c: u8) -> u8 {
    if !pc_state.get_flag_d() {
        let mut r = (a as i8 as i16) - (b as i8 as i16) - (c as i8 as i16);
        let rs = a.wrapping_sub(b).wrapping_sub(c) as i8;
        pc_state.set_flag_n(rs < 0); // Negative
        pc_state.set_flag_z(rs == 0); // Zero
        pc_state.set_flag_v(r != rs as i16); // Overflow

        r = (a as i16) - (b as i16) - (c as i16);
        pc_state.set_flag_c(0x100 != (r as u16 & 0x100)); // Carry (not borrow
        a.wrapping_sub(b).wrapping_sub(c)
    } else {
        // Decimal subtraction
        // FIXME need to fix flags

        let r = (((a >> 4) & 0xF) * 10 + ((a & 0xF) % 10)) as i16 - (((b >> 4) & 0xF) * 10 + ((b & 0xF) % 10)) as i16 - c as i16;

        // rc = a + b + c
        pc_state.set_flag_n(r < 0);
        pc_state.set_flag_z(r == 0x0);
        //  Need to check/fix conditions for V
        // self.pc_state.P.V = (rc != r) ? 1:0;   # Overflow
        pc_state.set_flag_v(true); // Overflow

        pc_state.set_flag_c((0..=99).contains(&r));
        (((((r / 10) % 10) << 4) & 0xf0) + (r % 10)) as u8
    }
}

pub fn compare(pc_state: &mut pc_state::PcState, a: u8, b: u8) {
    // TODO: Check/test
    let rs = a.wrapping_sub(b);
    pc_state.set_flag_n(0x80 == (rs & 0x80)); // Negative
    pc_state.set_flag_z(rs == 0); // Zero
    let r = a as i16 - b as i16;
    pc_state.set_flag_c(0x100 != (r & 0x100)); // Carry (not borrow)
}

pub fn cpx(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    compare(pc_state, pc_state.get_x(), data);
    0
}

pub fn cpy(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    compare(pc_state, pc_state.get_y(), data);
    0
}

pub fn cmp(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    compare(pc_state, pc_state.get_a(), data);
    0
}
pub fn bit(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_n(0x80 == (data & 0x80));
    pc_state.set_flag_v(0x40 == (data & 0x40));
    pc_state.set_flag_z((pc_state.get_a() & data) == 0x0);
    0
}

pub fn sta(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.get_a()
}

pub fn dcp(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let decremented = data.wrapping_sub(1);
    pc_state::set_status_nz(pc_state, decremented);
    compare(pc_state, pc_state.get_a(), decremented);
    decremented
}

pub fn sty(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.get_y()
}

pub fn stx(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.get_x()
}

pub fn sax(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.get_a() & pc_state.get_x()
}

pub fn dec(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let incremented = data.wrapping_sub(1);
    pc_state::set_status_nz(pc_state, incremented);
    incremented
}

pub fn clc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_c(false);
    0
}

pub fn cld(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_d(false);
    0
}

pub fn cli(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_i(false);
    0
}

pub fn clv(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_v(false);
    0
}

pub fn sec(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_c(true);
    0
}

pub fn sei(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_i(true);
    0
}

pub fn sed(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state.set_flag_d(true);
    0
}

pub fn inc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let incremented = data.wrapping_add(1);
    pc_state::set_status_nz(pc_state, incremented);
    incremented
}

// Undocumented instruction
pub fn isc(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let increment = inc(clock, pc_state, memory, data);
    sbc(clock, pc_state, memory, increment);
    increment
}

pub fn slo(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    let shift = asl(clock, pc_state, memory, data);
    or(clock, pc_state, memory, shift);
    shift
}

pub fn lax(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
     lda(clock, pc_state, memory, data);
     ldx(clock, pc_state, memory, data)
}

pub fn t_no_status(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    data
}

pub fn t_status(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory, data: u8) -> u8 {
    pc_state::set_status_nz(pc_state, data);
    data
}

pub fn php_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    // T1 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
    // T2 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.set_flag_b(true);
    pc_state.set_flag_x1(true);
    memory.write_sp(clock, pc_state.get_s(), pc_state.get_p());
    pc_state.increment_s(-1);
    // T0 - Next kid
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}

pub fn plp_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    // T1 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
    // T2 Stack Ptr. (Discard data)
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.read_sp(clock, pc_state.get_s());
    // T3 Stack Ptr + 1.
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_p(memory.read_sp(clock, pc_state.get_s()));
    // T0 - Next instruction
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}

pub fn pha_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    // T1 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
    // T2 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.write_sp(clock, pc_state.get_s(), pc_state.get_a());
    pc_state.increment_s(-1);
    // T0 - Next kid
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}

pub fn pla_instruction(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState, memory: &mut memory::Memory) {
    // T1 - PC + 1
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_pc(1);
    // T2 Stack Ptr. (Discard data)
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    memory.read_sp(clock, pc_state.get_s());
    // T3 Stack Ptr + 1.
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
    pc_state.increment_s(1);
    pc_state.set_a(memory.read_sp(clock, pc_state.get_s()));
    pc_state::set_status_nz(pc_state, pc_state.get_a());
    // T0 - Next instruction
    clock.increment(pc_state::PcState::CYCLES_TO_CLOCK as u32);
}
