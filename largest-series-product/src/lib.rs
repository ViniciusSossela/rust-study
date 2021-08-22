#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

/*
    unwrap -> extraí o valor que esta dentro, fn como first(), encapsula o valor dentro de Options, e então se faz o unwrap para extrair o valor
    to_owned -> tipo um copy/clone de object, devido a imutability
    match -> é tipo um swith, mais poderoso
*/ 
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let v_alphas = string_digits.matches(char::is_alphabetic).collect::<Vec<&str>>();

    if span > string_digits.len() { return Err(Error::SpanTooLong); }
    else if span == 0 { return Ok(1u64); }
    else if v_alphas.len() > 0 {
        return Err(Error::InvalidDigit(v_alphas
            .first().unwrap()
            .to_owned()
            .to_string()
            .pop().unwrap()));
    }

    let mut digits: Vec<u64> = string_digits
        .split("")
        .map(|s| s.parse())
        .filter(|s| match s {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|s| s.unwrap())
        .collect();
    digits.reverse();
    
    Ok(windows_values(digits, span).first().unwrap().to_owned())
}

/*
    windows -> retornar um interator do tamanh/size:
    /// let slice = ['r', 'u', 's', 't'];
    /// let mut iter = slice.windows(2);
    /// assert_eq!(iter.next().unwrap(), &['r', 'u']);
    /// assert_eq!(iter.next().unwrap(), &['u', 's']);
    /// assert_eq!(iter.next().unwrap(), &['s', 't']);
    /// assert!(iter.next().is_none());
    
    fold -> tipo um REDUCE()
*/ 
fn windows_values(digits: Vec<u64>, span: usize) -> Vec<u64> {
    /* Depois aplicamos o `.fold(1u64, |acc, i| acc * i))`,
    que inicia a multiplicação com um `1u64`, e depois
    multiplicamos cada um deles pelo acumulador `acc`.*/

    let mut windows_values = digits
        .windows(span)
        .map(|w| w.to_vec())
        .map(|v| v.iter().fold(1, |acc, i| acc*i))
        .collect::<Vec<u64>>();
    windows_values.sort();
    windows_values.reverse();
    windows_values
}
