
pub mod OPCODES {

    /*
        OP contains full instructions for special operations, like
        NOP or RFE.

        Inner modules R, I and J contain *only opcodes* for their
        respective operation types
    */
    pub const NOP : u32 = 0x00000000;
    pub const RFE : u32 = 0x42000001;
    pub const HLT : u32 = 0x42000010;

    pub const SYSCALL: u32 = 0x68000000;

    pub mod R {

        pub const ADD  : u32 = 0b100000;
        pub const ADDU : u32 = 0b100001;
        pub const AND  : u32 = 0b100100;
        pub const NOR  : u32 = 0b100111;
        pub const OR   : u32 = 0b100101;
        pub const SUB  : u32 = 0b100010;
        pub const SUBU : u32 = 0b100011;
        pub const XOR  : u32 = 0b100110;
        pub const SLT  : u32 = 0b101010;
        pub const SLTU : u32 = 0b101001;
        pub const DIV  : u32 = 0b011010;
        pub const DIVU : u32 = 0b011011;
        pub const MULT : u32 = 0b011000;
        pub const MULTU: u32 = 0b011001;
        pub const SLL  : u32 = 0b000000;
        pub const SRA  : u32 = 0b000011;
        pub const SRAV : u32 = 0b000111;
        pub const SRLV : u32 = 0b000110;
        pub const JARL : u32 = 0b001001;
        pub const JR   : u32 = 0b001000;
        pub const MFHI : u32 = 0b010000;
        pub const MFLO : u32 = 0b010010;
        pub const MTHI : u32 = 0b010001;
        pub const MTLO : u32 = 0b010011;
    }

    pub mod I {

        pub const ADDI : u32 = 0b001000;
        pub const ADDIU: u32 = 0b001001;
        pub const ANDI : u32 = 0b001100;
        pub const ORI  : u32 = 0b001101;
        pub const XORI : u32 = 0b001110;
        pub const SLTI : u32 = 0b001010;
        pub const SLTIU: u32 = 0b001011;
        pub const LHI  : u32 = 0b011001;
        pub const LLO  : u32 = 0b011000;
        pub const BEQ  : u32 = 0b000100;
        pub const BNE  : u32 = 0b000101;
        pub const BGTZ : u32 = 0b000111;
        pub const BLEZ : u32 = 0b000110;
        pub const LB   : u32 = 0b100000;
        pub const LBU  : u32 = 0b100100;
        pub const LH   : u32 = 0b100001;
        pub const LHU  : u32 = 0b100101;
        pub const LW   : u32 = 0b100011;
        pub const SB   : u32 = 0b101000;
        pub const SH   : u32 = 0b101001;
        pub const SW   : u32 = 0b101011;
        
    }

    pub mod J {

        pub const J  : u32 = 0b000010;
        pub const JAL: u32 = 0b000011;

    }
}
