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
            program_counter: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            memory,
            stack_pointer: 0,
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
            /* Fetch */
            let opcode = self.memory.read(cycles, self.program_counter);
            self.program_counter += 1;

            /* Execute */
            self.execute(cycles, opcode);
        }
    }

    pub fn execute(&mut self, cycles: &mut isize, opcode: u8) {
        let instruction = OPCODES[opcode as usize];
        match instruction {
            Some(Instruction::AddWithCarry(_)) => {}
            Some(Instruction::And(_)) => {}
            Some(Instruction::ArithmeticShiftLeft(_)) => {}
            Some(Instruction::BranchOnCarryClear(_)) => {}
            Some(Instruction::BranchOnCarrySet(_)) => {}
            Some(Instruction::BranchOnEqual(_)) => {}
            Some(Instruction::BitTest(_)) => {}
            Some(Instruction::BranchOnMinus(_)) => {}
            Some(Instruction::BranchOnNotEqual(_)) => {}
            Some(Instruction::BranchOnPlus(_)) => {}
            Some(Instruction::Break(_)) => {}
            Some(Instruction::BranchOnOverflowClear(_)) => {}
            Some(Instruction::BranchOnOverflowSet(_)) => {}
            Some(Instruction::ClearCarry(_)) => {}
            Some(Instruction::ClearDecimal(_)) => {}
            Some(Instruction::ClearInterruptDisable(_)) => {}
            Some(Instruction::ClearOverflow(_)) => {}
            Some(Instruction::Compare(_)) => {}
            Some(Instruction::CompareWithX(_)) => {}
            Some(Instruction::CompareWithY(_)) => {}
            Some(Instruction::Decrement(_)) => {}
            Some(Instruction::DecrementX(_)) => {}
            Some(Instruction::DecrementY(_)) => {}
            Some(Instruction::ExclusiveOr(_)) => {}
            Some(Instruction::Increment(_)) => {}
            Some(Instruction::IncrementX(_)) => {}
            Some(Instruction::IncrementY(_)) => {}
            Some(Instruction::Jump(_)) => {}
            Some(Instruction::JumpSubroutine(_)) => {}
            Some(Instruction::LoadAccumulator(mode)) => { self.load_accumulator(cycles, mode) }
            Some(Instruction::LoadX(mode)) => { self.load_x(cycles, mode) }
            Some(Instruction::LoadY(mode)) => { self.load_y(cycles, mode) }
            Some(Instruction::LogicalShiftRight(_)) => {}
            Some(Instruction::NoOperation(_)) => {}
            Some(Instruction::OrWithAccumulator(_)) => {}
            Some(Instruction::PushAccumulator(_)) => {}
            Some(Instruction::PushProcessorStatus(_)) => {}
            Some(Instruction::PullAccumulator(_)) => {}
            Some(Instruction::PullProcessorStatus(_)) => {}
            Some(Instruction::RotateLeft(_)) => {}
            Some(Instruction::RotateRight(_)) => {}
            Some(Instruction::ReturnFormInterrupt(_)) => {}
            Some(Instruction::ReturnFromSubroutine(_)) => {}
            Some(Instruction::SubtractWithCarry(_)) => {}
            Some(Instruction::SetCarry(_)) => {}
            Some(Instruction::SetDecimal(_)) => {}
            Some(Instruction::SetInterruptDisable(_)) => {}
            Some(Instruction::StoreAccumulator(mode)) => { self.store_accumulator(cycles, mode) }
            Some(Instruction::StoreX(mode)) => { self.store_x(cycles, mode) }
            Some(Instruction::StoreY(mode)) => { self.store_y(cycles, mode) }
            Some(Instruction::TransferAccumulatorToX(_)) => { self.transfer_accumulator_to_x() }
            Some(Instruction::TransferAccumulatorToY(_)) => { self.transfer_accumulator_to_y() }
            Some(Instruction::TransferStackpointerToX(_)) => { self.transfer_stack_pointer_to_x() }
            Some(Instruction::TransferXToAccumulator(_)) => { self.transfer_x_to_accumulator() }
            Some(Instruction::TransferXToStackpointer(_)) => { self.transfer_x_to_stack_pointr() }
            Some(Instruction::TransferYToAccumulator(_)) => { self.transfer_y_to_accumulator() }
            None => {}
        }
    }

    fn get_address(&mut self, cycles: &mut isize, mode: AddressingMode) -> Option<u16> {
        match mode {
            AddressingMode::Absolute => {
                let adress_low = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                let adress_high = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                let address = (adress_low as u16) + ((adress_high as u16) << 8);
                Some(address)
            }
            AddressingMode::AbsoluteXIndexed => {
                let adress_low = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                let adress_high = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                let mut address = (adress_low as u16) + ((adress_high as u16) << 8);
                address += self.x as u16;
                Some(address)
            }
            AddressingMode::AbsoluteYIndexed => {
                let adress_low = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                let adress_high = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                let mut address = (adress_low as u16) + ((adress_high as u16) << 8);
                address += self.y as u16;
                Some(address)
            }
            AddressingMode::Relative => {
                let offset = self.memory.read(cycles, self.program_counter) as i8;
                self.program_counter += 1;
                let address = self.program_counter + (offset as u16);
                Some(address)
            }
            AddressingMode::Zeropage => {
                let address = self.memory.read(cycles, self.program_counter) as u16;
                self.program_counter += 1;
                Some(address)
            }
            AddressingMode::ZeropageXIndexed => {
                let mut address = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                address += self.x;
                Some(address as u16)
            }
            AddressingMode::ZeropageYIndexed => {
                let mut address = self.memory.read(cycles, self.program_counter);
                self.program_counter += 1;
                address += self.y;
                Some(address as u16)
            }
            AddressingMode::Indirect => { panic!("Not implemented yet!") }
            AddressingMode::XIndexedIndirect => { panic!("Not implemented yet!") }
            AddressingMode::IndirectYIndexed => { panic!("Not implemented yet!") }
            _ => None,
        }
    }

    fn get_value(&mut self, cycles: &mut isize, mode: AddressingMode) -> u8 {
        let address = self.get_address(cycles, mode);
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

    fn load_accumulator(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(cycles, mode);
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn load_x(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(cycles, mode);
        self.x = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn load_y(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let value = self.get_value(cycles, mode);
        self.y = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn store_accumulator(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let address = self.get_address(cycles, mode)
            .expect("Couldn't get the address of this instruction.");
        self.memory.write(cycles, address, self.accumulator);
    }

    fn store_x(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let address = self.get_address(cycles, mode)
            .expect("Couldn't get the address of this instruction.");
        self.memory.write(cycles, address, self.x);
    }

    fn store_y(&mut self, cycles: &mut isize, mode: AddressingMode) {
        let address = self.get_address(cycles, mode)
            .expect("Couldn't get the address of this instruction.");
        self.memory.write(cycles, address, self.y);
    }

    fn transfer_accumulator_to_x(&mut self) {
        let value = self.accumulator;
        self.x = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn transfer_accumulator_to_y(&mut self) {
        let value = self.accumulator;
        self.y = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn transfer_stack_pointer_to_x(&mut self) {
        let value = self.stack_pointer;
        self.x = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn transfer_x_to_accumulator(&mut self) {
        let value = self.x;
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn transfer_x_to_stack_pointr(&mut self) {
        let value = self.x;
        self.stack_pointer = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }

    fn transfer_y_to_accumulator(&mut self) {
        let value = self.y;
        self.accumulator = value;
        self.zero = value == 0;
        self.negative = (value >> 7) != 0;
    }
}
