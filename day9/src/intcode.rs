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
  /// Adjust relative base: 9;
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
  /// Adjust relative base: 2;
  /// Exit: 1
  len: usize,
  /// Modes of parameters for current operation
  ///
  /// Position mode: 0;
  /// Immediate mode: 1;
  /// Relative mode: 2
  modes: Vec<u8>,
}

impl IntcodeOperation {
  /// Creates a new IntcodeOperation object from the given operation value
  fn new(op: u64) -> Result<IntcodeOperation, &'static str> {
    // extract opcode from operation value
    let op_str = op.to_string();
    let code: u8;
    if op_str.len() == 1 {
      code = op_str[0..].parse::<u8>().unwrap();
    } else {
      code = op_str[(op_str.len() - 2)..].parse::<u8>().unwrap();
    }

    // check if opcode is valid
    let valid_opcodes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 99];
    if !valid_opcodes.contains(&code) {
      eprintln!("Invalid opcode: {}", code);
      return Err("Opcode is not valid.");
    }

    // create map of operation lengths
    let valid_lens: Vec<usize> = vec![4, 4, 2, 2, 3, 3, 4, 4, 2, 1];
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
  fn op_add(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of add operation.");
    }
    let op_l = prg.get_value(addr_l as usize);

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 2) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 2,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 2) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of add operation.");
    }
    let op_r = prg.get_value(addr_r as usize);

    let store_addr = match self.modes[2] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 3) as isize,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 3) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if store_addr == -1 {
      return Err("Unrecognized mode for parameter of input operation.");
    }
    prg.set_value(store_addr as usize, op_l + op_r);

    Ok(prg.instruction_pointer + self.len)
  }

  /// Multiplies two parameters together and store product in program memory
  fn op_mult(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of multiply operation.");
    }
    let op_l = prg.get_value(addr_l as usize);

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 2) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 2,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 2) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of multiply operation.");
    }
    let op_r = prg.get_value(addr_r as usize);

    let store_addr = match self.modes[2] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 3) as isize,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 3) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if store_addr == -1 {
      return Err("Unrecognized mode for parameter of input operation.");
    }
    prg.set_value(store_addr as usize, op_l * op_r);
    Ok(prg.instruction_pointer + self.len)
  }

  /// Receives integer input from user and stores in program memory
  fn op_input(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    let value: i64;
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
        value = input[..(input.len() - 2)].parse::<i64>().unwrap();
      }
    };

    let store_addr = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,\
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if store_addr == -1 {
      return Err("Unrecognized mode for parameter of input operation.");
    }
    prg.set_value(store_addr as usize, value);
    Ok(prg.instruction_pointer + self.len)
  }

  /// Retrieves value from program memory and outputs to console
  fn op_output(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    let addr = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr == -1 {
      return Err("Unrecognized mode for output operation address.");
    }
    let value = prg.get_value(addr as usize);
    match prg.input_mode {
      ProgramInputMode::Provided => prg.output.push(value),
      ProgramInputMode::User => println!("Program emitted value: {}", value),
    };
    Ok(prg.instruction_pointer + self.len)
  }

  /// Jumps to address given by second parameter if first parameter is non-zero
  fn op_jump_true(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    // get value
    let addr_c = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_c == -1 {
      return Err("Unrecognized mode for jump operation value.");
    }
    let op_c = prg.get_value(addr_c as usize);

    // get jump address
    let addr_j = match self.modes[1] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 2) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 2,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 2) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_j == -1 {
      return Err("Unrecognized mode for jump operation address.");
    }
    let op_j = prg.get_value(addr_j as usize);

    if op_c != 0 {
      return Ok(op_j as usize);
    }

    Ok(prg.instruction_pointer + self.len)
  }

  /// Jumps to address given by second parameter if first parameter is zero
  fn op_jump_false(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    // get value
    let addr_c = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_c == -1 {
      return Err("Unrecognized mode for jump operation value.");
    }
    let op_c = prg.get_value(addr_c as usize);

    // get jump address
    let addr_j = match self.modes[1] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 2) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 2,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 2) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_j == -1 {
      return Err("Unrecognized mode for jump operation address.");
    }
    let op_j = prg.get_value(addr_j as usize);

    if op_c == 0 {
      return Ok(op_j as usize);
    }
    Ok(prg.instruction_pointer + self.len)
  }

  /// Stores 1 in program memory if first parameter is less than second parameter; otherwise 0
  fn op_less_than(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of less than operation.");
    }
    let op_l = prg.get_value(addr_l as usize);

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 2) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 2,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 2) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of less than operation.");
    }
    let op_r = prg.get_value(addr_r as usize);

    let store_addr = match self.modes[2] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 3) as isize,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 3) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if store_addr == -1 {
      return Err("Unrecognized mode for parameter of input operation.");
    }

    if op_l < op_r {
      prg.set_value(store_addr as usize, 1);
    } else {
      prg.set_value(store_addr as usize, 0);
    }
    Ok(prg.instruction_pointer + self.len)
  }

  /// Stores 1 in program memory if first two parameters are equal; otherwise 0
  fn op_equals(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    // get first parameter
    let addr_l = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_l == -1 {
      return Err("Unrecognized mode for first parameter of equals operation.");
    }
    let op_l = prg.get_value(addr_l as usize);

    // get second parameter
    let addr_r = match self.modes[1] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 2) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 2,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 2) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if addr_r == -1 {
      return Err("Unrecognized mode for second parameter of equals operation.");
    }
    let op_r = prg.get_value(addr_r as usize);

    let store_addr = match self.modes[2] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 3) as isize,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 3) + prg.relative_base as i64) as isize,
      // return -1 for unrecognized mode
      _ => -1,
    };
    if store_addr == -1 {
      return Err("Unrecognized mode for parameter of input operation.");
    }

    if op_l == op_r {
      prg.set_value(store_addr as usize, 1);
    } else {
      prg.set_value(store_addr as usize, 0);
    }
    Ok(prg.instruction_pointer + self.len)
  }

  /// Adjusts the program's relative base address
  fn op_adj_rel_base(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    let addr_adj = match self.modes[0] {
      // position mode
      0 => prg.get_value(prg.instruction_pointer + 1) as isize,
      // immediate mode
      1 => prg.instruction_pointer as isize + 1,
      // relative mode
      2 => (prg.get_value(prg.instruction_pointer + 1) + prg.relative_base as i64) as isize,
      _ => -1,
    };
    if addr_adj == -1 {
      return Err("Unrecognized mode for parameter of relative base adjustment operation.");
    }
    let val_adj = prg.get_value(addr_adj as usize) as isize;
    prg.relative_base = (prg.relative_base as isize + val_adj) as usize;
    Ok(prg.instruction_pointer + self.len)
  }

  /// Performs the current Intcode operation using the Intcode program memory
  fn perform(&self, prg: &mut IntcodeProgram) -> Result<usize, &'static str> {
    match self.opcode {
      1 => return self.op_add(prg),
      2 => return self.op_mult(prg),
      3 => return self.op_input(prg),
      4 => return self.op_output(prg),
      5 => return self.op_jump_true(prg),
      6 => return self.op_jump_false(prg),
      7 => return self.op_less_than(prg),
      8 => return self.op_equals(prg),
      9 => return self.op_adj_rel_base(prg),
      _ => return Err("Invalid opcode."),
    }
  }
}

