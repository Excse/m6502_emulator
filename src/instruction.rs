use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    Immediate,
    Implied,
    Indirect,
    XIndexedIndirect,
    IndirectYIndexed,
    Relative,
    Zeropage,
    ZeropageXIndexed,
    ZeropageYIndexed,
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressingMode::Accumulator => write!(f, "Implied"),
            AddressingMode::Absolute => write!(f, "Absolute"),
            AddressingMode::AbsoluteXIndexed => write!(f, "Absolute x indexed"),
            AddressingMode::AbsoluteYIndexed => write!(f, "Absolute y indexed"),
            AddressingMode::Immediate => write!(f, "Immediate"),
            AddressingMode::Implied => write!(f, "Implied"),
            AddressingMode::Indirect => write!(f, "Indirect"),
            AddressingMode::XIndexedIndirect => write!(f, "X indexed indirect"),
            AddressingMode::IndirectYIndexed => write!(f, "Indirect y indexed"),
            AddressingMode::Relative => write!(f, "Relative"),
            AddressingMode::Zeropage => write!(f, "Zeropage"),
            AddressingMode::ZeropageXIndexed => write!(f, "Zeropage x indexed"),
            AddressingMode::ZeropageYIndexed => write!(f, "Zeropage y indexed")
        }
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    AddWithCarry(AddressingMode),
    And(AddressingMode),
    ArithmeticShiftLeft(AddressingMode),
    BranchOnCarryClear(AddressingMode),
    BranchOnCarrySet(AddressingMode),
    BranchOnEqual(AddressingMode),
    BitTest(AddressingMode),
    BranchOnMinus(AddressingMode),
    BranchOnNotEqual(AddressingMode),
    BranchOnPlus(AddressingMode),
    Break(AddressingMode),
    BranchOnOverflowClear(AddressingMode),
    BranchOnOverflowSet(AddressingMode),
    ClearCarry(AddressingMode),
    ClearDecimal(AddressingMode),
    ClearInterruptDisable(AddressingMode),
    ClearOverflow(AddressingMode),
    Compare(AddressingMode),
    CompareWithX(AddressingMode),
    CompareWithY(AddressingMode),
    Decrement(AddressingMode),
    DecrementX(AddressingMode),
    DecrementY(AddressingMode),
    ExclusiveOr(AddressingMode),
    Increment(AddressingMode),
    IncrementX(AddressingMode),
    IncrementY(AddressingMode),
    Jump(AddressingMode),
    JumpSubroutine(AddressingMode),
    LoadAccumulator(AddressingMode),
    LoadX(AddressingMode),
    LoadY(AddressingMode),
    LogicalShiftRight(AddressingMode),
    NoOperation(AddressingMode),
    OrWithAccumulator(AddressingMode),
    PushAccumulator(AddressingMode),
    PushProcessorStatus(AddressingMode),
    PullAccumulator(AddressingMode),
    PullProcessorStatus(AddressingMode),
    RotateLeft(AddressingMode),
    RotateRight(AddressingMode),
    ReturnFormInterrupt(AddressingMode),
    ReturnFromSubroutine(AddressingMode),
    SubtractWithCarry(AddressingMode),
    SetCarry(AddressingMode),
    SetDecimal(AddressingMode),
    SetInterruptDisable(AddressingMode),
    StoreAccumulator(AddressingMode),
    StoreX(AddressingMode),
    StoreY(AddressingMode),
    TransferAccumulatorToX(AddressingMode),
    TransferAccumulatorToY(AddressingMode),
    TransferStackpointerToX(AddressingMode),
    TransferXToAccumulator(AddressingMode),
    TransferXToStackpointer(AddressingMode),
    TransferYToAccumulator(AddressingMode),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::AddWithCarry(mode) => { write!(f, "Add with carry: {mode}") }
            Instruction::And(mode) => { write!(f, "And: {mode}") }
            Instruction::ArithmeticShiftLeft(mode) => { write!(f, "Arithmetic shift left: {mode}") }
            Instruction::BranchOnCarryClear(mode) => { write!(f, "Branch on carry clear: {mode}") }
            Instruction::BranchOnCarrySet(mode) => { write!(f, "Branch on carry set: {mode}") }
            Instruction::BranchOnEqual(mode) => { write!(f, "Branch on equal: {mode}") }
            Instruction::BitTest(mode) => { write!(f, "Bit tests: {mode}") }
            Instruction::BranchOnMinus(mode) => { write!(f, "Branch on minus: {mode}") }
            Instruction::BranchOnNotEqual(mode) => { write!(f, "Branch on not equal: {mode}") }
            Instruction::BranchOnPlus(mode) => { write!(f, "Branch on plus: {mode}") }
            Instruction::Break(mode) => { write!(f, "Break: {mode}") }
            Instruction::BranchOnOverflowClear(mode) => { write!(f, "Branch on overflow clear: {mode}") }
            Instruction::BranchOnOverflowSet(mode) => { write!(f, "Branch on overflow set: {mode}") }
            Instruction::ClearCarry(mode) => { write!(f, "Clear carry: {mode}") }
            Instruction::ClearDecimal(mode) => { write!(f, "Clear decimal: {mode}") }
            Instruction::ClearInterruptDisable(mode) => { write!(f, "Clear interrupt disable: {mode}") }
            Instruction::ClearOverflow(mode) => { write!(f, "Clear overflow: {mode}") }
            Instruction::Compare(mode) => { write!(f, "Compare: {mode}") }
            Instruction::CompareWithX(mode) => { write!(f, "Compare with x: {mode}") }
            Instruction::CompareWithY(mode) => { write!(f, "Compare with y: {mode}") }
            Instruction::Decrement(mode) => { write!(f, "Decrement: {mode}") }
            Instruction::DecrementX(mode) => { write!(f, "Decrement x: {mode}") }
            Instruction::DecrementY(mode) => { write!(f, "Decrement y: {mode}") }
            Instruction::ExclusiveOr(mode) => { write!(f, "Exclusive or: {mode}") }
            Instruction::Increment(mode) => { write!(f, "Increment: {mode}") }
            Instruction::IncrementX(mode) => { write!(f, "Increment x: {mode}") }
            Instruction::IncrementY(mode) => { write!(f, "Increment y: {mode}") }
            Instruction::Jump(mode) => { write!(f, "Jump: {mode}") }
            Instruction::JumpSubroutine(mode) => { write!(f, "Jump subroutine: {mode}") }
            Instruction::LoadAccumulator(mode) => { write!(f, "Load accumulator: {mode}") }
            Instruction::LoadX(mode) => { write!(f, "Load x: {mode}") }
            Instruction::LoadY(mode) => { write!(f, "Load y: {mode}") }
            Instruction::LogicalShiftRight(mode) => { write!(f, "Logical shift right: {mode}") }
            Instruction::NoOperation(mode) => { write!(f, "No operation: {mode}") }
            Instruction::OrWithAccumulator(mode) => { write!(f, "Or with accumulator: {mode}") }
            Instruction::PushAccumulator(mode) => { write!(f, "Push accumulator: {mode}") }
            Instruction::PushProcessorStatus(mode) => { write!(f, "Push processor status: {mode}") }
            Instruction::PullAccumulator(mode) => { write!(f, "Pull accumulator: {mode}") }
            Instruction::PullProcessorStatus(mode) => { write!(f, "Pull processor status: {mode}") }
            Instruction::RotateLeft(mode) => { write!(f, "Rotate left: {mode}") }
            Instruction::RotateRight(mode) => { write!(f, "Rotate right: {mode}") }
            Instruction::ReturnFormInterrupt(mode) => { write!(f, "Return from interrupt: {mode}") }
            Instruction::ReturnFromSubroutine(mode) => { write!(f, "Return from subroutine: {mode}") }
            Instruction::SubtractWithCarry(mode) => { write!(f, "Subtract with carry: {mode}") }
            Instruction::SetCarry(mode) => { write!(f, "Set carry: {mode}") }
            Instruction::SetDecimal(mode) => { write!(f, "Set decimal: {mode}") }
            Instruction::SetInterruptDisable(mode) => { write!(f, "Set interrupt disable: {mode}") }
            Instruction::StoreAccumulator(mode) => { write!(f, "Store accumulator: {mode}") }
            Instruction::StoreX(mode) => { write!(f, "Store x: {mode}") }
            Instruction::StoreY(mode) => { write!(f, "Store y: {mode}") }
            Instruction::TransferAccumulatorToX(mode) => { write!(f, "Transfer accumulator to x: {mode}") }
            Instruction::TransferAccumulatorToY(mode) => { write!(f, "Transfer accumulator to y: {mode}") }
            Instruction::TransferStackpointerToX(mode) => { write!(f, "Transfer stackpointer to x: {mode}") }
            Instruction::TransferXToAccumulator(mode) => { write!(f, "Transfer x to accumulator: {mode}") }
            Instruction::TransferXToStackpointer(mode) => { write!(f, "Transfer x to stackpointer: {mode}") }
            Instruction::TransferYToAccumulator(mode) => { write!(f, "Transfer y to accumulator: {mode}") }
        }
    }
}

