#[derive(Debug)]
pub enum Lexer {
    SPACE, // " "
    NEWLINE, // \n
    BOOL_EQ, // ==
    BOOL_NE, // !=
    BOOL_GT, // >
    BOOL_LT, // <
    BOOL_GE, // >=
    BOOL_LE, // <=
    BOOL_AND, // &&
    BOOL_OR, // ||
    BOOL_NOT, // !!
    ADD, // +
    SUB, // -
    MUL, // *
    DIV, // /
    MOD, // %
    BIN_AND, // &
    BIN_OR, // |
    BIN_XOR, // ^
    BIN_NOT, // !
    BIN_SHL, // <<
    BIN_SHR, // >>
    ASSIGN, // =
    ASSIGN_ADD, // +=
    ASSIGN_SUB, // -=
    ASSIGN_MUL, // *=
    ASSIGN_DIV, // /=
    ASSIGN_MOD, // %=
    ASSIGN_BIN_AND, // &=
    ASSIGN_BIN_OR, // |=
    ASSIGN_BIN_XOR, // ^=
    ASSIGN_BIN_NOT, // !=
    ASSIGN_BIN_SHL, // <<=
    ASSIGN_BIN_SHR, // >>=
    INC, // ++
    DEC, // --

    AT_MARKE, // @
    COMMENT, // //
    DOC_COMMENT, // ///
    ARROW, // ->
    COLON, // :
    DOBLE_COLON, // ::
    SEMICOLON, // ;
    COMMA, // ,
    DOT, // .
    DOBLE_QUOTE, // "
    SINGLE_QUOTE, // '
    BACK_QUOTE, // `
    L_PAREN, // (
    R_PAREN, // )
    L_BRACE, // {
    R_BRACE, // }
    L_BRACKET, // [
    R_BRACKET, // ]
    L_ANGLE, // <
    R_ANGLE, // >

    TEXT(String), // Text
    COMMENT_TEXT(String), // Comment text
    NUMBER(String), // Number

    TYPE(String), // Type

}

impl Lexer {
    pub fn tokenizer(input: &str) -> Vec<Lexer> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == ' ' {
                tokens.push(Lexer::SPACE);
                continue;
            } else if c == '\n' {
                tokens.push(Lexer::NEWLINE);
                continue;
            }

