use std::collections::HashMap;
use std::io;
use std::vec::Vec;

#[derive(Debug)]
struct IntcodeOperation {
  /// Opcode of current operation
  ///
  /// Add: 1;
  /// Multiply: 2;
  /// Get input: 3;
  /// Print value: 4;
  /// Jump-if-true: 5;
  /// Jump-if-false: 6;
  /// Less than: 7;
  /// Equals: 8;
  /// Exit: 99
  opcode: u8,
  /// Length of current operation
  ///
  /// Add: 4;
  /// Multiply: 4;
  /// Get input: 2;
  /// Print value: 2;
  /// Jump-if-true: 3;
  /// Jump-if-false: 3;
  /// Less than: 4;
  /// Equals: 4;
  /// Exit: 1
  len: usize,
  /// Modes of parameters for current operation
  ///
  /// Position mode: 0
  /// Immediate mode: 1
  modes: Vec<u8>,
}

impl IntcodeOperation {
  /// Creates a new IntcodeOperation object from the given operation value
  fn new(op: u32) -> Result<IntcodeOperation, &'static str> {
    // extract opcode from operation value
    let op_str = op.to_string();
    let code: u8;
    if op_str.len() == 1 {
      code = op_str[0..].parse::<u8>().unwrap();
    } else {
      code = op_str[(op_str.len() - 2)..].parse::<u8>().unwrap();
    }

