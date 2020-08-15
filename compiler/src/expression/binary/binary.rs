//! Enforces a binary expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Expression, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_binary_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        left: Expression,
        right: Expression,
        span: Span,
    ) -> Result<(ConstrainedValue<F, G>, ConstrainedValue<F, G>), ExpressionError> {
        let mut resolved_left = self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_type.clone(),
            left,
            span.clone(),
        )?;
        let mut resolved_right = self.enforce_operand(
            cs,
            file_scope.clone(),
            function_scope.clone(),
            expected_type.clone(),
            right,
            span.clone(),
        )?;

        resolved_left.resolve_types(&mut resolved_right, expected_type, span)?;

        Ok((resolved_left, resolved_right))
    }
}
