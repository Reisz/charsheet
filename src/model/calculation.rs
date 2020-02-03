use super::ValueId;
use std::{
    cmp::{max, min},
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
};

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Min,
    Max,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
}

impl BinaryOp {
    fn exec(&self, a: i32, b: i32) -> i32 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
            Self::Rem => a % b,
            Self::Min => min(a, b),
            Self::Max => max(a, b),
            Self::Eq => (a == b) as i32,
            Self::Ne => (a != b) as i32,
            Self::Gt => (a > b) as i32,
            Self::Ge => (a >= b) as i32,
            Self::Lt => (a < b) as i32,
            Self::Le => (a <= b) as i32,
            Self::And => (a != 0 && b != 0) as i32,
            Self::Or => (a != 0 || b != 0) as i32,
        }
    }
}

enum UnaryOp {
    Abs,
    Neg,
    Not,
}

impl UnaryOp {
    fn exec(&self, val: i32) -> i32 {
        match self {
            Self::Abs => val.abs(),
            Self::Neg => -val,
            Self::Not => (val == 0) as i32,
        }
    }
}

enum Element {
    Const(i32),
    Value(usize),
    MultiplyF(f32, usize),

    Unary(UnaryOp, usize),
    Binary(BinaryOp, usize, usize),
}

/// Represents a calculation based on values of a character.
pub struct Calculation {
    storage: Vec<Element>,
    values: Vec<ValueId>,

    output: usize,
}

/// Coversion into a Calculation.
pub trait IntoCalculation {
    /// Performs the conversion.
    fn into_calc(self) -> Calculation;
}

impl<T: Into<Calculation>> IntoCalculation for T {
    fn into_calc(self) -> Calculation {
        self.into()
    }
}

impl Calculation {
    fn insert(mut self, element: Element) -> Self {
        let idx = self.storage.len();
        self.storage.push(element);
        self.output = idx;
        self
    }

    fn insert_value(&mut self, id: ValueId) -> usize {
        self.values
            .iter()
            .position(|&other_id| other_id == id)
            .unwrap_or_else(|| {
                let idx = self.values.len();
                self.values.push(id);
                idx
            })
    }

    fn append(&mut self, other: Calculation) -> usize {
        let offset = self.storage.len();

        let values: Vec<_> = other
            .values
            .into_iter()
            .map(|id| self.insert_value(id))
            .collect();

        self.storage
            .extend(other.storage.into_iter().map(|element| match element {
                Element::Const(c) => Element::Const(c),
                Element::Value(idx) => Element::Value(values[idx]),
                Element::MultiplyF(fac, val) => Element::MultiplyF(fac, val + offset),
                Element::Unary(op, val) => Element::Unary(op, val),
                Element::Binary(op, a, b) => Element::Binary(op, a + offset, b + offset),
            }));

        other.output + offset
    }

    fn unary(self, op: UnaryOp) -> Self {
        let val = self.output;
        self.insert(Element::Unary(op, val))
    }

    /// Evaluate to the absolute value.
    pub fn abs(self) -> Self {
        self.unary(UnaryOp::Abs)
    }

    fn binary(mut self, other: Calculation, op: BinaryOp) -> Self {
        let a = self.output;
        let b = self.append(other);
        self.insert(Element::Binary(op, a, b))
    }

    /// Multiply the calculation with a constant float.
    pub fn mul_f(self, f: f32) -> Self {
        let output = self.output;
        self.insert(Element::MultiplyF(f, output))
    }

