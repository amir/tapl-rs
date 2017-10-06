use std::str::FromStr;
use tapl::fullsimple::ast::{Term, Type};
use tapl::fullsimple::fullsimple::{Command, Context};
use tapl::fullsimple::fullsimple::{Binding, BindingType};
use tapl::fullsimple::fullsimple::{ContextError, RunError};
use tapl::fullsimple::fullsimple::{ContextTermResult, ContextTypeResult};
use tapl::fullsimple::fullsimple::{add_name, is_name_bound, name_to_index};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use tapl::fullsimple::ast::{Term, Type};
    use tapl::fullsimple::fullsimple::{Command, Context};
    use tapl::fullsimple::fullsimple::{Binding, BindingType};
    use tapl::fullsimple::fullsimple::{ContextError, RunError};
    use tapl::fullsimple::fullsimple::{ContextTermResult, ContextTypeResult};
    use tapl::fullsimple::fullsimple::{add_name, is_name_bound, name_to_index};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22Bool_22(&'input str),
        Term_22Float_22(&'input str),
        Term_22Nat_22(&'input str),
        Term_22String_22(&'input str),
        Term_22Unit_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22as_22(&'input str),
        Term_22else_22(&'input str),
        Term_22false_22(&'input str),
        Term_22fix_22(&'input str),
        Term_22if_22(&'input str),
        Term_22in_22(&'input str),
        Term_22inert_22(&'input str),
        Term_22iszero_22(&'input str),
        Term_22lambda_22(&'input str),
        Term_22let_22(&'input str),
        Term_22pred_22(&'input str),
        Term_22succ_22(&'input str),
        Term_22then_22(&'input str),
        Term_22timesfloat_22(&'input str),
        Term_22true_22(&'input str),
        Term_22unit_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(&'input str),
        NtATerm(ContextTermResult),
        NtAType(ContextTypeResult),
        NtAppTerm(ContextTermResult),
        NtArrowType(ContextTypeResult),
        NtAscribeTerm(ContextTermResult),
        NtFLOATV(f32),
        NtINTV(i32),
        NtLCID(String),
        NtPathTerm(ContextTermResult),
        NtTerm(ContextTermResult),
        NtTermSeq(ContextTermResult),
        NtType(ContextTypeResult),
        NtUCID(String),
        Nt____Term(ContextTermResult),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 1
        -27, -27, 0, -27, 0, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 27, -27, -27, 0, 0, -27, -27, 0, 0, 0, 0, 0, -27, 0, -27, -27, -27, -27, 0, -27,
        // State 2
        10, -34, 0, 0, 0, -34, 11, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, -34, 12, 0, 0, -34, 15, 0, 0, 0, 0, 0, -34, 0, 22, 23, 24, 25, 0, 26,
        // State 3
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 4
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 5
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 6
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 7
        -17, -17, 0, 29, 0, -17, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, 0, 0, -17, -17, 0, 0, 0, 0, 0, -17, 0, -17, -17, -17, -17, 0, -17,
        // State 8
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 9
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26,
        // State 11
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 12
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 26,
        // State 13
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 26,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26,
        // State 18
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 26,
        // State 19
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 26,
        // State 20
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 26,
        // State 21
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 22
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 23
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 24
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 25
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 26
        46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 27
        -18, -18, 0, 29, 0, -18, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, 0, 0, -18, -18, 0, 0, 0, 0, 0, -18, 0, -18, -18, -18, -18, 0, -18,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 26,
        // State 29
        0, -38, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -19, -19, 0, 29, 0, -19, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, -19, -19, 0, 0, 0, 0, 0, -19, 0, -19, -19, -19, -19, 0, -19,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 35
        -23, -23, 0, 29, 0, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, 0, 0, -23, -23, 0, 0, 0, 0, 0, -23, 0, -23, -23, -23, -23, 0, -23,
        // State 36
        0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -22, -22, 0, 29, 0, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, 0, -22, -22, 0, 0, 0, 0, 0, -22, 0, -22, -22, -22, -22, 0, -22,
        // State 39
        -21, -21, 0, 29, 0, -21, -21, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, 0, -21, -21, 0, 0, 0, 0, 0, -21, 0, -21, -21, -21, -21, 0, -21,
        // State 40
        10, 0, 0, 29, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 26,
        // State 41
        -25, -25, 63, -25, 0, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, -25, -25, -25, -25, 0, 0, -25, -25, 0, 0, 0, 0, 0, -25, 0, -25, -25, -25, -25, 0, -25,
        // State 42
        -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 43
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 44
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 45
        46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 46
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 47
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 48
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 49
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 50
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 51
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 52
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 53
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 54
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 55
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 56
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 57
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 60
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 61
        -20, -20, 0, 29, 0, -20, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, -20, -20, 0, 0, 0, 0, 0, -20, 0, -20, -20, -20, -20, 0, -20,
        // State 62
        46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 63
        0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 68
        0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 71
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 74
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 75
        10, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 0, 21, 22, 23, 24, 25, 0, 26,
        // State 76
        46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0,
        // State 77
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 78
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 79
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 80
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -27,
        -34,
        -33,
        -8,
        -9,
        -6,
        -17,
        -42,
        0,
        0,
        -4,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -3,
        -7,
        -28,
        -29,
        -30,
        0,
        -18,
        0,
        0,
        0,
        0,
        -19,
        0,
        0,
        -23,
        0,
        0,
        -22,
        -21,
        0,
        -25,
        -40,
        -26,
        -11,
        0,
        -12,
        -15,
        -16,
        -13,
        -14,
        -41,
        -32,
        -31,
        0,
        -1,
        0,
        0,
        0,
        0,
        0,
        -20,
        0,
        0,
        -39,
        0,
        0,
        -2,
        0,
        0,
        -24,
        -10,
        0,
        0,
        0,
        0,
        0,
        -35,
        -36,
        -37,
        -5,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 3, 0, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        2, 0, 0, 0, 4, 5, 6, 7, 28, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        2, 0, 3, 0, 4, 5, 6, 7, 8, 30, 31, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        2, 0, 0, 0, 4, 5, 6, 7, 33, 0, 0, 0, 0, 0,
        // State 13
        2, 0, 3, 0, 4, 5, 6, 7, 8, 34, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        2, 0, 0, 0, 4, 5, 6, 7, 36, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0,
        // State 18
        2, 0, 0, 0, 4, 5, 6, 7, 39, 0, 0, 0, 0, 0,
        // State 19
        2, 0, 0, 0, 4, 5, 6, 7, 40, 0, 0, 0, 0, 0,
        // State 20
        2, 0, 0, 0, 4, 5, 6, 7, 41, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 42, 0, 43, 0, 0, 0, 0, 0, 0, 0, 44, 45, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 53, 54, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 42, 0, 43, 0, 0, 0, 0, 0, 0, 0, 59, 45, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        2, 0, 0, 0, 4, 5, 6, 7, 62, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 42, 0, 43, 0, 0, 0, 0, 0, 0, 0, 64, 45, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        2, 0, 3, 0, 4, 5, 6, 7, 8, 30, 65, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        2, 0, 3, 0, 4, 5, 6, 7, 8, 66, 0, 0, 0, 0,
        // State 57
        2, 0, 3, 0, 4, 5, 6, 7, 8, 67, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 42, 0, 43, 0, 0, 0, 0, 0, 0, 0, 69, 45, 0,
        // State 60
        2, 0, 3, 0, 4, 5, 6, 7, 8, 70, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 42, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        2, 0, 3, 0, 4, 5, 6, 7, 8, 78, 0, 0, 0, 0,
        // State 74
        2, 0, 3, 0, 4, 5, 6, 7, 8, 79, 0, 0, 0, 0,
        // State 75
        2, 0, 3, 0, 4, 5, 6, 7, 8, 80, 0, 0, 0, 0,
        // State 76
        0, 42, 0, 43, 0, 0, 0, 0, 0, 0, 0, 81, 45, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""->""###,
            r###"".""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###"">""###,
            r###""Bool""###,
            r###""Float""###,
            r###""Nat""###,
            r###""String""###,
            r###""Unit""###,
            r###""[""###,
            r###""]""###,
            r###""as""###,
            r###""else""###,
            r###""false""###,
            r###""fix""###,
            r###""if""###,
            r###""in""###,
            r###""inert""###,
            r###""iszero""###,
            r###""lambda""###,
            r###""let""###,
            r###""pred""###,
            r###""succ""###,
            r###""then""###,
            r###""timesfloat""###,
            r###""true""###,
            r###""unit""###,
            r###"r#"[0-9]*\\.[0-9]+"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[A-Z][a-zA-Z]*"#"###,
            r###"r#"[a-z][a-z]*"#"###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Term<
        'input,
    >(
        input: &'input str,
    ) -> Result<ContextTermResult, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (11, _) if true => 7,
                (12, _) if true => 8,
                (13, _) if true => 9,
                (14, _) if true => 10,
                (15, _) if true => 11,
                (16, _) if true => 12,
                (17, _) if true => 13,
                (18, _) if true => 14,
                (19, _) if true => 15,
                (20, _) if true => 16,
                (21, _) if true => 17,
                (22, _) if true => 18,
                (23, _) if true => 19,
                (24, _) if true => 20,
                (25, _) if true => 21,
                (26, _) if true => 22,
                (27, _) if true => 23,
                (28, _) if true => 24,
                (29, _) if true => 25,
                (30, _) if true => 26,
                (31, _) if true => 27,
                (32, _) if true => 28,
                (33, _) if true => 29,
                (34, _) if true => 30,
                (35, _) if true => 31,
                (0, _) if true => 32,
                (1, _) if true => 33,
                (2, _) if true => 34,
                (3, _) if true => 35,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 36 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2d_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22Bool_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22Float_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22Nat_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22String_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22Unit_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22as_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22else_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22fix_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22if_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22in_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (26, __tok0) => __Symbol::Term_22inert_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (27, __tok0) => __Symbol::Term_22iszero_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (28, __tok0) => __Symbol::Term_22lambda_22((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (29, __tok0) => __Symbol::Term_22let_22((__tok0)),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            (30, __tok0) => __Symbol::Term_22pred_22((__tok0)),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
                            (31, __tok0) => __Symbol::Term_22succ_22((__tok0)),
                            _ => unreachable!(),
                        },
                        28 => match __lookahead.1 {
                            (32, __tok0) => __Symbol::Term_22then_22((__tok0)),
                            _ => unreachable!(),
                        },
                        29 => match __lookahead.1 {
                            (33, __tok0) => __Symbol::Term_22timesfloat_22((__tok0)),
                            _ => unreachable!(),
                        },
                        30 => match __lookahead.1 {
                            (34, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        31 => match __lookahead.1 {
                            (35, __tok0) => __Symbol::Term_22unit_22((__tok0)),
                            _ => unreachable!(),
                        },
                        32 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        33 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        34 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        35 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ContextTermResult,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ATerm = "(", TermSeq, ")" => ActionFn(29);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTermSeq(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            2 => {
                // ATerm = "inert", "[", Type, "]" => ActionFn(30);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtType(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_Term_22inert_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            3 => {
                // ATerm = "true" => ActionFn(31);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            4 => {
                // ATerm = "false" => ActionFn(32);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            5 => {
                // ATerm = "<", LCID, "=", Term, ">", "as", Type => ActionFn(33);
                let __sym6 = __pop_NtType(__symbols);
                let __sym5 = __pop_Term_22as_22(__symbols);
                let __sym4 = __pop_Term_22_3e_22(__symbols);
                let __sym3 = __pop_NtTerm(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtLCID(__symbols);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            6 => {
                // ATerm = LCID => ActionFn(34);
                let __sym0 = __pop_NtLCID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            7 => {
                // ATerm = "unit" => ActionFn(35);
                let __sym0 = __pop_Term_22unit_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            8 => {
                // ATerm = FLOATV => ActionFn(36);
                let __sym0 = __pop_NtFLOATV(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            9 => {
                // ATerm = INTV => ActionFn(37);
                let __sym0 = __pop_NtINTV(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            10 => {
                // AType = "(", Type, ")" => ActionFn(8);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            11 => {
                // AType = UCID => ActionFn(9);
                let __sym0 = __pop_NtUCID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            12 => {
                // AType = "Bool" => ActionFn(10);
                let __sym0 = __pop_Term_22Bool_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            13 => {
                // AType = "String" => ActionFn(11);
                let __sym0 = __pop_Term_22String_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            14 => {
                // AType = "Unit" => ActionFn(12);
                let __sym0 = __pop_Term_22Unit_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            15 => {
                // AType = "Float" => ActionFn(13);
                let __sym0 = __pop_Term_22Float_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            16 => {
                // AType = "Nat" => ActionFn(14);
                let __sym0 = __pop_Term_22Nat_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAType(__nt), __end));
                1
            }
            17 => {
                // AppTerm = PathTerm => ActionFn(15);
                let __sym0 = __pop_NtPathTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            18 => {
                // AppTerm = AppTerm, PathTerm => ActionFn(16);
                let __sym1 = __pop_NtPathTerm(__symbols);
                let __sym0 = __pop_NtAppTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            19 => {
                // AppTerm = "fix", PathTerm => ActionFn(17);
                let __sym1 = __pop_NtPathTerm(__symbols);
                let __sym0 = __pop_Term_22fix_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            20 => {
                // AppTerm = "timesfloat", PathTerm, PathTerm => ActionFn(18);
                let __sym2 = __pop_NtPathTerm(__symbols);
                let __sym1 = __pop_NtPathTerm(__symbols);
                let __sym0 = __pop_Term_22timesfloat_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            21 => {
                // AppTerm = "succ", PathTerm => ActionFn(19);
                let __sym1 = __pop_NtPathTerm(__symbols);
                let __sym0 = __pop_Term_22succ_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            22 => {
                // AppTerm = "pred", PathTerm => ActionFn(20);
                let __sym1 = __pop_NtPathTerm(__symbols);
                let __sym0 = __pop_Term_22pred_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            23 => {
                // AppTerm = "iszero", PathTerm => ActionFn(21);
                let __sym1 = __pop_NtPathTerm(__symbols);
                let __sym0 = __pop_Term_22iszero_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtAppTerm(__nt), __end));
                2
            }
            24 => {
                // ArrowType = AType, "->", ArrowType => ActionFn(6);
                let __sym2 = __pop_NtArrowType(__symbols);
                let __sym1 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym0 = __pop_NtAType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtArrowType(__nt), __end));
                3
            }
            25 => {
                // ArrowType = AType => ActionFn(7);
                let __sym0 = __pop_NtAType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtArrowType(__nt), __end));
                3
            }
            26 => {
                // AscribeTerm = ATerm, "as", Type => ActionFn(25);
                let __sym2 = __pop_NtType(__symbols);
                let __sym1 = __pop_Term_22as_22(__symbols);
                let __sym0 = __pop_NtATerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAscribeTerm(__nt), __end));
                4
            }
            27 => {
                // AscribeTerm = ATerm => ActionFn(26);
                let __sym0 = __pop_NtATerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAscribeTerm(__nt), __end));
                4
            }
            28 => {
                // FLOATV = r#"[0-9]*\\.[0-9]+"# => ActionFn(41);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFLOATV(__nt), __end));
                5
            }
            29 => {
                // INTV = r#"[0-9]+"# => ActionFn(40);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtINTV(__nt), __end));
                6
            }
            30 => {
                // LCID = r#"[a-z][a-z]*"# => ActionFn(39);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLCID(__nt), __end));
                7
            }
            31 => {
                // PathTerm = PathTerm, ".", LCID => ActionFn(22);
                let __sym2 = __pop_NtLCID(__symbols);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtPathTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPathTerm(__nt), __end));
                8
            }
            32 => {
                // PathTerm = PathTerm, ".", INTV => ActionFn(23);
                let __sym2 = __pop_NtINTV(__symbols);
                let __sym1 = __pop_Term_22_2e_22(__symbols);
                let __sym0 = __pop_NtPathTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPathTerm(__nt), __end));
                8
            }
            33 => {
                // PathTerm = AscribeTerm => ActionFn(24);
                let __sym0 = __pop_NtAscribeTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPathTerm(__nt), __end));
                8
            }
            34 => {
                // Term = AppTerm => ActionFn(1);
                let __sym0 = __pop_NtAppTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                9
            }
            35 => {
                // Term = "if", Term, "then", Term, "else", Term => ActionFn(2);
                let __sym5 = __pop_NtTerm(__symbols);
                let __sym4 = __pop_Term_22else_22(__symbols);
                let __sym3 = __pop_NtTerm(__symbols);
                let __sym2 = __pop_Term_22then_22(__symbols);
                let __sym1 = __pop_NtTerm(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                9
            }
            36 => {
                // Term = "lambda", LCID, ":", Type, ".", Term => ActionFn(3);
                let __sym5 = __pop_NtTerm(__symbols);
                let __sym4 = __pop_Term_22_2e_22(__symbols);
                let __sym3 = __pop_NtType(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtLCID(__symbols);
                let __sym0 = __pop_Term_22lambda_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                9
            }
            37 => {
                // Term = "let", LCID, "=", Term, "in", Term => ActionFn(4);
                let __sym5 = __pop_NtTerm(__symbols);
                let __sym4 = __pop_Term_22in_22(__symbols);
                let __sym3 = __pop_NtTerm(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtLCID(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                9
            }
            38 => {
                // TermSeq = Term => ActionFn(27);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTermSeq(__nt), __end));
                10
            }
            39 => {
                // TermSeq = Term, ";", TermSeq => ActionFn(28);
                let __sym2 = __pop_NtTermSeq(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTermSeq(__nt), __end));
                10
            }
            40 => {
                // Type = ArrowType => ActionFn(5);
                let __sym0 = __pop_NtArrowType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                11
            }
            41 => {
                // UCID = r#"[A-Z][a-zA-Z]*"# => ActionFn(38);
                let __sym0 = __pop_Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUCID(__nt), __end));
                12
            }
            42 => {
                // __Term = Term => ActionFn(0);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Bool_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Bool_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Float_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Float_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Nat_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Nat_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22String_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22String_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Unit_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Unit_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22as_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22as_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22else_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22else_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22false_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22false_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fix_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fix_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22if_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22if_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22in_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22in_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22inert_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22inert_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22iszero_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22iszero_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22lambda_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22lambda_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22let_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22let_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22pred_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22pred_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22succ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22succ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22then_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22then_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timesfloat_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timesfloat_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22true_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22true_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22unit_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22unit_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2a_5c_5c_2e_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtATerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtATerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTypeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAppTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAppTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArrowType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTypeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArrowType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAscribeTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAscribeTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFLOATV<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFLOATV(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtINTV<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtINTV(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLCID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLCID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPathTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPathTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTermSeq<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTermSeq(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTypeResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtUCID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtUCID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ContextTermResult, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Term::parse_Term;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use tapl::fullsimple::ast::{Term, Type};
    use tapl::fullsimple::fullsimple::{Command, Context};
    use tapl::fullsimple::fullsimple::{Binding, BindingType};
    use tapl::fullsimple::fullsimple::{ContextError, RunError};
    use tapl::fullsimple::fullsimple::{ContextTermResult, ContextTypeResult};
    use tapl::fullsimple::fullsimple::{add_name, is_name_bound, name_to_index};
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[0-9])*(?u:\\.)(?u:[0-9])+",
                "^(?u:[0-9])+",
                "^(?u:[A-Z])(?u:[A-Za-z])*",
                "^(?u:[a-z])(?u:[a-z])*",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\->)",
                "^(?u:\\.)",
                "^(?u::)",
                "^(?u:;)",
                "^(?u:<)",
                "^(?u:=)",
                "^(?u:>)",
                "^(?u:Bool)",
                "^(?u:Float)",
                "^(?u:Nat)",
                "^(?u:String)",
                "^(?u:Unit)",
                "^(?u:\\[)",
                "^(?u:\\])",
                "^(?u:as)",
                "^(?u:else)",
                "^(?u:false)",
                "^(?u:fix)",
                "^(?u:if)",
                "^(?u:in)",
                "^(?u:inert)",
                "^(?u:iszero)",
                "^(?u:lambda)",
                "^(?u:let)",
                "^(?u:pred)",
                "^(?u:succ)",
                "^(?u:then)",
                "^(?u:timesfloat)",
                "^(?u:true)",
                "^(?u:unit)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])*(?u:\\.)(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Z])(?u:[A-Za-z])*").unwrap(),
                __regex::Regex::new("^(?u:[a-z])(?u:[a-z])*").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\->)").unwrap(),
                __regex::Regex::new("^(?u:\\.)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:<)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:Bool)").unwrap(),
                __regex::Regex::new("^(?u:Float)").unwrap(),
                __regex::Regex::new("^(?u:Nat)").unwrap(),
                __regex::Regex::new("^(?u:String)").unwrap(),
                __regex::Regex::new("^(?u:Unit)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:as)").unwrap(),
                __regex::Regex::new("^(?u:else)").unwrap(),
                __regex::Regex::new("^(?u:false)").unwrap(),
                __regex::Regex::new("^(?u:fix)").unwrap(),
                __regex::Regex::new("^(?u:if)").unwrap(),
                __regex::Regex::new("^(?u:in)").unwrap(),
                __regex::Regex::new("^(?u:inert)").unwrap(),
                __regex::Regex::new("^(?u:iszero)").unwrap(),
                __regex::Regex::new("^(?u:lambda)").unwrap(),
                __regex::Regex::new("^(?u:let)").unwrap(),
                __regex::Regex::new("^(?u:pred)").unwrap(),
                __regex::Regex::new("^(?u:succ)").unwrap(),
                __regex::Regex::new("^(?u:then)").unwrap(),
                __regex::Regex::new("^(?u:timesfloat)").unwrap(),
                __regex::Regex::new("^(?u:true)").unwrap(),
                __regex::Regex::new("^(?u:unit)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 36 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t1, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t2, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t3, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t1(ctx.clone()).and_then(|v1| {
            t2(ctx.clone()).and_then(|v2| {
                t2(ctx).and_then(|v3| {
                    Ok(Term::If(Box::new(v1), Box::new(v2), Box::new(v3)))
                })
            })
        })
    })
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, ContextTypeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        let nctx = add_name(ctx.clone(), v.clone());
        t(nctx).and_then(|t2| {
            match ty(ctx) {
                Ok(ty1) => Ok(Term::Abs(v.clone(), ty1, Box::new(t2))),
                Err(e) => Err(RunError::ContextError(e)),
            }
        })
    })
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t1, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t2, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        let nctx = add_name(ctx.clone(), n.clone());
        t1(ctx).and_then(|v1| {
            t2(nctx).and_then(|v2| {
                Ok(Term::Let(n.clone(), Box::new(v1), Box::new(v2)))
            })
        })
    })
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTypeResult, usize),
) -> ContextTypeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, t1, _): (usize, ContextTypeResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t2, _): (usize, ContextTypeResult, usize),
) -> ContextTypeResult
{
    Box::new(move |ctx: Context| -> Result<Type, ContextError> {
        t1(ctx.clone()).and_then(|v1| {
            t2(ctx.clone()).and_then(|v2| {
                Ok(Type::Arrow(Box::new(v1), Box::new(v2)))
            })
        })
    })
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTypeResult, usize),
) -> ContextTypeResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTypeResult, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ContextTypeResult
{
    t
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, u, _): (usize, String, usize),
) -> ContextTypeResult
{
    Box::new(move |ctx: Context| -> Result<Type, ContextError> { 
        let b = Binding{label: u.clone(), binding: BindingType::NameBind};
        if is_name_bound(ctx.clone(), &b) {
            name_to_index(ctx.clone(), u.clone().as_str()).and_then(|i| {
                Ok(Type::Var(i, ctx.len()))
            })
        } else {
            Ok(Type::Id(u.clone()))
        }
    })
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTypeResult
{
    Box::new(move |_: Context| -> Result<Type, ContextError> {
        Ok(Type::Bool)
    })
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTypeResult
{
    Box::new(move |_: Context| -> Result<Type, ContextError> {
        Ok(Type::Str)
    })
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTypeResult
{
    Box::new(move |_: Context| -> Result<Type, ContextError> {
        Ok(Type::Unit)
    })
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTypeResult
{
    Box::new(move |_: Context| -> Result<Type, ContextError> {
        Ok(Type::Float)
    })
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTypeResult
{
    Box::new(move |_: Context| -> Result<Type, ContextError> {
        Ok(Type::Nat)
    })
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, e1, _): (usize, ContextTermResult, usize),
    (_, e2, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        e1(ctx.clone()).and_then(|e1v| {
            e2(ctx).and_then(|e2v| {
                Ok(Term::App(Box::new(e1v), Box::new(e2v)))
            })
        })
    })
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx).and_then(|t2| {
            Ok(Term::Fix(Box::new(t2)))
        })
    })
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t1, _): (usize, ContextTermResult, usize),
    (_, t2, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t1(ctx.clone()).and_then(|v1| {
            t2(ctx).and_then(|v2| {
                Ok(Term::TimesFloat(Box::new(v1), Box::new(v2)))
            })
        })
    })
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx).and_then(|v| {
            Ok(Term::Succ(Box::new(v)))
        })
    })
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx).and_then(|v| {
            Ok(Term::Pred(Box::new(v)))
        })
    })
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx).and_then(|v| {
            Ok(Term::IsZero(Box::new(v)))
        })
    })
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, String, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx).and_then(|t2| Ok(Term::Projection(Box::new(t2), v.clone())))
    })
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, i32, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx).and_then(|t2| Ok(Term::Projection(Box::new(t2), i.to_string())))
    })
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, ContextTypeResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx.clone()).and_then(|t2| {
            match ty(ctx) {
                Ok(ty1) => Ok(Term::Ascribe(Box::new(t2), ty1)),
                Err(e) => Err(RunError::ContextError(e)),
            }
         })
    })
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, t1, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ts, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        let u = String::from("_");

        t1(ctx.clone()).and_then(|et1| {
            let nctx = add_name(ctx.clone(), u.clone());
            ts(nctx).and_then(|ets| {
                Ok(Term::App(Box::new(Term::Abs(u, Type::Unit, Box::new(ets))), Box::new(et1)))
            })
        })
    })
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ts, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ContextTermResult
{
    ts
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTypeResult, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        match t(ctx) {
            Ok(t1) => Ok(Term::Inert(t1)),
            Err(e) => Err(RunError::ContextError(e)),
        }
    })
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTermResult
{
    Box::new(move |_: Context| -> Result<Term, RunError> {
        Ok(Term::True)
    })
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTermResult
{
    Box::new(move |_: Context| -> Result<Term, RunError> {
        Ok(Term::False)
    })
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, ContextTypeResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        t(ctx.clone()).and_then(|t1| {
            match ty(ctx) {
                Ok(ty1) => Ok(Term::Tag(v.clone(), Box::new(t1), ty1)),
                Err(e) => Err(RunError::ContextError(e)),
            }
        })
    })
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, String, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        match name_to_index(ctx.clone(), &v) {
            Ok(i) => Ok(Term::Var(i, ctx.len())),
            Err(e) => Err(RunError::ContextError(e)),
        }
    })
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ContextTermResult
{
    Box::new(move |_: Context| -> Result<Term, RunError> {
        Ok(Term::Unit)
    })
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, f, _): (usize, f32, usize),
) -> ContextTermResult
{
    Box::new(move |_: Context| -> Result<Term, RunError> {
        Ok(Term::Float(f))
    })
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, i32, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        fn f(n: i32) -> Term {
            match n {
                0 => Term::Zero,
                n => Term::Succ(Box::new(f(n-1))),
            }
        }
        Ok(f(i))
    })
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> f32
{
    f32::from_str(__0).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
