use std::str::FromStr;
use tapl::fullsimple::ast::{Term, Type};
use tapl::fullsimple::fullsimple::{Command, Context, ContextTermResult, RunError};
use tapl::fullsimple::fullsimple::{add_name, name_to_index};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use tapl::fullsimple::ast::{Term, Type};
    use tapl::fullsimple::fullsimple::{Command, Context, ContextTermResult, RunError};
    use tapl::fullsimple::fullsimple::{add_name, name_to_index};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_29_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22Bool_22(&'input str),
        Term_22Nat_22(&'input str),
        Term_22else_22(&'input str),
        Term_22false_22(&'input str),
        Term_22if_22(&'input str),
        Term_22iszero_28_22(&'input str),
        Term_22lambda_22(&'input str),
        Term_22pred_28_22(&'input str),
        Term_22succ_28_22(&'input str),
        Term_22then_22(&'input str),
        Term_22true_22(&'input str),
        Term_22zero_22(&'input str),
        Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(&'input str),
        NtATerm(ContextTermResult),
        NtLCID(String),
        NtTerm(ContextTermResult),
        NtTerm2(Box<Term>),
        NtType(Type),
        NtUCID(String),
        Nt____Term(ContextTermResult),
        Nt____Term2(Box<Term>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 0, 0, 7, 0, 0, 8,
        // State 1
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 2
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 3
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 4
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
        // State 6
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 7
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 8
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 12
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 13
        0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 0, 0, 7, 0, 0, 8,
        // State 14
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -6,
        -3,
        -17,
        -2,
        0,
        -1,
        -4,
        0,
        0,
        0,
        -15,
        -14,
        0,
        -5,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 3, 4, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 9, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 11, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        2, 3, 15, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"")""###,
            r###"".""###,
            r###"":""###,
            r###""Bool""###,
            r###""Nat""###,
            r###""else""###,
            r###""false""###,
            r###""if""###,
            r###""iszero(""###,
            r###""lambda""###,
            r###""pred(""###,
            r###""succ(""###,
            r###""then""###,
            r###""true""###,
            r###""zero""###,
            r###"r#"[A-Z][a-zA-Z]*"#"###,
            r###"r#"[a-z][a-z]*"#"###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (15, _) if true => 13,
                (16, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
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
                let __action = __ACTION[__state * 17 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22Bool_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22Nat_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22else_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22if_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22iszero_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22lambda_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22pred_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22succ_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22then_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22zero_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23((__tok0)),
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
                // ATerm = "true" => ActionFn(15);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            2 => {
                // ATerm = "false" => ActionFn(16);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            3 => {
                // ATerm = LCID => ActionFn(17);
                let __sym0 = __pop_NtLCID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            4 => {
                // LCID = r#"[a-z][a-z]*"# => ActionFn(14);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLCID(__nt), __end));
                1
            }
            5 => {
                // Term = "lambda", LCID, ":", Type, ".", Term => ActionFn(9);
                let __sym5 = __pop_NtTerm(__symbols);
                let __sym4 = __pop_Term_22_2e_22(__symbols);
                let __sym3 = __pop_NtType(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtLCID(__symbols);
                let __sym0 = __pop_Term_22lambda_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            6 => {
                // Term = ATerm => ActionFn(10);
                let __sym0 = __pop_NtATerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            7 => {
                // Term2 = "true" => ActionFn(2);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            8 => {
                // Term2 = "false" => ActionFn(3);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            9 => {
                // Term2 = "zero" => ActionFn(4);
                let __sym0 = __pop_Term_22zero_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            10 => {
                // Term2 = "succ(", Term2, ")" => ActionFn(5);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22succ_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            11 => {
                // Term2 = "pred(", Term2, ")" => ActionFn(6);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22pred_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            12 => {
                // Term2 = "iszero(", Term2, ")" => ActionFn(7);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22iszero_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            13 => {
                // Term2 = "if", Term2, "then", Term2, "else", Term2 => ActionFn(8);
                let __sym5 = __pop_NtTerm2(__symbols);
                let __sym4 = __pop_Term_22else_22(__symbols);
                let __sym3 = __pop_NtTerm2(__symbols);
                let __sym2 = __pop_Term_22then_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            14 => {
                // Type = "Nat" => ActionFn(11);
                let __sym0 = __pop_Term_22Nat_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                4
            }
            15 => {
                // Type = "Bool" => ActionFn(12);
                let __sym0 = __pop_Term_22Bool_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                4
            }
            16 => {
                // UCID = r#"[A-Z][a-zA-Z]*"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUCID(__nt), __end));
                5
            }
            17 => {
                // __Term = Term => ActionFn(1);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            18 => {
                // __Term2 = Term2 => ActionFn(0);
                let __sym0 = __pop_NtTerm2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term2(__nt), __end));
                7
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 8 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Term_22iszero_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22iszero_28_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22pred_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22pred_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22succ_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22succ_28_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22zero_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22zero_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtTerm2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
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
    fn __pop_Nt____Term2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Term::parse_Term;

mod __parse__Term2 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use tapl::fullsimple::ast::{Term, Type};
    use tapl::fullsimple::fullsimple::{Command, Context, ContextTermResult, RunError};
    use tapl::fullsimple::fullsimple::{add_name, name_to_index};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_29_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22Bool_22(&'input str),
        Term_22Nat_22(&'input str),
        Term_22else_22(&'input str),
        Term_22false_22(&'input str),
        Term_22if_22(&'input str),
        Term_22iszero_28_22(&'input str),
        Term_22lambda_22(&'input str),
        Term_22pred_28_22(&'input str),
        Term_22succ_28_22(&'input str),
        Term_22then_22(&'input str),
        Term_22true_22(&'input str),
        Term_22zero_22(&'input str),
        Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(&'input str),
        NtATerm(ContextTermResult),
        NtLCID(String),
        NtTerm(ContextTermResult),
        NtTerm2(Box<Term>),
        NtType(Type),
        NtUCID(String),
        Nt____Term(ContextTermResult),
        Nt____Term2(Box<Term>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 1
        -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 2
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 3
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 7
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 8
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0,
        // State 10
        15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 14
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 15
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 16
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 17
        0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 6, 7, 0, 8, 9, 0, 0,
        // State 19
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -18,
        -8,
        0,
        0,
        0,
        0,
        -7,
        -9,
        0,
        0,
        0,
        0,
        0,
        -12,
        -11,
        -10,
        0,
        0,
        -13,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 2, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 10, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 11, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 12, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 13, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 18, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 20, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"")""###,
            r###"".""###,
            r###"":""###,
            r###""Bool""###,
            r###""Nat""###,
            r###""else""###,
            r###""false""###,
            r###""if""###,
            r###""iszero(""###,
            r###""lambda""###,
            r###""pred(""###,
            r###""succ(""###,
            r###""then""###,
            r###""true""###,
            r###""zero""###,
            r###"r#"[A-Z][a-zA-Z]*"#"###,
            r###"r#"[a-z][a-z]*"#"###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Term2<
        'input,
    >(
        input: &'input str,
    ) -> Result<Box<Term>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (15, _) if true => 13,
                (16, _) if true => 14,
                (0, _) if true => 15,
                (1, _) if true => 16,
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
                let __action = __ACTION[__state * 17 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22Bool_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22Nat_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22else_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22if_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22iszero_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22lambda_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22pred_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22succ_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22then_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22zero_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23((__tok0)),
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
    ) -> Option<Result<Box<Term>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ATerm = "true" => ActionFn(15);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            2 => {
                // ATerm = "false" => ActionFn(16);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            3 => {
                // ATerm = LCID => ActionFn(17);
                let __sym0 = __pop_NtLCID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtATerm(__nt), __end));
                0
            }
            4 => {
                // LCID = r#"[a-z][a-z]*"# => ActionFn(14);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_5ba_2dz_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLCID(__nt), __end));
                1
            }
            5 => {
                // Term = "lambda", LCID, ":", Type, ".", Term => ActionFn(9);
                let __sym5 = __pop_NtTerm(__symbols);
                let __sym4 = __pop_Term_22_2e_22(__symbols);
                let __sym3 = __pop_NtType(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtLCID(__symbols);
                let __sym0 = __pop_Term_22lambda_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            6 => {
                // Term = ATerm => ActionFn(10);
                let __sym0 = __pop_NtATerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                2
            }
            7 => {
                // Term2 = "true" => ActionFn(2);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            8 => {
                // Term2 = "false" => ActionFn(3);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            9 => {
                // Term2 = "zero" => ActionFn(4);
                let __sym0 = __pop_Term_22zero_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            10 => {
                // Term2 = "succ(", Term2, ")" => ActionFn(5);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22succ_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            11 => {
                // Term2 = "pred(", Term2, ")" => ActionFn(6);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22pred_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            12 => {
                // Term2 = "iszero(", Term2, ")" => ActionFn(7);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22iszero_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            13 => {
                // Term2 = "if", Term2, "then", Term2, "else", Term2 => ActionFn(8);
                let __sym5 = __pop_NtTerm2(__symbols);
                let __sym4 = __pop_Term_22else_22(__symbols);
                let __sym3 = __pop_NtTerm2(__symbols);
                let __sym2 = __pop_Term_22then_22(__symbols);
                let __sym1 = __pop_NtTerm2(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTerm2(__nt), __end));
                3
            }
            14 => {
                // Type = "Nat" => ActionFn(11);
                let __sym0 = __pop_Term_22Nat_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                4
            }
            15 => {
                // Type = "Bool" => ActionFn(12);
                let __sym0 = __pop_Term_22Bool_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                4
            }
            16 => {
                // UCID = r#"[A-Z][a-zA-Z]*"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_5bA_2dZ_5d_5ba_2dzA_2dZ_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUCID(__nt), __end));
                5
            }
            17 => {
                // __Term = Term => ActionFn(1);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                6
            }
            18 => {
                // __Term2 = Term2 => ActionFn(0);
                let __sym0 = __pop_NtTerm2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 8 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Term_22iszero_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22iszero_28_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22pred_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22pred_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22succ_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22succ_28_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22zero_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22zero_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtTerm2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
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
    fn __pop_Nt____Term2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Term>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Term2::parse_Term2;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use tapl::fullsimple::ast::{Term, Type};
    use tapl::fullsimple::fullsimple::{Command, Context, ContextTermResult, RunError};
    use tapl::fullsimple::fullsimple::{add_name, name_to_index};
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
                "^(?u:[A-Z])(?u:[A-Za-z])*",
                "^(?u:[a-z])(?u:[a-z])*",
                "^(?u:\\))",
                "^(?u:\\.)",
                "^(?u::)",
                "^(?u:Bool)",
                "^(?u:Nat)",
                "^(?u:else)",
                "^(?u:false)",
                "^(?u:if)",
                "^(?u:iszero\\()",
                "^(?u:lambda)",
                "^(?u:pred\\()",
                "^(?u:succ\\()",
                "^(?u:then)",
                "^(?u:true)",
                "^(?u:zero)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[A-Z])(?u:[A-Za-z])*").unwrap(),
                __regex::Regex::new("^(?u:[a-z])(?u:[a-z])*").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\.)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:Bool)").unwrap(),
                __regex::Regex::new("^(?u:Nat)").unwrap(),
                __regex::Regex::new("^(?u:else)").unwrap(),
                __regex::Regex::new("^(?u:false)").unwrap(),
                __regex::Regex::new("^(?u:if)").unwrap(),
                __regex::Regex::new("^(?u:iszero\\()").unwrap(),
                __regex::Regex::new("^(?u:lambda)").unwrap(),
                __regex::Regex::new("^(?u:pred\\()").unwrap(),
                __regex::Regex::new("^(?u:succ\\()").unwrap(),
                __regex::Regex::new("^(?u:then)").unwrap(),
                __regex::Regex::new("^(?u:true)").unwrap(),
                __regex::Regex::new("^(?u:zero)").unwrap(),
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
                    for __i in 0 .. 17 {
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
    (_, __0, _): (usize, Box<Term>, usize),
) -> Box<Term>
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
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Term>
{
    Box::new(Term::True)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Term>
{
    Box::new(Term::False)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Term>
{
    Box::new(Term::Zero)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Term>
{
    Box::new(Term::Succ(__0))
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Term>
{
    Box::new(Term::Pred(__0))
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Term>
{
    Box::new(Term::IsZero(__0))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Box<Term>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __2, _): (usize, Box<Term>, usize),
) -> Box<Term>
{
    Box::new(Term::If(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Type, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    Box::new(move |ctx: Context| -> Result<Term, RunError> {
        let nctx = add_name(ctx, v.clone());
        t(nctx).and_then(|t2| Ok(Term::Abs(v.clone(), ty.clone(), Box::new(t2))))
    })
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ContextTermResult, usize),
) -> ContextTermResult
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::Nat
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::Bool
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action15<
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
fn __action16<
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
fn __action17<
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
