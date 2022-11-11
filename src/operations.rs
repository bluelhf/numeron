

#[derive(Default)]
pub struct Exponentiation;
impl Operation for Exponentiation {
    fn symbol(&self) -> &'static str { "^" }
    fn precedence(&self) -> u16 { 6 }
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64 { lhs.powf(rhs) }
}

#[derive(Default)]
pub struct Juxtaposition;
impl Operation for Juxtaposition {
    fn symbol(&self) -> &'static str { " " }
    fn precedence(&self) -> u16 { 6 }
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64 {
        eprintln!("WARN: Possible juxtaposition ambiguity, evaluating before product and quotient");
        lhs * rhs
    }
}

#[derive(Default)]
pub struct Multiplication;
impl Operation for Multiplication {
    fn symbol(&self) -> &'static str { "*" }
    fn precedence(&self) -> u16 { 5 }
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64 { lhs * rhs }
}

#[derive(Default)]
pub struct Division;
impl Operation for Division {
    fn symbol(&self) -> &'static str { "/" }
    fn precedence(&self) -> u16 { 5 }
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64 { lhs / rhs }
}

#[derive(Default)]
pub struct Addition;
impl Operation for Addition {
    fn symbol(&self) -> &'static str { "+" }
    fn precedence(&self) -> u16 { 0 }
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64 { lhs + rhs }
}

#[derive(Default)]
pub struct Subtraction;
impl Operation for Subtraction {
    fn symbol(&self) -> &'static str { "-" }
    fn precedence(&self) -> u16 { 0 }
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64 { lhs - rhs }
}

pub trait Operation {
    fn symbol(&self) -> &'static str;
    fn precedence(&self) -> u16;
    fn evaluate(&self, lhs: f64, rhs: f64) -> f64;
}

