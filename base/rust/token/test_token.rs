use token::*;

#[test]
fn test_token() {
    let token1 = Token::new();
    assert_eq!(*token1, 1);
    let token2 = Token::new();
    assert_eq!(*token2, 2);
}