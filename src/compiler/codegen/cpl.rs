use super::util::*;

pub fn generate(
    ops: &mut Assembler,
    _inst: &Instruction,
    _labels: &[DynamicLabel],
    _pc: u16,
    _base_addr: u16,
    _bus: &ExternalBus,
) -> GenerateEpilogue {
    dynasm!(ops
        ; not al
        ; or [rsp + 0x02], 0x30
    );

    true
}