    /// Evaluate to the smaller value between a and b.
    pub fn min(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Min)
    }

    /// Evaluate to the bigger value between a and b.
    pub fn max(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Max)
    }

    /// Evaluate to 1 if `a == b` else 0.
    pub fn eq(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Eq)
    }

    /// Evaluate to 1 if `a != b` else 0.
    pub fn ne(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Ne)
    }

    /// Evaluate to 1 if `a > b` else 0.
    pub fn gt(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Gt)
    }

    /// Evaluate to 1 if `a >= b` else 0.
    pub fn ge(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Ge)
    }

    /// Evaluate to 1 if `a < b` else 0.
    pub fn lt(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Lt)
    }

    /// Evaluate to 1 if `a <= b` else 0.
    pub fn le(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Le)
    }

    /// Evaluate to 1 if `a != 0 && b != 0` else 0.
    pub fn and(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::And)
    }

    /// Evaluate to 1 if `a != 0 || b != 0` else 0.
    pub fn or(self, other: impl IntoCalculation) -> Self {
        self.binary(other.into_calc(), BinaryOp::Or)
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = ValueId> + '_ {
        self.values.iter().cloned()
    }

    pub(crate) fn get(&self, values: &[i32]) -> i32 {
        self.eval(values, self.output)
    }

    fn eval(&self, values: &[i32], idx: usize) -> i32 {
        let eval = |&idx| self.eval(values, idx);

        match &self.storage[idx as usize] {
            Element::Const(v) => *v,
            Element::Value(idx) => values[*idx],

            Element::MultiplyF(fac, val) => (eval(val) as f32 * fac) as i32,
            Element::Unary(op, val) => op.exec(eval(val)),
            Element::Binary(op, a, b) => op.exec(eval(a), eval(b)),
        }
    }
}

macro_rules! binary {
    ($trait:ident($fn:ident) -> $op:expr) => {
        impl $trait for Calculation {
            type Output = Self;

            fn $fn(self, other: Calculation) -> Self {
                self.binary(other, $op)
            }
        }

        impl $trait<i32> for Calculation {
            type Output = Self;

            fn $fn(self, c: i32) -> Self {
                self.binary(c.into(), $op)
            }
        }

        impl $trait<ValueId> for Calculation {
            type Output = Self;

            fn $fn(self, id: ValueId) -> Self {
                self.binary(id.into(), $op)
            }
        }

        impl $trait for ValueId {
            type Output = Calculation;

            fn $fn(self, id: ValueId) -> Calculation {
                Calculation::from(self).binary(id.into(), $op)
            }
        }

        impl $trait<i32> for ValueId {
            type Output = Calculation;

            fn $fn(self, c: i32) -> Calculation {
                Calculation::from(self).binary(c.into(), $op)
            }
        }

        impl $trait<ValueId> for i32 {
            type Output = Calculation;

            fn $fn(self, id: ValueId) -> Calculation {
                Calculation::from(self).binary(id.into(), $op)
            }
        }

        impl $trait<Calculation> for i32 {
            type Output = Calculation;

            fn $fn(self, other: Calculation) -> Calculation {
                Calculation::from(self).binary(other, $op)
            }
        }
    };
}

binary!(Add(add) -> BinaryOp::Add);
binary!(Sub(sub) -> BinaryOp::Sub);
binary!(Mul(mul) -> BinaryOp::Mul);
binary!(Div(div) -> BinaryOp::Div);
binary!(Rem(rem) -> BinaryOp::Rem);

macro_rules! unary {
    ($trait:ident($fn:ident) -> $op:expr) => {
        impl $trait for Calculation {
            type Output = Self;

            fn $fn(self) -> Self {
                self.unary($op)
            }
        }

        impl $trait for ValueId {
            type Output = Calculation;

            fn $fn(self) -> Calculation {
                Calculation::from(self).unary($op)
            }
        }
    };
}

unary!(Neg(neg) -> UnaryOp::Neg);
unary!(Not(not) -> UnaryOp::Not);

impl From<ValueId> for Calculation {
    fn from(id: ValueId) -> Self {
        Self {
            storage: vec![Element::Value(0)],
            values: vec![id],

            output: 0,
        }
    }
}

impl From<i32> for Calculation {
    fn from(c: i32) -> Self {
        Self {
            storage: vec![Element::Const(c)],
            values: Vec::new(),

            output: 0,
        }
    }
}