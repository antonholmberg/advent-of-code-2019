
use std::ops::Index;
use std::ops::IndexMut;
use std::convert::TryFrom;

/// Struct representing a program.
#[derive(Eq, PartialEq, Debug)]
pub struct Program {
  current_index: usize,
  program_state: Vec<i32>,
}

/// Enum representing the result of an execution.
#[derive(Eq, PartialEq, Debug)]
pub enum ExecutionStatus {
  /// Execution was terminated due to an unknown opcode
  UnknownOpCode,
  /// Execution was successful
  Success,
}

/// Enum representing an operation to be performed during execution.
#[derive(Eq, PartialEq, Debug)]
enum Operation {
  Add,
  Multiply,
  Finish,
  Unknown
}

impl From<i32> for Operation {
  /// Converts from an opcode to an Operation enum.
  fn from(input: i32) -> Self {
    match input {
      1 => Self::Add,
      2 => Self::Multiply,
      99 => Self::Finish,
      _ => Self::Unknown
    }
  }
}

impl TryFrom<&str> for Program {
  type Error = std::num::ParseIntError;

  /// Parses an initial program state from an input string.
  ///
  /// # Example
  /// ```rust
  /// let program = Program::try_from("1,2,3")?;
  /// ```
  fn try_from(string: &str) -> Result<Self, Self::Error> {
    let state = string.split(",")
      .map(|input| input.parse::<i32>())
      .collect::<Result<Vec<i32>, Self::Error>>()?;

    Ok(Program {
      current_index: 0,
      program_state: state
    })
  }
}

impl Index<usize> for Program {
  type Output = i32;

  fn index(&self, index: usize) -> &Self::Output {
    &self.program_state[index]
  }
}

impl IndexMut<usize> for Program {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.program_state[index]
  }
}

impl Program {
  pub fn execute(&mut self) -> Result<ExecutionStatus, Box<dyn std::error::Error>> {
    loop {
      let opcode = self[self.current_index].clone();
      let x_index = self.program_state.get(self.current_index + 1).map(|element| element.clone());
      let y_index = self.program_state.get(self.current_index + 2).map(|element| element.clone());
      let output_index = self.program_state.get(self.current_index + 3).map(|element| element.clone());

      match Operation::from(opcode) {
        Operation::Add => {
          let x = self[usize::try_from(x_index.unwrap())?];
          let y = self[usize::try_from(y_index.unwrap())?];
          self[usize::try_from(output_index.unwrap())?] = x + y;
          self.current_index += 4
        },
        Operation::Multiply => {
          let x = self[usize::try_from(x_index.unwrap())?];
          let y = self[usize::try_from(y_index.unwrap())?];
          self[usize::try_from(output_index.unwrap())?] = x * y;
          self.current_index += 4
        },
        Operation::Finish => {
          return Ok(ExecutionStatus::Success);
        }
        Operation::Unknown => {
          return Ok(ExecutionStatus::UnknownOpCode);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_try_from() -> Result<(), Box<dyn std::error::Error>> {
    // Given

    // When
    let program = Program::try_from("1,2,3")?;

    // Then
    let expect = Program {
      current_index: 0,
      program_state: vec![1, 2, 3]
    };
    assert_eq!(expect, program, "Program did not parse correctly");

    Ok(())
  }

  #[test]
  fn test_index() {
    // Given
    let program = Program {
      current_index: 0,
      program_state: vec![1, 2, 3]
    };

    assert_eq!(1, program[0]);
  }

  #[test]
  fn test_index_mut() {
    // Given
    let mut program = Program {
      current_index: 0,
      program_state: vec![1, 2, 3]
    };

    // When
    program[0] = 2;

    // Then
    assert_eq!(2, program[0]);
  }

  #[test]
  fn test_execute() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let mut program = Program {
      current_index: 0,
      program_state: vec![1,0,0,0,99]
    };

    // When
    let execution = program.execute()?;

    // Then
    assert_eq!(ExecutionStatus::Success, execution);
    assert_eq!(vec![2,0,0,0,99], program.program_state);

    Ok(())
  }

  #[test]
  fn test_operation_from() {
    assert_eq!(Operation::Add, Operation::from(1));
    assert_eq!(Operation::Multiply, Operation::from(2));
    assert_eq!(Operation::Finish, Operation::from(99));
    assert_eq!(Operation::Unknown, Operation::from(3));
  }
}
