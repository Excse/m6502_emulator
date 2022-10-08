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
        $cpu.cycle(&mut cycles);

        $own_tests
        assert_eq!(cycles, 0);

        $(
            assert_eq!($cpu.$unchanged_field, $unchanged_field);
        )*
    };
}

/* ~~~~~~~~ Address Mode: Absolute ~~~~~~~~ */

#[test]
fn absolute_4_cycles() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xAD);
    memory.set(0x0201, 0x42);
    memory.set(0x0202, 0x42);
    memory.set(0x4242, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());

    instruction_test!(memory, cpu, 4, {
        assert_eq!(cpu.program_counter, 0x0203);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Address Mode: Implied ~~~~~~~~ */

#[test]
fn implied_7_cycles() {
    // TODO: Implement this test
}

#[test]
fn implied_2_cycles() {
    // TODO: Implement this test
}

#[test]
fn implied_3_cycles() {
    // TODO: Implement this test
}

#[test]
fn implied_4_cycles() {
    // TODO: Implement this test
}

#[test]
fn implied_6_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Accumulator ~~~~~~~~ */

#[test]
fn accumulator_2_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Immediate ~~~~~~~~ */

#[test]
fn immediate_2_cycles() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA9);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Address Mode: Zero page ~~~~~~~~ */

#[test]
fn zero_page_3_cycles() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA5);
    memory.set(0x0201, 0x0042);
    memory.set(0x0042, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn zero_page_5_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Zero page x ~~~~~~~~ */

#[test]
fn zero_page_x_4_cycles() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xB5);
    memory.set(0x0201, 0x0021);
    memory.set(0x0042, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0x0021;

    instruction_test!(memory, cpu, 4, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn zero_page_x_6_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Zero page y ~~~~~~~~ */

#[test]
fn zero_page_y_4_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Relative ~~~~~~~~ */

#[test]
fn relative_2_cycles_no_branch_no_new_page() {
    // TODO: Implement this test
}

#[test]
fn relative_3_cycles_branch_no_new_page() {
    // TODO: Implement this test
}

#[test]
fn relative_4_cycles_no_branch_new_page() {
    // TODO: Implement this test
}

#[test]
fn relative_5_cycles_branch_new_page() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Absolute ~~~~~~~~ */

#[test]
fn absolute_6_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Absolute x ~~~~~~~~ */

#[test]
fn absolute_x_4_cycles_no_page_crossing() {
    // TODO: Implement this test
}

#[test]
fn absolute_x_5_cycles_page_crossing() {
    // TODO: Implement this test
}

#[test]
fn absolute_x_7_cycles() {
    // TODO: Implement this test
}

#[test]
fn absolute_x_5_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Absolute y ~~~~~~~~ */

#[test]
fn absolute_y_4_cycles_no_page_crossing() {
    // TODO: Implement this test
}

#[test]
fn absolute_y_5_cycles_page_crossing() {
    // TODO: Implement this test
}

#[test]
fn absolute_y_5_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Indirect ~~~~~~~~ */

#[test]
fn indirect_5_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Indirect x ~~~~~~~~ */

#[test]
fn indirect_x_6_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Address Mode: Indirect y ~~~~~~~~ */

#[test]
fn indirect_y_5_cycles_no_page_crossing() {
    // TODO: Implement this test
}

#[test]
fn indirect_y_5_cycles_page_crossing() {
    // TODO: Implement this test
}

#[test]
fn indirect_y_6_cycles() {
    // TODO: Implement this test
}

/* ~~~~~~~~ Instruction: Load accumulator ~~~~~~~~ */

#[test]
fn load_accumulator_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA9);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());

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

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Load x ~~~~~~~~ */

#[test]
fn load_x_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA2);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());

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

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.x, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [accumulator, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Load y ~~~~~~~~ */

#[test]
fn load_y_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA0);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());

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

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.y, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Store accumulator ~~~~~~~~ */

#[test]
fn store_accumulator() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x85);
    memory.set(0x0201, 0x0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 42;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.memory.get(0x0000), 42);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow,
        zero, negative]);
}

/* ~~~~~~~~ Instruction: Store x ~~~~~~~~ */

#[test]
fn store_x() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x86);
    memory.set(0x0201, 0x0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 42;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.memory.get(0x0000), 42);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow,
        zero, negative]);
}

/* ~~~~~~~~ Instruction: Store y ~~~~~~~~ */

#[test]
fn store_y() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x84);
    memory.set(0x0201, 0x0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.y = 42;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.memory.get(0x0000), 42);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow,
        zero, negative]);
}

/* ~~~~~~~~ Instruction: Transfer accumulator to x ~~~~~~~~ */

#[test]
fn transfer_accumulator_to_x_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xAA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_accumulator_to_x_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xAA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_accumulator_to_x_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xAA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Transfer accumulator to x ~~~~~~~~ */

#[test]
fn transfer_accumulator_to_y_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA8);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.y, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_accumulator_to_y_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA8);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.y, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_accumulator_to_y_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xA8);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.y, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Transfer stack pointer to x ~~~~~~~~ */

