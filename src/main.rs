use std::{
    rc::Rc,
    fmt::{self, Formatter},
    io::{stdin, stdout, Write}
};

mod operations;
use operations::*;


fn main() -> Result<(), Error> {
    let operations: Vec<Rc<dyn Operation>> = vec![
        Rc::new(Exponentiation::default()),
        Rc::new(Addition::default()),
        Rc::new(Subtraction::default()),
        Rc::new(Multiplication::default()),
        Rc::new(Division::default()),
        Rc::new(Juxtaposition::default()),
    ];


    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().map_err(|_| Error::InvalidLiteralError("".to_string()))?;
        stdin().read_line(&mut input).map_err(|_| Error::InvalidLiteralError(input.clone()))?;
        match eval(operations.clone(), input.clone().as_str()) {
            Ok(num) => println!("{}", num),
            Err(e) => eprintln!("{}", e),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidLiteralError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidLiteralError(e) => { write!(f, "invalid literal: '{}'", e) }
        }
    }
}


fn eval<'a>(operations: Vec<Rc<dyn Operation>>, input: &str) -> Result<f64, Error> {
    let mut working_copy = input.trim().clone().to_string();

    let mut count = 0;
    let mut start = 0;
    let mut index = 0;
    for char in working_copy.clone().chars() {
        match char {
            '(' => {
                count += 1;
                if count == 1 { start = index; }
            },
            ')' => {
                count -= 1;
                if count == 0 {
                    working_copy.replace_range(start..index + 1,
                    format!(" {} ", eval(operations.clone(), &working_copy[start + 1..index])?).as_str());
                }
            },
            _   => { }
        }
        index += 1;
    };

    loop {
        let min = operations.iter()
                .filter(|x| working_copy.find(x.symbol()).is_some())
                .min_by_key(|x| x.precedence())
                .map(|x| x.precedence()).unwrap_or(0);
        let op = operations.iter().filter(|x| x.precedence() == min).max_by_key(|x| working_copy.find(x.symbol()).unwrap_or(usize::MIN)).unwrap();
        if let Some(pos) = working_copy.find(op.symbol()) {
            let max_index = working_copy.len();

            working_copy.replace_range(0..max_index, op.evaluate(
                    eval(operations.clone(), &working_copy[0..pos])?,
                    eval(operations.clone(), &working_copy[pos + 1..max_index])?
            ).to_string().as_str());
        } else if let Ok(value) = working_copy.parse::<f64>() {
            return Ok(value);
        } else {
            return Err(Error::InvalidLiteralError(working_copy.clone()));
        }
    }

}