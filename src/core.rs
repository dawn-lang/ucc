// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lasso::Rodeo as Interner;
pub use lasso::Spur as Symbol;

pub(crate) type Map<K, V> = fxhash::FxHashMap<K, V>;

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
}

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
pub struct ValueStack(Vec<Value>);

pub struct Context {
    interner: Interner,
    fns: Map<Symbol, Expr>,
}

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
                        let e = match v {
                            Value::Quote(e) => Expr::Quote(e),
                        };
                        vs.0.push(Value::Quote(Box::new(e)));
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
                        let e2 = vs.0.pop().unwrap().unquote();
                        let e1 = vs.0.pop().unwrap().unquote();
                        vs.0.push(Value::Quote(Box::new(Expr::Compose(e1, e2))));
                        Ok(())
                    }
                }
                Intrinsic::Apply => {
                    if vs.0.len() < 2 {
                        Err(EvalError::TooFewValues {
                            available: vs.0.len(),
                            expected: 2,
                        })
                    } else {
                        let e1 = vs.0.pop().unwrap().unquote();
                        *e = Expr::Compose(e1, Box::new(e.clone()));
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