    // check if opcode is valid
    let valid_opcodes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 99];
    if !valid_opcodes.contains(&code) {
      eprintln!("Invalid opcode: {}", code);
      return Err("Opcode is not valid.");
    }

    // create map of operation lengths
    let valid_lens: Vec<usize> = vec![4, 4, 2, 2, 3, 3, 4, 4, 1];
    let opcode_lens: HashMap<_, _> = valid_opcodes.iter().zip(valid_lens.iter()).collect();

    // extract parameter modes from operation value
    let mut op_modes: String;
    if op_str.len() == 1 {
      op_modes = "000".to_owned();
    } else {
      op_modes = op_str[..(op_str.len() - 2)].to_owned();
    }

    // add parameter modes to vector in reverse order
    let mut modes: Vec<u8> = Vec::<u8>::new();
    while op_modes.len() > 0 {
      modes.push(op_modes.remove(op_modes.len() - 1).to_digit(10).unwrap() as u8);
    }

    // make sure there is a mode for all three parameters
    while modes.len() < 3 {
      modes.push(0);
    }

    Ok(IntcodeOperation {
      opcode: code,
      len: **opcode_lens.get(&code).unwrap(),
      modes,
    })
  }

  /// Adds two parameters together and stores sum in program memory
  fn op_add(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of add operation.");
    }
    let op_l = prg.memory[addr_l as usize];

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.memory[ip + 2] as isize,
      // immediate mode
      1 => ip as isize + 2,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of add operation.");
    }
    let op_r = prg.memory[addr_r as usize];

    let store_addr = prg.memory[ip + 3] as usize;
    prg.memory[store_addr] = op_l + op_r;

    Ok(ip + self.len)
  }

  /// Multiplies two parameters together and store product in program memory
  fn op_mult(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of multiply operation.");
    }
    let op_l = prg.memory[addr_l as usize];

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.memory[ip + 2] as isize,
      // immediate mode
      1 => ip as isize + 2,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of multiply operation.");
    }
    let op_r = prg.memory[addr_r as usize];

    let store_addr = prg.memory[ip + 3] as usize;
    prg.memory[store_addr] = op_l * op_r;
    Ok(ip + self.len)
  }

  /// Receives integer input from user and stores in program memory
  fn op_input(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    let value: i32;
    match prg.input_mode {
      ProgramInputMode::Provided => {
        value = prg.input[prg.input_pointer];
        prg.input_pointer += 1;
      }
      ProgramInputMode::User => {
        let mut input = String::new();
        println!("Enter an integer:");
        io::stdin()
          .read_line(&mut input)
          .expect("Failed to read input.");
        value = input[..(input.len() - 2)].parse::<i32>().unwrap();
      }
    };

    let store_addr = prg.memory[ip + 1] as usize;
    prg.memory[store_addr] = value;
    Ok(ip + self.len)
  }

  /// Retrieves value from program memory and outputs to console
  fn op_output(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    let addr = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr == -1 {
      return Err("Unrecognized mode for output operation address.");
    }
    let value = prg.memory[addr as usize];
    match prg.input_mode {
      ProgramInputMode::Provided => prg.output.push(value),
      ProgramInputMode::User => println!("Program emitted value: {}", value),
    };
    Ok(ip + self.len)
  }

  /// Jumps to address given by second parameter if first parameter is non-zero
  fn op_jump_true(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    // get value
    let addr_c = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_c == -1 {
      return Err("Unrecognized mode for jump operation value.");
    }
    let op_c = prg.memory[addr_c as usize];

    // get jump address
    let addr_j = match self.modes[1] {
      // position mode
      0 => prg.memory[ip + 2] as isize,
      // immediate mode
      1 => ip as isize + 2,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_j == -1 {
      return Err("Unrecognized mode for jump operation address.");
    }
    let op_j = prg.memory[addr_j as usize];

    if op_c != 0 {
      return Ok(op_j as usize);
    }

    Ok(ip + self.len)
  }

  /// Jumps to address given by second parameter if first parameter is zero
  fn op_jump_false(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    // get value
    let addr_c = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_c == -1 {
      return Err("Unrecognized mode for jump operation value.");
    }
    let op_c = prg.memory[addr_c as usize];

    // get jump address
    let addr_j = match self.modes[1] {
      // position mode
      0 => prg.memory[ip + 2] as isize,
      // immediate mode
      1 => ip as isize + 2,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_j == -1 {
      return Err("Unrecognized mode for jump operation address.");
    }
    let op_j = prg.memory[addr_j as usize];

    if op_c == 0 {
      return Ok(op_j as usize);
    }
    Ok(ip + self.len)
  }

  /// Stores 1 in program memory if first parameter is less than second parameter; otherwise 0
  fn op_less_than(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of less than operation.");
    }
    let op_l = prg.memory[addr_l as usize];

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.memory[ip + 2] as isize,
      // immediate mode
      1 => ip as isize + 2,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of less than operation.");
    }
    let op_r = prg.memory[addr_r as usize];

    let store_addr = prg.memory[ip + 3] as usize;
    if op_l < op_r {
      prg.memory[store_addr] = 1;
    } else {
      prg.memory[store_addr] = 0;
    }
    Ok(ip + self.len)
  }

  /// Stores 1 in program memory if first two parameters are equal; otherwise 0
  fn op_equals(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.memory[ip + 1] as isize,
      // immediate mode
      1 => ip as isize + 1,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of equals operation.");
    }
    let op_l = prg.memory[addr_l as usize];

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.memory[ip + 2] as isize,
      // immediate mode
      1 => ip as isize + 2,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of equals operation.");
    }
    let op_r = prg.memory[addr_r as usize];

    let store_addr = prg.memory[ip + 3] as usize;
    if op_l == op_r {
      prg.memory[store_addr] = 1;
    } else {
      prg.memory[store_addr] = 0;
    }
    Ok(ip + self.len)
  }

  /// Performs the current Intcode operation using the Intcode program memory
  fn perform(&self, prg: &mut IntcodeProgram, ip: usize) -> Result<usize, &'static str> {
    if self.opcode == 1 {
      return self.op_add(prg, ip);
    } else if self.opcode == 2 {
      return self.op_mult(prg, ip);
    } else if self.opcode == 3 {
      return self.op_input(prg, ip);
    } else if self.opcode == 4 {
      return self.op_output(prg, ip);
    } else if self.opcode == 5 {
      return self.op_jump_true(prg, ip);
    } else if self.opcode == 6 {
      return self.op_jump_false(prg, ip);
    } else if self.opcode == 7 {
      return self.op_less_than(prg, ip);
    } else if self.opcode == 8 {
      return self.op_equals(prg, ip);
    }

    Err("Invalid opcode.")
  }
}

#[derive(Debug)]
enum ProgramInputMode {
  User,
  Provided,
}

