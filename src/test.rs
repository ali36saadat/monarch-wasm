use crate::core::Error;

pub type Validator<T> = fn(T) -> Result<T, Error>;

pub fn with_checks<T, R>(
    args: Vec<T>,
    validators: Vec<Validator<T>>,
    f: impl Fn(Vec<T>) -> R,
) -> Result<R, Error> {
    if args.len() != validators.len() {
        return Err(Error::Invalid("args and validators length mismatch"));
    }

    let mut validated = Vec::with_capacity(args.len());
    for (arg, validator) in args.into_iter().zip(validators.into_iter()) {
        validated.push(validator(arg)?);
    }

    Ok(f(validated))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Error;

    fn positive(x: i32) -> Result<i32, Error> {
        if x > 0 {
            Ok(x)
        } else {
            Err(Error::Invalid("must be positive"))
        }
    }

    fn nonzero(x: i32) -> Result<i32, Error> {
        if x != 0 {
            Ok(x)
        } else {
            Err(Error::Invalid("must be nonzero"))
        }
    }

    #[test]
    fn valid_inputs_work() {
        let args = vec![5, 10];
        let validators: Vec<Validator<i32>> = vec![positive, nonzero];
        let result = with_checks(args, validators, |vals| vals.iter().sum::<i32>());
        assert_eq!(result.unwrap(), 15);
    }

    #[test]
    fn invalid_input_fails() {
        let args = vec![5, 0];
        let validators: Vec<Validator<i32>> = vec![positive, nonzero];
        let result = with_checks(args, validators, |vals| vals.iter().sum::<i32>());
        match result {
            Err(Error::Invalid(msg)) if msg == "must be nonzero" => {}
            _ => panic!("expected 'must be nonzero' error"),
        }
    }

    #[test]
    fn length_mismatch_fails() {
        let args = vec![1];
        let validators: Vec<Validator<i32>> = vec![positive, nonzero];
        let result = with_checks(args, validators, |vals| vals.len());
        match result {
            Err(Error::Invalid(msg)) if msg == "args and validators length mismatch" => {} 
            _ => panic!("expected length mismatch error"),
        }
    }
}
