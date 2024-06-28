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
}
