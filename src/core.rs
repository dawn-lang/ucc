// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) use lasso::Rodeo as Interner;
use std::hash::Hash;

pub(crate) type Map<K, V> = fxhash::FxHashMap<K, V>;

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(pub(crate) lasso::Spur);

////////////
// Syntax //
////////////

/// Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Empty,
    Intrinsic(Intrinsic),
    Call(Symbol),
    Quote(Box<Expr>),
    Compose(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intrinsic {
    Swap,
    Clone,
    Drop,
    Quote,
    Compose,
    Apply,
}

///////////////
// Semantics //
///////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Quote(Box<Expr>),
}

impl Value {
    fn unquote(self) -> Box<Expr> {
        match self {
            Value::Quote(e) => e,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueStack(pub(crate) Vec<Value>);

pub struct Context {
    pub(crate) interner: Interner,
    pub(crate) fns: Map<Symbol, Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    EmptyExpr,
    TooFewValues { available: usize, expected: usize },
    UndefinedFn(Symbol),
}

impl Default for Context {
    fn default() -> Self {
        let interner = Interner::default();
        Context {
            interner,
            fns: Map::default(),
        }
    }
}

impl Context {
    pub fn small_step(&mut self, vs: &mut ValueStack, e: &mut Expr) -> Result<(), EvalError> {
        match e {
            Expr::Empty => Err(EvalError::EmptyExpr),
            Expr::Intrinsic(intr) => match intr {
                Intrinsic::Swap => {
                    if vs.0.len() < 2 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 2,
                        })
                    } else {
                        let v = vs.0.remove(vs.0.len() - 2);
                        vs.0.push(v);
                        *e = Expr::Empty;
                        Ok(())
                    }
                }
                Intrinsic::Clone => {
                    if vs.0.len() < 1 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 1,
                        })
                    } else {
                        vs.0.push(vs.0.last().unwrap().clone());
                        *e = Expr::Empty;
                        Ok(())
                    }
                }
                Intrinsic::Drop => {
                    if vs.0.len() < 1 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 1,
                        })
                    } else {
                        vs.0.pop();
                        *e = Expr::Empty;
                        Ok(())
                    }
                }
                Intrinsic::Quote => {
                    if vs.0.len() < 1 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 1,
                        })
                    } else {
                        let v = vs.0.pop().unwrap();
                        let qe = match v {
                            Value::Quote(e) => Expr::Quote(e),
                        };
                        vs.0.push(Value::Quote(Box::new(qe)));
                        *e = Expr::Empty;
                        Ok(())
                    }
                }
                Intrinsic::Compose => {
                    if vs.0.len() < 2 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 2,
                        })
                    } else {
                        let mut e2 = vs.0.pop().unwrap().unquote();
                        let mut e1 = vs.0.pop().unwrap().unquote();
                        while let Expr::Compose(e21, e22) = *e2 {
                            e1 = Box::new(Expr::Compose(e1, e21));
                            e2 = e22;
                        }
                        vs.0.push(Value::Quote(Box::new(Expr::Compose(e1, e2))));
                        *e = Expr::Empty;
                        Ok(())
                    }
                }
                Intrinsic::Apply => {
                    if vs.0.len() < 1 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 1,
                        })
                    } else {
                        let e1 = vs.0.pop().unwrap().unquote();
                        *e = *e1;
                        Ok(())
                    }
                }
            },
            Expr::Call(sym) => {
                if let Some(new_e) = self.fns.get(sym) {
                    *e = new_e.clone();
                    Ok(())
                } else {
                    Err(EvalError::UndefinedFn(*sym))
                }
            }
            Expr::Quote(qe) => {
                vs.0.push(Value::Quote(qe.clone()));
                *e = Expr::Empty;
                Ok(())
            }
            Expr::Compose(e1, e2) => {
                self.small_step(vs, e1)?;
                if **e1 == Expr::Empty {
                    *e = (**e2).clone()
                }
                Ok(())
            }
        }
    }
}

//////////////////////////
// Function Definitions //
//////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnDef(pub Symbol, pub Expr);

impl Context {
    pub fn define_fn(&mut self, fn_def: FnDef) -> Option<FnDef> {
        let result = self.fns.remove(&fn_def.0).map(|e| FnDef(fn_def.0, e));
        self.fns.insert(fn_def.0, fn_def.1);
        result
    }
}
