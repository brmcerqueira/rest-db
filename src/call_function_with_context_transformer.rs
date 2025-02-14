use swc_core::ecma::visit::Fold;
use swc_core::ecma::visit::swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, Ident, IdentName, MemberExpr, MemberProp, ThisExpr};

pub struct CallFunctionWithContextTransformer;

impl Fold for CallFunctionWithContextTransformer {
    fn fold_call_expr(&mut self, call: CallExpr) -> CallExpr {
        let name = call.callee.as_expr().unwrap().clone().ident().unwrap().sym;

        if name.starts_with("$") {
            let mut args = vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::This(ThisExpr { span: call.span })),
            }];

            args.extend(call.args);

            return CallExpr {
                callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                    span: call.span,
                    obj: Box::new(Expr::Ident(Ident {
                        span: call.span,
                        sym: name,
                        ..Default::default()
                    })),
                    prop: MemberProp::Ident(IdentName {
                        span: call.span,
                        sym: "call".into(),
                        ..Default::default()
                    }),
                }))),
                args,
                ..call
            };
        }

        call
    }
}
