use std::str::FromStr;
use opcode;
use std::rc::Rc;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Num {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use opcode;
    use std::rc::Rc;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_20_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(&'input str),
        Nt_28_3cSExpression_3e_20_22_20_22_29(opcode::SExpression),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2a(::std::vec::Vec<opcode::SExpression>),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2b(::std::vec::Vec<opcode::SExpression>),
        NtNum(f64),
        NtOpCode(opcode::OpCode),
        NtSExpression(opcode::SExpression),
        NtSExpression_3f(::std::option::Option<opcode::SExpression>),
        NtSpace_3cSExpression_3e(Vec<opcode::SExpression>),
        NtValues(Vec<opcode::SExpression>),
        Nt____Num(f64),
        Nt____OpCode(opcode::OpCode),
        Nt____SExpression(opcode::SExpression),
        Nt____Values(Vec<opcode::SExpression>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        3, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 2
        // State 1
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 2
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -20, // on EOF, reduce `__Num = Num => ActionFn(2);`
        -6, // on EOF, reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        2, // on Num, goto 1
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 1
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 2
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
    ];
    pub fn parse_Num<
        'input,
    >(
        input: &'input str,
    ) -> Result<f64, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_20_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__tok0),
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
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
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
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
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
    ) -> Option<Result<f64,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<SExpression> " ") = SExpression, " " => ActionFn(17);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__nt), __end));
                0
            }
            2 => {
                // (<SExpression> " ")* =  => ActionFn(15);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action15::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<SExpression> " ")* = (<SExpression> " ")+ => ActionFn(16);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<SExpression> " ")+ = SExpression, " " => ActionFn(20);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);
                let __sym2 = __pop_Term_22_20_22(__symbols);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                3
            }
            7 => {
                // OpCode = "+" => ActionFn(8);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            8 => {
                // OpCode = "-" => ActionFn(9);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            9 => {
                // OpCode = "*" => ActionFn(10);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            10 => {
                // OpCode = "/" => ActionFn(11);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            11 => {
                // SExpression = Num => ActionFn(4);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            12 => {
                // SExpression = "(", OpCode, Values, ")" => ActionFn(5);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtValues(__symbols);
                let __sym1 = __pop_NtOpCode(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            13 => {
                // SExpression? = SExpression => ActionFn(13);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            14 => {
                // SExpression? =  => ActionFn(14);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action14::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            15 => {
                // Space<SExpression> = SExpression => ActionFn(24);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            16 => {
                // Space<SExpression> =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            17 => {
                // Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            18 => {
                // Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            19 => {
                // Values = Space<SExpression> => ActionFn(6);
                let __sym0 = __pop_NtSpace_3cSExpression_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValues(__nt), __end));
                8
            }
            20 => {
                // __Num = Num => ActionFn(2);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            21 => {
                // __OpCode = OpCode => ActionFn(3);
                let __sym0 = __pop_NtOpCode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____OpCode(__nt), __end));
                10
            }
            22 => {
                // __SExpression = SExpression => ActionFn(0);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SExpression(__nt), __end));
                11
            }
            23 => {
                // __Values = Values => ActionFn(1);
                let __sym0 = __pop_NtValues(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Values(__nt), __end));
                12
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_20_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_20_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSpace_3cSExpression_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSpace_3cSExpression_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValues<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValues(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____OpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____OpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Values<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Values(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Num::parse_Num;

mod __parse__OpCode {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use opcode;
    use std::rc::Rc;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_20_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(&'input str),
        Nt_28_3cSExpression_3e_20_22_20_22_29(opcode::SExpression),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2a(::std::vec::Vec<opcode::SExpression>),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2b(::std::vec::Vec<opcode::SExpression>),
        NtNum(f64),
        NtOpCode(opcode::OpCode),
        NtSExpression(opcode::SExpression),
        NtSExpression_3f(::std::option::Option<opcode::SExpression>),
        NtSpace_3cSExpression_3e(Vec<opcode::SExpression>),
        NtValues(Vec<opcode::SExpression>),
        Nt____Num(f64),
        Nt____OpCode(opcode::OpCode),
        Nt____SExpression(opcode::SExpression),
        Nt____Values(Vec<opcode::SExpression>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        3, // on "*", goto 2
        4, // on "+", goto 3
        5, // on "-", goto 4
        6, // on "/", goto 5
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 1
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 2
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 3
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 4
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 5
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -21, // on EOF, reduce `__OpCode = OpCode => ActionFn(3);`
        -9, // on EOF, reduce `OpCode = "*" => ActionFn(10);`
        -7, // on EOF, reduce `OpCode = "+" => ActionFn(8);`
        -8, // on EOF, reduce `OpCode = "-" => ActionFn(9);`
        -10, // on EOF, reduce `OpCode = "/" => ActionFn(11);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        2, // on OpCode, goto 1
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 1
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 2
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 3
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 4
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 5
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
    ];
    pub fn parse_OpCode<
        'input,
    >(
        input: &'input str,
    ) -> Result<opcode::OpCode, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_20_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__tok0),
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
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
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
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
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
    ) -> Option<Result<opcode::OpCode,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<SExpression> " ") = SExpression, " " => ActionFn(17);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__nt), __end));
                0
            }
            2 => {
                // (<SExpression> " ")* =  => ActionFn(15);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action15::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<SExpression> " ")* = (<SExpression> " ")+ => ActionFn(16);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<SExpression> " ")+ = SExpression, " " => ActionFn(20);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);
                let __sym2 = __pop_Term_22_20_22(__symbols);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                3
            }
            7 => {
                // OpCode = "+" => ActionFn(8);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            8 => {
                // OpCode = "-" => ActionFn(9);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            9 => {
                // OpCode = "*" => ActionFn(10);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            10 => {
                // OpCode = "/" => ActionFn(11);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            11 => {
                // SExpression = Num => ActionFn(4);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            12 => {
                // SExpression = "(", OpCode, Values, ")" => ActionFn(5);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtValues(__symbols);
                let __sym1 = __pop_NtOpCode(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            13 => {
                // SExpression? = SExpression => ActionFn(13);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            14 => {
                // SExpression? =  => ActionFn(14);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action14::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            15 => {
                // Space<SExpression> = SExpression => ActionFn(24);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            16 => {
                // Space<SExpression> =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            17 => {
                // Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            18 => {
                // Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            19 => {
                // Values = Space<SExpression> => ActionFn(6);
                let __sym0 = __pop_NtSpace_3cSExpression_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValues(__nt), __end));
                8
            }
            20 => {
                // __Num = Num => ActionFn(2);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                9
            }
            21 => {
                // __OpCode = OpCode => ActionFn(3);
                let __sym0 = __pop_NtOpCode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            22 => {
                // __SExpression = SExpression => ActionFn(0);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SExpression(__nt), __end));
                11
            }
            23 => {
                // __Values = Values => ActionFn(1);
                let __sym0 = __pop_NtValues(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Values(__nt), __end));
                12
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_20_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_20_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSpace_3cSExpression_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSpace_3cSExpression_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValues<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValues(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____OpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____OpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Values<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Values(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__OpCode::parse_OpCode;

mod __parse__SExpression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use opcode;
    use std::rc::Rc;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_20_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(&'input str),
        Nt_28_3cSExpression_3e_20_22_20_22_29(opcode::SExpression),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2a(::std::vec::Vec<opcode::SExpression>),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2b(::std::vec::Vec<opcode::SExpression>),
        NtNum(f64),
        NtOpCode(opcode::OpCode),
        NtSExpression(opcode::SExpression),
        NtSExpression_3f(::std::option::Option<opcode::SExpression>),
        NtSpace_3cSExpression_3e(Vec<opcode::SExpression>),
        NtValues(Vec<opcode::SExpression>),
        Nt____Num(f64),
        Nt____OpCode(opcode::OpCode),
        Nt____SExpression(opcode::SExpression),
        Nt____Values(Vec<opcode::SExpression>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on " ", error
        4, // on "(", goto 3
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        5, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 4
        // State 1
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 2
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 3
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        7, // on "*", goto 6
        8, // on "+", goto 7
        9, // on "-", goto 8
        10, // on "/", goto 9
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 4
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 5
        0, // on " ", error
        16, // on "(", goto 15
        -16, // on ")", reduce `Space<SExpression> =  => ActionFn(25);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        17, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 16
        // State 6
        0, // on " ", error
        -9, // on "(", reduce `OpCode = "*" => ActionFn(10);`
        -9, // on ")", reduce `OpCode = "*" => ActionFn(10);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -9, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "*" => ActionFn(10);`
        // State 7
        0, // on " ", error
        -7, // on "(", reduce `OpCode = "+" => ActionFn(8);`
        -7, // on ")", reduce `OpCode = "+" => ActionFn(8);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -7, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "+" => ActionFn(8);`
        // State 8
        0, // on " ", error
        -8, // on "(", reduce `OpCode = "-" => ActionFn(9);`
        -8, // on ")", reduce `OpCode = "-" => ActionFn(9);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -8, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "-" => ActionFn(9);`
        // State 9
        0, // on " ", error
        -10, // on "(", reduce `OpCode = "/" => ActionFn(11);`
        -10, // on ")", reduce `OpCode = "/" => ActionFn(11);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -10, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "/" => ActionFn(11);`
        // State 10
        0, // on " ", error
        16, // on "(", goto 15
        -18, // on ")", reduce `Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        17, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 16
        // State 11
        -11, // on " ", reduce `SExpression = Num => ActionFn(4);`
        0, // on "(", error
        -11, // on ")", reduce `SExpression = Num => ActionFn(4);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 12
        19, // on " ", goto 18
        0, // on "(", error
        -15, // on ")", reduce `Space<SExpression> = SExpression => ActionFn(24);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 13
        0, // on " ", error
        0, // on "(", error
        -19, // on ")", reduce `Values = Space<SExpression> => ActionFn(6);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 14
        0, // on " ", error
        0, // on "(", error
        20, // on ")", goto 19
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 15
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        7, // on "*", goto 6
        8, // on "+", goto 7
        9, // on "-", goto 8
        10, // on "/", goto 9
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 16
        -6, // on " ", reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        0, // on "(", error
        -6, // on ")", reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 17
        22, // on " ", goto 21
        0, // on "(", error
        -17, // on ")", reduce `Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 18
        0, // on " ", error
        -4, // on "(", reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        -4, // on ")", reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        // State 19
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 20
        0, // on " ", error
        16, // on "(", goto 15
        -16, // on ")", reduce `Space<SExpression> =  => ActionFn(25);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        17, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 16
        // State 21
        0, // on " ", error
        -5, // on "(", reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        -5, // on ")", reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        // State 22
        0, // on " ", error
        0, // on "(", error
        24, // on ")", goto 23
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 23
        -12, // on " ", reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on "(", error
        -12, // on ")", reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -11, // on EOF, reduce `SExpression = Num => ActionFn(4);`
        -22, // on EOF, reduce `__SExpression = SExpression => ActionFn(0);`
        0, // on EOF, error
        -6, // on EOF, reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -12, // on EOF, reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        2, // on Num, goto 1
        0, // on OpCode, error
        3, // on SExpression, goto 2
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 1
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 2
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 3
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        6, // on OpCode, goto 5
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 4
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 5
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        11, // on (<SExpression> " ")+, goto 10
        12, // on Num, goto 11
        0, // on OpCode, error
        13, // on SExpression, goto 12
        0, // on SExpression?, error
        14, // on Space<SExpression>, goto 13
        15, // on Values, goto 14
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 6
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 7
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 8
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 9
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 10
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        12, // on Num, goto 11
        0, // on OpCode, error
        18, // on SExpression, goto 17
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 11
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 12
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 13
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 14
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 15
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        21, // on OpCode, goto 20
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 16
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 17
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 18
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 19
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 20
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        11, // on (<SExpression> " ")+, goto 10
        12, // on Num, goto 11
        0, // on OpCode, error
        13, // on SExpression, goto 12
        0, // on SExpression?, error
        14, // on Space<SExpression>, goto 13
        23, // on Values, goto 22
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 21
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 22
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 23
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
    ];
    pub fn parse_SExpression<
        'input,
    >(
        input: &'input str,
    ) -> Result<opcode::SExpression, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_20_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__tok0),
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
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
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
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
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
    ) -> Option<Result<opcode::SExpression,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<SExpression> " ") = SExpression, " " => ActionFn(17);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__nt), __end));
                0
            }
            2 => {
                // (<SExpression> " ")* =  => ActionFn(15);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action15::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<SExpression> " ")* = (<SExpression> " ")+ => ActionFn(16);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<SExpression> " ")+ = SExpression, " " => ActionFn(20);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);
                let __sym2 = __pop_Term_22_20_22(__symbols);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                3
            }
            7 => {
                // OpCode = "+" => ActionFn(8);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            8 => {
                // OpCode = "-" => ActionFn(9);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            9 => {
                // OpCode = "*" => ActionFn(10);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            10 => {
                // OpCode = "/" => ActionFn(11);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            11 => {
                // SExpression = Num => ActionFn(4);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            12 => {
                // SExpression = "(", OpCode, Values, ")" => ActionFn(5);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtValues(__symbols);
                let __sym1 = __pop_NtOpCode(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            13 => {
                // SExpression? = SExpression => ActionFn(13);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            14 => {
                // SExpression? =  => ActionFn(14);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action14::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            15 => {
                // Space<SExpression> = SExpression => ActionFn(24);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            16 => {
                // Space<SExpression> =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            17 => {
                // Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            18 => {
                // Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            19 => {
                // Values = Space<SExpression> => ActionFn(6);
                let __sym0 = __pop_NtSpace_3cSExpression_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValues(__nt), __end));
                8
            }
            20 => {
                // __Num = Num => ActionFn(2);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                9
            }
            21 => {
                // __OpCode = OpCode => ActionFn(3);
                let __sym0 = __pop_NtOpCode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____OpCode(__nt), __end));
                10
            }
            22 => {
                // __SExpression = SExpression => ActionFn(0);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            23 => {
                // __Values = Values => ActionFn(1);
                let __sym0 = __pop_NtValues(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Values(__nt), __end));
                12
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_20_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_20_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSpace_3cSExpression_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSpace_3cSExpression_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValues<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValues(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____OpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____OpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Values<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Values(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__SExpression::parse_SExpression;

mod __parse__Values {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use opcode;
    use std::rc::Rc;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_20_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(&'input str),
        Nt_28_3cSExpression_3e_20_22_20_22_29(opcode::SExpression),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2a(::std::vec::Vec<opcode::SExpression>),
        Nt_28_3cSExpression_3e_20_22_20_22_29_2b(::std::vec::Vec<opcode::SExpression>),
        NtNum(f64),
        NtOpCode(opcode::OpCode),
        NtSExpression(opcode::SExpression),
        NtSExpression_3f(::std::option::Option<opcode::SExpression>),
        NtSpace_3cSExpression_3e(Vec<opcode::SExpression>),
        NtValues(Vec<opcode::SExpression>),
        Nt____Num(f64),
        Nt____OpCode(opcode::OpCode),
        Nt____SExpression(opcode::SExpression),
        Nt____Values(Vec<opcode::SExpression>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on " ", error
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        8, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 7
        // State 1
        0, // on " ", error
        7, // on "(", goto 6
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        8, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 7
        // State 2
        -11, // on " ", reduce `SExpression = Num => ActionFn(4);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 3
        10, // on " ", goto 9
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 4
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 5
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 6
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 7
        -6, // on " ", reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 8
        16, // on " ", goto 15
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 9
        0, // on " ", error
        -4, // on "(", reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        // State 10
        0, // on " ", error
        22, // on "(", goto 21
        -16, // on ")", reduce `Space<SExpression> =  => ActionFn(25);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        23, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 22
        // State 11
        0, // on " ", error
        -9, // on "(", reduce `OpCode = "*" => ActionFn(10);`
        -9, // on ")", reduce `OpCode = "*" => ActionFn(10);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -9, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "*" => ActionFn(10);`
        // State 12
        0, // on " ", error
        -7, // on "(", reduce `OpCode = "+" => ActionFn(8);`
        -7, // on ")", reduce `OpCode = "+" => ActionFn(8);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -7, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "+" => ActionFn(8);`
        // State 13
        0, // on " ", error
        -8, // on "(", reduce `OpCode = "-" => ActionFn(9);`
        -8, // on ")", reduce `OpCode = "-" => ActionFn(9);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -8, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "-" => ActionFn(9);`
        // State 14
        0, // on " ", error
        -10, // on "(", reduce `OpCode = "/" => ActionFn(11);`
        -10, // on ")", reduce `OpCode = "/" => ActionFn(11);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -10, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `OpCode = "/" => ActionFn(11);`
        // State 15
        0, // on " ", error
        -5, // on "(", reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        // State 16
        0, // on " ", error
        22, // on "(", goto 21
        -18, // on ")", reduce `Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        23, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 22
        // State 17
        -11, // on " ", reduce `SExpression = Num => ActionFn(4);`
        0, // on "(", error
        -11, // on ")", reduce `SExpression = Num => ActionFn(4);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 18
        25, // on " ", goto 24
        0, // on "(", error
        -15, // on ")", reduce `Space<SExpression> = SExpression => ActionFn(24);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 19
        0, // on " ", error
        0, // on "(", error
        -19, // on ")", reduce `Values = Space<SExpression> => ActionFn(6);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 20
        0, // on " ", error
        0, // on "(", error
        26, // on ")", goto 25
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 21
        0, // on " ", error
        0, // on "(", error
        0, // on ")", error
        12, // on "*", goto 11
        13, // on "+", goto 12
        14, // on "-", goto 13
        15, // on "/", goto 14
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 22
        -6, // on " ", reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        0, // on "(", error
        -6, // on ")", reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 23
        28, // on " ", goto 27
        0, // on "(", error
        -17, // on ")", reduce `Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 24
        0, // on " ", error
        -4, // on "(", reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        -4, // on ")", reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -4, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        // State 25
        -12, // on " ", reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 26
        0, // on " ", error
        22, // on "(", goto 21
        -16, // on ")", reduce `Space<SExpression> =  => ActionFn(25);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        23, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, goto 22
        // State 27
        0, // on " ", error
        -5, // on "(", reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        -5, // on ")", reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        -5, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        // State 28
        0, // on " ", error
        0, // on "(", error
        30, // on ")", goto 29
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
        // State 29
        -12, // on " ", reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on "(", error
        -12, // on ")", reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -16, // on EOF, reduce `Space<SExpression> =  => ActionFn(25);`
        -18, // on EOF, reduce `Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);`
        -11, // on EOF, reduce `SExpression = Num => ActionFn(4);`
        -15, // on EOF, reduce `Space<SExpression> = SExpression => ActionFn(24);`
        -19, // on EOF, reduce `Values = Space<SExpression> => ActionFn(6);`
        -23, // on EOF, reduce `__Values = Values => ActionFn(1);`
        0, // on EOF, error
        -6, // on EOF, reduce `Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);`
        -17, // on EOF, reduce `Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);`
        -4, // on EOF, reduce `(<SExpression> " ")+ = SExpression, " " => ActionFn(20);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -5, // on EOF, reduce `(<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -12, // on EOF, reduce `SExpression = "(", OpCode, Values, ")" => ActionFn(5);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        2, // on (<SExpression> " ")+, goto 1
        3, // on Num, goto 2
        0, // on OpCode, error
        4, // on SExpression, goto 3
        0, // on SExpression?, error
        5, // on Space<SExpression>, goto 4
        6, // on Values, goto 5
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 1
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        3, // on Num, goto 2
        0, // on OpCode, error
        9, // on SExpression, goto 8
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 2
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 3
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 4
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 5
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 6
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        11, // on OpCode, goto 10
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 7
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 8
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 9
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 10
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        17, // on (<SExpression> " ")+, goto 16
        18, // on Num, goto 17
        0, // on OpCode, error
        19, // on SExpression, goto 18
        0, // on SExpression?, error
        20, // on Space<SExpression>, goto 19
        21, // on Values, goto 20
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 11
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 12
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 13
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 14
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 15
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 16
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        18, // on Num, goto 17
        0, // on OpCode, error
        24, // on SExpression, goto 23
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 17
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 18
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 19
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 20
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 21
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        27, // on OpCode, goto 26
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 22
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 23
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 24
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 25
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 26
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        17, // on (<SExpression> " ")+, goto 16
        18, // on Num, goto 17
        0, // on OpCode, error
        19, // on SExpression, goto 18
        0, // on SExpression?, error
        20, // on Space<SExpression>, goto 19
        29, // on Values, goto 28
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 27
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 28
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
        // State 29
        0, // on (<SExpression> " "), error
        0, // on (<SExpression> " ")*, error
        0, // on (<SExpression> " ")+, error
        0, // on Num, error
        0, // on OpCode, error
        0, // on SExpression, error
        0, // on SExpression?, error
        0, // on Space<SExpression>, error
        0, // on Values, error
        0, // on __Num, error
        0, // on __OpCode, error
        0, // on __SExpression, error
        0, // on __Values, error
    ];
    pub fn parse_Values<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<opcode::SExpression>, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 8 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_20_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2a_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2f_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__tok0),
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
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
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
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
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
    ) -> Option<Result<Vec<opcode::SExpression>,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<SExpression> " ") = SExpression, " " => ActionFn(17);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__nt), __end));
                0
            }
            2 => {
                // (<SExpression> " ")* =  => ActionFn(15);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action15::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<SExpression> " ")* = (<SExpression> " ")+ => ActionFn(16);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<SExpression> " ")+ = SExpression, " " => ActionFn(20);
                let __sym1 = __pop_Term_22_20_22(__symbols);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<SExpression> " ")+ = (<SExpression> " ")+, SExpression, " " => ActionFn(21);
                let __sym2 = __pop_Term_22_20_22(__symbols);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Num = r#"((-)|(\\+))?[0-9]+\\.?[0-9]*"# => ActionFn(7);
                let __sym0 = __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                3
            }
            7 => {
                // OpCode = "+" => ActionFn(8);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            8 => {
                // OpCode = "-" => ActionFn(9);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            9 => {
                // OpCode = "*" => ActionFn(10);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            10 => {
                // OpCode = "/" => ActionFn(11);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOpCode(__nt), __end));
                4
            }
            11 => {
                // SExpression = Num => ActionFn(4);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            12 => {
                // SExpression = "(", OpCode, Values, ")" => ActionFn(5);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtValues(__symbols);
                let __sym1 = __pop_NtOpCode(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtSExpression(__nt), __end));
                5
            }
            13 => {
                // SExpression? = SExpression => ActionFn(13);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            14 => {
                // SExpression? =  => ActionFn(14);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action14::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSExpression_3f(__nt), __end));
                6
            }
            15 => {
                // Space<SExpression> = SExpression => ActionFn(24);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            16 => {
                // Space<SExpression> =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            17 => {
                // Space<SExpression> = (<SExpression> " ")+, SExpression => ActionFn(26);
                let __sym1 = __pop_NtSExpression(__symbols);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            18 => {
                // Space<SExpression> = (<SExpression> " ")+ => ActionFn(27);
                let __sym0 = __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSpace_3cSExpression_3e(__nt), __end));
                7
            }
            19 => {
                // Values = Space<SExpression> => ActionFn(6);
                let __sym0 = __pop_NtSpace_3cSExpression_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValues(__nt), __end));
                8
            }
            20 => {
                // __Num = Num => ActionFn(2);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Num(__nt), __end));
                9
            }
            21 => {
                // __OpCode = OpCode => ActionFn(3);
                let __sym0 = __pop_NtOpCode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____OpCode(__nt), __end));
                10
            }
            22 => {
                // __SExpression = SExpression => ActionFn(0);
                let __sym0 = __pop_NtSExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SExpression(__nt), __end));
                11
            }
            23 => {
                // __Values = Values => ActionFn(1);
                let __sym0 = __pop_NtValues(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 13 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_20_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_20_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_28_2d_29_7c_28_5c_5c_2b_29_29_3f_5b0_2d9_5d_2b_5c_5c_2e_3f_5b0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cSExpression_3e_20_22_20_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cSExpression_3e_20_22_20_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSExpression_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSExpression_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSpace_3cSExpression_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSpace_3cSExpression_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValues<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValues(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Num<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Num(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____OpCode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::OpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____OpCode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, opcode::SExpression, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Values<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<opcode::SExpression>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Values(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Values::parse_Values;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        40 => /* '(' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        46 => /* '.' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 8;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((7, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
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
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, opcode::SExpression, usize),
) -> opcode::SExpression
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<opcode::SExpression>, usize),
) -> Vec<opcode::SExpression>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, opcode::OpCode, usize),
) -> opcode::OpCode
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> opcode::SExpression
{
    opcode::SExpression::atomic(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, o, _): (usize, opcode::OpCode, usize),
    (_, v, _): (usize, Vec<opcode::SExpression>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> opcode::SExpression
{
    { 
        let mut vals = v.iter()
            .map(|item| Rc::new(item.clone()))
            .collect(); 
        opcode::SExpression::op(
        opcode::Operation{ 
            opcode : o, 
            values : vals //.iter().map(|item| Rc::new(item)).collect()
            }
        )
    }
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<opcode::SExpression>, usize),
) -> Vec<opcode::SExpression>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> opcode::OpCode
{
    opcode::OpCode::Add
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> opcode::OpCode
{
    opcode::OpCode::Subtract
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> opcode::OpCode
{
    opcode::OpCode::Multiply
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> opcode::OpCode
{
    opcode::OpCode::Divide
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<opcode::SExpression>, usize),
    (_, e, _): (usize, ::std::option::Option<opcode::SExpression>, usize),
) -> Vec<opcode::SExpression>
{
    match e { // (2)
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, opcode::SExpression, usize),
) -> ::std::option::Option<opcode::SExpression>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<opcode::SExpression>
{
    None
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<opcode::SExpression>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<opcode::SExpression>, usize),
) -> ::std::vec::Vec<opcode::SExpression>
{
    v
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, opcode::SExpression, usize),
    (_, _, _): (usize, &'input str, usize),
) -> opcode::SExpression
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, opcode::SExpression, usize),
) -> ::std::vec::Vec<opcode::SExpression>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<opcode::SExpression>, usize),
    (_, e, _): (usize, opcode::SExpression, usize),
) -> ::std::vec::Vec<opcode::SExpression>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    __0: (usize, opcode::SExpression, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<opcode::SExpression>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action17(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<opcode::SExpression>, usize),
    __1: (usize, opcode::SExpression, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<opcode::SExpression>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action17(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<opcode::SExpression>, usize),
) -> Vec<opcode::SExpression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action15(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<opcode::SExpression>, usize),
    __1: (usize, ::std::option::Option<opcode::SExpression>, usize),
) -> Vec<opcode::SExpression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action16(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    __0: (usize, opcode::SExpression, usize),
) -> Vec<opcode::SExpression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action13(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<opcode::SExpression>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action14(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<opcode::SExpression>, usize),
    __1: (usize, opcode::SExpression, usize),
) -> Vec<opcode::SExpression>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action13(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<opcode::SExpression>, usize),
) -> Vec<opcode::SExpression>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action14(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __0,
        __temp0,
    )
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
