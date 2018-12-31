use std::fmt;

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
        self.op.execute(registers, self.input_a, self.input_b, self.output_c)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.op, self.input_a, self.input_b, self.output_c)?;
        Ok(())
    }
}

fn main() {
    let mut program: Vec<Instruction> = Vec::new();

    let ip_reg: usize = 4;
    program.push(Instruction::new(Opcode::addi, 4, 16, 4));
    program.push(Instruction::new(Opcode::seti, 1, 2, 5));
    program.push(Instruction::new(Opcode::seti, 1, 1, 1));
    program.push(Instruction::new(Opcode::mulr, 5, 1, 2));
    program.push(Instruction::new(Opcode::eqrr, 2, 3, 2));
    program.push(Instruction::new(Opcode::addr, 2, 4, 4));
    program.push(Instruction::new(Opcode::addi, 4, 1, 4));
    program.push(Instruction::new(Opcode::addr, 5, 0, 0));
    program.push(Instruction::new(Opcode::addi, 1, 1, 1));
    program.push(Instruction::new(Opcode::gtrr, 1, 3, 2));
    program.push(Instruction::new(Opcode::addr, 4, 2, 4));
    program.push(Instruction::new(Opcode::seti, 2, 4, 4));
    program.push(Instruction::new(Opcode::addi, 5, 1, 5));
    program.push(Instruction::new(Opcode::gtrr, 5, 3, 2));
    program.push(Instruction::new(Opcode::addr, 2, 4, 4));
    program.push(Instruction::new(Opcode::seti, 1, 8, 4));
    program.push(Instruction::new(Opcode::mulr, 4, 4, 4));
    program.push(Instruction::new(Opcode::addi, 3, 2, 3));
    program.push(Instruction::new(Opcode::mulr, 3, 3, 3));
    program.push(Instruction::new(Opcode::mulr, 4, 3, 3));
    program.push(Instruction::new(Opcode::muli, 3, 11, 3));
    program.push(Instruction::new(Opcode::addi, 2, 4, 2));
    program.push(Instruction::new(Opcode::mulr, 2, 4, 2));
    program.push(Instruction::new(Opcode::addi, 2, 6, 2));
    program.push(Instruction::new(Opcode::addr, 3, 2, 3));
    program.push(Instruction::new(Opcode::addr, 4, 0, 4));
    program.push(Instruction::new(Opcode::seti, 0, 8, 4));
    program.push(Instruction::new(Opcode::setr, 4, 1, 2));
    program.push(Instruction::new(Opcode::mulr, 2, 4, 2));
    program.push(Instruction::new(Opcode::addr, 4, 2, 2));
    program.push(Instruction::new(Opcode::mulr, 4, 2, 2));
    program.push(Instruction::new(Opcode::muli, 2, 14, 2));
    program.push(Instruction::new(Opcode::mulr, 2, 4, 2));
    program.push(Instruction::new(Opcode::addr, 3, 2, 3));
    program.push(Instruction::new(Opcode::seti, 0, 0, 0));
    program.push(Instruction::new(Opcode::seti, 0, 0, 4));

    let mut ip: u64 = 0;
    let mut registers = [0, 0, 0, 0, 0, 0];

    while (ip as usize) < program.len() {
        let next_inst: &Instruction = &program[ip as usize];
        registers[ip_reg] = ip;
        print!("ip={} {:?} {} ", ip, registers, next_inst);
        registers = next_inst.execute(&registers);
        println!("{:?}", registers);
        ip = registers[ip_reg] + 1;
        if ip == 1 {
            break;
        }
    }

    println!("Registers after setup: {:?}", registers);

    let mut r0 = 0;
    let r3 = registers[3];
    for r5 in 1..r3 + 1 {
        print!("r5: {} of {}\r", r5, r3);
        for r1 in 1..r3 + 1 {
            if r1 * r5 == r3 {
                r0 += r5;
            }
            if r1 * r5 > r3 {
                break;
            }
        }
    }

    println!("\n r0 halt: {}", r0);
}
