use std::collections::HashMap;
use std::io;
use std::vec::Vec;

#[derive(Debug)]
struct IntcodeOperation {
  /// Opcode of current operation
  /// 
  /// Add: 1
  /// Multiply: 2
  /// Get input: 3
  /// Print value: 4
  /// Exit: 99
  opcode: u8,
  /// Length of current operation
  /// 
  /// Add: 4
  /// Multiply: 4
  /// Get input: 2
  /// Print value: 2
  /// Exit: 99
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
    let valid_opcodes: Vec<u8> = vec![1, 2, 3, 4, 99];
    if !valid_opcodes.contains(&code) {
      return Err("Opcode is not valid.");
    }

    // create map of operation lengths
    let valid_lens: Vec<usize> = vec![4, 4, 2, 2, 1];
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


  /// Performs the current Intcode operation using the Intcode program memory
  fn perform(&self, mem: &mut Vec<i32>, ip: usize) -> Result<(), &'static str> {
    if self.opcode == 1 || self.opcode == 2 {
      // get first parameter of operation
      let addr_l = match self.modes[0] {
        // position mode
        0 => mem[ip + 1] as isize,
        // immediate mode
        1 => ip as isize + 1,
        // return -1 for any unrecognized mode
        _ => -1,
      };
      if addr_l == -1 {
        if self.opcode == 1 {
          return Err("Invalid mode for second parameter of addition operation.");
        } else {
          return Err("Invalid mode for second parameter of multiplication operation.");
        }
      }
      let op_l = mem[addr_l as usize];

      // get second parameter of operation
      let addr_r = match self.modes[1] {
        // position mode
        0 => mem[ip + 2] as isize,
        // immediate mode
        1 => ip as isize + 2,
        // return -1 for any unrecognized mode
        _ => -1,
      };
      if addr_r == -1 {
        if self.opcode == 1 {
          return Err("Invalid mode for second parameter of addition operation.");
        } else {
          return Err("Invalid mode for second parameter of multiplication operation.");
        }
      }
      let op_r = mem[addr_r as usize];

      // perform operation
      let store_addr = mem[ip + 3] as usize;
      if self.opcode == 1 {
        // opcode 1: store sum of two parameters
        mem[store_addr] = op_l + op_r;
      } else {
        // opcode 2: store product of two parameters
        mem[store_addr] = op_l * op_r;
      }
    } else if self.opcode == 3 {
      // get integer as input from stdin (ignore newline characters)
      let mut input = String::new();
      println!("Enter an integer: ");
      io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
      let value = input[..(input.len() - 2)].parse::<i32>().unwrap();

      // store value at address (first parameter)
      let addr = mem[ip + 1] as usize;
      mem[addr] = value;
    } else if self.opcode == 4 {
      // get value from memory
      let addr = match self.modes[0] {
        // position mode
        0 => mem[ip + 1] as isize,
        // immediate mode
        1 => ip as isize + 1,
        // return -1 for any unrecognized mode
        _ => -1,
      };
      if addr == -1 {
        return Err("Invalid mode for parameter of output operation.");
      }
      let value = mem[addr as usize];

      // print value to stdout
      println!("Program outputted value: {}", value);
    }

    Ok(())
  }
}

#[derive(Debug)]
pub struct IntcodeProgram {
  memory: Vec<i32>,
}

impl IntcodeProgram {
  /// Creates a new IntcodeProgram object using the given program data
  pub fn new(data: &String) -> Result<IntcodeProgram, &'static str> {
    if data.len() == 0 {
      return Err("No valid input provided.");
    }

    // spilt program data into vector of values
    let values: Vec<_> = data.split(',').collect();
    let mut memory: Vec<i32> = Vec::<i32>::new();

    // parse value strings as 32-bit unsigned ints
    // and push to program memory vector
    for value in values {
      let parsed = value.parse::<i32>().unwrap();
      memory.push(parsed);
    }

    Ok(IntcodeProgram { memory })
  }

  /// Executes the IntcodeProgram to completion
  pub fn run(&mut self) -> Result<(), &'static str> {
    // initialize instruction pointer to 0
    let mut ip: usize = 0;
    loop {
      let cur_op = IntcodeOperation::new(self.memory[ip] as u32).unwrap();

      // quit loop on exit opcode
      if cur_op.opcode == 99 {
        break;
      }

      // perform current operation
      if let Err(e) = cur_op.perform(&mut self.memory, ip) {
        eprintln!("Operation failed: {}", e);
        return Err("Operation failed during program execution.");
      };

      // advance instruction pointer by length of current operation
      ip += cur_op.len;
    }

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
