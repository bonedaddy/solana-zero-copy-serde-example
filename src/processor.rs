//! Program state processor

use std::{borrow::BorrowMut, convert::TryFrom};
use num_traits::{AsPrimitive, FromPrimitive, Num};
use rkyv::{AlignedVec, Deserialize, archived_value,archived_unsized_value,  check_archive, de::deserializers::AllocDeserializer, ser::{Serializer, serializers::WriteSerializer}};
use crate::{error::ProgramTemplateError, instruction::TemplateInstruction, state::UniswapV3Input, state::{UniswapV3State, ArchivedUniswapV3State, ArchivedUniswapV3Input, Internal, Test}};
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, account_info::next_account_info, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Initialize the pool
    pub fn init_with_borsh(
        _program_id: &Pubkey,
        uniswap_account: &AccountInfo,
        uniswap: &UniswapV3Input,
    ) -> ProgramResult {
        msg!("BORSH Got the state {}", uniswap.state[13].state[13].state[13]);
        
        overflow_it(uniswap_account)?;

        Ok(())
    }

    pub fn init_with_rkyv(
        _program_id: &Pubkey,
        uniswap_account: &AccountInfo,
        uniswap: &UniswapV3Input,
    ) -> ProgramResult {
        msg!("RKYV Got the state {}", uniswap.state[13].state[13].state[13]);
        
        let data =  uniswap_account.data.try_borrow_mut().unwrap();
        let mut state = &data[8..];
        
        let ser: u128 = 42;
        // TODO: have to writ custom Serializer allocator (which "allocates" only into mem)
        {
            //msg!("RKYV 128");
            //let mut serializer = WriteSerializer::new(AlignedVec::new());
            // very strange, there is no serialise when slice is used, so cannot serde into slice?.....
            //let mut serializer = WriteSerializer::new(&mut state);
            //let pos = serializer.serialize_value(&ser)
                //.expect("failed to archive test");
            //let buf = serializer.into_inner().to_vec();
            //let archived = unsafe { archived_value::<u128>(&buf[..], pos) };
            //let archived = unsafe { archived_value::<u128>(state, pos) };
            //msg!("RKYV 128 {}", archived);
        }
        {

            let mut pull = vec![0u8;500000];
            assert!(state.len() > pull.len());
            assert!(state[42] == 0);
            msg!("serde");
            assert!(check_archive::<Test>(&state[..],0).err().is_none());
            //let err = check_archive::<Test>(&pull[..],0).err();
             //msg!("{}",  err.to_string());
            let mut archived = unsafe { archived_unsized_value::<UniswapV3State>(&state, 0) };            
            let mut q: &u128 = &archived.state[13].state[13].state[13].state[13];

            //*q = 4;
        
        }    // it can non zero copy too!!!
        //let uniswap: UniswapV3State = archived.deserialize(&mut AllocDeserializer).unwrap();
        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {        
        let instruction: TemplateInstruction = num_traits::FromPrimitive::from_u8(input[0]).unwrap();
        match instruction {
            TemplateInstruction::WithBorsh => {
                msg!("Instruction: WithBorsh");
                match accounts {
                    [uniswap_account] => {                        
                        let mut input = &input[1..];
                        let uniswap = UniswapV3Input::deserialize(&mut input)?;                        
                        Self::init_with_borsh(&program_id, &uniswap_account, &uniswap)
                    },
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }                
            }
            TemplateInstruction::WithRkyv => {
                msg!("Instruction: WithRkyv");
                match accounts {
                    [uniswap_account] => {                        
                        let mut input = &input[1..];
                        let uniswap = UniswapV3Input::deserialize(&mut input)?;                        
                        Self::init_with_rkyv(&program_id, &uniswap_account, &uniswap)
                    },
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }            }
        }
    }
}

fn overflow_it(uniswap_account: &AccountInfo) -> Result<(), ProgramError> {
    let mut data = uniswap_account.try_borrow_mut_data().unwrap();
    let mut state = &data[1..];
    let uniswap:u128 = BorshDeserialize::deserialize(&mut state)?;
    //let uniswap = UniswapV3State::deserialize(&mut state)?;
    Ok(())
}
