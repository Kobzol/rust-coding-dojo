#![allow(dead_code)]

pub type AResult<T> = anyhow::Result<T>;

#[derive(Default, Copy, Clone, Debug)]
enum MemoryCell {
    Initialized(u8),
    #[default]
    Uninitialized,
}

impl From<u8> for MemoryCell {
    fn from(value: u8) -> Self {
        Self::Initialized(value)
    }
}

#[derive(Debug)]
struct CPU {
    memory: [MemoryCell; 256],
    register: [MemoryCell; 16],
}

impl CPU {
    pub(crate) fn print_state(&self) {
        dbg!(&self);
    }
}

impl CPU {
    pub fn execute_one(&mut self, i: &Instruction) -> AResult<()> {
        match i {
            Instruction::Move(dst, src) => {
                if let MemoryCell::Uninitialized = self.register[src.0] {
                    anyhow::bail!("reading uninitialized memory");
                }
                self.register[dst.0] = self.register[src.0];
            }
            Instruction::Set(dst, ref value) => {
                self.register[dst.0] = (*value).into();
            }
        }
        Ok(())
    }
    pub fn run(&mut self, instructions: &[Instruction]) -> AResult<()> {
        for i in instructions {
            self.execute_one(i)?;
        }
        Ok(())
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            memory: [MemoryCell::Uninitialized; 256],
            register: [MemoryCell::Uninitialized; 16],
        }
    }
}

struct RegisterIndex(usize);

enum Instruction {
    Set(RegisterIndex, u8),
    Move(RegisterIndex, RegisterIndex),
}

fn main() -> AResult<()> {
    let instructions = vec![
        Instruction::Set(RegisterIndex(1), 42),
        Instruction::Move(RegisterIndex(0), RegisterIndex(1)),
    ];
    let mut cpu = CPU::default();
    cpu.run(instructions.as_slice())?;
    cpu.print_state();
    Ok(())
}
