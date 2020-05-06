#![allow(
    clippy::match_bool, // Code reads cleaner this way sometimes
    clippy::fn_to_numeric_cast, // Necessary for dynasm
    clippy::transmute_ptr_to_ptr // Makes code cleaner when the destination type is clear
)]
#![feature(proc_macro_hygiene)]

extern crate dynasm;

use std::fs;

use structopt::StructOpt;

mod compiler;
mod cpu_state;
mod gb;

use gb::bus::Bus;

#[derive(StructOpt)]
#[structopt(name = "gbjit")]
#[structopt(about = r#"
A WIP just-in-time compiler for the GameBoy and GameBoy Colour.

Currently just disassembles a given binary.
"#)]
struct Args {
    /// GB bios file
    bios: String,

    /// GB rom to run
    rom: String,

    /// Whether to print disassembled pages before executing
    #[structopt(short, long)]
    disassemble: bool,

    /// Whether to print just the commands or the full instructions
    #[structopt(short, long)]
    full_disassembly: bool,

    /// Whether to print the disassembly of the code block
    #[structopt(short, long)]
    x64_disasm: bool,

    /// Whether to generate log traces for each instruction executed
    #[structopt(short, long)]
    trace_pc: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::from_args();

    let mut gb_bus = Bus::new(&args.bios, &args.rom)?;
    let data = fs::read(args.bios)?;

    let bus = compiler::ExternalBus {
        read: Bus::read,
        write: Bus::write,
    };

    let cycle_state = compiler::CycleState::new();
    cycle_state.set_hard_limit(102);
    cycle_state.set_interrupt_limit(50);

    let options = compiler::CompileOptions {
        trace_pc: args.trace_pc,
    };
    let oneoffs = compiler::OneoffTable::generate(&bus, &options).unwrap();

    let block = compiler::compile(0, data.as_slice(), bus, &oneoffs, &options)?;

    if args.disassemble {
        print_disassembly(&block, args.full_disassembly);
    }

    if args.x64_disasm {
        println!("Disassembly:");
        for i in block.disassemble()? {
            println!("{}", i);
        }
        println!();
    }

    let mut cpu_state = cpu_state::CpuState::new();

    block.enter(&mut cpu_state, &mut gb_bus, &cycle_state);

    println!("{:?}", cpu_state);
    println!("{:?}", cycle_state);

    Ok(())
}

fn print_disassembly<T>(block: &compiler::CodeBlock<T>, full: bool) {
    let insts = block.instructions();
    let mut idx = 0;
    while idx < insts.len() {
        let i = &insts[idx];
        if full {
            println!("{:#05x}: {:?}", idx, i);
        } else {
            println!("{:#05x?}: ", idx);
            match i {
                Ok(i) => println!("{:?}", i.cmd),
                Err(bytes) => println!("{:02x?}", bytes),
            }
        }
        idx += match i {
            Ok(i) => i.size() as usize,
            Err(bytes) => bytes.len(),
        };
    }
}
