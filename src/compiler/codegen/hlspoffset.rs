use super::util::*;

pub fn generate(
    ops: &mut Assembler,
    inst: &Instruction,
    _labels: &[DynamicLabel],
    _pc: u16,
    _base_addr: u16,
    _bus: &ExternalBus,
) -> GenerateEpilogue {
    let offset = parse_cmd!(inst, HlSpOffset(offset) => offset);

    dynasm!(ops
        ; mov dx, r12w
        ; add dl, BYTE offset as _
        ; lahf
        ; adc dh, 0
        ; and ah, BYTE 0x11
        ; mov [rsp + 0x02], ah
    );

    true
}
