use lib_malbolge::{ExecState, VM};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct MalbolgeVM {
    vm: VM,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum JsExecState {
    Input,
    Output,
    Finished,
    InstructionExectuted,
}

#[wasm_bindgen]
pub struct MyTuple(pub JsExecState, pub i64);

#[wasm_bindgen]
impl MalbolgeVM {
    #[wasm_bindgen(constructor)]
    pub fn new(program: String) -> Self {
        Self {
            vm: VM::load(program).unwrap(),
        }
    }

    #[wasm_bindgen]
    pub fn exec(&mut self) -> MyTuple {
        match self.vm.exec() {
            Ok(state) => match state {
                ExecState::InstructionExecuted => MyTuple(JsExecState::InstructionExectuted, -1),
                ExecState::Input => MyTuple(JsExecState::Input, -1),
                ExecState::Output(out) => MyTuple(JsExecState::Output, out as i64),
                ExecState::Finished => MyTuple(JsExecState::Finished, -1),
            },
            Err(_) => MyTuple(JsExecState::Finished, -1),
        }
    }

    #[wasm_bindgen]
    pub fn input_js(&mut self, input: u8, size: usize) {
        self.vm.input_instruction(input, size);
    }
}