pub static OPCODES: [Option<Instruction>; 256] = [
    /* 0x00 */
    Some(Instruction::Break(AddressingMode::Implied)),
    /* 0x01 */
    Some(Instruction::OrWithAccumulator(AddressingMode::XIndexedIndirect)),
    /* 0x02 */
    None,
    /* 0x03 */
    None,
    /* 0x04 */
    None,
    /* 0x05 */
    Some(Instruction::OrWithAccumulator(AddressingMode::Zeropage)),
    /* 0x06 */
    Some(Instruction::ArithmeticShiftLeft(AddressingMode::Zeropage)),
    /* 0x07 */
    None,
    /* 0x08 */
    Some(Instruction::PushProcessorStatus(AddressingMode::Implied)),
    /* 0x09 */
    Some(Instruction::OrWithAccumulator(AddressingMode::Immediate)),
    /* 0x0A */
    Some(Instruction::ArithmeticShiftLeft(AddressingMode::Accumulator)),
    /* 0x0B */
    None,
    /* 0x0C */
    None,
    /* 0x0D */
    Some(Instruction::OrWithAccumulator(AddressingMode::Absolute)),
    /* 0x0E */
    Some(Instruction::ArithmeticShiftLeft(AddressingMode::Absolute)),
    /* 0x0F */
    None,
    /* 0x10 */
    Some(Instruction::BranchOnPlus(AddressingMode::Relative)),
    /* 0x11 */
    Some(Instruction::OrWithAccumulator(AddressingMode::IndirectYIndexed)),
    /* 0x12 */
    None,
    /* 0x13 */
    None,
    /* 0x14 */
    None,
    /* 0x15 */
    Some(Instruction::OrWithAccumulator(AddressingMode::ZeropageXIndexed)),
    /* 0x16 */
    Some(Instruction::ArithmeticShiftLeft(AddressingMode::ZeropageXIndexed)),
    /* 0x17 */
    None,
    /* 0x18 */
    Some(Instruction::ClearCarry(AddressingMode::Implied)),
    /* 0x19 */
    Some(Instruction::OrWithAccumulator(AddressingMode::AbsoluteYIndexed)),
    /* 0x1A */
    None,
    /* 0x1B */
    None,
    /* 0x1C */
    None,
    /* 0x1D */
    Some(Instruction::OrWithAccumulator(AddressingMode::AbsoluteXIndexed)),
    /* 0x1E */
    Some(Instruction::ArithmeticShiftLeft(AddressingMode::AbsoluteXIndexed)),
    /* 0x1F */
    None,
    /* 0x20 */
    Some(Instruction::JumpSubroutine(AddressingMode::Absolute)),
    /* 0x21 */
    Some(Instruction::And(AddressingMode::XIndexedIndirect)),
    /* 0x22 */
    None,
    /* 0x23 */
    None,
    /* 0x24 */
    Some(Instruction::BitTest(AddressingMode::Zeropage)),
    /* 0x25 */
    Some(Instruction::And(AddressingMode::Zeropage)),
    /* 0x26 */
    Some(Instruction::RotateLeft(AddressingMode::Zeropage)),
    /* 0x27 */
    None,
    /* 0x28 */
    Some(Instruction::PullProcessorStatus(AddressingMode::Implied)),
    /* 0x29 */
    Some(Instruction::And(AddressingMode::Immediate)),
    /* 0x2A */
    Some(Instruction::RotateLeft(AddressingMode::Accumulator)),
    /* 0x2B */
    None,
    /* 0x2C */
    Some(Instruction::BitTest(AddressingMode::Absolute)),
    /* 0x2D */
    Some(Instruction::And(AddressingMode::Absolute)),
    /* 0x2E */
    Some(Instruction::RotateLeft(AddressingMode::Absolute)),
    /* 0x2F */
    None,
    /* 0x30 */
    Some(Instruction::BranchOnMinus(AddressingMode::Relative)),
    /* 0x31 */
    Some(Instruction::And(AddressingMode::IndirectYIndexed)),
    /* 0x32 */
    None,
    /* 0x33 */
    None,
    /* 0x34 */
    None,
    /* 0x35 */
    Some(Instruction::And(AddressingMode::ZeropageXIndexed)),
    /* 0x36 */
    Some(Instruction::RotateLeft(AddressingMode::ZeropageXIndexed)),
    /* 0x37 */
    None,
    /* 0x38 */
    Some(Instruction::SetCarry(AddressingMode::Implied)),
    /* 0x39 */
    Some(Instruction::And(AddressingMode::AbsoluteYIndexed)),
    /* 0x3A */
    None,
    /* 0x3B */
    None,
    /* 0x3C */
    None,
    /* 0x3D */
    Some(Instruction::And(AddressingMode::AbsoluteXIndexed)),
    /* 0x3E */
    Some(Instruction::RotateLeft(AddressingMode::AbsoluteXIndexed)),
    /* 0x3F */
    None,
    /* 0x40 */
    Some(Instruction::ReturnFormInterrupt(AddressingMode::Implied)),
    /* 0x41 */
    Some(Instruction::ExclusiveOr(AddressingMode::XIndexedIndirect)),
    /* 0x42 */
    None,
    /* 0x43 */
    None,
    /* 0x44 */
    None,
    /* 0x45 */
    Some(Instruction::ExclusiveOr(AddressingMode::Zeropage)),
    /* 0x46 */
    Some(Instruction::LogicalShiftRight(AddressingMode::Zeropage)),
    /* 0x47 */
    None,
    /* 0x48 */
    Some(Instruction::PushAccumulator(AddressingMode::Implied)),
    /* 0x49 */
    Some(Instruction::ExclusiveOr(AddressingMode::Immediate)),
    /* 0x4A */
    Some(Instruction::LogicalShiftRight(AddressingMode::Accumulator)),
    /* 0x4B */
    None,
    /* 0x4C */
    Some(Instruction::Jump(AddressingMode::Absolute)),
    /* 0x4D */
    Some(Instruction::ExclusiveOr(AddressingMode::Absolute)),
    /* 0x4E */
    Some(Instruction::LogicalShiftRight(AddressingMode::Absolute)),
    /* 0x4F */
    None,
    /* 0x50 */
    Some(Instruction::BranchOnOverflowClear(AddressingMode::Relative)),
    /* 0x51 */
    Some(Instruction::ExclusiveOr(AddressingMode::IndirectYIndexed)),
    /* 0x52 */
    None,
    /* 0x53 */
    None,
    /* 0x54 */
    None,
    /* 0x55 */
    Some(Instruction::ExclusiveOr(AddressingMode::ZeropageXIndexed)),
    /* 0x56 */
    Some(Instruction::LogicalShiftRight(AddressingMode::ZeropageXIndexed)),
    /* 0x57 */
    None,
    /* 0x58 */
    Some(Instruction::ClearInterruptDisable(AddressingMode::Implied)),
    /* 0x59 */
    Some(Instruction::ExclusiveOr(AddressingMode::AbsoluteYIndexed)),
    /* 0x5A */
    None,
    /* 0x5B */
    None,
    /* 0x5C */
    None,
    /* 0x5D */
    Some(Instruction::ExclusiveOr(AddressingMode::AbsoluteXIndexed)),
    /* 0x5E */
    Some(Instruction::LogicalShiftRight(AddressingMode::AbsoluteXIndexed)),
    /* 0x5F */
    None,
    /* 0x60 */
    Some(Instruction::ReturnFromSubroutine(AddressingMode::Implied)),
    /* 0x61 */
    Some(Instruction::AddWithCarry(AddressingMode::XIndexedIndirect)),
    /* 0x62 */
    None,
    /* 0x63 */
    None,
    /* 0x64 */
    None,
    /* 0x65 */
    Some(Instruction::AddWithCarry(AddressingMode::Zeropage)),
    /* 0x66 */
    Some(Instruction::RotateRight(AddressingMode::Zeropage)),
    /* 0x67 */
    None,
    /* 0x68 */
    Some(Instruction::PullAccumulator(AddressingMode::Implied)),
    /* 0x69 */
    Some(Instruction::AddWithCarry(AddressingMode::Immediate)),
    /* 0x6A */
    Some(Instruction::RotateRight(AddressingMode::Accumulator)),
    /* 0x6B */
    None,
    /* 0x6C */
    Some(Instruction::Jump(AddressingMode::Indirect)),
    /* 0x6D */
    Some(Instruction::AddWithCarry(AddressingMode::Absolute)),
    /* 0x6E */
    Some(Instruction::RotateRight(AddressingMode::Absolute)),
    /* 0x6F */
    None,
    /* 0x70 */
    Some(Instruction::BranchOnOverflowSet(AddressingMode::Relative)),
    /* 0x71 */
    Some(Instruction::AddWithCarry(AddressingMode::IndirectYIndexed)),
    /* 0x72 */
    None,
    /* 0x73 */
    None,
    /* 0x74 */
    None,
    /* 0x75 */
    Some(Instruction::AddWithCarry(AddressingMode::ZeropageXIndexed)),
    /* 0x76 */
    Some(Instruction::RotateRight(AddressingMode::ZeropageXIndexed)),
    /* 0x77 */
    None,
    /* 0x78 */
    Some(Instruction::SetInterruptDisable(AddressingMode::Implied)),
    /* 0x79 */
    Some(Instruction::AddWithCarry(AddressingMode::AbsoluteYIndexed)),
    /* 0x7A */
    None,
    /* 0x7B */
    None,
    /* 0x7C */
    None,
    /* 0x7D */
    Some(Instruction::AddWithCarry(AddressingMode::AbsoluteXIndexed)),
    /* 0x7E */
    Some(Instruction::RotateRight(AddressingMode::AbsoluteXIndexed)),
    /* 0x7F */
    None,
    /* 0x80 */
    None,
    /* 0x81 */
    Some(Instruction::StoreAccumulator(AddressingMode::XIndexedIndirect)),
    /* 0x82 */
    None,
    /* 0x83 */
    None,
    /* 0x84 */
    Some(Instruction::StoreY(AddressingMode::Zeropage)),
    /* 0x85 */
    Some(Instruction::StoreAccumulator(AddressingMode::Zeropage)),
    /* 0x86 */
    Some(Instruction::StoreX(AddressingMode::Zeropage)),
    /* 0x87 */
    None,
    /* 0x88 */
    Some(Instruction::DecrementY(AddressingMode::Implied)),
    /* 0x89 */
    None,
    /* 0x8A */
    Some(Instruction::TransferXToAccumulator(AddressingMode::Implied)),
    /* 0x8B */
    None,
    /* 0x8C */
    Some(Instruction::StoreY(AddressingMode::Absolute)),
    /* 0x8D */
    Some(Instruction::StoreAccumulator(AddressingMode::Absolute)),
    /* 0x8E */
    Some(Instruction::StoreX(AddressingMode::Absolute)),
    /* 0x8F */
    None,
    /* 0x90 */
    Some(Instruction::BranchOnCarryClear(AddressingMode::Relative)),
    /* 0x91 */
    Some(Instruction::StoreAccumulator(AddressingMode::IndirectYIndexed)),
    /* 0x92 */
    None,
    /* 0x93 */
    None,
    /* 0x94 */
    Some(Instruction::StoreY(AddressingMode::ZeropageXIndexed)),
    /* 0x95 */
    Some(Instruction::StoreAccumulator(AddressingMode::ZeropageXIndexed)),
    /* 0x96 */
    Some(Instruction::StoreX(AddressingMode::ZeropageYIndexed)),
    /* 0x97 */
    None,
    /* 0x98 */
    Some(Instruction::TransferYToAccumulator(AddressingMode::Implied)),
    /* 0x99 */
    Some(Instruction::StoreAccumulator(AddressingMode::AbsoluteYIndexed)),
    /* 0x9A */
    Some(Instruction::TransferXToStackpointer(AddressingMode::Implied)),
    /* 0x9B */
    None,
    /* 0x9C */
    None,
    /* 0x9D */
    Some(Instruction::StoreAccumulator(AddressingMode::AbsoluteXIndexed)),
    /* 0x9E */
    None,
    /* 0x9F */
    None,
    /* 0xA0 */
    Some(Instruction::LoadY(AddressingMode::Immediate)),
    /* 0xA1 */
    Some(Instruction::LoadAccumulator(AddressingMode::XIndexedIndirect)),
    /* 0xA2 */
    Some(Instruction::LoadX(AddressingMode::Immediate)),
    /* 0xA3 */
    None,
    /* 0xA4 */
    Some(Instruction::LoadY(AddressingMode::Zeropage)),
    /* 0xA5 */
    Some(Instruction::LoadAccumulator(AddressingMode::Zeropage)),
    /* 0xA6 */
    Some(Instruction::LoadX(AddressingMode::Zeropage)),
    /* 0xA7 */
    None,
    /* 0xA8 */
    Some(Instruction::TransferAccumulatorToY(AddressingMode::Implied)),
    /* 0xA9 */
    Some(Instruction::LoadAccumulator(AddressingMode::Immediate)),
    /* 0xAA */
    Some(Instruction::TransferAccumulatorToX(AddressingMode::Implied)),
    /* 0xAB */
    None,
    /* 0xAC */
    Some(Instruction::LoadY(AddressingMode::Absolute)),
    /* 0xAD */
    Some(Instruction::LoadAccumulator(AddressingMode::Absolute)),
    /* 0xAE */
    Some(Instruction::LoadX(AddressingMode::Absolute)),
    /* 0xAF */
    None,
    /* 0xB0 */
    Some(Instruction::BranchOnCarrySet(AddressingMode::Relative)),
    /* 0xB1 */
    Some(Instruction::LoadAccumulator(AddressingMode::IndirectYIndexed)),
    /* 0xB2 */
    None,
    /* 0xB3 */
    None,
    /* 0xB4 */
    Some(Instruction::LoadY(AddressingMode::ZeropageXIndexed)),
    /* 0xB5 */
    Some(Instruction::LoadAccumulator(AddressingMode::ZeropageXIndexed)),
    /* 0xB6 */
    Some(Instruction::LoadX(AddressingMode::ZeropageYIndexed)),
    /* 0xB7 */
    None,
    /* 0xB8 */
    Some(Instruction::ClearOverflow(AddressingMode::Implied)),
    /* 0xB9 */
    Some(Instruction::LoadAccumulator(AddressingMode::AbsoluteYIndexed)),
    /* 0xBA */
    Some(Instruction::TransferStackpointerToX(AddressingMode::Implied)),
    /* 0xBB */
    None,
    /* 0xBC */
    Some(Instruction::LoadY(AddressingMode::AbsoluteXIndexed)),
    /* 0xBD */
    Some(Instruction::LoadAccumulator(AddressingMode::AbsoluteXIndexed)),
    /* 0xBE */
    Some(Instruction::LoadX(AddressingMode::AbsoluteYIndexed)),
    /* 0xBF */
    None,
    /* 0xC0 */
    Some(Instruction::CompareWithY(AddressingMode::Immediate)),
    /* 0xC1 */
    Some(Instruction::Compare(AddressingMode::XIndexedIndirect)),
    /* 0xC2 */
    None,
    /* 0xC3 */
    None,
    /* 0xC4 */
    Some(Instruction::CompareWithY(AddressingMode::Zeropage)),
    /* 0xC5 */
    Some(Instruction::Compare(AddressingMode::Zeropage)),
    /* 0xC6 */
    Some(Instruction::Decrement(AddressingMode::Zeropage)),
    /* 0xC7 */
    None,
    /* 0xC8 */
    Some(Instruction::IncrementY(AddressingMode::Implied)),
    /* 0xC9 */
    Some(Instruction::Compare(AddressingMode::Immediate)),
    /* 0xCA */
    Some(Instruction::DecrementX(AddressingMode::Implied)),
    /* 0xCB */
    None,
    /* 0xCC */
    Some(Instruction::CompareWithY(AddressingMode::Absolute)),
    /* 0xCD */
    Some(Instruction::Compare(AddressingMode::Absolute)),
    /* 0xCE */
    Some(Instruction::Decrement(AddressingMode::Absolute)),
    /* 0xCF */
    None,
    /* 0xD0 */
    Some(Instruction::BranchOnNotEqual(AddressingMode::Relative)),
    /* 0xD1 */
    Some(Instruction::Compare(AddressingMode::IndirectYIndexed)),
    /* 0xD2 */
    None,
    /* 0xD3 */
    None,
    /* 0xD4 */
    None,
    /* 0xD5 */
    Some(Instruction::Compare(AddressingMode::ZeropageXIndexed)),
    /* 0xD6 */
    Some(Instruction::Decrement(AddressingMode::ZeropageXIndexed)),
    /* 0xD7 */
    None,
    /* 0xD8 */
    Some(Instruction::ClearDecimal(AddressingMode::Implied)),
    /* 0xD9 */
    Some(Instruction::Compare(AddressingMode::AbsoluteYIndexed)),
    /* 0xDA */
    None,
    /* 0xDB */
    None,
    /* 0xDC */
    None,
    /* 0xDD */
    Some(Instruction::Compare(AddressingMode::AbsoluteXIndexed)),
    /* 0xDE */
    Some(Instruction::Decrement(AddressingMode::AbsoluteXIndexed)),
    /* 0xDF */
    None,
    /* 0xE0 */
    Some(Instruction::CompareWithX(AddressingMode::Immediate)),
    /* 0xE1 */
    Some(Instruction::SubtractWithCarry(AddressingMode::XIndexedIndirect)),
    /* 0xE2 */
    None,
    /* 0xE3 */
    None,
    /* 0xE4 */
    Some(Instruction::CompareWithX(AddressingMode::Zeropage)),
    /* 0xE5 */
    Some(Instruction::SubtractWithCarry(AddressingMode::Zeropage)),
    /* 0xE6 */
    Some(Instruction::Increment(AddressingMode::Zeropage)),
    /* 0xE7 */
    None,
    /* 0xE8 */
    Some(Instruction::IncrementX(AddressingMode::Implied)),
    /* 0xE9 */
    Some(Instruction::SubtractWithCarry(AddressingMode::Immediate)),
    /* 0xEA */
    Some(Instruction::NoOperation(AddressingMode::Implied)),
    /* 0xEB */
    None,
    /* 0xEC */
    Some(Instruction::CompareWithX(AddressingMode::Absolute)),
    /* 0xED */
    Some(Instruction::SubtractWithCarry(AddressingMode::Absolute)),
    /* 0xEE */
    Some(Instruction::Increment(AddressingMode::Absolute)),
    /* 0xEF */
    None,
    /* 0xF0 */
    Some(Instruction::BranchOnEqual(AddressingMode::Relative)),
    /* 0xF1 */
    Some(Instruction::SubtractWithCarry(AddressingMode::IndirectYIndexed)),
    /* 0xF2 */
    None,
    /* 0xF3 */
    None,
    /* 0xF4 */
    None,
    /* 0xF5 */
    Some(Instruction::SubtractWithCarry(AddressingMode::ZeropageXIndexed)),
    /* 0xF6 */
    Some(Instruction::Increment(AddressingMode::ZeropageXIndexed)),
    /* 0xF7 */
    None,
    /* 0xF8 */
    Some(Instruction::SetDecimal(AddressingMode::Implied)),
    /* 0xF9 */
    Some(Instruction::SubtractWithCarry(AddressingMode::AbsoluteYIndexed)),
    /* 0xFA */
    None,
    /* 0xFB */
    None,
    /* 0xFC */
    None,
    /* 0xFD */
    Some(Instruction::SubtractWithCarry(AddressingMode::AbsoluteXIndexed)),
    /* 0xFE */
    Some(Instruction::Increment(AddressingMode::AbsoluteXIndexed)),
    /* 0xFF */
    None,
];
