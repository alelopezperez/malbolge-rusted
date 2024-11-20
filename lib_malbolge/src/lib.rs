mod ten_trit;
use std::io::Read;

use ten_trit::TenTrit;

const XLAT1: &str = "+b(29e*j1VMEKLyC})8&m#~W>qxdRp0wkrUo[D7,XTcA\"lI.v%{gJh4G\\-=O@5`_3i<?Z';FNQuY]szf$!BS/|t:Pn6^Ha";
const XLAT2: &str = "5z]&gqtyfr$(we4{WP)H-Zn,[%\\3dL+Q;>U!pJS72FhOA1CB6v^=I_0/8|jsb9m<.TVac`uY*MK'X~xDl}REokN:#?G\"i@";

pub struct VM {
    pub register_a: TenTrit,
    pub register_c: TenTrit,
    pub register_d: TenTrit,
    pub memory: [TenTrit; 59_049],
}

impl VM {
    const INSTRUCTIONS: &str = "i</*jpov";
    fn post_execution_increase(&mut self) {
        let index = self.memory[usize::from(self.register_c)].get() - 33;
        let index = index as usize;
        self.memory[self.register_c.get() as usize] =
            TenTrit::new(XLAT2.chars().nth(index).unwrap() as u16);

        self.register_c = self.register_c + TenTrit::new(1);

        self.register_d = self.register_d + TenTrit::new(1);
    }

    pub fn load(program: String) -> Result<Self, ()> {
        let chars = program
            .chars()
            .filter(|a| !a.is_whitespace())
            .collect::<Vec<_>>();

        let mut memory = Vec::new();
        for (i, letter) in chars.iter().enumerate() {
            let op_code = XLAT1
                .chars()
                .nth(((*letter as usize) - 33 + i) % 94)
                .unwrap();
            match VM::INSTRUCTIONS.find(op_code) {
                Some(_translation) => memory.push(TenTrit::new(*letter as u16)),
                None => return Err(()),
            }
        }

        if memory.len() > 59_049 {
            return Err(());
        }

        while memory.len() < 59_049 {
            let filler = VM::crazy_op(
                memory[memory.len() - 1].get(),
                memory[memory.len() - 2].get(),
            );
            memory.push(TenTrit::new(filler));
        }

        Ok(Self {
            register_a: TenTrit::new(0),
            register_c: TenTrit::new(0),
            register_d: TenTrit::new(0),
            memory: memory.try_into().unwrap(),
        })
    }

    fn crazy_op(x: u16, y: u16) -> u16 {
        let mut crazy = 0;
        const P9: [u16; 5] = [1, 9, 81, 729, 6561];
        const O_TABLE: [[u16; 9]; 9] = [
            [4, 3, 3, 1, 0, 0, 1, 0, 0],
            [4, 3, 5, 1, 0, 2, 1, 0, 2],
            [5, 5, 4, 2, 2, 1, 2, 2, 1],
            [4, 3, 3, 1, 0, 0, 7, 6, 6],
            [4, 3, 5, 1, 0, 2, 7, 6, 8],
            [5, 5, 4, 2, 2, 1, 8, 8, 7],
            [7, 6, 6, 7, 6, 6, 4, 3, 3],
            [7, 6, 8, 7, 6, 8, 4, 3, 5],
            [8, 8, 7, 8, 8, 7, 5, 5, 4],
        ];

        (0..5).for_each(|i| {
            let s = y / P9[i] % 9;

            let d = x / P9[i] % 9;

            let oa = O_TABLE[s as usize][d as usize];

            let oz = oa * P9[i];

            crazy += oz;
        });
        crazy
    }

    pub fn exec(&mut self) {
        loop {
            let instruction =
                (self.memory[usize::from(self.register_c)].get() - 33 + self.register_c.get()) % 94;
            let op_code = XLAT1.chars().nth(instruction as usize).unwrap();

            match op_code {
                'i' => {
                    // C = [D] or jmp [d]
                    self.register_c = self.memory[usize::from(self.register_d)];

                    self.post_execution_increase();
                }
                '<' => {
                    // PRINT(A%256)
                    // TODO: impl to_ascii_char for TRIT
                    print!(
                        "{}",
                        char::from_u32((self.register_a.get() % 256) as u32).unwrap()
                    );
                    self.post_execution_increase();
                }
                '/' => {
                    let mut buf: [u8; 1] = Default::default();
                    let mut stdin = std::io::stdin();
                    let handle = stdin.read(&mut buf).unwrap();

                    let input = buf[0] as u16;

                    if handle == 0 {
                        //EOF
                        println!("EOF");
                        self.register_a = TenTrit::MAX;
                    } else {
                        self.register_a = TenTrit::new(input);
                    }
                    self.post_execution_increase();
                }
                '*' => {
                    // A = [D] = ROTATE_RIGHT([D])
                    let rotate = self.memory[self.register_d.get() as usize].get();
                    let rotate = TenTrit::new(rotate / 3 + rotate % 3 * 19683);

                    self.memory[self.register_d.get() as usize] = rotate;
                    self.register_a = rotate;

                    self.post_execution_increase();
                }
                'j' => {
                    // D = [D]
                    self.register_d = self.memory[self.register_d.get() as usize];
                    self.post_execution_increase();
                }
                'p' => {
                    // A = [D] = CRAZY_OP(A, [D])
                    let crazy = VM::crazy_op(
                        self.register_a.get(),
                        self.memory[self.register_d.get() as usize].get(),
                    );
                    self.register_a = TenTrit::new(crazy);
                    self.memory[self.register_d.get() as usize] = TenTrit::new(crazy);

                    self.post_execution_increase();
                }
                'v' => {
                    return;
                }
                _ => {
                    self.post_execution_increase();
                }
            };
        }
    }
}
