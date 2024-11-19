mod ten_trit;
use std::usize;

use ten_trit::TenTrit;

const XLAT1: &str = "+b(29e*j1VMEKLyC})8&m#~W>qxdRp0wkrUo[D7,XTcA\"lI.v%{gJh4G\\-=O@5`_3i<?Z';FNQuY]szf$!BS/|t:Pn6^Ha";
const XLAT2: &str = "5z]&gqtyfr$(we4{WP)H-Zn,[%\\3dL+Q;>U!pJS72FhOA1CB6v^=I_0/8|jsb9m<.TVac`uY*MK'X~xDl}REokN:#?G\"i@";
fn transform_char(char: &char) -> char {
    let char = XLAT1.chars().nth((*char as usize - 33) % 94).unwrap();
    char
}
pub struct VM {
    pub register_a: TenTrit,
    pub register_c: TenTrit,
    pub register_d: TenTrit,
    pub memory: [TenTrit; 59_049],
}

impl VM {
    const INSTRUCTIONS: &str = "i</*jpov";
    fn new() -> Self {
        Self {
            register_a: TenTrit::new(0),
            register_d: TenTrit::new(0),
            register_c: TenTrit::new(0),
            memory: [TenTrit::new(0); 59_049],
        }
    }
    fn post_execution_increase(&mut self) {
        let index = self.memory[usize::from(self.register_c)] - TenTrit::new(33);
        let index = index.get() as usize;
        XLAT2.chars().nth(index);
        self.register_c = self.register_c + TenTrit::new(1);
    }

    fn load(program: String) -> Self {
        let chars = program
            .chars()
            .filter(|a| !a.is_whitespace())
            .collect::<Vec<_>>();

        let mut memory = Vec::new();
        for (i, letter) in chars.iter().enumerate() {
            let op_code = XLAT1.chars().nth((*letter as usize - 33 + i) % 94).unwrap();
            match VM::INSTRUCTIONS.find(op_code) {
                Some(_) => memory.push(TenTrit::new(*letter as u16)),
                None => todo!(),
            }
        }

        if memory.len() > 59_049 {
            todo!()
        }

        while memory.len() < 59_049 {
            memory.push(memory[memory.len() - 1]);
            memory.push(memory[memory.len() - 2]);
        }

        Self {
            register_a: TenTrit::new(0),
            register_c: TenTrit::new(0),
            register_d: TenTrit::new(0),
            memory: memory.try_into().unwrap(),
        }
    }

    fn exec(&mut self) {
        let instruction = &(self.memory[usize::from(self.register_c)].get() as u8 as char);
        let op_code = transform_char(instruction);
        match op_code {
            'i' => {
                // C = [D] or jmp [d]
                self.register_c = self.memory[usize::from(self.register_d)];

                self.post_execution_increase();
            }
            '<' => {
                self.post_execution_increase();
            }
            '/' => {
                self.post_execution_increase();
            }
            '*' => {
                self.post_execution_increase();
            }
            'j' => {
                self.post_execution_increase();
            }
            'p' => {
                self.post_execution_increase();
            }
            'v' => {
                self.post_execution_increase();
            }
            'o' | _ => {
                self.post_execution_increase();
            }
        };
    }
}
