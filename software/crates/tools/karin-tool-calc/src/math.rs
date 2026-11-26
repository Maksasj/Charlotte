pub fn eval_expr(input: &str) -> Result<f64, String> {
    let normalized = input
        .replace('×', "*")
        .replace('÷', "/")
        .replace("plus", "+")
        .replace("minus", "-")
        .replace("times", "*");
    let tokens = tokenize(&normalized)?;
    let mut cursor = 0;
    let value = parse_add_sub(&tokens, &mut cursor)?;

    if cursor != tokens.len() {
        return Err("unexpected trailing tokens".into());
    }
    if !value.is_finite() {
        return Err("non-finite result".into());
    }
    Ok(value)
}

pub fn format_number(value: f64) -> String {
    if value.fract().abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.4}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[derive(Debug, Clone)]
enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        match chars[idx] {
            ' ' | '\t' | '\n' => idx += 1,
            '+' => {
                tokens.push(Token::Plus);
                idx += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                idx += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                idx += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                idx += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                idx += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                idx += 1;
            }
            ch if ch.is_ascii_digit() || ch == '.' => {
                let start = idx;
                while idx < chars.len() && (chars[idx].is_ascii_digit() || chars[idx] == '.') {
                    idx += 1;
                }
                let raw: String = chars[start..idx].iter().collect();
                let number: f64 = raw.parse().map_err(|_| format!("invalid number '{raw}'"))?;
                tokens.push(Token::Num(number));
            }
            other => return Err(format!("invalid character '{other}'")),
        }
    }

    if tokens.is_empty() {
        return Err("empty expression".into());
    }
    Ok(tokens)
}

fn parse_add_sub(tokens: &[Token], cursor: &mut usize) -> Result<f64, String> {
    let mut left = parse_mul_div(tokens, cursor)?;
    while *cursor < tokens.len() {
        match tokens[*cursor] {
            Token::Plus => {
                *cursor += 1;
                left += parse_mul_div(tokens, cursor)?;
            }
            Token::Minus => {
                *cursor += 1;
                left -= parse_mul_div(tokens, cursor)?;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_mul_div(tokens: &[Token], cursor: &mut usize) -> Result<f64, String> {
    let mut left = parse_unary(tokens, cursor)?;
    while *cursor < tokens.len() {
        match tokens[*cursor] {
            Token::Star => {
                *cursor += 1;
                left *= parse_unary(tokens, cursor)?;
            }
            Token::Slash => {
                *cursor += 1;
                let rhs = parse_unary(tokens, cursor)?;
                if rhs.abs() < 1e-12 {
                    return Err("division by zero".into());
                }
                left /= rhs;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_unary(tokens: &[Token], cursor: &mut usize) -> Result<f64, String> {
    if matches!(tokens.get(*cursor), Some(Token::Minus)) {
        *cursor += 1;
        return Ok(-parse_unary(tokens, cursor)?);
    }
    parse_primary(tokens, cursor)
}

fn parse_primary(tokens: &[Token], cursor: &mut usize) -> Result<f64, String> {
    match tokens.get(*cursor) {
        Some(Token::Num(n)) => {
            *cursor += 1;
            Ok(*n)
        }
        Some(Token::LParen) => {
            *cursor += 1;
            let value = parse_add_sub(tokens, cursor)?;
            if !matches!(tokens.get(*cursor), Some(Token::RParen)) {
                return Err("missing closing parenthesis".into());
            }
            *cursor += 1;
            Ok(value)
        }
        Some(_) => Err("expected number or '('".into()),
        None => Err("unexpected end of expression".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_expr_works() {
        assert_eq!(eval_expr("12 * 7").unwrap(), 84.0);
        assert_eq!(eval_expr("(2 + 3) * 4 - 6 / 2").unwrap(), 17.0);
        assert!(eval_expr("5 / 0").is_err());
    }
}
