#![crate_name = "unnecessary_mut_params_lint"]
#![crate_type = "dylib"]
#![feature(phase, plugin_registrar)]

#[phase(plugin, link)] extern crate syntax;
#[phase(plugin, link)] extern crate rustc;

use syntax::ast;
#[allow(unused_imports)] use syntax::ast::{TokenTree, TtToken};
use syntax::codemap::Span;
#[allow(unused_imports)] use syntax::ext::build::AstBuilder;
#[allow(unused_imports)] use syntax::parse::token;
use syntax::visit::FnKind;

use rustc::lint::LintPassObject;
use rustc::lint::{Context, LintPass, LintArray};
#[allow(unused_imports)] use rustc::middle::def;
use rustc::plugin::Registry;

declare_lint!(
    UNNECESSARY_MUT_PARAMS_USAGE_FUNCS,
    Warn,
    "Warn about unnecessarily mutable parameters to functions")

struct Pass;
impl LintPass for Pass {
    fn get_lints(&self) -> LintArray { lint_array!(UNNECESSARY_MUT_PARAMS_USAGE_FUNCS) }

    #[allow(unused_variables)]
    fn check_fn(&mut self,
                context: &Context,
                kind:    FnKind,
                decl:    &ast::FnDecl,
                block:   &ast::Block,
                span:    Span,
                node_id: ast::NodeId) {
        let mutable_args = decl.inputs.iter().filter(is_mut).collect::<Vec<_>>();
        unimplemented!();
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box Pass as LintPassObject);
}

fn is_mut(arg: &&ast::Arg) -> bool {
    match arg.ty.deref().node {
        ast::TyPtr(ast::MutTy {ty: _, mutbl: ast::MutMutable}) => true,
        ast::TyRptr(_, ast::MutTy {ty: _, mutbl: ast::MutMutable}) => true,
        ast::TyInfer => {unimplemented!()},
        _ => false,
    }
}