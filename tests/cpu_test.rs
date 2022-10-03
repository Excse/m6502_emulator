use std::borrow::BorrowMut;
use m6052_emulator::cpu::{CPU, Memory};

#[derive(Clone)]
struct TestMemory {
    data: [u8; 0xFFFF],
}

impl TestMemory {
    pub fn new() -> TestMemory {
        TestMemory { data: [0; 0xFFFF] }
    }
}

impl Memory for TestMemory {
    fn read(&self, cycles: &mut isize, address: u16) -> u8 {
        *cycles -= 1;
        self.get(address)
    }

    fn get(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, cycles: &mut isize, address: u16, value: u8) {
        *cycles -= 1;
        self.set(address, value);
    }

    fn set(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

macro_rules! instruction_test {
        ($memory: expr, $cpu: expr, $expected_cycles: expr, $own_tests: block,
        [$($unchanged_field: ident),*]) => {
            $(
                let $unchanged_field = $cpu.$unchanged_field;
            )*

            let mut cycles = $expected_cycles;
            $cpu.run(&mut cycles);

            $own_tests
            assert_eq!(cycles, 0);

            $(
                assert_eq!($cpu.$unchanged_field, $unchanged_field);
            )*
        };
    }

#[test]
fn load_accumulator_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA9);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_accumulator_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA9);
    memory.set(0x0201, 0b0000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_accumulator_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA9);
    memory.set(0x0201, 0b1000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_x_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA2);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.x, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [accumulator, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_x_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA2);
    memory.set(0x0201, 0b0000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.x, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [accumulator, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_x_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA2);
    memory.set(0x0201, 0b1000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.x, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [accumulator, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_y_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA0);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.y, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_y_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA0);
    memory.set(0x0201, 0b0000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.y, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn load_y_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA0);
    memory.set(0x0201, 0b1000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.y, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn store_accumulator() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x85);
    memory.set(0x0201, 0x0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.accumulator = 42;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.memory.get(0x0000), 42);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow,
        zero, negative]);
}

#[test]
fn store_x() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x86);
    memory.set(0x0201, 0x0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.x = 42;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.memory.get(0x0000), 42);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow,
        zero, negative]);
}

#[test]
fn store_y() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x84);
    memory.set(0x0201, 0x0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.y = 42;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.memory.get(0x0000), 42);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow,
        zero, negative]);
}

#[test]
fn transfer_accumulator_to_x() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xAA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.accumulator = 42;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.x, 42);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow, zero,
        negative]);
}

#[test]
fn transfer_accumulator_to_y() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA8);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.accumulator = 42;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.y, 42);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow, zero,
        negative]);
}

#[test]
fn transfer_stack_pointer_to_x() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xBA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.stack_pointer = 42;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.x, 42);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow, zero,
        negative]);
}

#[test]
fn transfer_x_to_accumulator() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x8A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.x = 42;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 42);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow, zero, negative]);
}

#[test]
fn transfer_x_to_stack_pointer() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x9A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.x = 42;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.stack_pointer, 42);
    }, [x, y, accumulator, carry, interrupt, decimal, _break, overflow, zero, negative]);
}

#[test]
fn transfer_y_to_accumulator() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x98);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.program_counter = 0x0200;
    cpu.y = 42;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 42);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow, zero, negative]);
}