#[test]
fn transfer_stack_pointer_to_x_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xBA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_stack_pointer_to_x_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xBA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_stack_pointer_to_x_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0xBA);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [y, accumulator, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Transfer x to accumulator ~~~~~~~~ */

#[test]
fn transfer_x_to_accumulator_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x8A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_x_to_accumulator_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x8A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_x_to_accumulator_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x8A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.x, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Transfer x to stack pointer ~~~~~~~~ */

#[test]
fn transfer_x_to_stack_pointer_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x9A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, accumulator, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_x_to_stack_pointer_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x9A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, accumulator, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_x_to_stack_pointer_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x9A);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.x = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, accumulator, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Transfer y to accumulator ~~~~~~~~ */

#[test]
fn transfer_y_to_accumulator_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x98);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.y = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_y_to_accumulator_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x98);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.y = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.accumulator, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn transfer_y_to_accumulator_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x98);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.y = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Push accumulator to the stack ~~~~~~~~ */

#[test]
fn push_accumulator() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x48);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_1010;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.memory.get(0x01FF), 0b0010_1010);
    }, [x, y, accumulator, carry, interrupt, decimal, _break, overflow, zero, negative]);
}

/* ~~~~~~~~ Instruction: Push processor status to the stack ~~~~~~~~ */

#[test]
fn push_processor_status() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x08);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.negative = true;
    cpu.overflow = false;
    cpu._break = false;
    cpu.decimal = true;
    cpu.interrupt = false;
    cpu.zero = true;
    cpu.carry = false;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.memory.get(0x01FF), 0b10101010);
    }, [x, y, accumulator, carry, interrupt, decimal, _break, overflow, zero, negative]);
}

/* ~~~~~~~~ Instruction: Pull accumulator ~~~~~~~~ */

#[test]
fn pull_accumulator_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x68);
    memory.set(0x01FF, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer -= 1;

    instruction_test!(memory, cpu, 4, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0x00FF);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn pull_accumulator_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x68);
    memory.set(0x01FF, 0b0000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer -= 1;

    instruction_test!(memory, cpu, 4, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0x00FF);
        assert_eq!(cpu.accumulator, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn pull_accumulator_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x68);
    memory.set(0x01FF, 0b1000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer -= 1;

    instruction_test!(memory, cpu, 4, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0x00FF);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Pull processor status ~~~~~~~~ */

#[test]
fn pull_processor_status() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x28);
    memory.set(0x01FF, 0b10101010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.stack_pointer -= 1;

    instruction_test!(memory, cpu, 4, {
        assert_eq!(cpu.program_counter, 0x0201);
        assert_eq!(cpu.stack_pointer, 0x00FF);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.overflow, false);
        assert_eq!(cpu._break, false);
        assert_eq!(cpu.decimal, true);
        assert_eq!(cpu.interrupt, false);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.carry, false);
    }, [x, y, accumulator]);
}

/* ~~~~~~~~ Instruction: And ~~~~~~~~ */

#[test]
fn and_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x29);
    memory.set(0x0201, 0xFF);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_1010;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn and_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x29);
    memory.set(0x0201, 0xFF);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn and_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x29);
    memory.set(0x0201, 0xFF);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b1000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Exclusive or ~~~~~~~~ */

#[test]
fn exclusive_or_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x49);
    memory.set(0x0201, 0b0010_0010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_1000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn exclusive_or_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x49);
    memory.set(0x0201, 0b0101_0101);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0101_0101;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn exclusive_or_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x49);
    memory.set(0x0201, 0b0111_1111);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b1111_1111;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Or ~~~~~~~~ */

#[test]
fn or_no_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x09);
    memory.set(0x0201, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_1000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0010_1010);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn or_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x09);
    memory.set(0x0201, 0b0000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b0000_0000);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

#[test]
fn or_negative() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x09);
    memory.set(0x0201, 0b1000_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0000_0000;

    instruction_test!(memory, cpu, 2, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.accumulator, 0b1000_0000);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, true);
    }, [x, y, stack_pointer, carry, interrupt, decimal, _break, overflow]);
}

/* ~~~~~~~~ Instruction: Or ~~~~~~~~ */

#[test]
fn bittest_not_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x24);
    memory.set(0x0201, 0x0042);
    memory.set(0x0042, 0b0010_1010);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_1010;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.zero, false);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.overflow, false);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break]);
}

#[test]
fn bittest_zero() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x24);
    memory.set(0x0201, 0x0042);
    memory.set(0x0042, 0b0000_1000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_0010;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, false);
        assert_eq!(cpu.overflow, false);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break]);
}

#[test]
fn bittest_all_flags() {
    let mut memory = TestMemory::new();
    memory.set(0x0200, 0x24);
    memory.set(0x0201, 0x0042);
    memory.set(0x0042, 0b1100_0000);

    let mut cpu = CPU::new(memory.borrow_mut());
    cpu.accumulator = 0b0010_1010;

    instruction_test!(memory, cpu, 3, {
        assert_eq!(cpu.program_counter, 0x0202);
        assert_eq!(cpu.zero, true);
        assert_eq!(cpu.negative, true);
        assert_eq!(cpu.overflow, true);
    }, [x, y, accumulator, stack_pointer, carry, interrupt, decimal, _break]);
}
