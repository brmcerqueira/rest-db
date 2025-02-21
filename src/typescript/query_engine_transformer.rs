use swc_core::ecma::visit::swc_ecma_ast::{
    BlockStmt, CallExpr, Callee, Expr, ExprOrSpread, FnExpr, Function, Ident, IdentName,
    MemberExpr, MemberProp, ReturnStmt, Stmt, ThisExpr,
};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub struct QueryEngineTransformer;

impl VisitMut for QueryEngineTransformer {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Call(call_expr) => {
                let expr = call_expr.callee.as_expr().unwrap().clone();
                if let Some(ident) = expr.ident() {
                    if ident.sym.starts_with("$") {
                        let mut args = vec![ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::This(ThisExpr {
                                span: call_expr.span,
                            })),
                        }];

                        args.extend(call_expr.args.clone());

                        *call_expr = CallExpr {
                            span: call_expr.span,
                            ctxt: call_expr.ctxt,
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                span: call_expr.span,
                                obj: Box::new(Expr::Ident(Ident {
                                    span: call_expr.span,
                                    sym: ident.sym,
                                    ..Default::default()
                                })),
                                prop: MemberProp::Ident(IdentName {
                                    span: call_expr.span,
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
            Expr::Arrow(arrow_expr) => {
                *expr = *Box::new(Expr::Fn(FnExpr {
                    ident: None,
                    function: Box::new(Function {
                        params: arrow_expr
                            .params
                            .iter()
                            .map(|param| param.clone().into())
                            .collect(),
                        decorators: Default::default(),
                        span: arrow_expr.span,
                        ctxt: arrow_expr.ctxt,
                        body: if arrow_expr.body.is_block_stmt() {
                            arrow_expr.body.clone().block_stmt()
                        } else {
                            Some(BlockStmt {
                                span: arrow_expr.span,
                                ctxt: arrow_expr.ctxt,
                                stmts: vec![Stmt::from(ReturnStmt {
                                    span: arrow_expr.span,
                                    arg: arrow_expr.body.clone().expr(),
                                })],
                            })
                        },
                        is_async: arrow_expr.is_async,
                        is_generator: arrow_expr.is_generator,
                        return_type: arrow_expr.return_type.clone(),
                        type_params: arrow_expr.type_params.clone(),
                    }),
                }));
            }
            _ => {}
        }

        expr.visit_mut_children_with(self);
    }
}
