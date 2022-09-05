use solana_program::{
    program_error::ProgramError,
};
use std::convert::TryInto;
#[derive(Debug)]
pub enum InstructionValue{
    Increment,
    Decrement,
    Set(u32)
}
impl InstructionValue{
pub fn unpack(input:&[u8])->Result<Self,ProgramError>{
let (&tag,rest)=input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
match tag{
    0=>return Ok(InstructionValue::Increment),
    1=>return Ok(InstructionValue::Decrement),
    2=>{
        if rest.len()!=4{
            return Err(ProgramError::InvalidInstructionData);
        }
        let val: Result<[u8; 4],_>=rest[..4].try_into();
        match val{
            Ok(i)=>{
return Ok(InstructionValue::Set(u32::from_le_bytes(i)));
            }
            _=>{return Err(ProgramError::InvalidInstructionData);}
        }
    }
    _=>{return Err(ProgramError::InvalidInstructionData)}
}
}
}