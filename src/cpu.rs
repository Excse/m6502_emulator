use crate::instruction::{AddressingMode, Instruction, OPCODES};

pub trait Memory: MemoryClone {
    fn read(&self, cycles: &mut isize, address: u16) -> u8;

    fn get(&self, address: u16) -> u8;

    fn write(&mut self, cycles: &mut isize, address: u16, value: u8);

    fn set(&mut self, address: u16, value: u8);
}

pub trait MemoryClone {
    fn clone_box(&self) -> Box<dyn Memory>;
}

impl<T> MemoryClone for T where T: 'static + Memory + Clone {
    fn clone_box(&self) -> Box<dyn Memory> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Memory> {
    fn clone(&self) -> Box<dyn Memory> {
        self.clone_box()
    }
}

pub struct CPU<'a> {
    pub program_counter: u16,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,

    pub memory: &'a mut (dyn Memory + 'a),

    /* All the status register flags */
    pub negative: bool,
    pub overflow: bool,
    pub _break: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

impl<'a> CPU<'a> {
    pub fn new(memory: &'a mut dyn Memory) -> CPU<'a> {
        CPU {
            program_counter: 0x0200,
            accumulator: 0,
            x: 0,
            y: 0,
            memory,
            stack_pointer: 0xFF,
            negative: false,
            overflow: false,
            _break: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
        }
    }

    pub fn run(&mut self, cycles: &mut isize) {
        while *cycles > 0 {
            self.cycle(cycles);
        }
    }

    pub fn cycle(&mut self, cycles: &mut isize) {
        /* Fetch */
        let opcode = self.memory.read(cycles, self.program_counter);
        self.program_counter += 1;

        /* Decode */
        let instruction = OPCODES[opcode as usize]
            .expect("Illegal opcode!");

        /* Execute */
        self.execute(cycles, instruction);
    }

    pub fn execute(&mut self, cycles: &mut isize, instruction: Instruction) {
        match instruction {
            Instruction::AddWithCarry(_) => {}
            Instruction::And(mode) => { self.and(cycles, mode) }
            Instruction::ArithmeticShiftLeft(_) => {}
            Instruction::BranchOnCarryClear(_) => {}
            Instruction::BranchOnCarrySet(_) => {}
            Instruction::BranchOnEqual(_) => {}
            Instruction::BitTest(mode) => { self.bittest(cycles, mode) }
            Instruction::BranchOnMinus(_) => {}
            Instruction::BranchOnNotEqual(_) => {}
            Instruction::BranchOnPlus(_) => {}
            Instruction::Break(_) => {}
            Instruction::BranchOnOverflowClear(_) => {}
            Instruction::BranchOnOverflowSet(_) => {}
            Instruction::ClearCarry(_) => {}
            Instruction::ClearDecimal(_) => {}
            Instruction::ClearInterruptDisable(_) => {}
            Instruction::ClearOverflow(_) => {}
            Instruction::Compare(_) => {}
            Instruction::CompareWithX(_) => {}
            Instruction::CompareWithY(_) => {}
            Instruction::Decrement(_) => {}
            Instruction::DecrementX(_) => {}
            Instruction::DecrementY(_) => {}
            Instruction::ExclusiveOr(mode) => { self.exclusive_or(cycles, mode) }
            Instruction::Increment(_) => {}
            Instruction::IncrementX(_) => {}
            Instruction::IncrementY(_) => {}
            Instruction::Jump(_) => {}
            Instruction::JumpSubroutine(_) => {}
            Instruction::LoadAccumulator(mode) => { self.load_accumulator(cycles, mode) }
            Instruction::LoadX(mode) => { self.load_x(cycles, mode) }
            Instruction::LoadY(mode) => { self.load_y(cycles, mode) }
            Instruction::LogicalShiftRight(_) => {}
            Instruction::NoOperation(_) => {}
            Instruction::OrWithAccumulator(mode) => { self.or_with_accumulator(cycles, mode) }
            Instruction::PushAccumulator(_) => { self.push_accumulator(cycles) }
            Instruction::PushProcessorStatus(_) => { self.push_processor_status(cycles) }
            Instruction::PullAccumulator(_) => { self.pull_accumulator(cycles) }
            Instruction::PullProcessorStatus(_) => { self.pull_processor_status(cycles) }
            Instruction::RotateLeft(_) => {}
            Instruction::RotateRight(_) => {}
            Instruction::ReturnFormInterrupt(_) => {}
            Instruction::ReturnFromSubroutine(_) => {}
            Instruction::SubtractWithCarry(_) => {}
            Instruction::SetCarry(_) => {}
            Instruction::SetDecimal(_) => {}
            Instruction::SetInterruptDisable(_) => {}
            Instruction::StoreAccumulator(mode) => { self.store_accumulator(cycles, mode) }
            Instruction::StoreX(mode) => { self.store_x(cycles, mode) }
            Instruction::StoreY(mode) => { self.store_y(cycles, mode) }
            Instruction::TransferAccumulatorToX(_) => { self.transfer_accumulator_to_x(cycles) }
            Instruction::TransferAccumulatorToY(_) => { self.transfer_accumulator_to_y(cycles) }
            Instruction::TransferStackpointerToX(_) => { self.transfer_stack_pointer_to_x(cycles) }
            Instruction::TransferXToAccumulator(_) => { self.transfer_x_to_accumulator(cycles) }
            Instruction::TransferXToStackpointer(_) => { self.transfer_x_to_stack_pointr(cycles) }
            Instruction::TransferYToAccumulator(_) => { self.transfer_y_to_accumulator(cycles) }
        }
    }

    fn get_processor_status(&self) -> u8 {
        (self.negative as u8) << 7 | (self.overflow as u8) << 6 | 0x20
            | (self._break as u8) << 4 | (self.decimal as u8) << 3
            | (self.interrupt as u8) << 2 | (self.zero as u8) << 1
            | (self.carry as u8) << 0
    }

    fn set_processor_status(&mut self, value: u8) {
        self.negative = (value & 0x80) != 0;
        self.overflow = (value & 0x40) != 0;
        self._break = (value & 0x10) != 0;
        self.decimal = (value & 0x8) != 0;
        self.interrupt = (value & 0x4) != 0;
        self.zero = (value & 0x2) != 0;
        self.carry = (value & 0x1) != 0;
    }

    fn get_address(&mut self, page_crossing: bool, cycles: &mut isize,
                   mode: AddressingMode) -> Option<u16> {
        match mode {
            AddressingMode::Absolute => {
                let address_low = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;
                let address_high = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;

                let address = address_low + (address_high << 8);
                Some(address)
            }
            AddressingMode::AbsoluteXIndexed => {
                let address_low = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;
                let address_high = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;

                let mut address = address_low + (address_high << 8);
                address += self.x as u16;

                if page_crossing {
                    let address_low_x = address_low + self.x as u16;
                    if address_low_x > 0xFF {
                        *cycles -= 1;
                    }
                }

                Some(address)
            }
            AddressingMode::AbsoluteYIndexed => {
                let address_low = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;
                let address_high = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;

                let mut address = address_low + (address_high << 8);
                address += self.y as u16;

                if page_crossing {
                    let address_low_x = address_low + self.x as u16;
                    if address_low_x > 0xFF {
                        *cycles -= 1;
                    }
                }

                Some(address)
            }
            AddressingMode::Relative => {
                let offset = self.memory.read(cycles, self.program_counter) as i8;
                self.program_counter += 1;
                let address = self.program_counter.wrapping_add(offset as u16);
                Some(address)
            }
            AddressingMode::Zeropage => {
                let address = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                Some(address as u16)
            }
            AddressingMode::ZeropageXIndexed => {
                let mut address = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                address += self.x;
                *cycles -= 1;
                Some(address as u16)
            }
            AddressingMode::ZeropageYIndexed => {
                let mut address = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                address += self.y;
                *cycles -= 1;
                Some(address as u16)
            }
            AddressingMode::Indirect => { panic!("Not implemented yet!") }
            AddressingMode::XIndexedIndirect => { panic!("Not implemented yet!") }
            AddressingMode::IndirectYIndexed => { panic!("Not implemented yet!") }
            _ => None,
        }
    }

    fn get_value(&mut self, page_crossing: bool, cycles: &mut isize, mode: AddressingMode) -> u8 {
        let address = self.get_address(page_crossing, cycles, mode);
        match address {
            Some(address) => {
                let value = self.memory.read(cycles, address);
                value
            }
            None => match mode {
                AddressingMode::Accumulator => self.accumulator,
                AddressingMode::Immediate => {
                    let value = self.memory.read(cycles, self.program_counter);
                    self.program_counter += 1;
                    value
                }
                _ => { panic!("There went something terribly wrong.") }
            }
        }
    }

    fn push_stack(&mut self, cycles: &mut isize, value: u8) {
        let address = 0x0100 + self.stack_pointer as u16;
        self.memory.write(cycles, address, value);
        self.stack_pointer -= 1;
        *cycles -= 1;
    }

    fn pop_stack(&mut self, cycles: &mut isize) -> u8 {
        self.stack_pointer += 1;
        *cycles -= 1;
        let address = 0x0100 + self.stack_pointer as u16;
        let value = self.memory.read(cycles, address);
        value
    }

    fn load_accumulator(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
    }

    fn load_x(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        self.x = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
    }

    fn load_y(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        self.y = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
    }

    fn store_accumulator(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let address = self.get_address(false, cycles, mode)
            .expect("Couldn't get the address of this instruction.");
        self.memory.write(cycles, address, self.accumulator);
    }

    fn store_x(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let address = self.get_address(false, cycles, mode)
            .expect("Couldn't get the address of this instruction.");
        self.memory.write(cycles, address, self.x);
    }

    fn store_y(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let address = self.get_address(false, cycles, mode)
            .expect("Couldn't get the address of this instruction.");
        self.memory.write(cycles, address, self.y);
    }

    fn transfer_accumulator_to_x(&mut self, cycles: &mut isize) {
        let value = self.accumulator;
        self.x = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn transfer_accumulator_to_y(&mut self, cycles: &mut isize) {
        let value = self.accumulator;
        self.y = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn transfer_stack_pointer_to_x(&mut self, cycles: &mut isize) {
        let value = self.stack_pointer;
        self.x = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn transfer_x_to_accumulator(&mut self, cycles: &mut isize) {
        let value = self.x;
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn transfer_x_to_stack_pointr(&mut self, cycles: &mut isize) {
        let value = self.x;
        self.stack_pointer = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn transfer_y_to_accumulator(&mut self, cycles: &mut isize) {
        let value = self.y;
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn push_accumulator(&mut self, cycles: &mut isize) {
        self.push_stack(cycles, self.accumulator);
    }

    fn push_processor_status(&mut self, cycles: &mut isize) {
        let status = self.get_processor_status();
        self.push_stack(cycles, status);
    }

    fn pull_accumulator(&mut self, cycles: &mut isize) {
        let value = self.pop_stack(cycles);
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value & 0x80) != 0;
        *cycles -= 1;
    }

    fn pull_processor_status(&mut self, cycles: &mut isize) {
        let value = self.pop_stack(cycles);
        self.set_processor_status(value);
        *cycles -= 1;
    }

    fn and(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        let result = self.accumulator & value;
        self.accumulator = result;
        self.zero = result == 0;
        self.negative = (result & 0x80) != 0;
    }

    fn exclusive_or(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        let result = self.accumulator ^ value;
        self.accumulator = result;
        self.zero = result == 0;
        self.negative = (result & 0x80) != 0;
    }

    fn or_with_accumulator(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        let result = self.accumulator | value;
        self.accumulator = result;
        self.zero = result == 0;
        self.negative = (result & 0x80) != 0;
    }

    fn bittest(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(true, cycles, mode);
        let result = self.accumulator & value;
        self.zero = result == 0;
        self.negative = (value & 0x80) != 0;
        self.overflow = (value & 0x40) != 0;
    }
}
