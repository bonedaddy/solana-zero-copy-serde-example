//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use rkyv::{
    de::deserializers::AllocDeserializer,
    ser::{serializers::WriteSerializer, Serializer},
    AlignedVec, Archive, Archived, Deserialize, Serialize,
};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::UniswapV3Input;

use num_derive::{FromPrimitive, ToPrimitive};

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum TemplateInstruction {
    /// Example.
    ///
    ///   0. `[w]` Example account.
    WithBorsh,

    /// Example.
    ///
    ///   0. `[w]` Example account.
    WithRkyv,

    /// Reads data previously save by rkyv
    ReadRkyv,
}

/// Create `Example` instruction
pub fn init_borsh(
    program_id: &Pubkey,
    example_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let op = num_traits::ToPrimitive::to_u8(&TemplateInstruction::WithBorsh).unwrap();
    let mut data: Vec<u8> = Vec::with_capacity(1 + UniswapV3Input::LEN);
    data.push(op);
    let mut state = UniswapV3Input::new().try_to_vec()?;
    data.append(&mut state);

    let accounts = vec![AccountMeta::new(*example_account, false)];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn read_rkyv(
    program_id: &Pubkey,
    example_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let op = num_traits::ToPrimitive::to_u8(&TemplateInstruction::ReadRkyv).unwrap();
    let accounts = vec![AccountMeta::new_readonly(*example_account, false)];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: vec![op],
    })
}

pub fn init_rkyv(
    program_id: &Pubkey,
    example_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let op = num_traits::ToPrimitive::to_u8(&TemplateInstruction::WithRkyv).unwrap();
    let mut data: Vec<u8> = Vec::with_capacity(8 + UniswapV3Input::LEN);

    data.push(op);
    // rkyv requires 8 byte alignments ...
    data.push(0);
    data.push(0);
    data.push(0);
    data.push(0);
    data.push(0);
    data.push(0);
    data.push(0);
    let mut ser = WriteSerializer::new(AlignedVec::new());
    ser.serialize_value(&UniswapV3Input::new()).unwrap();
    let mut state = ser.into_inner().to_vec();
    data.append(&mut state);

    let accounts = vec![AccountMeta::new(*example_account, false)];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
