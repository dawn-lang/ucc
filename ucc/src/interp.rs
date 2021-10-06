// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::core::*;
use crate::display::*;
use crate::parse::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InterpItem {
    FnDef(FnDef),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InterpCommand {
    Eval(Vec<InterpItem>),
    Trace(Expr),
    Show(Symbol),
    List,
    Drop,
    Clear,
    Reset,
    Help,
}

pub(crate) static HELP: &'static str = "\
Commands available:

   <expr>                   evaluate <expr>
   {fn <sym> = <expr>}      define <sym> as <expr>
   :trace <expr>            trace the evaluation of <expr>
   :show <sym>              show the definition of <sym>
   :list                    list the defined symbols
   :drop                    drop the current value stack
   :clear                   clear all definitions
   :reset                   reset the interpreter
   :help                    display this list of commands
";

pub struct Interp {
    ctx: Context,
    vs: ValueStack,
}

impl Default for Interp {
    fn default() -> Self {
        let fn_def_srcs = [
            "{fn true = drop}",
            "{fn false = swap drop}",
            "{fn and = clone apply}",
            "{fn quote2 = quote swap quote swap compose}",
            "{fn quote3 = quote2 swap quote swap compose}",
            "{fn rotate3 = quote2 swap quote compose apply}",
            "{fn rotate4 = quote3 swap quote compose apply}",
            "{fn compose2 = compose}",
            "{fn compose3 = compose2 compose}",
            "{fn compose4 = compose3 compose}",
            "{fn compose5 = compose4 compose}",
            "{fn n0 = drop}",
            "{fn n1 = apply}",
            "{fn n2 = clone compose apply}",
            "{fn n3 = [clone] n2 [compose] n2 apply}",
            "{fn n4 = [clone] n3 [compose] n3 apply}",
            "{fn succ = [[clone]] swap clone [[compose]] swap [apply] compose5}",
            "{fn add = [succ] swap apply}",
            "{fn mul = [n0] rotate3 quote [add] compose rotate3 apply}",
        ];
        let mut ctx = Context::default();
        for fn_def_src in fn_def_srcs.iter() {
            let fn_def = FnDefParser::new()
                .parse(&mut ctx.interner, fn_def_src)
                .unwrap();
            assert_eq!(ctx.define_fn(fn_def), None);
        }
        Self {
            ctx,
            vs: ValueStack::default(),
        }
    }
}

impl Interp {
    pub fn interp(&mut self, cell: &str, w: &mut dyn fmt::Write) -> fmt::Result {
        match InterpCommandParser::new().parse(&mut self.ctx.interner, cell) {
            Err(err) => {
                // TODO: better error messages
                w.write_fmt(format_args!("{:?}\n", err))
            }
            Ok(InterpCommand::Eval(is)) => {
                for i in is {
                    match i {
                        InterpItem::FnDef(fn_def) => {
                            let name = fn_def.0.resolve(&self.ctx.interner);
                            if let Some(_) = self.ctx.define_fn(fn_def) {
                                w.write_fmt(format_args!("Redefined `{}`.\n", name))?;
                            } else {
                                w.write_fmt(format_args!("Defined `{}`.\n", name))?;
                            }
                        }
                        InterpItem::Expr(mut e) => {
                            w.write_fmt(format_args!(
                                "{} {}\n",
                                self.vs.resolve(&self.ctx.interner),
                                e.resolve(&self.ctx.interner)
                            ))?;
                            let result = self.ctx.eval(&mut self.vs, &mut e);
                            w.write_fmt(format_args!(
                                "⇓ {} {}\n",
                                self.vs.resolve(&self.ctx.interner),
                                e.resolve(&self.ctx.interner)
                            ))?;
                            if let Err(err) = result {
                                // TODO: better error messages
                                w.write_fmt(format_args!("{:?}\n", err))?;
                                break;
                            }
                        }
                    }
                }
                Ok(())
            }
            Ok(InterpCommand::Trace(mut e)) => {
                w.write_fmt(format_args!(
                    "{} {}\n",
                    self.vs.resolve(&self.ctx.interner),
                    e.resolve(&self.ctx.interner)
                ))?;
                while e != Expr::default() {
                    if let Err(err) = self.ctx.small_step(&mut self.vs, &mut e) {
                        // TODO: better error messages
                        w.write_fmt(format_args!("{:?}\n", err))?;
                        break;
                    }
                    w.write_fmt(format_args!(
                        "⟶ {} {}\n",
                        self.vs.resolve(&self.ctx.interner),
                        e.resolve(&self.ctx.interner)
                    ))?;
                }
                Ok(())
            }
            Ok(InterpCommand::Show(sym)) => {
                if let Some(e) = self.ctx.fns.get(&sym) {
                    w.write_fmt(format_args!(
                        "{{fn {} = {}}}\n",
                        sym.resolve(&self.ctx.interner),
                        e.resolve(&self.ctx.interner)
                    ))
                } else {
                    w.write_fmt(format_args!("Not defined.\n"))
                }
            }
            Ok(InterpCommand::List) => {
                let mut names: Vec<String> = self
                    .ctx
                    .fns
                    .keys()
                    .map(|sym| sym.resolve(&self.ctx.interner))
                    .collect();
                names.sort_unstable();
                if let Some(name) = names.first() {
                    w.write_str(name.as_str())?;
                }
                for name in names.iter().skip(1) {
                    w.write_str(" ")?;
                    w.write_str(name.as_str())?;
                }
                w.write_str("\n")?;
                Ok(())
            }
            Ok(InterpCommand::Drop) => {
                self.vs = ValueStack::default();
                w.write_fmt(format_args!("Values dropped.\n"))
            }
            Ok(InterpCommand::Clear) => {
                self.ctx.fns.clear();
                w.write_fmt(format_args!("Definitions cleared.\n"))
            }
            Ok(InterpCommand::Reset) => {
                *self = Interp::default();
                w.write_fmt(format_args!("Reset.\n"))
            }
            Ok(InterpCommand::Help) => w.write_str(HELP),
        }
    }
}
