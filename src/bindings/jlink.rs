//! Bindings auto generated from SEGGER RTOS Plug-in S

use std::option::Option;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void};

use num_derive::FromPrimitive;

pub const CORE_NONE: u32 = 0;
pub const CORE_ANY: u32 = 4294967295;
pub const CORE_CORTEX_M1: u32 = 16777471;
pub const CORE_COLDFIRE: u32 = 50331647;
pub const CORE_CORTEX_M3: u32 = 50331903;
pub const CORE_CORTEX_M3_R1P0: u32 = 50331664;
pub const CORE_CORTEX_M3_R1P1: u32 = 50331665;
pub const CORE_CORTEX_M3_R2P0: u32 = 50331680;
pub const CORE_SIM: u32 = 83886079;
pub const CORE_XSCALE: u32 = 100663295;
pub const CORE_CORTEX_M0: u32 = 100663551;
pub const CORE_CORTEX_M_V8BASEL: u32 = 100729087;
pub const CORE_ARM7: u32 = 134217727;
pub const CORE_ARM7TDMI: u32 = 117440767;
pub const CORE_ARM7TDMI_R3: u32 = 117440575;
pub const CORE_ARM7TDMI_R4: u32 = 117440591;
pub const CORE_ARM7TDMI_S: u32 = 117441023;
pub const CORE_ARM7TDMI_S_R3: u32 = 117440831;
pub const CORE_ARM7TDMI_S_R4: u32 = 117440847;
pub const CORE_CORTEX_A8: u32 = 134217983;
pub const CORE_CORTEX_A7: u32 = 134742271;
pub const CORE_CORTEX_A9: u32 = 134807807;
pub const CORE_CORTEX_A12: u32 = 134873343;
pub const CORE_CORTEX_A15: u32 = 134938879;
pub const CORE_CORTEX_A17: u32 = 135004415;
pub const CORE_ARM9: u32 = 167772159;
pub const CORE_ARM9TDMI_S: u32 = 150995455;
pub const CORE_ARM920T: u32 = 153092351;
pub const CORE_ARM922T: u32 = 153223423;
pub const CORE_ARM926EJ_S: u32 = 153485823;
pub const CORE_ARM946E_S: u32 = 155582975;
pub const CORE_ARM966E_S: u32 = 157680127;
pub const CORE_ARM968E_S: u32 = 157811199;
pub const CORE_ARM11: u32 = 201326591;
pub const CORE_ARM1136: u32 = 188153855;
pub const CORE_ARM1136J: u32 = 188089087;
pub const CORE_ARM1136J_S: u32 = 188089343;
pub const CORE_ARM1136JF: u32 = 188090111;
pub const CORE_ARM1136JF_S: u32 = 188090367;
pub const CORE_ARM1156: u32 = 190251007;
pub const CORE_ARM1176: u32 = 192348159;
pub const CORE_ARM1176J: u32 = 192283391;
pub const CORE_ARM1176J_S: u32 = 192283647;
pub const CORE_ARM1176JF: u32 = 192284415;
pub const CORE_ARM1176JF_S: u32 = 192284671;
pub const CORE_CORTEX_R4: u32 = 201326847;
pub const CORE_CORTEX_R5: u32 = 201392383;
pub const CORE_RX: u32 = 234881023;
pub const CORE_RX610: u32 = 218169343;
pub const CORE_RX62N: u32 = 218234879;
pub const CORE_RX62T: u32 = 218300415;
pub const CORE_RX63N: u32 = 218365951;
pub const CORE_RX630: u32 = 218431487;
pub const CORE_RX63T: u32 = 218497023;
pub const CORE_RX621: u32 = 218562559;
pub const CORE_RX62G: u32 = 218628095;
pub const CORE_RX631: u32 = 218693631;
pub const CORE_RX65N: u32 = 218759167;
pub const CORE_RX210: u32 = 219217919;
pub const CORE_RX21A: u32 = 219283455;
pub const CORE_RX220: u32 = 219348991;
pub const CORE_RX230: u32 = 219414527;
pub const CORE_RX231: u32 = 219480063;
pub const CORE_RX23T: u32 = 219545599;
pub const CORE_RX24T: u32 = 219611135;
pub const CORE_RX111: u32 = 220266495;
pub const CORE_RX110: u32 = 220332031;
pub const CORE_RX113: u32 = 220397567;
pub const CORE_RX130: u32 = 220463103;
pub const CORE_RX64M: u32 = 221315071;
pub const CORE_RX71M: u32 = 221380607;
pub const CORE_CORTEX_M4: u32 = 234881279;
pub const CORE_CORTEX_M7: u32 = 234946815;
pub const CORE_CORTEX_M_V8MAINL: u32 = 235012351;
pub const CORE_CORTEX_A5: u32 = 251658495;
pub const CORE_POWER_PC: u32 = 285212671;
pub const CORE_POWER_PC_N1: u32 = 285147391;
pub const CORE_POWER_PC_N2: u32 = 285147647;
pub const CORE_MIPS: u32 = 301989887;
pub const CORE_MIPS_M4K: u32 = 285278207;
pub const CORE_MIPS_MICROAPTIV: u32 = 285343743;
pub const CORE_EFM8_UNSPEC: u32 = 318767103;
pub const CORE_CIP51: u32 = 302055423;

pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R0:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 0;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R1:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 1;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R2:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 2;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R3:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 3;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R4:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 4;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R5:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 5;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R6:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 6;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R7:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 7;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R8:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 8;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R9:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 9;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R10:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 10;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R11:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 11;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R12:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 12;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_SP:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 13;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_LR:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 14;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_PC:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 15;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_XPSR:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 16;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_MSP:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 17;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_PSP:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 18;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_PRIMASK:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 19;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_BASEPRI:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 20;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_FAULTMASK:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 21;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_CONTROL:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 22;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_FPSCR:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 23;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S0:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 24;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S1:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 25;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S2:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 26;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S3:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 27;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S4:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 28;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S5:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 29;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S6:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 30;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S7:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 31;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S8:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 32;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S9:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 33;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S10:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 34;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S11:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 35;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S12:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 36;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S13:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 37;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S14:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 38;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S15:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 39;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S16:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 40;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S17:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 41;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S18:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 42;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S19:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 43;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S20:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 44;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S21:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 45;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S22:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 46;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S23:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 47;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S24:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 48;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S25:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 49;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S26:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 50;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S27:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 51;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S28:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 52;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S29:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 53;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S30:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 54;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S31:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 55;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D0:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 56;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D1:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 57;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D2:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 58;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D3:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 59;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D4:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 60;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D5:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 61;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D6:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 62;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D7:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 63;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D8:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 64;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D9:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 65;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D10:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 66;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D11:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 67;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D12:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 68;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D13:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 69;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D14:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 70;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D15:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 71;
pub const RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_NUMREGS:
    RTOS_PLUGIN_CPU_REGS_CORTEX_M = 72;

pub type RTOS_PLUGIN_CPU_REGS_CORTEX_M = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RTOS_SYMBOLS {
    pub name: *const c_char,
    pub optional: c_int,
    pub address: c_uint,
}

#[derive(FromPrimitive)]
pub enum Registers {
    R0 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R0 as isize,
    R1 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R1 as isize,
    R2 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R2 as isize,
    R3 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R3 as isize,
    R4 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R4 as isize,
    R5 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R5 as isize,
    R6 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R6 as isize,
    R7 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R7 as isize,
    R8 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R8 as isize,
    R9 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R9 as isize,
    R10 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R10 as isize,
    R11 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R11 as isize,
    R12 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_R12 as isize,
    SP = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_SP as isize,
    LR = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_LR as isize,
    PC = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_PC as isize,
    XPSR = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_XPSR as isize,
    MSP = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_MSP as isize,
    PSP = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_PSP as isize,
    PRIMASK = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_PRIMASK as isize,
    BASEPRI = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_BASEPRI as isize,
    FAULTMASK = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_FAULTMASK as isize,
    CONTROL = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_CONTROL as isize,
    FPSCR = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_FPSCR as isize,
    S0 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S0 as isize,
    S1 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S1 as isize,
    S2 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S2 as isize,
    S3 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S3 as isize,
    S4 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S4 as isize,
    S5 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S5 as isize,
    S6 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S6 as isize,
    S7 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S7 as isize,
    S8 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S8 as isize,
    S9 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S9 as isize,
    S10 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S10 as isize,
    S11 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S11 as isize,
    S12 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S12 as isize,
    S13 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S13 as isize,
    S14 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S14 as isize,
    S15 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S15 as isize,
    S16 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S16 as isize,
    S17 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S17 as isize,
    S18 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S18 as isize,
    S19 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S19 as isize,
    S20 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S20 as isize,
    S21 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S21 as isize,
    S22 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S22 as isize,
    S23 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S23 as isize,
    S24 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S24 as isize,
    S25 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S25 as isize,
    S26 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S26 as isize,
    S27 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S27 as isize,
    S28 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S28 as isize,
    S29 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S29 as isize,
    S30 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S30 as isize,
    S31 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_S31 as isize,
    D0 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D0 as isize,
    D1 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D1 as isize,
    D2 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D2 as isize,
    D3 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D3 as isize,
    D4 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D4 as isize,
    D5 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D5 as isize,
    D6 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D6 as isize,
    D7 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D7 as isize,
    D8 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D8 as isize,
    D9 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D9 as isize,
    D10 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D10 as isize,
    D11 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D11 as isize,
    D12 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D12 as isize,
    D13 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D13 as isize,
    D14 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D14 as isize,
    D15 = RTOS_PLUGIN_CPU_REGS_CORTEX_M_RTOS_PLUGIN_CPU_REG_CORTEX_M_D15 as isize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// GDB Server API that can be used by the plug-in.
pub struct GdbApi {
    /// Frees allocated memory block.
    ///
    /// # Arguments
    /// * `p` - Pointer to the memory block.
    pub free: Option<unsafe extern "C" fn(p: *mut c_void)>,

