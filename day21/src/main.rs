use std::{collections::HashSet, fmt};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
enum Opcode {
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
}

fn one_if_true(x: bool) -> u64 {
    if x {
        1
    } else {
        0
    }
}

impl Opcode {
    fn execute(&self, registers: &[u64; 6], input_a: u64, input_b: u64, output_c: u64) -> [u64; 6] {
        let mut result = registers.clone();
        result[output_c as usize] = match self {
            Opcode::addr => registers[input_a as usize] + registers[input_b as usize],
            Opcode::addi => registers[input_a as usize] + input_b,
            Opcode::mulr => registers[input_a as usize] * registers[input_b as usize],
            Opcode::muli => registers[input_a as usize] * input_b,
            Opcode::banr => registers[input_a as usize] & registers[input_b as usize],
            Opcode::bani => registers[input_a as usize] & input_b,
            Opcode::borr => registers[input_a as usize] | registers[input_b as usize],
            Opcode::bori => registers[input_a as usize] | input_b,
            Opcode::setr => registers[input_a as usize],
            Opcode::seti => input_a,
            Opcode::gtir => one_if_true(input_a > registers[input_b as usize]),
            Opcode::gtri => one_if_true(registers[input_a as usize] > input_b),
            Opcode::gtrr => one_if_true(registers[input_a as usize] > registers[input_b as usize]),
            Opcode::eqir => one_if_true(input_a == registers[input_b as usize]),
            Opcode::eqri => one_if_true(registers[input_a as usize] == input_b),
            Opcode::eqrr => one_if_true(registers[input_a as usize] == registers[input_b as usize]),
        };
        result
    }
}

struct Instruction {
    op: Opcode,
    input_a: u64,
    input_b: u64,
    output_c: u64,
}

impl Instruction {
    fn new(op: Opcode, input_a: u64, input_b: u64, output_c: u64) -> Instruction {
        Instruction { op, input_a, input_b, output_c }
    }

    fn execute(&self, registers: &[u64; 6]) -> [u64; 6] {
        self.op
            .execute(registers, self.input_a, self.input_b, self.output_c)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {} {}",
            self.op, self.input_a, self.input_b, self.output_c
        )?;
        Ok(())
    }
}

fn main() {
    let mut program: Vec<Instruction> = Vec::new();

    let ip_reg: usize = 3;
    program.push(Instruction::new(Opcode::seti, 123, 0, 4));
    program.push(Instruction::new(Opcode::bani, 4, 456, 4));
    program.push(Instruction::new(Opcode::eqri, 4, 72, 4));
    program.push(Instruction::new(Opcode::addr, 4, 3, 3));
    program.push(Instruction::new(Opcode::seti, 0, 0, 3));
    program.push(Instruction::new(Opcode::seti, 0, 9, 4));
    program.push(Instruction::new(Opcode::bori, 4, 65536, 2));
    program.push(Instruction::new(Opcode::seti, 6152285, 4, 4));
    program.push(Instruction::new(Opcode::bani, 2, 255, 1));
    program.push(Instruction::new(Opcode::addr, 4, 1, 4));
    program.push(Instruction::new(Opcode::bani, 4, 16777215, 4));
    program.push(Instruction::new(Opcode::muli, 4, 65899, 4));
    program.push(Instruction::new(Opcode::bani, 4, 16777215, 4));
    program.push(Instruction::new(Opcode::gtir, 256, 2, 1));
    program.push(Instruction::new(Opcode::addr, 1, 3, 3));
    program.push(Instruction::new(Opcode::addi, 3, 1, 3));
    program.push(Instruction::new(Opcode::seti, 27, 4, 3));
    program.push(Instruction::new(Opcode::seti, 0, 3, 1));
    program.push(Instruction::new(Opcode::addi, 1, 1, 5));
    program.push(Instruction::new(Opcode::muli, 5, 256, 5));
    program.push(Instruction::new(Opcode::gtrr, 5, 2, 5));
    program.push(Instruction::new(Opcode::addr, 5, 3, 3));
    program.push(Instruction::new(Opcode::addi, 3, 1, 3));
    program.push(Instruction::new(Opcode::seti, 25, 9, 3));
    program.push(Instruction::new(Opcode::addi, 1, 1, 1));
    program.push(Instruction::new(Opcode::seti, 17, 4, 3));
    program.push(Instruction::new(Opcode::setr, 1, 9, 2));
    program.push(Instruction::new(Opcode::seti, 7, 4, 3));
    program.push(Instruction::new(Opcode::eqrr, 4, 0, 1));
    program.push(Instruction::new(Opcode::addr, 1, 3, 3));
    program.push(Instruction::new(Opcode::seti, 5, 6, 3));

    // First star:
    // By visual inspection, the only instruction that checks r0 is instruction 28.
    // Instruction 28 checks if r0 is equal to r4 then halts.
    // Check what r4 is the first time instruction 28 is executed.
    //
    // Second star:
    // There must be a loop in the values that r4 takes in instruction 28.
    // Record whether we have seen this value before, and if we haven't, add it to our list.
    // Otherwise, the value we added last was the one that makes the longest without looping.

    let mut ip: u64 = 0;
    let mut registers = [0, 0, 0, 0, 0, 0];

    let mut r4_vals: HashSet<u64> = HashSet::new();
    let mut r4_history: Vec<u64> = Vec::new();

    r4_vals = HashSet::new();
    r4_history = Vec::new();

    while (ip as usize) < program.len() {
        let next_inst: &Instruction = &program[ip as usize];
        registers[ip_reg] = ip;
        //print!("ip={} {:?} {} ", ip, registers, next_inst);
        registers = next_inst.execute(&registers);
        //println!("{:?}", registers);
        ip = registers[ip_reg] + 1;
        if ip == 28 {
            if r4_vals.contains(&registers[4]) {
                println!("R4 repeats with: {}", registers[4]);
                println!("R4 values: {:?}", r4_history);
                break;
            }
            r4_vals.insert(registers[4]);
            r4_history.push(registers[4]);
            if r4_vals.len() % 1000 == 0 {
                println!("Checked r4 {} times", r4_vals.len());
            }
        } else if ip == 17 {
            //print!("subroutine: {} -> ", registers[2]);
            // Skip the subroutine, which divides r2 by 256 integerwise
            registers[1] = registers[2] / 256;
            registers[2] = registers[1];
            registers[ip_reg] = 7;
            ip = 8;
        }
    }
    println!("Regs at halt: {:?}", registers);
}
