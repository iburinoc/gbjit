use std::mem;

use capstone::Capstone;
use capstone::Error as CsError;
use dynasmrt::{AssemblyOffset, ExecutableBuffer};

use super::Instruction;

pub struct CodeBlock {
    base_addr: u16,
    buf: ExecutableBuffer,
    entry: extern "sysv64" fn(),
    offsets: Vec<AssemblyOffset>,
    instructions: Vec<Instruction>,
}

impl CodeBlock {
    pub(super) fn new(
        base_addr: u16,
        buf: ExecutableBuffer,
        entry: AssemblyOffset,
        offsets: Vec<AssemblyOffset>,
        instructions: Vec<Instruction>,
    ) -> Self {
        let entry = unsafe { mem::transmute(buf.ptr(entry)) };
        CodeBlock {
            base_addr,
            buf,
            entry,
            offsets,
            instructions,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        self.instructions.as_slice()
    }

    // TODO: Make cpu state and memory a parameter
    pub fn enter(&self) {
        (self.entry)()
    }

    pub fn disassemble(&self) -> Result<Vec<String>, CsError> {
        use capstone::arch::x86;
        use capstone::arch::{BuildsCapstone, BuildsCapstoneSyntax};

        let cs = Capstone::new()
            .x86()
            .mode(x86::ArchMode::Mode64)
            .syntax(x86::ArchSyntax::Intel)
            .detail(false)
            .build()?;

        let base_addr = self.buf.ptr(AssemblyOffset(0)) as u64;

        let instructions = cs.disasm_all(&*self.buf, base_addr)?;

        Ok(instructions.iter().map(|x| x.to_string()).collect())
    }
}
