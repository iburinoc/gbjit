use super::util::*;

fn generate_operation(ops: &mut Assembler, cmd: AluCommand) {
    macro_rules! emit_op {
        ($ops:expr, $o:tt) => {
            dynasm!($ops
                ; $o al, ah
            );
        };
    }
    use AluCommand::*;
    match cmd {
        Add => emit_op!(ops, add),
        Adc => emit_op!(ops, adc),
        Sub => emit_op!(ops, sub),
        Sbc => emit_op!(ops, sbb),
        And => emit_op!(ops, and),
        Xor => emit_op!(ops, xor),
        Or => emit_op!(ops, or),
        Cp => emit_op!(ops, cmp),
    }
}

pub(super) fn generate(
    ops: &mut Assembler,
    inst: &Instruction,
    bus: &ExternalBus,
) -> EpilogueDescription {
    let (cmd, op) = parse_cmd!(inst, AluHalf { cmd, op } => (cmd, op));

    match op {
        AluOperand::Loc(loc) => load_location(ops, bus, loc),
        AluOperand::Imm(v) => {
            dynasm!(ops
                ; mov ah, BYTE v as _
            );
        }
    };

    match cmd {
        AluCommand::Adc | AluCommand::Sbc => load_carry_flag(ops),
        _ => {}
    }

    generate_operation(ops, cmd);
    dynasm!(ops
        ; lahf
    );
    match cmd {
        AluCommand::Sub | AluCommand::Sbc | AluCommand::Cp => {
            dynasm!(ops
                ; or ah, BYTE 0x20 as _
            );
        }
        _ => {}
    };
    dynasm!(ops
        ; mov [rsp + 0x02], ah
    );

    Default::default()
}
