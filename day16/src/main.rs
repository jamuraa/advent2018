use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, prelude::*, BufReader},
    slice::Iter,
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
enum Opcode {
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
}

fn one_if_true(x: bool) -> u16 {
    if x {
        1
    } else {
        0
    }
}

impl Opcode {
    fn all_iter() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode; 16] = [
            Opcode::addr, Opcode::addi, Opcode::mulr, Opcode::muli, Opcode::banr, Opcode::bani, Opcode::borr, Opcode::bori,
            Opcode::setr, Opcode::seti, Opcode::gtir, Opcode::gtri, Opcode::gtrr, Opcode::eqir, Opcode::eqri, Opcode::eqrr,
        ];
        OPCODES.into_iter()
    }

    fn execute(&self, registers: &[u16; 4], input_a: u8, input_b: u8, output_c: u8) -> [u16; 4] {
        let mut result = registers.clone();
        result[output_c as usize] = match self {
            Opcode::addr => registers[input_a as usize] + registers[input_b as usize],
            Opcode::addi => registers[input_a as usize] + input_b as u16,
            Opcode::mulr => registers[input_a as usize] * registers[input_b as usize],
            Opcode::muli => registers[input_a as usize] * input_b as u16,
            Opcode::banr => registers[input_a as usize] & registers[input_b as usize],
            Opcode::bani => registers[input_a as usize] & input_b as u16,
            Opcode::borr => registers[input_a as usize] | registers[input_b as usize],
            Opcode::bori => registers[input_a as usize] | input_b as u16,
            Opcode::setr => registers[input_a as usize],
            Opcode::seti => input_a as u16,
            Opcode::gtir => one_if_true(input_a as u16 > registers[input_b as usize]),
            Opcode::gtri => one_if_true(registers[input_a as usize] > input_b as u16),
            Opcode::gtrr => one_if_true(registers[input_a as usize] > registers[input_b as usize]),
            Opcode::eqir => one_if_true(input_a as u16 == registers[input_b as usize]),
            Opcode::eqri => one_if_true(registers[input_a as usize] == input_b as u16),
            Opcode::eqrr => one_if_true(registers[input_a as usize] == registers[input_b as usize]),
        };
        result
    }
}

struct Sample([u16; 4], [u8; 4], [u16; 4]);

impl Sample {
    fn new(before: &[u16], instruction: &[u8], after: &[u16]) -> Sample {
        assert!(before.len() == 4, "Before should be 4 numbers not {:?}", before);
        assert!(instruction.len() == 4, "Instruction should be 4 numbers not {:?}", instruction);
        assert!(after.len() == 4, "After should be 4 numbers not {:?}", after);
        let mut before_ar = [0; 4];
        before_ar.copy_from_slice(&before[..4]);
        let mut inst_ar = [0; 4];
        inst_ar.copy_from_slice(&instruction[..4]);
        let mut after_ar = [0; 4];
        after_ar.copy_from_slice(&after[..4]);
        Sample(before_ar, inst_ar, after_ar)
    }

    /// Dowse the opcodes this sample could have.
    fn dowse_instructions(&self) -> HashSet<Opcode> {
        let coded = &self.1;
        Opcode::all_iter()
            .filter(|opcode| opcode.execute(&self.0, coded[1], coded[2], coded[3]) == self.2)
            .cloned()
            .collect()
    }

    /// The opcode value that is given in the instruction
    fn opcode_val(&self) -> u8 {
        self.1[0]
    }
}

fn main() -> io::Result<()> {
    let mut samples: Vec<Sample> = Vec::new();

    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        let before = numbers_in_string(&line);
        let line2 = lines.next().unwrap()?;
        let instruction = numbers_in_string(&line2);
        let line3 = lines.next().unwrap()?;
        let after = numbers_in_string(&line3);
        samples.push(Sample::new(&before, &instruction, &after));
        // There's a blank line that we throw away
        lines.next();
    }

    println!("{} samples loaded", samples.len());

    let mut more_than_three = 0;

    let mut dowsed: HashMap<u8, HashSet<Opcode>> = HashMap::new();

    for sample in samples {
        let possible = sample.dowse_instructions();
        if possible.len() >= 3 {
            more_than_three += 1;
        }
        let determined = {
            let possible_before = dowsed
                .entry(sample.opcode_val())
                .or_insert(Opcode::all_iter().cloned().collect());
            possible_before.retain(|op| possible.contains(op));
            if possible_before.len() == 1 {
                Some((
                    sample.opcode_val(),
                    possible_before.iter().next().unwrap().clone(),
                ))
            } else {
                None
            }
        };

        if let Some((found_val, opcode)) = determined {
            for (val, opcodes_possible) in dowsed.iter_mut() {
                if val != &found_val {
                    opcodes_possible.remove(&opcode);
                }
            }
        }
    }

    println!(
        "{} samples could represent >= three opcodes",
        more_than_three
    );

    println!("We determined these opcodes: {:?}", dowsed);

    // Execute the program.
    let mut registers = [0; 4];
    let f = File::open("program.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        let instr: Vec<u8> = numbers_in_string(&line);
        let opcode = dowsed[&instr[0]].iter().next().unwrap();
        println!("{:?} Executing {:?} ({:?})", registers, opcode, instr);
        registers = opcode.execute(&registers, instr[1], instr[2], instr[3]);
    }
    println!("{:?} at the end", registers);

    Ok(())
}

fn numbers_in_string<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.split(|x| !char::is_numeric(x) && x != '-')
        .filter(|x| !x.is_empty())
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}
