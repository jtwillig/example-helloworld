use solana_program::program_error::ProgramError;

pub enum HelloInstruction {
    Increment,
    Decrement,
    Reset
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if let Some(&byte) = input.get(0) {
            match byte {
                0 => Ok(HelloInstruction::Increment),
                1 => Ok(HelloInstruction::Decrement),
                2 => Ok(HelloInstruction::Reset),
                _ => Err(ProgramError::InvalidInstructionData)
            }
        } else {
            Err(ProgramError::InvalidInstructionData)
        }
    }
}
