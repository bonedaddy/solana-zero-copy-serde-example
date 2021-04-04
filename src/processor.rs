//! Program state processor

use crate::{
    error::ProgramTemplateError,
    instruction::TemplateInstruction,
    state::UniswapV3Input,
    state::{ArchivedUniswapV3Input, ArchivedUniswapV3State, Layer1, UniswapV3State},
};
use borsh::BorshDeserialize;
use num_traits::{AsPrimitive, FromPrimitive, Num};
use rkyv::{
    archived_unsized_value, archived_unsized_value_mut, archived_value, check_archive,
    de::deserializers::AllocDeserializer,
    ser::{serializers::WriteSerializer, Serializer},
    AlignedVec, Deserialize,
};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use std::{borrow::BorrowMut, convert::TryFrom, pin::Pin};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Initialize the pool
    pub fn init_with_borsh(
        _program_id: &Pubkey,
        uniswap_account: &AccountInfo,
        uniswap: &UniswapV3Input,
    ) -> ProgramResult {
        overflow_it(uniswap_account)?;

        Ok(())
    }

    pub fn init_with_rkyv(
        _program_id: &Pubkey,
        uniswap_account: &AccountInfo,
        uniswap: &ArchivedUniswapV3Input,
    ) -> ProgramResult {
        // msg!(
        //     "rkyv Got the state {}",
        //     uniswap.state[13].state[13].state[13]
        // );
        // no_overflow_rkyv(uniswap_account);
        Ok(())
    }

    pub fn read_with_rkyv(_program_id: &Pubkey, uniswap_account: &AccountInfo) -> ProgramResult {
        let data = uniswap_account.data.try_borrow().unwrap();
        let state = &data[..];
        let archived = unsafe { archived_unsized_value::<UniswapV3State>(state, 0) };
        let state = &archived.state[13].state[13].state[13].state[13];
        msg!("rkyv Got the state {}", state);

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction: TemplateInstruction =
            num_traits::FromPrimitive::from_u8(input[0]).unwrap();
        match instruction {
            TemplateInstruction::WithBorsh => {
                msg!("Instruction: WithBorsh");
                match accounts {
                    [uniswap_account] => {
                        let mut input = &input[1..];
                        let uniswap = UniswapV3Input::deserialize(&mut input)?;
                        Self::init_with_borsh(&program_id, &uniswap_account, &uniswap)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            TemplateInstruction::WithRkyv => {
                msg!("Instruction: WithRkyv");
                match accounts {
                    [uniswap_account] => {
                        let mut input = &input[8..];
                        let uniswap = check_archive::<UniswapV3Input>(&input[..], 0);
                        match uniswap {
                            Ok(uniswap) => {
                                Self::init_with_rkyv(&program_id, &uniswap_account, &uniswap)
                            }
                            Err(error) => {
                                msg!("Rkyv Input Error {}", error.to_string());
                                Err(ProgramError::InvalidArgument)
                            }
                        }
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            TemplateInstruction::ReadRkyv => {
                msg!("Instruction: ReadRkyv");
                match accounts {
                    [uniswap_account] => Self::read_with_rkyv(&program_id, &uniswap_account),
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
        }
    }
}

fn no_overflow_rkyv(uniswap_account: &AccountInfo) {
    // can wrap it into some less verbose pattern with check_archive call
    let mut data = uniswap_account.data.try_borrow_mut().unwrap();
    let mut state = &mut data[8..];
    let pin = Pin::new(state);
    let mut archived = unsafe { archived_unsized_value_mut::<UniswapV3State>(pin, 0) };
    let mut layer = &mut archived.state[13].state[13].state[13];
    layer.state[13] = 42;
}

fn overflow_it(uniswap_account: &AccountInfo) -> Result<(), ProgramError> {
    let mut data = uniswap_account.try_borrow_mut_data().unwrap();
    let mut state = &data[1..];
    let uniswap: UniswapV3State = BorshDeserialize::deserialize(&mut state)?;
    Ok(())
}