#[derive(Debug)]
pub struct IntcodeProgram {
  memory: Vec<i32>,
  instruction_pointer: usize,
  input_mode: ProgramInputMode,
  input: Vec<i32>,
  input_pointer: usize,
  pub output: Vec<i32>,
  pub active: bool,
}

impl IntcodeProgram {
  /// Creates a new IntcodeProgram object using the given program data
  pub fn new(data: &String, prg_input: Option<Vec<i32>>) -> Result<IntcodeProgram, &'static str> {
    if data.len() == 0 {
      return Err("No valid input provided.");
    }

    // set input mode
    let (input, input_mode) = match prg_input {
      Some(p) => (p, ProgramInputMode::Provided),
      None => (Vec::<i32>::new(), ProgramInputMode::User),
    };
    let input_pointer: usize = 0;
    let output: Vec<i32> = Vec::new();

    // spilt program data into vector of values
    let values: Vec<_> = data.split(',').collect();
    let mut memory: Vec<i32> = Vec::<i32>::new();

    // parse value strings as 32-bit signed ints
    // and push to program memory vector
    for value in values {
      let parsed = value.parse::<i32>().unwrap();
      memory.push(parsed);
    }

    Ok(IntcodeProgram {
      memory,
      instruction_pointer: 0,
      input_mode,
      input,
      input_pointer,
      output,
      active: true,
    })
  }

  /// Executes the IntcodeProgram to completion
  pub fn run(&mut self) -> Result<(), &'static str> {
    loop {
      let cur_op = IntcodeOperation::new(self.memory[self.instruction_pointer] as u32).unwrap();

      // quit loop on exit opcode
      if cur_op.opcode == 99 {
        self.active = false;
        break;
      }

      // perform current operation
      let result = cur_op.perform(self, self.instruction_pointer);
      if let Err(e) = result {
        eprintln!("Operation failed: {}", e);
        return Err("Operation failed during program execution.");
      } else if let Ok(new_pos) = result {
        // update instruction pointer
        self.instruction_pointer = new_pos;
      };
    }

    Ok(())
  }

  /// Executes the IntcodeProgram until a read operation is encountered
  pub fn run_until_input(&mut self) -> Result<(), &'static str> {
    loop {
      let cur_op = IntcodeOperation::new(self.memory[self.instruction_pointer] as u32).unwrap();

      // quit loop on exit and read opcodes
      if cur_op.opcode == 99 || cur_op.opcode == 3 {
        if cur_op.opcode == 99 {
          self.active = false;
        }
        break;
      }

      // perform current operation
      let result = cur_op.perform(self, self.instruction_pointer);
      if let Err(e) = result {
        eprintln!("Operation failed: {}", e);
        return Err("Operation failed during program execution.");
      } else if let Ok(new_pos) = result {
        // update instruction pointer
        self.instruction_pointer = new_pos;
      };
    }

    Ok(())
  }

  /// Manually performs read operation while program is waiting for input
  pub fn inject_input(&mut self, value: i32) -> Result<(), &'static str> {
    let read_op = IntcodeOperation::new(self.memory[self.instruction_pointer] as u32).unwrap();
    if read_op.opcode != 3 {
      return Err("Can only inject input when program is performing a read instruction!");
    }

    self.input.push(value);
    self.input_pointer = self.input.len() - 1;

    let result = read_op.perform(self, self.instruction_pointer);
    if let Err(e) = result {
      eprintln!("Read operation failed: {}", e);
      return Err(e);
    } else if let Ok(new_pos) = result {
      self.instruction_pointer = new_pos;
    };

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn mult_op_with_modes() {
    // execute program "1002,4,3,4,33"
    let mut prg = IntcodeProgram::new(&"1002,4,3,4,33".to_owned()).unwrap();
    assert_eq!(prg.memory, vec![1002, 4, 3, 4, 33]);
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.memory, vec![1002, 4, 3, 4, 99]);
  }

  #[test]
  fn add_op_with_negatives() {
    // execute program "1101,100,-1,4,0"
    let mut prg = IntcodeProgram::new(&"1101,100,-1,4,0".to_owned()).unwrap();
    assert_eq!(prg.memory, vec![1101, 100, -1, 4, 0]);
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.memory, vec![1101, 100, -1, 4, 99]);
  }
}