    /// Allocates a memory block.
    ///
    /// # Arguments
    /// * `size` - Size of requested memory block.
    ///
    /// # Returns
    /// Pointer to the allocated memory block. null if the memory could not be allocated.
    pub malloc: Option<unsafe extern "C" fn(size: c_uint) -> *mut c_void>,

    /// Reallocates a memory block.
    /// If necessary, copies contents of old memory block into new block.
    ///
    /// # Arguments
    /// * `p`    - Pointer to the existing memory block.
    /// * `size` - Size of newly requested memory block.
    ///
    /// # Returns
    /// Pointer to the allocated memory block. null if the memory could not be allocated.
    pub realloc: Option<unsafe extern "C" fn(p: *mut c_void, size: c_uint) -> *mut c_void>,

    /// Prints a formatted log message to the J-Link GDB Server's output.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    pub output: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Prints a formatted log message to the J-Link GDB Server's debug output.
    /// The debug output is disables in non-debug builds of the J-Link GDB Server.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    ///
    /// # Notes
    /// Since debug builds of the J-Link GDB Server are not publicly available, the debug output should not be used.
    pub output_debug: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Prints a formatted log message to the J-Link GDB Server's warning output.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    pub output_warning: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Prints a formatted log message to the J-Link GDB Server's error output.
    ///
    /// # Arguments
    /// * `fmt` - Format string (c style).
    /// * `...` - Variables needed by the format string.
    pub output_error: Option<unsafe extern "C" fn(fmt: *const c_char, ...)>,

    /// Reads a byte array from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    /// * `size` - Number of bytes to read.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_byte_array:
        Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_char, size: c_uint) -> c_int>,

    /// Reads one byte from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_u8: Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_uchar) -> c_char>,

    /// Reads two bytes from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_u16: Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_ushort) -> c_char>,

    /// Reads four bytes from the target system.
    /// If necessary, the target CPU is halted in order to read memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to read from.
    /// * 'buff` - Pointer to buffer for read-out.
    ///
    /// # Returns
    /// * `0`  - Reading memory OK.
    /// * `<0` - Reading memory failed.
    pub read_u32: Option<unsafe extern "C" fn(addr: c_uint, buff: *mut c_uint) -> c_char>,

    /// Writes a byte array to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    /// * `size` - Number of bytes to write.
    ///
    /// # Returns
    /// * `0`  - Writing memory OK.
    /// * `<0` - Writing memory failed.
    pub write_byte_array:
        Option<unsafe extern "C" fn(addr: c_uint, buff: *const c_char, size: c_uint) -> c_int>,

    /// Writes one byte to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    pub write_u8: Option<unsafe extern "C" fn(Addr: c_uint, Data: c_uchar)>,

    /// Writes two bytes to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    pub write_u16: Option<unsafe extern "C" fn(Addr: c_uint, Data: c_ushort)>,

    /// Writes four bytes to the target system.
    /// If necessary, the target CPU is halted in order to write memory.
    ///
    /// # Arguments
    /// * `addr` - Target address to write to.
    /// * 'buff` - Pointer to buffer containig data.
    pub write_u32: Option<unsafe extern "C" fn(Addr: c_uint, Data: c_uint)>,

    /// Converts two bytes according to the target's endianness.
    ///
    /// # Arguments
    /// * `p` - Pointer to a u16
    ///
    /// # Returns
    /// The converted value.
    pub convert_u16: Option<unsafe extern "C" fn(p: *const c_uchar) -> c_uint>,

    /// Converts three bytes according to the target's endianness.
    ///
    /// # Arguments
    /// * `p` - Pointer to a u24
    ///
    /// # Returns
    /// The converted value.
    pub convert_u24: Option<unsafe extern "C" fn(p: *const c_uchar) -> c_uint>,

    /// Converts four bytes according to the target's endianness.
    ///
    /// # Arguments
    /// * `p` - Pointer to a u32
    ///
    /// # Returns
    /// The converted value.
    pub convert_u32: Option<unsafe extern "C" fn(p: *const c_uchar) -> c_uint>,

    /// Reads a register from the target's CPU.
    ///
    /// # Arguments
    /// * `reg_index` - Index of the register, refer to [Registers](crate::bindings::jlink::Registers)
    ///
    /// # Returns
    /// Value of the register
    pub read_register: Option<unsafe extern "C" fn(reg_index: c_uint) -> c_uint>,

    /// Writes a value to the target's CPU register.
    ///
    /// # Arguments
    /// * `reg_index` - Index of the register, refer to [Registers](crate::bindings::jlink::Registers)
    /// * `value`     - Value of the register
    pub write_register: Option<unsafe extern "C" fn(reg_index: c_uint, value: c_uint)>,

    /// WTF Segger?! Just WTF?
    pub dummy: *mut c_void,
}
