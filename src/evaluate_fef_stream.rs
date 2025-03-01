use std::{collections::BTreeMap, convert::Infallible, io::Read};

use anyhow::Context;
use fef::v0::{
    config::DEFAULT_CONFIG,
    expr::{
        self,
        error::ComposeError,
        traits::{BinaryOperationExpr, Composer, UnaryOperationExpr},
    },
    raw::VariableLengthEnum,
    read::read_expression,
};

pub(crate) fn evaluate_stream_as_fef_expr(
    read: &mut impl Read,
    variable_values: BTreeMap<VariableLengthEnum, f64>,
) -> anyhow::Result<f64> {
    let mut evaluator = FefStreamEvaluator { variable_values };
    let out = read_expression(read, &DEFAULT_CONFIG, &mut evaluator).context("FEF Read")?;
    Ok(out)
}

struct FefStreamEvaluator {
    variable_values: BTreeMap<VariableLengthEnum, f64>,
}

impl Composer<f64> for FefStreamEvaluator {
    type Error = Infallible;
    fn compose_addition(
        &mut self,
        expr: expr::ExprAddition<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (lhs, rhs) = expr.into();
        Ok(lhs + rhs)
    }

    fn compose_binary_float_32_literal(
        &mut self,
        expr: expr::ExprBinaryFloat32Literal<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let inner: f32 = match expr.try_into() {
            Ok(inner) => inner,
            Err(_) => unreachable!("Infallible"),
        };
        Ok(inner as f64)
    }

    fn compose_binary_float_64_literal(
        &mut self,
        expr: expr::ExprBinaryFloat64Literal<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let inner: f64 = match expr.try_into() {
            Ok(inner) => inner,
            Err(_) => unreachable!("Infallible"),
        };
        Ok(inner)
    }

    fn compose_cube(
        &mut self,
        expr: expr::ExprCube<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(expr.into_inner().powi(3))
    }

    fn compose_cube_root(
        &mut self,
        expr: expr::ExprCubeRoot<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(expr.into_inner().cbrt())
    }

    fn compose_division(
        &mut self,
        expr: expr::ExprDivision<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (dividend, divisor) = expr.into();
        Ok(dividend / divisor)
    }

    fn compose_false_literal(
        &mut self,
        _expr: expr::ExprFalseLiteral<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(0.0)
    }

    fn compose_int_division(
        &mut self,
        expr: expr::ExprIntDivision<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (dividend, divisor) = expr.into();
        Ok((dividend / divisor).floor())
    }

    fn compose_int_root(
        &mut self,
        expr: expr::ExprIntRoot<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (root, base) = expr.into();
        Ok(base.powf(1.0 / root).floor())
    }

    fn compose_modulo(
        &mut self,
        expr: expr::ExprModulo<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (dividend, divisor) = expr.into();
        Ok(dividend % divisor)
    }

    fn compose_multiplication(
        &mut self,
        expr: expr::ExprMultiplication<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (lhs, rhs) = expr.into();
        Ok(lhs * rhs)
    }

    fn compose_negation(
        &mut self,
        expr: expr::ExprNegation<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(-expr.into_inner())
    }

    fn compose_power(
        &mut self,
        expr: expr::ExprPower<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (base, exponent) = expr.into();
        Ok(base.powf(exponent))
    }

    fn compose_reciprocal(
        &mut self,
        expr: expr::ExprReciprocal<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(1.0 / expr.into_inner())
    }

    fn compose_root(
        &mut self,
        expr: expr::ExprRoot<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let base = *expr.rhs();
        let root = *expr.lhs();
        Ok(base.powf(1.0 / root))
    }

    fn compose_signed_int_literal(
        &mut self,
        expr: expr::ExprSignedIntLiteral<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let inner: i64 = match expr.try_into() {
            Ok(inner) => inner,
            Err(_) => unreachable!("Infallible"),
        };
        Ok(inner as f64)
    }

    fn compose_square(
        &mut self,
        expr: expr::ExprSquare<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let inner = expr.into_inner();
        Ok(inner.powi(2))
    }

    fn compose_square_root(
        &mut self,
        expr: expr::ExprSquareRoot<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(expr.into_inner().sqrt())
    }

    fn compose_subtraction(
        &mut self,
        expr: expr::ExprSubtraction<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let (lhs, rhs) = expr.into();
        Ok(lhs - rhs)
    }

    fn compose_true_literal(
        &mut self,
        _expr: expr::ExprTrueLiteral<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        Ok(1.0)
    }

    fn compose_unsigned_int_literal(
        &mut self,
        expr: expr::ExprUnsignedIntLiteral<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let inner: u64 = match expr.try_into() {
            Ok(inner) => inner,
            Err(_) => unreachable!("Infallible"),
        };
        Ok(inner as f64)
    }

    fn compose_variable(
        &mut self,
        expr: expr::ExprVariable<f64>,
    ) -> Result<f64, ComposeError<Self::Error>> {
        let identifier: VariableLengthEnum = expr.into();
        Ok(*self.variable_values.get(&identifier).expect("Variable not specified in metadata. This evaluator requires all variables to be specified."))
    }
}
