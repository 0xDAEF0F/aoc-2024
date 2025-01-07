/*
    Making a quine on this machine isn't as complicated as it looks:
    - op out only every reads 0-3 or the last 3 bits of reg A, B, or C
    - reg B and C are only ever set by:
        - xoring with literal 0-7 (ie on low 3 bits)
        - anding with last 3 bits of 0-3 or a reg (ie set to 0-7)
        - rshift of reg A
    - that means the whole program is basically just shifting off reg A,
      mutating the last 3 bits, and outputting it 3 bits at a time.
    - the xor and jump means we can't easily reverse it but above means that
      if you can get 3 bits in A that gives a valid out value, it will
      always output the same 3 bit value if lshifted by 3
    - not all series of values will eventually produce a correct answer so
      we search the full space, another DFS babee
*/

struct Computer {
    register_a: u32,
    register_b: u32,
    register_c: u32,

    instruction_pointer: usize,
    program: Vec<u32>,

    stdout: Vec<u32>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            register_a: 0,
            register_b: 0,
            register_c: 0,

            instruction_pointer: 0,
            program: vec![],

            stdout: vec![],
        }
    }

    fn run(&mut self) {
        self.instruction_pointer = 0;
        self.stdout.clear();

        while self.instruction_pointer + 1 < self.program.len()
            && (&self.stdout[..] == &self.program[0..self.stdout.len()])
        {
            match self.program[self.instruction_pointer] {
                0 => self.adv(), //
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(), //
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(), //
                7 => self.cdv(), //
                _ => break,
            }
        }
    }

    fn _stdout(&self) -> String {
        self.stdout
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    // opcode: "0"
    fn adv(&mut self) {
        let numerator = self.register_a;
        let denominator = 2_u32.pow(self.find_combo_operand());

        self.register_a = numerator / denominator;

        self.instruction_pointer += 2;
    }

    // opcode: "1"
    fn bxl(&mut self) {
        let operand = self.program[self.instruction_pointer + 1];
        self.register_b ^= operand;

        self.instruction_pointer += 2;
    }

    // opcode: "2"
    fn bst(&mut self) {
        self.register_b = self.find_combo_operand() % 8;

        self.instruction_pointer += 2;
    }

    // opcode: "3"
    fn jnz(&mut self) {
        match self.register_a {
            0 => {
                self.instruction_pointer += 2;
            }
            _ => {
                let literal_operand = self.program[self.instruction_pointer + 1] as usize;
                self.instruction_pointer = literal_operand;
            }
        }
    }

    // opcode: "4"
    fn bxc(&mut self) {
        self.register_b ^= self.register_c;

        self.instruction_pointer += 2;
    }

    // opcode: "5"
    fn out(&mut self) {
        let value = self.find_combo_operand() % 8;

        self.stdout.push(value);

        self.instruction_pointer += 2;
    }

    // opcode: "6"
    fn bdv(&mut self) {
        let numerator = self.register_a;
        let denominator = 2_u32.pow(self.find_combo_operand());

        self.register_b = numerator / denominator;

        self.instruction_pointer += 2;
    }

    // opcode: "7"
    fn cdv(&mut self) {
        let numerator = self.register_a;
        let denominator = 2_u32.pow(self.find_combo_operand());

        self.register_c = numerator / denominator;

        self.instruction_pointer += 2;
    }

    fn find_combo_operand(&self) -> u32 {
        match self.program[self.instruction_pointer + 1] {
            n @ 0..=3 => n,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unreachable!("reserved"),
            _ => unreachable!("invalid"),
        }
    }
}

fn main() {
    let mut computer = Computer::new();

    let mut initial = 1;

    computer.program = vec![2, 4, 1, 1, 7, 5, 4, 4, 1, 4, 0, 3, 5, 5, 3, 0];

    loop {
        computer.register_a = initial;
        computer.run();

        if computer.stdout == computer.program {
            break;
        }

        initial += 1;

        if initial % 1_000_000 == 0 {
            println!("stdout: {:?}", computer.stdout);
            println!("program: {:?}", computer.program);
            println!("Trying: {}", initial);
        }
    }
}
