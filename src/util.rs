const INTEGER_ASCII: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
const STRING_ASCII: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

pub fn convert_integer(value: String) -> usize {
    let mut result = 0;
    for c in value.chars() {
        let index = INTEGER_ASCII.find(c).unwrap();
        result = result * INTEGER_ASCII.len() + index;
    }
    result
}

pub fn deconvert_integer(value: usize) -> String {
    let mut result = String::new();
    let mut value = value;
    while value > 0 {
        let index = value % INTEGER_ASCII.len();
        result.push(INTEGER_ASCII.chars().nth(index).unwrap());
        value /= INTEGER_ASCII.len();
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
    fn test_deconvert_string() {
        assert_eq!(deconvert_string("get echo".to_string()), "123");
    }
}
