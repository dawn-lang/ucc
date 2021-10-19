// Copyright (c) 2021 Scott J Maddox
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::builtin::FN_DEF_SRCS;
use crate::core::*;
use crate::display::*;
use crate::interp::*;
use crate::parse::*;
use std::io;

pub struct NonBlockingInterp {
    ctx: Context,
    vs: ValueStack,
    command: Option<InterpCommand>,
    is_first_eval_step: bool,
}

impl Default for NonBlockingInterp {
    fn default() -> Self {
        let mut ctx = Context::default();
        for fn_def_src in FN_DEF_SRCS.iter() {
            let fn_def = FnDefParser::new()
                .parse(&mut ctx.interner, fn_def_src)
                .unwrap();
            assert_eq!(ctx.define_fn(fn_def), None);
        }
        Self {
            ctx,
            vs: ValueStack::default(),
            command: None,
            is_first_eval_step: true,
        }
    }
}

impl NonBlockingInterp {
    pub fn is_done(&self) -> bool {
        self.command.is_none()
    }

    pub fn interp_start(&mut self, input: &str, w: &mut dyn io::Write) {
        match InterpCommandParser::new().parse(&mut self.ctx.interner, input) {
            Err(err) => {
                // TODO: better error messages
                w.write_fmt(format_args!("{:?}\n", err)).unwrap();
            }
            Ok(InterpCommand::Eval(is)) => {
                self.is_first_eval_step = true;
                self.command = Some(InterpCommand::Eval(is));
            }
            Ok(InterpCommand::Trace(e)) => {
                w.write_fmt(format_args!(
                    "{} {}\n",
                    self.vs.resolve(&self.ctx.interner),
                    e.resolve(&self.ctx.interner)
                ))
                .unwrap();
                self.command = Some(InterpCommand::Trace(e));
            }
            Ok(InterpCommand::Show(sym)) => {
                if let Some(e) = self.ctx.fns.get(&sym) {
                    w.write_fmt(format_args!(
                        "{{fn {} = {}}}\n",
                        sym.resolve(&self.ctx.interner),
                        e.resolve(&self.ctx.interner)
                    ))
                    .unwrap();
                } else {
                    w.write_fmt(format_args!("Not defined.\n")).unwrap();
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
                    w.write_all(name.as_bytes()).unwrap();
                }
                for name in names.iter().skip(1) {
                    w.write_all(" ".as_bytes()).unwrap();
                    w.write_all(name.as_bytes()).unwrap();
                }
                w.write_all("\n".as_bytes()).unwrap();
            }
            Ok(InterpCommand::Drop) => {
                self.vs = ValueStack::default();
                w.write_fmt(format_args!("Values dropped.\n")).unwrap();
            }
            Ok(InterpCommand::Clear) => {
                self.ctx.fns.clear();
                w.write_fmt(format_args!("Definitions cleared.\n")).unwrap();
            }
            Ok(InterpCommand::Reset) => {
                *self = Self::default();
                w.write_fmt(format_args!("Reset.\n")).unwrap();
            }
            Ok(InterpCommand::Help) => {
                w.write_all(HELP.as_bytes()).unwrap();
            }
        }
    }

    pub fn interp_step(&mut self, w: &mut dyn io::Write) {
        match self.command.take() {
            Some(InterpCommand::Eval(mut is)) => {
                if !is.is_empty() {
                    match is.remove(0) {
                        InterpItem::FnDef(fn_def) => {
                            let name = fn_def.0.resolve(&self.ctx.interner);
                            if let Some(_) = self.ctx.define_fn(fn_def) {
                                w.write_fmt(format_args!("Redefined `{}`.\n", name))
                                    .unwrap();
                            } else {
                                w.write_fmt(format_args!("Defined `{}`.\n", name)).unwrap();
                            }
                        }
                        InterpItem::Expr(mut e) => {
                            if self.is_first_eval_step {
                                w.write_fmt(format_args!(
                                    "{} {}\n",
                                    self.vs.resolve(&self.ctx.interner),
                                    e.resolve(&self.ctx.interner)
                                ))
                                .unwrap();
                            }
                            if e != Expr::default() {
                                if let Err(err) = self.ctx.small_step(&mut self.vs, &mut e) {
                                    w.write_fmt(format_args!(
                                        "⇓ {} {}\n",
                                        self.vs.resolve(&self.ctx.interner),
                                        e.resolve(&self.ctx.interner)
                                    )).unwrap();
                                    // TODO: better error messages
                                    w.write_fmt(format_args!(
                                        "{:?}\n",
                                        err.resolve(&self.ctx.interner)
                                    ))
                                    .unwrap();
                                    return;
                                } else {
                                    self.ctx.compress(&mut self.vs);
                                    is.insert(0, InterpItem::Expr(e));
                                    self.is_first_eval_step = false;
                                }
                            } else {
                                w.write_fmt(format_args!(
                                    "⇓ {} {}\n",
                                    self.vs.resolve(&self.ctx.interner),
                                    e.resolve(&self.ctx.interner)
                                ))
                                .unwrap();
                                self.is_first_eval_step = true;
                            }
                        }
                    }
                    self.command = Some(InterpCommand::Eval(is));
                }
            }
            Some(InterpCommand::Trace(mut e)) => {
                if e != Expr::default() {
                    if let Err(err) = self.ctx.small_step(&mut self.vs, &mut e) {
                        // TODO: better error messages
                        w.write_fmt(format_args!("{:?}\n", err.resolve(&self.ctx.interner)))
                            .unwrap();
                        return;
                    }
                    w.write_fmt(format_args!(
                        "⟶ {} {}\n",
                        self.vs.resolve(&self.ctx.interner),
                        e.resolve(&self.ctx.interner)
                    ))
                    .unwrap();
                    if self.ctx.compress(&mut self.vs) {
                        w.write_fmt(format_args!(
                            "= {} {}\n",
                            self.vs.resolve(&self.ctx.interner),
                            e.resolve(&self.ctx.interner)
                        )).unwrap();
                    }
                    self.command = Some(InterpCommand::Trace(e));
                }
            }
            _ => panic!(),
        }
    }
}
