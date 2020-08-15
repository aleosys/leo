use crate::{Assignee, ConditionalStatement, Declare, Expression, FormattedMacro, Identifier, Span, Variables};
use leo_ast::{
    operations::AssignOperation,
    statements::{
        AssignStatement,
        DefinitionStatement,
        ExpressionStatement,
        ForStatement,
        MacroStatement,
        ReturnStatement,
        Statement as AstStatement,
    },
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statement {
    Return(Expression, Span),
    Definition(Declare, Variables, Vec<Expression>, Span),
    Assign(Assignee, Expression, Span),
    Conditional(ConditionalStatement, Span),
    Iteration(Identifier, Expression, Expression, Vec<Statement>, Span),
    AssertEq(Expression, Expression, Span),
    Macro(FormattedMacro),
    Expression(Expression, Span),
}

impl<'ast> From<ReturnStatement<'ast>> for Statement {
    fn from(statement: ReturnStatement<'ast>) -> Self {
        Statement::Return(Expression::from(statement.expression), Span::from(statement.span))
    }
}

impl<'ast> From<DefinitionStatement<'ast>> for Statement {
    fn from(statement: DefinitionStatement<'ast>) -> Self {
        let span = Span::from(statement.span);

        let expressions = statement
            .expressions
            .into_iter()
            .map(|e| {
                let mut expression = Expression::from(e);
                expression.set_span(&span);

                expression
            })
            .collect::<Vec<_>>();

        Statement::Definition(
            Declare::from(statement.declare),
            Variables::from(statement.variables),
            expressions,
            span,
        )
    }
}

impl<'ast> From<AssignStatement<'ast>> for Statement {
    fn from(statement: AssignStatement<'ast>) -> Self {
        match statement.assign {
            AssignOperation::Assign(ref _assign) => Statement::Assign(
                Assignee::from(statement.assignee),
                Expression::from(statement.expression),
                Span::from(statement.span),
            ),
            operation_assign => {
                // convert assignee into postfix expression
                let converted = Expression::from(statement.assignee.clone());

                match operation_assign {
                    AssignOperation::AddAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Add(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::SubAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Sub(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::MulAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Mul(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::DivAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Div(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::PowAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Pow(
                            Box::new(converted),
                            Box::new(Expression::from(statement.expression)),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::Assign(ref _assign) => unimplemented!("cannot assign twice to assign statement"),
                }
            }
        }
    }
}

impl<'ast> From<ForStatement<'ast>> for Statement {
    fn from(statement: ForStatement<'ast>) -> Self {
        Statement::Iteration(
            Identifier::from(statement.index),
            Expression::from(statement.start),
            Expression::from(statement.stop),
            statement
                .statements
                .into_iter()
                .map(|statement| Statement::from(statement))
                .collect(),
            Span::from(statement.span),
        )
    }
}

impl<'ast> From<MacroStatement<'ast>> for Statement {
    fn from(statement: MacroStatement<'ast>) -> Self {
        match statement {
            MacroStatement::AssertEq(assert_eq) => Statement::AssertEq(
                Expression::from(assert_eq.left),
                Expression::from(assert_eq.right),
                Span::from(assert_eq.span),
            ),
            MacroStatement::Formatted(formatted) => Statement::Macro(FormattedMacro::from(formatted)),
        }
    }
}

impl<'ast> From<ExpressionStatement<'ast>> for Statement {
    fn from(statement: ExpressionStatement<'ast>) -> Self {
        let span = Span::from(statement.span);
        let mut expression = Expression::from(statement.expression);

        expression.set_span(&span);

        Statement::Expression(expression, span)
    }
}

impl<'ast> From<AstStatement<'ast>> for Statement {
    fn from(statement: AstStatement<'ast>) -> Self {
        match statement {
            AstStatement::Return(statement) => Statement::from(statement),
            AstStatement::Definition(statement) => Statement::from(statement),
            AstStatement::Assign(statement) => Statement::from(statement),
            AstStatement::Conditional(statement) => {
                let span = Span::from(statement.span.clone());
                Statement::Conditional(ConditionalStatement::from(statement), span)
            }
            AstStatement::Iteration(statement) => Statement::from(statement),
            AstStatement::Assert(statement) => Statement::from(statement),
            AstStatement::Expression(statement) => Statement::from(statement),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref expression, ref _span) => write!(f, "return {}", expression),
            Statement::Definition(ref declare, ref variable, ref expressions, ref _span) => {
                let formatted_expressions = expressions
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(",");

                write!(f, "{} {} = {};", declare, variable, formatted_expressions)
            }
            Statement::Assign(ref variable, ref statement, ref _span) => write!(f, "{} = {};", variable, statement),
            Statement::Conditional(ref statement, ref _span) => write!(f, "{}", statement),
            Statement::Iteration(ref var, ref start, ref stop, ref list, ref _span) => {
                write!(f, "for {} in {}..{} {{\n", var, start, stop)?;
                for l in list {
                    write!(f, "\t\t{}\n", l)?;
                }
                write!(f, "\t}}")
            }
            Statement::AssertEq(ref left, ref right, ref _span) => write!(f, "assert_eq({}, {});", left, right),
            Statement::Macro(ref macro_) => write!(f, "{}", macro_),
            Statement::Expression(ref expression, ref _span) => write!(f, "{};", expression),
        }
    }
}
