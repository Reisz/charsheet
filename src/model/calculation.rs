
use super::ValueId;
use std::fmt::{self, Display, Formatter};

enum Element {
    Const(i32),
    Value(usize),

    Add(u32, u32),
    Multiply(u32, u32),
    MultiplyF(u32, f32),

    Equals(u32, u32),
    GreaterThan(u32, u32),
    Not(u32),
    And(u32, u32),
    Or(u32, u32),
}

/// Represents a calculation based on values of a character.
#[derive(Default)]
pub struct Calculation {
    storage: Vec<Element>,
    values: Vec<ValueId>,

    output: Option<u32>,
}

impl Calculation {
    /// Get a new empty calculation.
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, element: Element) -> u32 {
        let idx = self.storage.len() as u32;
        self.storage.push(element);
        idx
    }

    /// Use a constant value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use charsheet::model::Calculation;
    ///
    /// let mut calc = Calculation::new();
    /// let c = calc.constant(5);
    /// calc.set_output(c);
    /// assert_eq!(format!("{}", calc), "5");
    /// ```
    pub fn constant(&mut self, c: i32) -> u32 {
        self.insert(Element::Const(c))
    }

    /// Read a value from the character.
    pub fn value(&mut self, id: ValueId) -> u32 {
        let element = Element::Value(
            if let Some(idx) = self.values.iter().position(|&other_id| other_id == id) {
                idx
            } else {
                let idx = self.values.len();
                self.values.push(id);
                idx
            },
        );
        self.insert(element)
    }

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

    /// Multiply two elements.
    pub fn multiply(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Multiply(a, b))
    }

    /// Multiply an element with a constant float.
    pub fn multiply_float(&mut self, val: u32, f: f32) -> u32 {
        self.insert(Element::MultiplyF(val, f))
    }

    /// Evaluate to 1 if `a == b` else 0.
    pub fn equals(&mut self, a: u32, b: u32) -> u32 {
        self.insert(Element::Equals(a, b))
    }

    /// Evaluate to 1 if `a > b` else 0.
    pub fn greater(&mut self, a: u32, b:u32) -> u32 {
        self.insert(Element::GreaterThan(a, b))
    }

    /// Evaluate to 1 if `val == 0` else 0.
    pub fn not(&mut self, val: u32) -> u32 {
        self.insert(Element::Not(val))
    }

    /// Evaluate to 1 if `a != 0 && b != 0` else 0.
    pub fn and(&mut self, a: u32, b:u32) -> u32 {
        self.insert(Element::And(a, b))
    }

    /// Evaluate to 1 if `a != 0 || b != 0` else 0.
    pub fn or(&mut self, a: u32, b:u32) -> u32 {
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
            Element::Multiply(a, b) => eval(a) * eval(b),
            Element::MultiplyF(a, f) => (eval(a) as f32 * f) as i32,

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
            Element::Multiply(a, b) => op(a, " * ", b),
            Element::MultiplyF(val, fac) => {
                write!(f, "({} *", fac)?;
                self.write(f, val)?;
                f.write_str(")")
            },

            Element::Equals(a, b) => op(a, " == ", b),
            Element::GreaterThan(a, b) => op(a, " > ", b),
            Element::Not(val) => {
                f.write_str("!")?;
                self.write(f, val)
            },
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_element() {
        let calc = Calculation::new();
        assert_eq!(calc.get(&vec![]), 0);
    }

    #[test]
    fn addition() {
        let mut calc = Calculation::new();
        let a = calc.constant(2);
        let b = calc.constant(3);
        let add = calc.add(a, b);
        calc.set_output(add);
        assert_eq!(calc.get(&vec![]), 5);
    }

    #[test]
    fn multiply() {
        let mut calc = Calculation::new();
        let a = calc.constant(2);
        let b = calc.constant(3);
        let mul = calc.multiply(a, b);
        calc.set_output(mul);
        assert_eq!(calc.get(&vec![]), 6);
    }

    #[test]
    fn multiply_float() {
        let mut calc = Calculation::new();
        let val = calc.constant(12);
        let mul = calc.multiply_float(val, 0.5);
        calc.set_output(mul);
        assert_eq!(calc.get(&vec![]), 6);
    }

    #[test]
    fn equals() {}
}
