use crate::instruction::*;

pub struct Vm {
  registers: [i32; 32],
  pc: usize,
  program: Vec<u8>,
  reminder: u32,
  equal_flag: bool,
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      registers: [0; 32],
      pc: 0,
      program: vec![],
      reminder: 0,
      equal_flag: false,
    }
  }

  pub fn run(&mut self){
    loop {
      if !self.execute_instruction() {
        break;
      }
    }
  }

  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  fn execute_instruction(&mut self) -> bool{
      if self.pc >= self.program.len() {
        return false;
      }
      match self.decode_opcode() {
        Opcode::EQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 == register2 {
                self.equal_flag = true;
            } else {
                self.equal_flag = false;
            }
            self.next_8_bits();
        },
        Opcode::NEQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 == register2 {
                self.equal_flag = false;
            } else {
                self.equal_flag = true;
            }
            self.next_8_bits();
        },
        Opcode::GT => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 > register2 {
                self.equal_flag = true;
            } else {
                self.equal_flag = false;
            }
            self.next_8_bits();
        },
        Opcode::LT => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 < register2 {
                self.equal_flag = true;
            } else {
                self.equal_flag = false;
            }
            self.next_8_bits();
        },
        Opcode::GTQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 >= register2 {
                self.equal_flag = true;
            } else {
                self.equal_flag = false;
            }
            self.next_8_bits();
        },
        Opcode::LTQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 <= register2 {
                self.equal_flag = true;
            } else {
                self.equal_flag = false;
            }
            self.next_8_bits();
        },
        Opcode::JEQ => {
            let register = self.next_8_bits() as usize;
            let target = self.registers[register];
            if self.equal_flag {
                self.pc = target as usize;
            }
        },
        Opcode::JMPF => {
            let value = self.registers[self.next_8_bits() as usize];
            self.pc += value as usize;
        },
        Opcode::JMPB => {
            let value = self.registers[self.next_8_bits() as usize];
            self.pc -= value as usize;
        },
        Opcode::JMP => {
          let target = self.registers[self.next_8_bits() as usize];
          self.pc = target as usize;
        },
        Opcode::ADD => {
          let register1 = self.registers[self.next_8_bits() as usize];
          let register2 = self.registers[self.next_8_bits() as usize];
          self.registers[self.next_8_bits() as usize] = register1 + register2;
        },
        Opcode::SUB => {
          let register1 = self.registers[self.next_8_bits() as usize];
          let register2 = self.registers[self.next_8_bits() as usize];
          self.registers[self.next_8_bits() as usize] = register1 - register2;
        },
        Opcode::MUL => {
          let register1 = self.registers[self.next_8_bits() as usize];
          let register2 = self.registers[self.next_8_bits() as usize];
          self.registers[self.next_8_bits() as usize] = register1 * register2;
        },
        Opcode::DIV => {
          let register1 = self.registers[self.next_8_bits() as usize];
          let register2 = self.registers[self.next_8_bits() as usize];
          self.registers[self.next_8_bits() as usize] = register1 / register2;
          self.reminder = (register1 % register2) as u32;
        },
        Opcode::LOAD => {
          let register = self.next_8_bits() as usize;
          let number = self.next_16_bits() as i32;
          println!("{}", &number);
          self.registers[register] = number;
        },
        Opcode::HLT => {
          println!("HLT encountered");
          return false;
        },
        _ => {
          println!("Unknown opcode found, Terminating !!");
          return false;
        },
      }
      true
  }

  fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    result
  }

  fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
    self.pc += 2;
    result
  }

  fn decode_opcode(&mut self) -> Opcode {
    let opcode = Opcode::from(self.program[self.pc]);
    self.pc += 1;
    opcode
  }
}

#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn test_create_vm(){
    let test_vm = Vm::new();
    assert_eq!(test_vm.registers[0], 0);
  }
  #[test]
  fn test_opcode_hlt() {
    let mut test_vm = Vm::new();
    let test_bytes = vec![0,0,0,0];
    test_vm.program = test_bytes;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_igl() {
    let mut test_vm = Vm::new();
    let test_bytes = vec![200,0,0,0];
    test_vm.program = test_bytes;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_load_opcode() {
    let mut test_vm = Vm::new();
    test_vm.program = vec![1, 0, 1, 244];
    test_vm.run_once();
    assert_eq!(test_vm.registers[0], 500);
  }
  #[test]
  fn test_jmp_opcode() {
    let mut test_vm = Vm::new();
    test_vm.registers[0] = 25;
    test_vm.program = vec![6, 0, 0, 0];
    test_vm.run_once();
    assert_eq!(test_vm.pc, 25);
  }

  #[test]
  fn test_jmpf_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 5;
      test_vm.program = vec![7, 0, 0, 0];
      test_vm.run_once();
      assert_eq!(test_vm.pc, 7);
  }

  #[test]
  fn test_jmpb_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 2;
      test_vm.program = vec![8, 0, 0, 0];
      test_vm.run_once();
      assert_eq!(test_vm.pc, 0);
  }

  #[test]
  fn test_eq_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 10;
      test_vm.registers[1] = 10;
      test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
      test_vm.registers[1] = 20;
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, false);
  }

  #[test]
  fn test_neq_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 10;
      test_vm.registers[1] = 10;
      test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, false);
      test_vm.registers[1] = 20;
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_gt_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 10;
      test_vm.registers[1] = 10;
      test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, false);
      test_vm.registers[0] = 20;
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_lt_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 10;
      test_vm.registers[1] = 10;
      test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, false);
      test_vm.registers[1] = 20;
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_gtq_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 10;
      test_vm.registers[1] = 10;
      test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
      test_vm.registers[1] = 20;
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, false);
  }

  #[test]
  fn test_ltq_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 10;
      test_vm.registers[1] = 10;
      test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0];
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
      test_vm.registers[1] = 20;
      test_vm.run_once();
      assert_eq!(test_vm.equal_flag, true);
  }

  #[test]
  fn test_jeq_opcode() {
      let mut test_vm = Vm::new();
      test_vm.registers[0] = 7;
      test_vm.equal_flag = true;
      test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
      test_vm.run_once();
      assert_eq!(test_vm.pc, 7);
  }
}