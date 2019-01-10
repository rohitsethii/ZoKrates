use ir::variable::Variable;
use num::Zero;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub};
use zokrates_field::field::Field;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuadComb<T: Field> {
    pub left: LinComb<T>,
    pub right: LinComb<T>,
}

impl<T: Field> QuadComb<T> {
    pub fn from_linear_combinations(left: LinComb<T>, right: LinComb<T>) -> Self {
        QuadComb { left, right }
    }
}

impl<T: Field> From<Variable> for QuadComb<T> {
    fn from(v: Variable) -> QuadComb<T> {
        LinComb::from(v).into()
    }
}

impl<T: Field> fmt::Display for QuadComb<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) * ({})", self.left, self.right,)
    }
}

impl<T: Field> From<LinComb<T>> for QuadComb<T> {
    fn from(lc: LinComb<T>) -> QuadComb<T> {
        QuadComb::from_linear_combinations(LinComb::one(), lc)
    }
}

#[derive(PartialEq, Clone, Eq, Debug, Serialize, Deserialize)]
pub struct LinComb<T: Field>(pub HashMap<Variable, T>);

impl<T: Field> LinComb<T> {
    pub fn summand<U: Into<T>>(mult: U, var: Variable) -> LinComb<T> {
        let mut res = HashMap::new();
        res.insert(var, mult.into());
        LinComb(res)
    }

    pub fn one() -> LinComb<T> {
        Self::summand(1, Variable::One)
    }
}

impl<T: Field> fmt::Display for LinComb<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(k, v)| format!("{} * {}", v, k))
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

impl<T: Field> From<Variable> for LinComb<T> {
    fn from(v: Variable) -> LinComb<T> {
        let mut r = HashMap::new();
        r.insert(v, T::one());
        LinComb(r)
    }
}

impl<T: Field> Add<LinComb<T>> for LinComb<T> {
    type Output = LinComb<T>;

    fn add(self, other: LinComb<T>) -> LinComb<T> {
        let mut res = self.0.clone();
        for (k, v) in other.0 {
            let new_val = v + res.get(&k).unwrap_or(&T::zero());
            if new_val == T::zero() {
                res.remove(&k)
            } else {
                res.insert(k, new_val)
            };
        }
        LinComb(res)
    }
}

impl<T: Field> Sub<LinComb<T>> for LinComb<T> {
    type Output = LinComb<T>;

    fn sub(self, other: LinComb<T>) -> LinComb<T> {
        let mut res = self.0.clone();
        for (k, v) in other.0 {
            let new_val = T::zero() - v + res.get(&k).unwrap_or(&T::zero());
            if new_val == T::zero() {
                res.remove(&k)
            } else {
                res.insert(k, new_val)
            };
        }
        LinComb(res)
    }
}

impl<T: Field> Zero for LinComb<T> {
    fn zero() -> LinComb<T> {
        LinComb(HashMap::new())
    }
    fn is_zero(&self) -> bool {
        self.0.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zokrates_field::field::FieldPrime;

    mod linear {

        use super::*;
        #[test]
        fn add_zero() {
            let a: LinComb<FieldPrime> = LinComb::zero();
            let b: LinComb<FieldPrime> = Variable::Private(42).into();
            let c = a + b.clone();
            assert_eq!(c, b);
        }
        #[test]
        fn add() {
            let a: LinComb<FieldPrime> = Variable::Private(42).into();
            let b: LinComb<FieldPrime> = Variable::Private(42).into();
            let c = a + b.clone();
            let mut expected_map = HashMap::new();
            expected_map.insert(Variable::Private(42), FieldPrime::from(2));
            assert_eq!(c, LinComb(expected_map));
        }
        #[test]
        fn sub() {
            let a: LinComb<FieldPrime> = Variable::Private(42).into();
            let b: LinComb<FieldPrime> = Variable::Private(42).into();
            let c = a - b.clone();
            assert_eq!(c, LinComb::zero());
        }
    }

    mod quadratic {
        use super::*;
        #[test]
        fn from_linear() {
            let a: LinComb<FieldPrime> = LinComb::summand(3, Variable::Private(42))
                + LinComb::summand(4, Variable::Private(33));
            let expected = QuadComb {
                left: LinComb::one(),
                right: a.clone(),
            };
            assert_eq!(QuadComb::from(a), expected);
        }

        #[test]
        fn zero() {
            let a: LinComb<FieldPrime> = LinComb::zero();
            let expected: QuadComb<FieldPrime> = QuadComb {
                left: LinComb::one(),
                right: LinComb::zero(),
            };
            assert_eq!(QuadComb::from(a), expected);
        }
    }
}