#[derive(Debug)]
enum ProgramInputMode {
  User,
  Provided,
}

#[derive(Debug)]
pub struct IntcodeProgram {
  memory: HashMap<usize, i64>,
  relative_base: usize,
  instruction_pointer: usize,
  input_mode: ProgramInputMode,
  input: Vec<i64>,
  input_pointer: usize,
  pub output: Vec<i64>,
  pub active: bool,
}

impl IntcodeProgram {
  /// Creates a new IntcodeProgram object using the given program data
  pub fn new(data: &String, prg_input: Option<Vec<i64>>) -> Result<IntcodeProgram, &'static str> {
    if data.len() == 0 {
      return Err("No valid input provided.");
    }

    // set input mode
    let (input, input_mode) = match prg_input {
      Some(p) => (p, ProgramInputMode::Provided),
      None => (Vec::<i64>::new(), ProgramInputMode::User),
    };
    let input_pointer: usize = 0;
    let output: Vec<i64> = Vec::new();

    // spilt program data into vector of values
    let values: Vec<_> = data.split(',').collect();
    let mut memory: HashMap<usize, i64> = HashMap::new();

    // parse value strings as 32-bit signed ints
    // and push to program memory vector
    let mut i: usize = 0;
    for value in values {
      let parsed = value.parse::<i64>().unwrap();
      memory.insert(i, parsed);
      i += 1;
    }

