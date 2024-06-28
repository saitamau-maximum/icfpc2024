use crate::tokenizer::Token;

const INTEGER_ASCII: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn parse_integer(indicator: Token) -> usize {
    assert!(matches!(indicator, Token::Integer(_)));
    let mut result = 0;
    for c in indicator.to_string().chars() {
        let index = INTEGER_ASCII.find(c).unwrap();
        result = result * INTEGER_ASCII.len() + index;
    }
    result
}

const STRING_ASCII: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

fn parse_string(indicator: Token) -> String {
    assert!(matches!(indicator, Token::String(_)));
    let mut result = String::new();
    for c in indicator.to_string().chars() {
        let index = INTEGER_ASCII.find(c).unwrap();
        result.push(STRING_ASCII.chars().nth(index).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer() {
        assert_eq!(INTEGER_ASCII.len(), 94);
        let input = Token::Integer("/6".to_string());
        let expected = 1337;
        assert_eq!(parse_integer(input), expected);
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(STRING_ASCII.len(), 94);
        let input = Token::String("B%,,/}Q/2,$_".to_string());
        let expected = "Hello World!";
        assert_eq!(parse_string(input), expected);
    }
}
