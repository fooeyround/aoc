use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    //division A / (2^Combo) trunc -> A
    Adv,
    //B XOR Literal -> B
    Bxl,
    //Combo % 8 -> B
    Bst,
    //if A != 0: Jump to Literal; DO NOT + by 2 after
    Jnz,
    // B XOR C -> B
    Bxc,
    //Combo % 8 -> OUTPUT[]
    Out,
    //division A / (2^Combo) trunc -> B
    Bdv,
    //division A / (2^Combo) trunc -> C
    Cdv,
}

impl Operation {
    fn from_raw(raw: usize) -> Self {
        match raw {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid Operation"),
        }
    }

    fn combo(&self, operand: usize, registers: &RegisterState) -> usize {
        match operand {
            0..=3 => operand,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => panic!("This should never be in a program"),
        }
    }

    fn perform(
        &self,
        operand: usize,
        registers: &mut RegisterState,
        pc: &mut usize,
        output: &mut Vec<usize>,
    ) -> bool {
        match self {
            Self::Adv => {
                registers.a = (registers.a as f64 / (2 ^ self.combo(operand, &registers)) as f64)
                    .trunc() as usize
            }
            Self::Bxl => registers.b = registers.b ^ operand,
            Self::Bst => registers.b = self.combo(operand, &registers) % 8,
            Self::Jnz => {
                if registers.a != 0 {
                    *pc = operand / 2;
                    return false;
                }
            }
            Self::Bxc => registers.b = registers.b ^ registers.c,
            Self::Out => output.push(self.combo(operand, &registers) % 8),
            Self::Bdv => {
                registers.b = (registers.a as f64 / (2 ^ self.combo(operand, &registers)) as f64)
                    .trunc() as usize
            }
            Self::Cdv => {
                registers.c = (registers.a as f64 / (2 ^ self.combo(operand, &registers)) as f64)
                    .trunc() as usize
            }
        }
        return true;
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Operation,
    operand: usize,
}

#[derive(Debug)]
struct RegisterState {
    a: usize,
    b: usize,
    c: usize,
}

fn parse_input(raw_input: &str) -> (RegisterState, Vec<Instruction>) {
    let (regs, prgm) = raw_input.split_once("\n\n").unwrap();
    let regs_fmt = regs
        .split("\n")
        .map(|f| &f["(Register A: ".len() - 1..])
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let prgm_fmt = prgm["Program: ".len() - 1..]
        .split(",")
        .filter(|f| !f.trim().is_empty())
        .map(|f| f.trim().parse::<usize>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|f| f.collect::<Vec<usize>>())
        .map(|f| Instruction {
            opcode: Operation::from_raw(f[0]),
            operand: f[1],
        })
        .collect::<Vec<Instruction>>();

    (
        RegisterState {
            a: regs_fmt[0],
            b: regs_fmt[1],
            c: regs_fmt[2],
        },
        prgm_fmt,
    )
}

pub fn solve1(raw_input: &str) -> String {
    let (mut registers, instructions) = parse_input(raw_input);
    let mut pc = 0;
    let mut output = vec![];
    while pc < instructions.len() {
        if instructions[pc].opcode.perform(
            instructions[pc].operand,
            &mut registers,
            &mut pc,
            &mut output,
        ) {
            pc += 1;
        }
    }

    let mut output_fmt = output
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>();
    output_fmt.pop();

    return output_fmt.to_string();
}
pub fn solve2(raw_input: &str) -> String {
    let input = parse_input(raw_input);

    return 0.to_string();
}