    Ok(IntcodeProgram {
      memory,
      relative_base: 0,
      instruction_pointer: 0,
      input_mode,
      input,
      input_pointer,
      output,
      active: true,
    })
  }

  /// Retrieves value from program memory
  fn get_value(&mut self, address: usize) -> i64 {
    let entry = self.memory.entry(address).or_insert(0);
    *entry
  }

  /// Stores value in program memory
  fn set_value(&mut self, address: usize, value: i64) {
    let entry = self.memory.entry(address).or_insert(0);
    *entry = value;
  }

  /// Executes the IntcodeProgram to completion
  pub fn run(&mut self) -> Result<(), &'static str> {
    loop {
      let cur_op = IntcodeOperation::new(self.get_value(self.instruction_pointer) as u64).unwrap();

      // quit loop on exit opcode
      if cur_op.opcode == 99 {
        self.active = false;
        break;
      }

      // perform current operation
      let result = cur_op.perform(self);
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
      let cur_op = IntcodeOperation::new(self.get_value(self.instruction_pointer) as u64).unwrap();

      // quit loop on exit and read opcodes
      if cur_op.opcode == 99 || cur_op.opcode == 3 {
        if cur_op.opcode == 99 {
          self.active = false;
        }
        break;
      }

      // perform current operation
      let result = cur_op.perform(self);
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
  pub fn inject_input(&mut self, value: i64) -> Result<(), &'static str> {
    let read_op = IntcodeOperation::new(self.get_value(self.instruction_pointer) as u64).unwrap();
    if read_op.opcode != 3 {
      return Err("Can only inject input when program is performing a read instruction!");
    }

    self.input.push(value);
    self.input_pointer = self.input.len() - 1;

    let result = read_op.perform(self);
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
    let mut prg = IntcodeProgram::new(&"1002,4,3,4,33".to_owned(), None).unwrap();

    let expected_mem: Vec<i64> = vec![1002, 4, 3, 4, 33];
    for i in 0..expected_mem.len() {
      assert_eq!(prg.get_value(i), expected_mem[i]);
    }

    // last value should be exit opcode
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.get_value(4), 99);
  }

  #[test]
  fn add_op_with_negatives() {
    // execute program "1101,100,-1,4,0"
    let mut prg = IntcodeProgram::new(&"1101,100,-1,4,0".to_owned(), None).unwrap();

    let expected_mem: Vec<i64> = vec![1101, 100, -1, 4, 0];
    for i in 0..expected_mem.len() {
      assert_eq!(prg.get_value(i), expected_mem[i]);
    }

    // last value should be exit opcode
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.get_value(4), 99);
  }

  #[test]
  fn quine_program() {
    let mut prg = IntcodeProgram::new(
      &"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_owned(),
      Some(Vec::<i64>::new()),
    )
    .unwrap();

    let expected_mem: Vec<i64> = vec![
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    for i in 0..expected_mem.len() {
      assert_eq!(prg.get_value(i), expected_mem[i]);
    }

    // program should output its own memory
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.output, expected_mem);
  }

  #[test]
  fn output_16_digit_number() {
    let mut prg = IntcodeProgram::new(
      &"1102,34915192,34915192,7,4,7,99,0".to_owned(),
      Some(Vec::<i64>::new()),
    )
    .unwrap();

    let expected_mem: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    for i in 0..expected_mem.len() {
      assert_eq!(prg.get_value(i), expected_mem[i]);
    }

    // program should output a 16-digit number
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.output[0], 1219070632396864);
  }

  #[test]
  fn output_middle_number() {
    let mut prg = IntcodeProgram::new(
      &"104,1125899906842624,99".to_owned(),
      Some(Vec::<i64>::new()),
    )
    .unwrap();
    let expected_mem: Vec<i64> = vec![104, 1125899906842624, 99];

    // program should output the second number in memory
    assert_eq!(prg.run().unwrap(), ());
    assert_eq!(prg.output[0], expected_mem[1]);
  }
}