            match c {
                '/' => {
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            // Start of a comment
                            chars.next(); // consume second slash
                            let _is_doc = if let Some(&third) = chars.peek() {
                                if third == '/' {
                                    chars.next(); // consume third slash
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            };
                            let mut comment_text = String::new();
                            while let Some(&ch) = chars.peek() {
                                if ch == '\n' {
                                    break;
                                } else {
                                    comment_text.push(ch);
                                    chars.next();
                                }
                            }
                            tokens.push(Lexer::COMMENT_TEXT(comment_text));
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_DIV);
                            continue;
                        }
                    }
                    tokens.push(Lexer::DIV);
                }
                '=' => {
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(Lexer::BOOL_EQ);
                            continue;
                        }
                    }
                    tokens.push(Lexer::ASSIGN);
                }
                '!' => {
                    if let Some(&next) = chars.peek() {
                        if next == '!' {
                            chars.next();
                            tokens.push(Lexer::BOOL_NOT);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::BOOL_NE);
                            continue;
                        }
                    }
                    tokens.push(Lexer::BIN_NOT);
                }
                '<' => {
                    if let Some(&next) = chars.peek() {
                        if next == '<' {
                            chars.next();
                            if let Some(&next2) = chars.peek() {
                                if next2 == '=' {
                                    chars.next();
                                    tokens.push(Lexer::ASSIGN_BIN_SHL);
                                    continue;
                                }
                            }
                            tokens.push(Lexer::BIN_SHL);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::BOOL_LE);
                            continue;
                        }
                    }
                    tokens.push(Lexer::L_ANGLE);
                }
                '>' => {
                    if let Some(&next) = chars.peek() {
                        if next == '>' {
                            chars.next();
                            if let Some(&next2) = chars.peek() {
                                if next2 == '=' {
                                    chars.next();
                                    tokens.push(Lexer::ASSIGN_BIN_SHR);
                                    continue;
                                }
                            }
                            tokens.push(Lexer::BIN_SHR);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::BOOL_GE);
                            continue;
                        }
                    }
                    tokens.push(Lexer::R_ANGLE);
                }
                '+' => {
                    if let Some(&next) = chars.peek() {
                        if next == '+' {
                            chars.next();
                            tokens.push(Lexer::INC);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_ADD);
                            continue;
                        }
                    }
                    tokens.push(Lexer::ADD);
                }
                '-' => {
                    if let Some(&next) = chars.peek() {
                        if next == '-' {
                            chars.next();
                            tokens.push(Lexer::DEC);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_SUB);
                            continue;
                        } else if next == '>' {
                            chars.next();
                            tokens.push(Lexer::ARROW);
                            continue;
                        }
                    }
                    tokens.push(Lexer::SUB);
                }
                '*' => {
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_MUL);
                            continue;
                        }
                    }
                    tokens.push(Lexer::MUL);
                }
                '%' => {
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_MOD);
                            continue;
                        }
                    }
                    tokens.push(Lexer::MOD);
                }
                '&' => {
                    if let Some(&next) = chars.peek() {
                        if next == '&' {
                            chars.next();
                            tokens.push(Lexer::BOOL_AND);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_BIN_AND);
                            continue;
                        }
                    }
                    tokens.push(Lexer::BIN_AND);
                }
                '|' => {
                    if let Some(&next) = chars.peek() {
                        if next == '|' {
                            chars.next();
                            tokens.push(Lexer::BOOL_OR);
                            continue;
                        } else if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_BIN_OR);
                            continue;
                        }
                    }
                    tokens.push(Lexer::BIN_OR);
                }
                '^' => {
                    if let Some(&next) = chars.peek() {
                        if next == '=' {
                            chars.next();
                            tokens.push(Lexer::ASSIGN_BIN_XOR);
                            continue;
                        }
                    }
                    tokens.push(Lexer::BIN_XOR);
                }
                '@' => tokens.push(Lexer::AT_MARKE),
                ':' => {
                    if let Some(&next) = chars.peek() {
                        if next == ':' {
                            chars.next();
                            tokens.push(Lexer::DOBLE_COLON);
                            continue;
                        }
                    }
                    tokens.push(Lexer::COLON);
                }
                ';' => tokens.push(Lexer::SEMICOLON),
                ',' => tokens.push(Lexer::COMMA),
                '.' => tokens.push(Lexer::DOT),
                '"' => {
                    let mut s = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '"' {
                            break;
                        }
                        s.push(ch);
                    }
                    tokens.push(Lexer::TEXT(s));
                    continue;
                }
                '\'' => {
                    let mut s = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '\'' {
                            break;
                        }
                        s.push(ch);
                    }
                    tokens.push(Lexer::TEXT(s));
                    continue;
                }
                '`' => tokens.push(Lexer::BACK_QUOTE),
                '(' => tokens.push(Lexer::L_PAREN),
                ')' => tokens.push(Lexer::R_PAREN),
                '{' => tokens.push(Lexer::L_BRACE),
                '}' => tokens.push(Lexer::R_BRACE),
                '[' => tokens.push(Lexer::L_BRACKET),
                ']' => tokens.push(Lexer::R_BRACKET),
                ch if ch.is_digit(10) => {
                    let mut number = ch.to_string();
                    while let Some(&next) = chars.peek() {
                        if next.is_digit(10) {
                            number.push(next);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Lexer::NUMBER(number));
                    continue;
                }
                ch if ch.is_alphabetic() || ch == '_' => {
                    let mut text = ch.to_string();
                    while let Some(&next) = chars.peek() {
                        if next.is_alphanumeric() || next == '_' {
                            text.push(next);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let token = match text.as_str() {
                        "usize" | "isize" | "i8" | "i16" | "i32" | "i64" | "i128" |
                        "u8" | "u16" | "u32" | "u64" | "u128" | "bool" | "float" | "double" =>
                            Lexer::TYPE(text),
                        _ => Lexer::TEXT(text),
                    };
                    tokens.push(token);
                    continue;
                }
                _ => {}
            }
        }
        tokens
    }
}

#[derive(Debug)]
pub enum AST {
    // Represents a variable declaration (e.g., "@a: i32= expression;")
    VarDecl {
        var_type: Box<AST>,
        identifier: String,
        initializer: Box<AST>,
    },
    // Represents a literal value (number, string, etc.)
    Literal(String),
    // Represents an identifier usage
    Identifier(String),
    // Represents a binary operation (e.g., addition, subtraction)
    BinaryOp {
        op: &'static str,
        left: Box<AST>,
        right: Box<AST>,
    },
    // Represents a comment
    Comment(String),
    // Represents a block of statements
    Block(Vec<AST>),
    Type(String), // Represents a type
    // Fallback for unrecognized constructs
    Empty,
}

