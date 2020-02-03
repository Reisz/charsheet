use super::ValueId;
use std::{
    cmp::{max, min},
    fmt::{self, Display, Formatter},
};

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Min,
    Max,
    Equals,
    GreaterThan,
    And,
    Or,
}

enum UnaryOp {
    Neg,
    Not,
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

impl Calculation {
    /// Create a new calculation based on a character value.
    pub fn new(id: ValueId) -> Self {
        Self {
            storage: vec![Element::Value(0)],
            values: vec![id],

            output: 0,
        }
    }

    fn insert(&mut self, element: Element) {
        let idx = self.storage.len();
        self.storage.push(element);
        self.output = idx;
    }

    fn append(&mut self, other: Calculation) {
        let offset = self.storage.len();
        self.storage
            .extend(other.storage.into_iter().map(|element| match element {
                Element::Const(c) => Element::Const(c),
                Element::Value(_) => todo!(),
                Element::MultiplyF(fac, val) => Element::MultiplyF(fac, val + offset),
                Element::Unary(op, val) => Element::Unary(op, val),
                Element::Binary(op, a, b) => Element::Binary(op, a + offset, b + offset),
            }))
    }

    // Read a value from the character.
    // pub fn value(&mut self, id: ValueId) -> u32 {
    //     let element = Element::Value(
    //         if let Some(idx) = self.values.iter().position(|&other_id| other_id == id) {
    //             idx
    //         } else {
    //             let idx = self.values.len();
    //             self.values.push(id);
    //             idx
    //         },
    //     );
    // }

    /// Add two elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let add = calc.add(c1, c2);
    ///
    /// calc.set_output(add);
    /// assert_eq!(format!("{}", calc), "(2 + 3)");
    /// ```
    pub fn add(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Add(a, b))
    }

