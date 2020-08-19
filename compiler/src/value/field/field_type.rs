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

//! A data type that represents a field value

use crate::errors::FieldError;
use leo_typed::Span;

use snark_std::field::Field as FieldStd;
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        curves::{FieldGadget, FpGadget},
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
            select::CondSelectGadget,
            uint::UInt8,
            ToBitsGadget,
            ToBytesGadget,
        },
    },
};

use crate::FieldCircuitBuilder;
use serde::export::PhantomData;
use snark_std::ops::Equal;
use std::{
    borrow::Borrow,
    cell::{RefCell, RefMut},
    cmp::Ordering,
    rc::Rc,
};

#[derive(Clone, Debug)]
pub enum FieldType<F: Field + PrimeField> {
    Constant(F),
    Allocated(FpGadget<F>),
}

impl<F: Field + PrimeField> FieldType<F> {
    pub fn get_value(&self) -> Option<F> {
        match self {
            FieldType::Constant(field) => Some(*field),
            FieldType::Allocated(gadget) => gadget.get_value(),
        }
    }

    pub fn constant(string: String, span: Span) -> Result<Self, FieldError> {
        let value = F::from_str(&string).map_err(|_| FieldError::invalid_field(string, span))?;

        Ok(FieldType::Constant(value))
    }

    pub fn negate<CS: ConstraintSystem<F>>(&self, cs: CS, span: Span) -> Result<Self, FieldError> {
        match self {
            FieldType::Constant(field) => Ok(FieldType::Constant(field.neg())),
            FieldType::Allocated(field) => {
                let result = field.negate(cs).map_err(|e| FieldError::negate_operation(e, span))?;

                Ok(FieldType::Allocated(result))
            }
        }
    }

    pub fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: Span) -> Result<Self, FieldError> {
        match (self, other) {
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                Ok(FieldType::Constant(self_value.add(other_value)))
            }

            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                let result = self_value
                    .add(cs, other_value)
                    .map_err(|e| FieldError::binary_operation(format!("+"), e, span))?;

                Ok(FieldType::Allocated(result))
            }

            (FieldType::Constant(constant_value), FieldType::Allocated(allocated_value))
            | (FieldType::Allocated(allocated_value), FieldType::Constant(constant_value)) => Ok(FieldType::Allocated(
                allocated_value
                    .add_constant(cs, constant_value)
                    .map_err(|e| FieldError::binary_operation(format!("+"), e, span))?,
            )),
        }
    }

    pub fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: Span) -> Result<Self, FieldError> {
        match (self, other) {
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                Ok(FieldType::Constant(self_value.sub(other_value)))
            }

            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                let result = self_value
                    .sub(cs, other_value)
                    .map_err(|e| FieldError::binary_operation(format!("-"), e, span))?;

                Ok(FieldType::Allocated(result))
            }

            (FieldType::Constant(constant_value), FieldType::Allocated(allocated_value))
            | (FieldType::Allocated(allocated_value), FieldType::Constant(constant_value)) => Ok(FieldType::Allocated(
                allocated_value
                    .sub_constant(cs, constant_value)
                    .map_err(|e| FieldError::binary_operation(format!("+"), e, span))?,
            )),
        }
    }

    pub fn mul<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: Span) -> Result<Self, FieldError> {
        match (self, other) {
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                Ok(FieldType::Constant(self_value.mul(other_value)))
            }

            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                let result = self_value
                    .mul(cs, other_value)
                    .map_err(|e| FieldError::binary_operation(format!("*"), e, span))?;

                Ok(FieldType::Allocated(result))
            }

            (FieldType::Constant(constant_value), FieldType::Allocated(allocated_value))
            | (FieldType::Allocated(allocated_value), FieldType::Constant(constant_value)) => Ok(FieldType::Allocated(
                allocated_value
                    .mul_by_constant(cs, constant_value)
                    .map_err(|e| FieldError::binary_operation(format!("*"), e, span))?,
            )),
        }
    }

    pub fn div<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self, span: Span) -> Result<Self, FieldError> {
        let inverse = match other {
            FieldType::Constant(constant) => {
                let constant_inverse = constant
                    .inverse()
                    .ok_or(FieldError::no_inverse(constant.to_string(), span.clone()))?;

                FieldType::Constant(constant_inverse)
            }
            FieldType::Allocated(allocated) => {
                let allocated_inverse = allocated
                    .inverse(&mut cs)
                    .map_err(|e| FieldError::binary_operation(format!("+"), e, span.clone()))?;

                FieldType::Allocated(allocated_inverse)
            }
        };

        self.mul(cs, &inverse, span)
    }

    pub fn alloc_helper<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>>(
        value_gen: Fn,
    ) -> Result<F, SynthesisError> {
        let field_string = match value_gen() {
            Ok(value) => {
                let string_value = value.borrow().clone();
                Ok(string_value)
            }
            _ => Err(SynthesisError::AssignmentMissing),
        }?;

        F::from_str(&field_string).map_err(|_| SynthesisError::AssignmentMissing)
    }

    pub fn allocated<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<FpGadget<F>, SynthesisError> {
        match self {
            FieldType::Constant(constant) => FpGadget::alloc(&mut cs.ns(|| format!("{:?}", constant)), || Ok(constant)),
            FieldType::Allocated(allocated) => FpGadget::alloc(&mut cs.ns(|| format!("{:?}", allocated)), || {
                allocated.value.ok_or(SynthesisError::AssignmentMissing)
            }),
        }
    }
}

