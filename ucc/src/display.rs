// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::core::{Expr, Interner, Intrinsic, Symbol, Value, ValueStack};
use std::fmt;

pub(crate) type ResolvedSymbol = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedExpr {
    Empty,
    Intrinsic(Intrinsic),
    Call(ResolvedSymbol),
    Quote(Box<ResolvedExpr>),
    Compose(Vec<ResolvedExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedValue {
    Quote(Box<ResolvedExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedValueStack(pub(crate) Vec<ResolvedValue>);

pub(crate) trait Resolve {
    type Output;
    fn resolve(&self, interner: &Interner) -> Self::Output;
}

impl Resolve for Symbol {
    type Output = ResolvedSymbol;
    fn resolve(&self, interner: &Interner) -> Self::Output {
        interner.resolve(&self.0).to_owned()
    }
}

impl Resolve for Expr {
    type Output = ResolvedExpr;
    fn resolve(&self, interner: &Interner) -> Self::Output {
        match self {
            Expr::Intrinsic(i) => ResolvedExpr::Intrinsic(*i),
            Expr::Call(sym) => ResolvedExpr::Call(sym.resolve(interner)),
            Expr::Quote(e) => ResolvedExpr::Quote(Box::new(e.resolve(interner))),
            Expr::Compose(es) => {
                ResolvedExpr::Compose(es.iter().map(|e| e.resolve(interner)).collect())
            }
        }
    }
}

impl Resolve for Value {
    type Output = ResolvedValue;
    fn resolve(&self, interner: &Interner) -> Self::Output {
        match self {
            Value::Quote(e) => ResolvedValue::Quote(Box::new(e.resolve(interner))),
        }
    }
}

impl Resolve for ValueStack {
    type Output = ResolvedValueStack;
    fn resolve(&self, interner: &Interner) -> Self::Output {
        ResolvedValueStack(self.0.iter().map(|v| v.resolve(interner)).collect())
    }
}

impl ResolvedExpr {
    fn is_compose(&self) -> bool {
        match self {
            ResolvedExpr::Compose(..) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intrinsic::Swap => "swap".fmt(f),
            Intrinsic::Clone => "clone".fmt(f),
            Intrinsic::Drop => "drop".fmt(f),
            Intrinsic::Quote => "quote".fmt(f),
            Intrinsic::Compose => "compose".fmt(f),
            Intrinsic::Apply => "apply".fmt(f),
        }
    }
}

impl fmt::Display for ResolvedExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedExpr::Empty => Ok(()),
            ResolvedExpr::Intrinsic(i) => i.fmt(f),
            ResolvedExpr::Call(sym) => sym.fmt(f),
            ResolvedExpr::Quote(e) => write!(f, "[{}]", e),
            ResolvedExpr::Compose(es) => {
                if let Some(e) = es.first() {
                    if e.is_compose() {
                        write!(f, "({})", e)?;
                    } else {
                        write!(f, "{}", e)?;
                    }
                }
                for e in es.iter().skip(1) {
                    " ".fmt(f)?;
                    if e.is_compose() {
                        write!(f, "({})", e)?;
                    } else {
                        write!(f, "{}", e)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for ResolvedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedValue::Quote(v) => write!(f, "[{}]", v),
        }
    }
}

impl fmt::Display for ResolvedValueStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "⟨".fmt(f)?;
        if let Some(v) = self.0.first() {
            v.fmt(f)?;
        }
        for v in self.0.iter().skip(1) {
            " ".fmt(f)?;
            v.fmt(f)?;
        }
        "⟩".fmt(f)
    }
}