    /// Subtract two elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let add = calc.sub(c1, c2);
    ///
    /// calc.set_output(add);
    /// assert_eq!(format!("{}", calc), "(2 - 3)");
    /// ```
    pub fn sub(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Sub(a, b))
    }

    /// Multiply two elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let mul = calc.multiply(c1, c2);
    ///
    /// calc.set_output(mul);
    /// assert_eq!(format!("{}", calc), "(2 * 3)");
    /// ```
    pub fn multiply(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Multiply(a, b))
    }

    /// Multiply an element with a constant float.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c = calc.constant(2);
    ///
    /// let mulf = calc.multiply_float(c, 0.5);
    ///
    /// calc.set_output(mulf);
    /// assert_eq!(format!("{}", calc), "(0.5 * 2)");
    /// ```
    pub fn multiply_float(&mut self, val: u32, f: f32) -> u32 {
        self.insert(Element::MultiplyF(val, f))
    }

    /// Evaluate to the smaller value between a and b.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let eq = calc.min(c1, c2);
    ///
    /// calc.set_output(eq);
    /// assert_eq!(format!("{}", calc), "min(2, 3)");
    /// ```
    pub fn min(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Min(a, b))
    }

    /// Evaluate to the bigger value between a and b.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let eq = calc.max(c1, c2);
    ///
    /// calc.set_output(eq);
    /// assert_eq!(format!("{}", calc), "max(2, 3)");
    /// ```
    pub fn max(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Max(a, b))
    }

    /// Evaluate to 1 if `a == b` else 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let eq = calc.equals(c1, c2);
    ///
    /// calc.set_output(eq);
    /// assert_eq!(format!("{}", calc), "(2 == 3)");
    /// ```
    pub fn equals(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Equals(a, b))
    }

    /// Evaluate to 1 if `a > b` else 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let gt = calc.greater(c1, c2);
    ///
    /// calc.set_output(gt);
    /// assert_eq!(format!("{}", calc), "(2 > 3)");
    /// ```
    pub fn greater(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::GreaterThan(a, b))
    }

    /// Evaluate to 1 if `val == 0` else 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c = calc.constant(2);
    ///
    /// let not = calc.not(c);
    ///
    /// calc.set_output(not);
    /// assert_eq!(format!("{}", calc), "!2");
    /// ```
    pub fn not(&mut self, val: u32) -> u32 {
        self.insert(Element::Not(val))
    }

    /// Evaluate to 1 if `a != 0 && b != 0` else 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let and = calc.and(c1, c2);
    ///
    /// calc.set_output(and);
    /// assert_eq!(format!("{}", calc), "(2 && 3)");
    /// ```
    pub fn and(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::And(a, b))
    }

    /// Evaluate to 1 if `a != 0 || b != 0` else 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c1 = calc.constant(2);
    /// let c2 = calc.constant(3);
    ///
    /// let or = calc.or(c1, c2);
    ///
    /// calc.set_output(or);
    /// assert_eq!(format!("{}", calc), "(2 || 3)");
    /// ```
    pub fn or(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Or(a, b))
    }

    /// Set the eleemnt to use as output.
    pub fn set_output(&mut self, id: u32) {
        // TODO: check unused
        self.output = Some(id);
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = ValueId> + '_ {
        self.values.iter().cloned()
    }

    pub(crate) fn get(&self, values: &[i32]) -> i32 {
        if let Some(output) = self.output {
            self.eval(values, output)
        } else {
            0
        }
    }

    fn eval(&self, values: &[i32], idx: u32) -> i32 {
        let eval = |idx| self.eval(values, idx);

        match self.storage[idx as usize] {
            Element::Const(v) => v,
            Element::Value(idx) => values[idx],

            Element::Add(a, b) => eval(a) + eval(b),
            Element::Sub(a, b) => eval(a) - eval(b),
            Element::Multiply(a, b) => eval(a) * eval(b),
            Element::MultiplyF(a, f) => (eval(a) as f32 * f) as i32,

            Element::Min(a, b) => min(eval(a), eval(b)),
            Element::Max(a, b) => max(eval(a), eval(b)),

            Element::Equals(a, b) => (eval(a) == eval(b)) as i32,
            Element::GreaterThan(a, b) => (eval(a) > eval(b)) as i32,
            Element::Not(val) => (eval(val) == 0) as i32,
            Element::And(a, b) => (eval(a) != 0 && eval(b) != 0) as i32,
            Element::Or(a, b) => (eval(a) != 0 || eval(b) != 0) as i32,
        }
    }

    fn write(&self, f: &mut Formatter<'_>, idx: u32) -> fmt::Result {
        let mut op = |a, sep, b| -> fmt::Result {
            f.write_str("(")?;
            self.write(f, a)?;
            f.write_str(sep)?;
            self.write(f, b)?;
            f.write_str(")")
        };

        match self.storage[idx as usize] {
            Element::Const(v) => write!(f, "{}", v),
            Element::Value(_) => write!(f, "?"),

            Element::Add(a, b) => op(a, " + ", b),
            Element::Sub(a, b) => op(a, " - ", b),
            Element::Multiply(a, b) => op(a, " * ", b),
            Element::MultiplyF(val, fac) => {
                write!(f, "({} * ", fac)?;
                self.write(f, val)?;
                f.write_str(")")
            }

            Element::Min(a, b) => {
                f.write_str("min(")?;
                self.write(f, a)?;
                f.write_str(", ")?;
                self.write(f, b)?;
                f.write_str(")")
            }
            Element::Max(a, b) => {
                f.write_str("max(")?;
                self.write(f, a)?;
                f.write_str(", ")?;
                self.write(f, b)?;
                f.write_str(")")
            }

            Element::Equals(a, b) => op(a, " == ", b),
            Element::GreaterThan(a, b) => op(a, " > ", b),
            Element::Not(val) => {
                f.write_str("!")?;
                self.write(f, val)
            }
            Element::And(a, b) => op(a, " && ", b),
            Element::Or(a, b) => op(a, " || ", b),
        }
    }
}

impl Display for Calculation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(output) = self.output {
            self.write(f, output)
        } else {
            Ok(())
        }
    }
}