impl<F: Field + PrimeField> AllocGadget<String, F> for FieldType<F> {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::alloc(cs, || Self::alloc_helper(value_gen))?;

        Ok(FieldType::Allocated(value))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::alloc_input(cs, || Self::alloc_helper(value_gen))?;

        Ok(FieldType::Allocated(value))
    }
}

impl<F: Field + PrimeField> PartialEq for FieldType<F> {
    fn eq(&self, other: &Self) -> bool {
        let self_value = self.get_value();
        let other_value = other.get_value();

        self_value.is_some() && other_value.is_some() && self_value.eq(&other_value)
    }
}

impl<F: Field + PrimeField> Eq for FieldType<F> {}

impl<F: Field + PrimeField> PartialOrd for FieldType<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = self.get_value();
        let other_value = other.get_value();

        Option::from(self_value.cmp(&other_value))
    }
}

impl<F: Field + PrimeField> EvaluateEqGadget<F> for FieldType<F> {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        match (self, other) {
            (FieldType::Constant(first), FieldType::Constant(second)) => Ok(Boolean::constant(first.eq(second))),
            (FieldType::Constant(_), FieldType::Allocated(_)) => unimplemented!(),
            (FieldType::Allocated(_), FieldType::Constant(_)) => unimplemented!(),
            (FieldType::Allocated(first), FieldType::Allocated(second)) => {
                let builder = FieldCircuitBuilder {
                    0: Rc::new(RefCell::new(cs)),
                    1: Default::default(),
                };
                let first_std = FieldStd::from((first.clone(), builder.clone()));
                let second_std = FieldStd::from((second.clone(), builder.clone()));

                let result_std = first_std.eq(&second_std).map_err(|_| SynthesisError::Unsatisfiable)?;
                let result_option = result_std.to_gadget_unsafe();

                result_option.ok_or(SynthesisError::Unsatisfiable)
            }
        }
    }
}

impl<F: Field + PrimeField> EqGadget<F> for FieldType<F> {}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for FieldType<F> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            // c - c
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                if self_value == other_value {
                    return Ok(());
                }
                Err(SynthesisError::AssignmentMissing)
            }
            // a - a
            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                self_value.conditional_enforce_equal(cs, other_value, condition)
            }
            // c - a = a - c
            (FieldType::Constant(constant_value), FieldType::Allocated(allocated_value))
            | (FieldType::Allocated(allocated_value), FieldType::Constant(constant_value)) => {
                let constant_gadget = FpGadget::from(&mut cs, constant_value);

                constant_gadget.conditional_enforce_equal(cs, allocated_value, condition)
            }
        }
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as ConditionalEqGadget<F>>::cost()
    }
}

impl<F: Field + PrimeField> CondSelectGadget<F> for FieldType<F> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            if cond { Ok(first.clone()) } else { Ok(second.clone()) }
        } else {
            let first_gadget = first.allocated(&mut cs)?;
            let second_gadget = second.allocated(&mut cs)?;
            let result = FpGadget::conditionally_select(cs, cond, &first_gadget, &second_gadget)?;

            Ok(FieldType::Allocated(result))
        }
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as CondSelectGadget<F>>::cost()
    }
}

impl<F: Field + PrimeField> ToBitsGadget<F> for FieldType<F> {
    fn to_bits<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits(cs)
    }

    fn to_bits_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits_strict(cs)
    }
}

impl<F: Field + PrimeField> ToBytesGadget<F> for FieldType<F> {
    fn to_bytes<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes(cs)
    }

    fn to_bytes_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes_strict(cs)
    }
}

impl<F: Field + PrimeField> std::fmt::Display for FieldType<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.get_value().ok_or(std::fmt::Error))
    }
}
