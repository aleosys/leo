// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.
use crate::{Expression, ExpressionValue, ResolvedNode, SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

impl Expression {
    /// Resolve the type of `!expression`
    pub(crate) fn not(
        table: &mut SymbolTable,
        expected_type: Option<Type>,
        expression: UnresolvedExpression,
        span: Span,
    ) -> Result<Self, ()> {
        let type_ = Type::Boolean;

        // Check the expected type if given
        Type::check_type(&expected_type, &type_, span.clone()).unwrap();

        // Resolve lhs and rhs expressions to boolean type
        let boolean_type = Some(type_.clone());
        let expression_resolved = Self::resolve(table, (boolean_type, expression)).unwrap();

        Ok(Expression {
            type_,
            value: ExpressionValue::Not(Box::new(expression_resolved), span),
        })
    }
}
