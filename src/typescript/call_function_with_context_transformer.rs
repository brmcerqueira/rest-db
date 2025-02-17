use swc_core::common::DUMMY_SP;
use swc_core::ecma::visit::swc_ecma_ast::{
    CallExpr, Callee, Expr, ExprOrSpread, Ident, IdentName, MemberExpr, MemberProp, ThisExpr,
};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub struct CallFunctionWithContextTransformer;

impl VisitMut for CallFunctionWithContextTransformer {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Call(call_expr) => {
                let expr = call_expr.callee.as_expr().unwrap().clone();
                if let Some(ident) = expr.ident() {
                    if ident.sym.starts_with("$") {
                        let mut args = vec![ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::This(ThisExpr { span: DUMMY_SP })),
                        }];

                        args.extend(call_expr.args.clone());

                        *call_expr = CallExpr {
                            span: DUMMY_SP,
                            ctxt: call_expr.ctxt,
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: Box::new(Expr::Ident(Ident {
                                    span: DUMMY_SP,
                                    sym: ident.sym,
                                    ..Default::default()
                                })),
                                prop: MemberProp::Ident(IdentName {
                                    span: DUMMY_SP,
                                    sym: "call".into(),
                                    ..Default::default()
                                }),
                            }))),
                            args,
                            type_args: call_expr.type_args.clone(),
                        };
                    }
                }
            }
            _ => {}
        }

        expr.visit_mut_children_with(self);
    }
}
