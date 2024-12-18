use itertools::Itertools;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
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
    ) {
        let mut jumped = false;
        match self {
            Self::Adv => registers.a >>= self.combo(operand, &registers),
            Self::Bxl => registers.b ^= operand,
            Self::Bst => registers.b = self.combo(operand, &registers) % 8,
            Self::Jnz => {
                if registers.a != 0 {
                    *pc = operand;
                    jumped = true;
                }
            }
            Self::Bxc => registers.b ^= registers.c,
            Self::Out => output.push(self.combo(operand, &registers) % 8),
            Self::Bdv => registers.b = registers.a >> self.combo(operand, &registers),
            Self::Cdv => registers.c = registers.a >> self.combo(operand, &registers),
        }
        if !jumped {
            *pc += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Operation,
    operand: usize,
}

#[derive(Debug, Clone, Copy)]
struct RegisterState {
    a: usize,
    b: usize,
    c: usize,
}

fn parse_input(raw_input: &str) -> (RegisterState, Vec<Instruction>) {
    let (regs, prgm) = raw_input.split_once("\n\n").unwrap();
    let regs_fmt = regs
        .split("\n")
        .map(|f| f["(Register A: ".len() - 1..].trim())
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
        instructions[pc].opcode.perform(
            instructions[pc].operand,
            &mut registers,
            &mut pc,
            &mut output,
        );
    }

    let mut output_fmt = output
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>();
    output_fmt.pop();

    return output_fmt.to_string();
}

//By reverse enginnering, we know a ends with zero.
//b is some relation to a, but more importantly, a >> 3 each time. (So we will <<)
//We compute the output for each of the 8 possibilities and only go for a correct one.
pub fn solve2(raw_input: &str) -> String {
    let (init_registers, instructions) = parse_input(raw_input);

    let instr_layed_out = instructions
        .iter()
        .map(|inst| [inst.opcode as usize, inst.operand])
        .flatten()
        .collect::<Vec<usize>>();

    let mut starting_a = 0;

    for index in (2..instr_layed_out.len()).rev() {
        starting_a <<= 3;
        println!("{:?}", starting_a);
        for step in 0..8 {
            let mut registers = init_registers.clone();
            registers.a = starting_a | step;
            let mut pc = 0;
            let mut output = vec![];
            while pc < instructions.len() && output.is_empty() {
                instructions[pc].opcode.perform(
                    instructions[pc].operand,
                    &mut registers,
                    &mut pc,
                    &mut output,
                );
            }
            if output[0] == instr_layed_out[index] {
                starting_a |= step;
                break;
            } else {
                dbg!(&output, &instr_layed_out);
            }
        }
    }

    return starting_a.to_string();
}
