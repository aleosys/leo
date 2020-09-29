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
use crate::{ResolvedNode, SymbolTable, TypeError};
use leo_typed::{Identifier, IntegerType, Span, Type as UnresolvedType};

use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of an identifier in a Leo program. Cannot be implicit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    Array(Box<Type>, Vec<usize>),
    Tuple(Vec<Type>),

    // User defined types
    Circuit(Identifier),
    Function(Identifier),
}

impl ResolvedNode for Type {
    type Error = TypeError;
    type UnresolvedNode = (UnresolvedType, Span);

    /// Resolves the given type. Cannot be an implicit or Self type.
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let type_ = unresolved.0;
        let span = unresolved.1;

        Ok(match type_ {
            UnresolvedType::Address => Type::Address,
            UnresolvedType::Boolean => Type::Boolean,
            UnresolvedType::Field => Type::Field,
            UnresolvedType::Group => Type::Group,
            UnresolvedType::IntegerType(integer) => Type::IntegerType(integer),

            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::resolve(table, (*type_, span))?;

                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::resolve(table, (type_, span.clone())))
                    .collect::<Result<Vec<_>, _>>()?;

                Type::Tuple(tuple_types)
            }

            UnresolvedType::Circuit(identifier) => {
                // Lookup the circuit type in the symbol table
                let exists = table.get_variable(&identifier.name);

                // Throw an error if the circuit does not exist
                if exists.is_none() {
                    return Err(TypeError::undefined_circuit(identifier));
                }

                Type::Circuit(identifier)
            }

            UnresolvedType::SelfType => {
                // Throw an error for using `Self` outside of a circuit
                return Err(TypeError::self_not_available(span));
            }
        })
    }
}

impl Type {
    /// Resolve a type inside of a circuit definition.
    /// If this type is SelfType, return the circuit's type
    pub fn from_circuit(
        table: &mut SymbolTable,
        type_: UnresolvedType,
        circuit_name: Identifier,
        span: Span,
    ) -> Result<Self, TypeError> {
        Ok(match type_ {
            UnresolvedType::Array(type_, dimensions) => {
                let array_type = Type::from_circuit(table, *type_, circuit_name, span)?;
                Type::Array(Box::new(array_type), dimensions)
            }
            UnresolvedType::Tuple(types) => {
                let tuple_types = types
                    .into_iter()
                    .map(|type_| Type::from_circuit(table, type_, circuit_name.clone(), span.clone()))
                    .collect::<Result<Vec<_>, _>>()?;

                Type::Tuple(tuple_types)
            }
            UnresolvedType::SelfType => Type::Circuit(circuit_name),
            // The unresolved type does not depend on the current circuit definition
            unresolved => Type::resolve(table, (unresolved, span))?,
        })
    }

    /// Returns `Ok` if the given expected type is `Some` and  expected type == actual type
    pub fn check_type(expected_option: &Option<Self>, actual: &Type, span: Span) -> Result<(), TypeError> {
        if let Some(expected) = expected_option {
            if expected.ne(actual) {
                return Err(TypeError::mismatched_types(expected, actual, span));
            }
        }
        Ok(())
    }

    /// Returns `Ok` if self is an expected integer type `Type::IntegerType`
    pub fn check_type_integer(&self, span: Span) -> Result<(), TypeError> {
        match self {
            Type::IntegerType(_) => Ok(()),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_integer(type_, span)),
        }
    }

    /// Returns array element type and dimensions if self is an expected array type `Type::Array`
    pub fn get_type_array(&self, span: Span) -> Result<(&Type, &Vec<usize>), TypeError> {
        match self {
            Type::Array(element_type, dimensions) => Ok((element_type, dimensions)),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_array(type_, span)),
        }
    }

    /// Returns tuple element types if self is an expected tuple type `Type::Tuple`
    pub fn get_type_tuple(&self, span: Span) -> Result<&Vec<Type>, TypeError> {
        match self {
            Type::Tuple(types) => Ok(types),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_tuple(type_, span)),
        }
    }

    /// Returns circuit identifier if self is an expected circuit type `Type::Circuit`
    pub fn get_type_circuit(&self, span: Span) -> Result<&Identifier, TypeError> {
        match self {
            Type::Circuit(identifier) => Ok(identifier),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_circuit(type_, span)),
        }
    }

    /// Returns function identifier if self is an expected function type `Type::Function`
    pub fn get_type_function(&self, span: Span) -> Result<&Identifier, TypeError> {
        match self {
            Type::Function(identifier) => Ok(identifier),
            // Throw mismatched type error
            type_ => Err(TypeError::invalid_function(type_, span)),
        }
    }

    // /// Returns the data type - discarding wrapping types
    // pub fn get_type_data(&self, _span: Span) -> Result<&Type, ()> {
    //     match self {
    //         Type::Array(element_type, _dimensions) => Ok(element_type),
    //         Type::Tuple(types) => unimplemented!("")
    //     }
    // }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::IntegerType(integer_type) => write!(f, "{}", integer_type),

            Type::Array(type_, dimensions) => {
                let dimensions_string = dimensions
                    .iter()
                    .map(|dimension| format!("{}", dimension))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "[{}; ({})]", *type_, dimensions_string)
            }
            Type::Tuple(tuple) => {
                let tuple_string = tuple.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ");

                write!(f, "({})", tuple_string)
            }

            Type::Circuit(identifier) => write!(f, "circuit {}", identifier),
            Type::Function(identifier) => write!(f, "function {}", identifier),
        }
    }
}
