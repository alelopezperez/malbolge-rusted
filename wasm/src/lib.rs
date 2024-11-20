use lib_malbolge::VM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct MalbolgeVM {
    vm: VM,
}

#[wasm_bindgen]
impl MalbolgeVM {
    #[wasm_bindgen(constructor)]
    pub fn load_program(program: String) -> Self {
        Self {
            vm: VM::load(program).unwrap(),
        }
    }

    #[wasm_bindgen]
    pub fn exec(&mut self) {
        self.vm.exec();
    }
}