impl AST {
    // A minimal parser stub that transforms a slice of Lexer tokens into an AST.
    // You can extend this to build a full parser.
    pub fn parse(tokens: &[Lexer]) -> Vec<AST> {
        let mut asts = Vec::new();
        let mut pos: usize = 0;

        // Helper: skip whitespace and newlines
        fn skip_whitespace(tokens: &[Lexer], pos: &mut usize) {
            while *pos < tokens.len() {
                match &tokens[*pos] {
                    Lexer::SPACE | Lexer::NEWLINE => *pos += 1,
                    _ => break,
                }
            }
        }

        // Parse a variable declaration of the form:
        // <Type> <identifier> = <literal> ;
        while pos < tokens.len() {
            skip_whitespace(tokens, &mut pos);
            if pos >= tokens.len() {
                break;
            }
        while pos < tokens.len() {
            skip_whitespace(tokens, &mut pos);
            if pos >= tokens.len() {
                break;
            }
            match &tokens[pos] {
                Lexer::AT_MARKE => {
                    pos += 1;
                    skip_whitespace(tokens, &mut pos);
                    let identifier = match tokens.get(pos) {
                        Some(Lexer::TEXT(id)) => {
                            pos += 1;
                            id.clone()
                        },
                        _ => {
                            pos += 1;
                            continue;
                        }
                    };
                    skip_whitespace(tokens, &mut pos);
                    if let Some(Lexer::COLON) = tokens.get(pos) {
                        pos += 1;
                    } else {
                        pos += 1;
                        continue;
                    }
                    skip_whitespace(tokens, &mut pos);
                    let var_type = match tokens.get(pos) {
                        Some(Lexer::TYPE(t)) => {
                            pos += 1;
                            AST::Type(t.clone())
                        },
                        Some(Lexer::TEXT(t)) => {
                            pos += 1;
                            AST::Type(t.clone())
                        },
                        _ => {
                            pos += 1;
                            continue;
                        }
                    };
                    skip_whitespace(tokens, &mut pos);
                    if let Some(Lexer::ASSIGN) = tokens.get(pos) {
                        pos += 1;
                    } else {
                        pos += 1;
                        continue;
                    }
                    skip_whitespace(tokens, &mut pos);
                    let initializer = if let Some(token) = tokens.get(pos) {
                        match token {
                            Lexer::TEXT(s) => {
                                pos += 1;
                                AST::Literal(s.clone())
                            },
                            Lexer::NUMBER(s) => {
                                pos += 1;
                                AST::Literal(s.clone())
                            },
                            _ => {
                                pos += 1;
                                AST::Empty
                            }
                        }
                    } else {
                        AST::Empty
                    };
                    while pos < tokens.len() {
                        match tokens.get(pos) {
                            Some(Lexer::SEMICOLON) => {
                                pos += 1;
                                break;
                            },
                            _ => pos += 1,
                        }
                    }
                    asts.push(AST::VarDecl {
                        var_type: Box::new(var_type),
                        identifier,
                        initializer: Box::new(initializer),
                    });
                },
                _ => {
                    pos += 1;
                }
            }
        }
        }
        // Handle other constructs (e.g., comments, literals, etc.)
        // This is a simplified example. You can extend this to handle more constructs.
        while pos < tokens.len() {
            match &tokens[pos] {
                Lexer::COMMENT_TEXT(text) => {
                    asts.push(AST::Comment(text.clone()));
                }
                Lexer::TEXT(text) => {
                    asts.push(AST::Identifier(text.clone()));
                }
                Lexer::NUMBER(num) => {
                    asts.push(AST::Literal(num.clone()));
                }
                _ => {}
            }
            pos += 1;
        }
        asts
    }
}




fn main() {
    let input = r#"
        @a: i32 = 5;
        @b: bool = true;
        @c = "Hello, World!";
    "#;

    let tokens = Lexer::tokenizer(input);
    println!("Tokens: {:?}", tokens);

    let ast = AST::parse(&tokens);
    println!("AST: {:?}", ast);
}