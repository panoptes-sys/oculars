//! Prague execution upgrade.

use crate::{
    eip::macros::eip_set,
    eips::{
        eip2537::Eip2537, eip2935::Eip2935, eip6110::Eip6110, eip7002::Eip7002, eip7623::Eip7623,
        eip7685::Eip7685, eip7702::Eip7702, eip7840::Eip7840,
    },
    execution::ExecutionUpgrade,
    forks::cancun::Cancun,
};

/// Prague execution upgrade.
pub struct Prague;

impl ExecutionUpgrade for Prague {
    type EipSet = eip_set!(
        Cancun + Eip2537,
        Eip2935,
        Eip6110,
        Eip7002,
        Eip7623,
        Eip7685,
        Eip7702,
        Eip7840
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eips::eip2::Eip2;

    #[test]
    fn eip_support() {
        assert!(Prague::includes::<Eip2>());

        assert!(Prague::includes::<Eip2537>());
        assert!(Prague::includes::<Eip2935>());
        assert!(Prague::includes::<Eip6110>());
        assert!(Prague::includes::<Eip7002>());
        assert!(Prague::includes::<Eip7623>());
        assert!(Prague::includes::<Eip7685>());
        assert!(Prague::includes::<Eip7702>());
        assert!(Prague::includes::<Eip7840>());
    }

    #[test]
    fn instruction_support() {
        use asm::instruction::*;

        macro_rules! supports_instructions {
            ($($instr: path),+) => {
                $(assert!(Prague::supports_instruction::<$instr>());)+
            };
        }

        supports_instructions!(
            Stop,
            Add,
            Mul,
            Sub,
            Div,
            SDiv,
            Mod,
            SMod,
            AddMod,
            MulMod,
            Exp,
            SignExtend,
            Lt,
            Gt,
            SLt,
            SGt,
            Eq,
            IsZero,
            And,
            Or,
            Xor,
            Not,
            Byte,
            Shl,
            Shr,
            Sar,
            Keccak256,
            Address,
            Balance,
            Origin,
            Caller,
            CallValue,
            CallDataLoad,
            CallDataSize,
            CallDataCopy,
            CodeSize,
            CodeCopy,
            GasPrice,
            ExtCodeSize,
            ExtCodeCopy,
            ReturnDataSize,
            ReturnDataCopy,
            ExtCodeHash,
            BlockHash,
            CoinBase,
            Timestamp,
            Number,
            PrevRandao,
            GasLimit,
            ChainId,
            SelfBalance,
            BaseFee,
            BlobHash,
            BlobBaseFee,
            Pop,
            MLoad,
            MStore,
            MStore8,
            SLoad,
            SStore,
            Jump,
            JumpI,
            Pc,
            MSize,
            Gas,
            JumpDest,
            TLoad,
            TStore,
            MCopy,
            Push<0>,
            Push<1>,
            Push<2>,
            Push<3>,
            Push<4>,
            Push<5>,
            Push<6>,
            Push<7>,
            Push<8>,
            Push<9>,
            Push<10>,
            Push<11>,
            Push<12>,
            Push<13>,
            Push<14>,
            Push<15>,
            Push<16>,
            Push<17>,
            Push<18>,
            Push<19>,
            Push<20>,
            Push<21>,
            Push<22>,
            Push<23>,
            Push<24>,
            Push<25>,
            Push<26>,
            Push<27>,
            Push<28>,
            Push<29>,
            Push<30>,
            Push<31>,
            Push<32>,
            Dup<1>,
            Dup<2>,
            Dup<3>,
            Dup<4>,
            Dup<5>,
            Dup<6>,
            Dup<7>,
            Dup<8>,
            Dup<9>,
            Dup<10>,
            Dup<11>,
            Dup<12>,
            Dup<13>,
            Dup<14>,
            Dup<15>,
            Dup<16>,
            Swap<1>,
            Swap<2>,
            Swap<3>,
            Swap<4>,
            Swap<5>,
            Swap<6>,
            Swap<7>,
            Swap<8>,
            Swap<9>,
            Swap<10>,
            Swap<11>,
            Swap<12>,
            Swap<13>,
            Swap<14>,
            Swap<15>,
            Swap<16>,
            Log<0>,
            Log<1>,
            Log<2>,
            Log<3>,
            Log<4>,
            Create,
            Call,
            CallCode,
            Return,
            DelegateCall,
            Create2,
            StaticCall,
            Revert,
            Invalid,
            SelfDestruct,
            Unknown
        );
    }
}
