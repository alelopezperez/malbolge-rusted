mod ten_trit;
use std::usize;

use ten_trit::TenTrit;

const XLAT1: &str= "+b(29e*j1VMEKLyC})8&m#~W>qxdRp0wkrUo[D7,XTcA\"lI.v%{gJh4G\\-=O@5`_3i<?Z';FNQuY]szf$!BS/|t:Pn6^Ha";
fn transform_char(char: &char) -> Result<char, ()> {
    let char = XLAT1.chars().nth(*char as usize - 33).unwrap();
    Ok(char)
}
pub struct VM {
    pub register_a: TenTrit,
    pub register_c: TenTrit,
    pub register_d: TenTrit,
    pub memory: [TenTrit; 59_049],
}

impl VM {
    fn new() -> Self {
        Self {
            register_a: TenTrit::new(0),
            register_d: TenTrit::new(0),
            register_c: TenTrit::new(0),
            memory: [TenTrit::new(0); 59_049],
        }
    }

    fn exec(&mut self) {
        match self.memory[usize::from(self.register_c)].get() as u8 as char {
            'i' => {
                self.register_c = self.memory[usize::from(self.register_d)];
                self.register_d = self.register_d + TenTrit::new(1);
            }
            '<' => {}
            '/' => {}
            '*' => {}
            'j' => {}
            'p' => {}
            'v' => {}
            'o' | _ => {}
        };
    }
}

