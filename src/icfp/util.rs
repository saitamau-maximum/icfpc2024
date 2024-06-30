use num_bigint::BigInt;

pub const INTEGER_ASCII: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
pub const STRING_ASCII: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

pub fn convert_integer_to_bigint(value: String) -> BigInt {
    let mut result = BigInt::from(0);
    for c in value.chars() {
        let index = INTEGER_ASCII.find(c).unwrap();
        result = result * INTEGER_ASCII.len() + index;
    }
    result
}

pub fn convert_integer(value: String) -> usize {
    let mut result = 0;
    for c in value.chars() {
        let index = INTEGER_ASCII.find(c).unwrap();
        result = result * INTEGER_ASCII.len() + index;
    }
    result
}

pub fn deconvert_integer_from_bigint(value: BigInt) -> String {
    let mut result = String::new();
    let mut value2 = value.clone();
    while value2 > BigInt::from(0) {
        let index = value2.clone() % INTEGER_ASCII.len();
        let usize_index = index.to_string().parse::<usize>().unwrap();
        result.push(INTEGER_ASCII.chars().nth(usize_index).unwrap());
        value2 /= INTEGER_ASCII.len();
    }
    if value == BigInt::from(0) {
        result.push(INTEGER_ASCII.chars().nth(0).unwrap());
    }
    result.chars().rev().collect()
}

pub fn deconvert_integer(value: usize) -> String {
    let mut result = String::new();
    let mut value2 = value;
    while value2 > 0 {
        let index = value2 % INTEGER_ASCII.len();
        result.push(INTEGER_ASCII.chars().nth(index).unwrap());
        value2 /= INTEGER_ASCII.len();
    }
    if value == 0 {
        result.push(INTEGER_ASCII.chars().nth(0).unwrap());
    }
    result.chars().rev().collect()
}

pub fn convert_string(value: String) -> String {
    let mut result = String::new();
    for c in value.chars() {
        let index = INTEGER_ASCII.find(c).unwrap();
        result.push(STRING_ASCII.chars().nth(index).unwrap());
    }
    result
}

pub fn deconvert_string(value: String) -> String {
    let mut result = String::new();
    for c in value.chars() {
        let index = STRING_ASCII.find(c).unwrap();
        result.push(INTEGER_ASCII.chars().nth(index).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deconvert_integer() {
        assert_eq!(
            deconvert_integer_from_bigint(BigInt::from(3389)),
            "E&".to_string()
        );
    }
}
