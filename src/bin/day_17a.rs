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
        while self.instruction_pointer + 1 < self.program.len() {
            match self.program[self.instruction_pointer] {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => break,
            }
        }
    }

    fn stdout(&self) -> String {
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

    computer.register_a = 46337277;
    computer.program = vec![2, 4, 1, 1, 7, 5, 4, 4, 1, 4, 0, 3, 5, 5, 3, 0];

    computer.run();

    println!("stdout: {}", computer.stdout());
}
