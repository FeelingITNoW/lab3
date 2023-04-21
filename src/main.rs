use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}


pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(expr) => Value::IntValue(eval_arith_expr(expr)),
        BoolExpr(expr) => Value::BoolValue(eval_bool_expr(expr))
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {
        BinArithExpr{left, right, op} => match op {
            BinArithOp::AddOp => eval_arith_expr(*left) + eval_arith_expr(*right),
            BinArithOp::SubOp => eval_arith_expr(*left) - eval_arith_expr(*right),
            BinArithOp::MulOp => eval_arith_expr(*left) * eval_arith_expr(*right),
            BinArithOp::IntDivOp => eval_arith_expr(*left) / eval_arith_expr(*right),
        }, 

        IntLit(arith_expr) => arith_expr, 
    }
}

pub fn eval_bool_expr(my_bool_expr: BoolExpr) -> bool {
    match my_bool_expr {
        ArithCmpExpr{left,right,op} => {
            match op {
                ArithCmpOp::LtOp => eval_arith_expr(*left) < eval_arith_expr(*right), 
                ArithCmpOp::LteOp => eval_arith_expr(*left) <= eval_arith_expr(*right), 
                ArithCmpOp::GtOp => eval_arith_expr(*left) > eval_arith_expr(*right), 
                ArithCmpOp::GteOp => eval_arith_expr(*left) >= eval_arith_expr(*right), 
                ArithCmpOp::ArithEqOp => eval_arith_expr(*left) == eval_arith_expr(*right),
                ArithCmpOp::ArithNeqOp => eval_arith_expr(*left) != eval_arith_expr(*right)
            }
        }

        BinBoolExpr{left, right, op} => {
            match op {
                BinLogicOp::AndOp => eval_bool_expr(*left) && eval_bool_expr(*right),
                BinLogicOp::OrOp => eval_bool_expr(*left) || eval_bool_expr(*right),
                BinLogicOp::BoolEqOp => eval_bool_expr(*left) == eval_bool_expr(*right),
                BinLogicOp::BoolNeqOp => eval_bool_expr(*left) != eval_bool_expr(*right),
            }
        }

        NotExpr(bool_expr) => !eval_bool_expr(*bool_expr), 
        BoolLit(expr) => expr 
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_one() {
        let expr = ArithExpr(IntLit(24));
        
        assert_eq!(eval(expr), IntValue(24));
    }

    #[test]
    fn test_two() {
        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::AddOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(64)), right: Box::new(IntLit(8)), op: BinArithOp::IntDivOp});
        let expr = BoolExpr(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::LtOp });
        assert_eq!(eval(expr), BoolValue(false));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::AddOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(64)), right: Box::new(IntLit(8)), op: BinArithOp::IntDivOp});
        let expr = BoolExpr(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GtOp });
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let expr = BoolExpr(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::ArithNeqOp });
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let expr = BoolExpr(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::ArithEqOp });
        assert_eq!(eval(expr), BoolValue(false));
        
        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let expr = BoolExpr(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::LteOp });
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let expr = BoolExpr(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp });
        assert_eq!(eval(expr), BoolValue(false));


    }

    #[test]
    fn test_three() {
        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let expr = (BoolExpr(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp }))));
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let expr = (BoolExpr(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp }))));
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let left_logic_expr =(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp })));
        let right_logic_expr = BoolLit(false);
        let expr = BoolExpr(BinBoolExpr{ left: Box::new(left_logic_expr), right:Box::new(right_logic_expr), op: (AndOp) });
        //println!("{:?}", eval(expr));
        assert_eq!(eval(expr), BoolValue(false));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let left_logic_expr =(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp })));
        let right_logic_expr = BoolLit(false);
        let expr = BoolExpr(BinBoolExpr{ left: Box::new(left_logic_expr), right:Box::new(right_logic_expr), op: (OrOp) });
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let left_logic_expr =(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp })));
        let right_logic_expr = BoolLit(false);
        let expr = BoolExpr(BinBoolExpr{ left: Box::new(right_logic_expr), right:Box::new(left_logic_expr), op: (OrOp) });
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let left_logic_expr =(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp })));
        let right_logic_expr = BoolLit(false);
        let expr = BoolExpr(BinBoolExpr{ left: Box::new(left_logic_expr), right:Box::new(right_logic_expr), op: (BoolEqOp) });
        assert_eq!(eval(expr), BoolValue(false));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let left_logic_expr =(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp })));
        let right_logic_expr = BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: (AndOp) };
        let expr = BoolExpr(BinBoolExpr{ left: Box::new(left_logic_expr), right:Box::new(right_logic_expr), op: (BoolNeqOp) });
        //println!("{:?}", eval(expr));
        assert_eq!(eval(expr), BoolValue(true));

        let left_expr = Box::new((BinArithExpr { left: Box::new(IntLit(24)), right: Box::new(IntLit((32))), op: (BinArithOp::SubOp) }));
        let right_expr = Box::new(BinArithExpr{left: Box::new(IntLit(2)), right: Box::new(IntLit(8)), op: BinArithOp::MulOp});
        let left_logic_expr =(NotExpr(Box::new(ArithCmpExpr { left: left_expr, right: right_expr, op: ArithCmpOp::GteOp })));
        let right_logic_expr = BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: (AndOp) };
        let expr = BoolExpr(BinBoolExpr{ left: Box::new(left_logic_expr), right:Box::new(right_logic_expr), op: (BoolNeqOp) });
        println!("{:?}", eval(expr));
    }

    #[test]
    fn test_main() {
        main();
    }
}