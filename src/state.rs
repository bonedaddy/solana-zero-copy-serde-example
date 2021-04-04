//! State transition types
use borsh::{BorshDeserialize, BorshSerialize};
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

// poor man way to generate fat struc (where is my macro?)
#[derive(
    BorshDeserialize, BorshSerialize, Archive, Clone, Copy, Default, Serialize, Deserialize,
)]
#[archive(derive(CheckBytes))]
#[repr(C)]
pub struct Layer1 {
    pub state: [u128; 15],
}

#[derive(
    BorshDeserialize, BorshSerialize, Archive, Clone, Copy, Default, Serialize, Deserialize,
)]
#[repr(C)]
#[archive(derive(CheckBytes))]
pub struct Internal2 {
    pub state: [Layer1; 15],
}

/// example of possible big state

#[derive(
    BorshDeserialize, BorshSerialize, Archive, Clone, Copy, Default, Serialize, Deserialize,
)]
#[repr(C)]
#[archive(derive(CheckBytes))]
pub struct UniswapV3Input {
    // it has big state with gradual fees/pools
    /// some big state
    pub state: [Internal2; 15], // until const generics, neither borsh nor rkyv support long arrays
}

#[derive(
    BorshDeserialize, BorshSerialize, Archive, Clone, Copy, Default, Serialize, Deserialize,
)]
#[repr(C)]
#[archive(derive(CheckBytes))]
pub struct UniswapV3State {
    pub state: [UniswapV3Input; 31],
}

impl UniswapV3State {
    pub const LEN: usize = 1 + 31 * UniswapV3Input::LEN;
}

impl UniswapV3Input {
    pub const LEN: usize = 1 + 15 * (1 + 15 * (1 + 15 * 16)); // making this larger and using as input fails on client side with check of 65k size (so cannot make it overflow via input directly)
    pub fn new() -> Self {
        let mut it = Self::default();
        it.state[13].state[13].state[13] = 13;
        it
    }
}

#[cfg(test)]
mod tests {
    use crate::state::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use rkyv::{
        archived_value,
        de::deserializers::AllocDeserializer,
        ser::{serializers::WriteSerializer, Serializer},
        AlignedVec, Archive, Archived, Deserialize, Serialize,
    };

    #[test]
    fn do_test() {
        let mut pull = vec![0u8; 500000];
        let archived = unsafe { archived_value::<UniswapV3State>(&pull[..], 0) };
    }
}
