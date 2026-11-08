use crate::ast::*;
use crate::error::{Galat, Span};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, Galat> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip empty semicolons
            if self.match_token(&[TokenType::Semicolon]) {
                continue;
            }
            statements.push(self.declaration()?);
        }

        Ok(Program::new(statements))
    }

    // --- Declarations & Statements ---

    fn declaration(&mut self) -> Result<Stmt, Galat> {
        let mut attributes = Vec::new();

        // Parse attributes: #[turunkan(Json, Serialisasi)] or @dekorator
        while self.match_token(&[TokenType::Hash]) {
            let hash_tok = self.previous().clone();
            let attr_span = hash_tok.span.clone();
            if hash_tok.lexeme == "@" || !self.check(&TokenType::LeftBracket) {
                let dec_name = self.consume_identifier("Harapkan nama dekorator setelah '@'")?.lexeme;
                let mut arguments = Vec::new();
                if self.match_token(&[TokenType::LeftParen]) {
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            let arg = if let TokenType::StringLiteral(s) = &self.peek().token_type {
                                let val = s.clone();
                                self.advance();
                                val
                            } else {
                                self.consume_identifier("Harapkan argumen dekorator")?.lexeme
                            };
                            arguments.push(arg);
                            if !self.match_token(&[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Harapkan ')' setelah argumen dekorator")?;
                }
                attributes.push(Attribute {
                    name: format!("@{}", dec_name),
                    arguments,
                    span: attr_span,
                });
                continue;
            }

            self.consume(TokenType::LeftBracket, "Harapkan '[' setelah '#'")?;
            let attr_name = self.consume_identifier("Harapkan nama atribut")?.lexeme;
            let mut arguments = Vec::new();
            if self.match_token(&[TokenType::LeftParen]) {
                if !self.check(&TokenType::RightParen) {
                    loop {
                        let arg = if let TokenType::StringLiteral(s) = &self.peek().token_type {
                            let val = s.clone();
                            self.advance();
                            val
                        } else {
                            self.consume_identifier("Harapkan argumen atribut (identifier atau string)")?.lexeme
                        };
                        arguments.push(arg);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah argumen atribut")?;
            }
            self.consume(TokenType::RightBracket, "Harapkan ']' untuk menutup atribut")?;
            attributes.push(Attribute {
                name: attr_name,
                arguments,
                span: attr_span,
            });
        }

        if self.match_token(&[TokenType::Misal]) {
            self.var_declaration(false)
        } else if self.match_token(&[TokenType::Tetap]) {
            self.var_declaration(true)
        } else if self.match_token(&[TokenType::Asinkron]) {
            self.consume(TokenType::Fungsi, "Harapkan 'fungsi' setelah kata kunci 'asinkron'")?;
            if self.check_identifier() {
                self.function_declaration(attributes)
            } else {
                let expr = self.function_expression_with_keyword_consumed(self.previous().span.clone())?;
                self.match_token(&[TokenType::Semicolon]);
                Ok(Stmt::Expression(expr))
            }
        } else if self.match_token(&[TokenType::Fungsi]) {
            if self.check_identifier() {
                self.function_declaration(attributes)
            } else if self.is_go_receiver_method_at_current() {
                self.go_receiver_method_declaration()
            } else {
                let expr = self.function_expression_with_keyword_consumed(self.previous().span.clone())?;
                self.match_token(&[TokenType::Semicolon]);
                Ok(Stmt::Expression(expr))
            }
        } else if self.match_token(&[TokenType::Struktur]) {
            self.struct_declaration(attributes)
        } else if self.check_identifier() && (self.peek().lexeme == "union" || self.peek().lexeme == "gabungan") {
            self.union_declaration()
        } else if self.match_token(&[TokenType::Enum]) {
            self.enum_declaration(attributes)
        } else if self.match_token(&[TokenType::Sifat]) {
            self.trait_declaration()
        } else if self.match_token(&[TokenType::Terapkan]) {
            self.impl_declaration()
        } else if self.match_token(&[TokenType::Eksternal]) {
            self.external_block()
        } else if self.match_token(&[TokenType::Impor]) {
            self.import_statement()
        } else if self.check_identifier() && (self.peek().lexeme == "dari" || self.peek().lexeme == "from") && self.is_from_import_at_current() {
            self.from_import_statement()
        } else if self.match_token(&[TokenType::Coba]) {
            self.try_catch_statement()
        } else if self.match_token(&[TokenType::Lempar]) {
            self.throw_statement()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self, is_const: bool) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();

        // Check for destructuring declaration: misal [a, b, c] = data;
        if self.match_token(&[TokenType::LeftBracket]) {
            let mut names = Vec::new();
            if !self.check(&TokenType::RightBracket) {
                loop {
                    let id = self.consume_identifier("Harapkan nama variabel dalam dekonstruksi")?;
                    names.push(id.lexeme);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightBracket, "Harapkan ']' setelah daftar variabel")?;
            self.consume(TokenType::Assign, "Harapkan '=' setelah dekonstruksi variabel")?;
            let initializer = self.expression()?;
            self.match_token(&[TokenType::Semicolon]);
            return Ok(Stmt::DestructureDecl {
                names,
                initializer,
                is_const,
                span,
            });
        }

        let name_token = self.consume_identifier("Harapkan nama variabel setelah 'misal'/'tetap'")?;
        let name = name_token.lexeme.clone();

        // Go/Python-style multi-variable declaration without brackets: buat hasil, galat = bagi(10, 2);
        if self.match_token(&[TokenType::Comma]) {
            let mut names = vec![name];
            loop {
                let next_id = self.consume_identifier("Harapkan nama variabel setelah ','")?;
                names.push(next_id.lexeme);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
            if self.check(&TokenType::Colon) {
                self.advance();
            }
            self.consume(TokenType::Assign, "Harapkan '=' setelah daftar variabel")?;
            let initializer = self.parse_destructure_rhs(names.len(), &span)?;
            self.match_token(&[TokenType::Semicolon]);
            return Ok(Stmt::DestructureDecl {
                names,
                initializer,
                is_const,
                span,
            });
        }

        // Optional type annotation: misal x: Angka = 10;
        if self.match_token(&[TokenType::Colon]) {
            if !self.check(&TokenType::Assign) {
                let _var_type = self.parse_type_annotation()?;
            }
        }

        let initializer = if self.match_token(&[TokenType::Assign]) {
            Some(self.expression()?)
        } else if is_const {
            return Err(Galat::sintaks(
                format!("Konstanta 'tetap {}' harus memiliki nilai awal", name),
                &span,
            ));
        } else {
            None
        };

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::VarDecl {
            name,
            is_const,
            initializer,
            span,
        })
    }

    fn parse_destructure_rhs(&mut self, names_len: usize, span: &Span) -> Result<Expr, Galat> {
        let first_expr = self.expression()?;
        if self.match_token(&[TokenType::Comma]) {
            let mut exprs = vec![first_expr];
            loop {
                exprs.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
            return Ok(Expr::Array(exprs, span.clone()));
        }
        // Go comma-ok idiom for channel receive: `buat pesan, ok = <-ch`
        if names_len == 2 {
            if let Expr::Call {
                ref callee,
                ref arguments,
                span: ref cspan,
            } = first_expr
            {
                if let Expr::Identifier(ref fn_name, _) = **callee {
                    if fn_name == "saluran_terima" && arguments.len() == 1 {
                        return Ok(Expr::Call {
                            callee: Box::new(Expr::Identifier(
                                "saluran_terima_ok".to_string(),
                                cspan.clone(),
                            )),
                            arguments: arguments.clone(),
                            span: cspan.clone(),
                        });
                    }
                }
            }
        }
        Ok(first_expr)
    }

    fn is_go_receiver_method_at_current(&self) -> bool {
        if !self.check(&TokenType::LeftParen) {
            return false;
        }
        let mut idx = self.current + 1;
        let mut depth = 1usize;
        while idx < self.tokens.len() && depth > 0 {
            match self.tokens[idx].token_type {
                TokenType::LeftParen => depth += 1,
                TokenType::RightParen => depth -= 1,
                _ => {}
            }
            idx += 1;
        }
        // After closing ')' of receiver `(m: Mobil)`, must be `Identifier` followed by `(`
        idx + 1 < self.tokens.len()
            && matches!(self.tokens[idx].token_type, TokenType::Identifier(_))
            && self.tokens[idx + 1].token_type == TokenType::LeftParen
    }

    fn go_receiver_method_declaration(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        self.consume(TokenType::LeftParen, "Harapkan '(' pada receiver metode Go")?;
        let _ = self.match_token(&[TokenType::Star, TokenType::Ampersand]);
        let first_tok = self.consume_identifier("Harapkan nama receiver atau tipe struct")?;
        let (recv_var, target_name) = if self.match_token(&[TokenType::Colon]) {
            let _ = self.match_token(&[TokenType::Star, TokenType::Ampersand]);
            let ty_tok = self.consume_identifier("Harapkan nama tipe struct pada receiver")?;
            (first_tok.lexeme, ty_tok.lexeme)
        } else if self.check_identifier() {
            let _ = self.match_token(&[TokenType::Star, TokenType::Ampersand]);
            let ty_tok = self.consume_identifier("Harapkan nama tipe struct pada receiver")?;
            (first_tok.lexeme, ty_tok.lexeme)
        } else {
            ("ini".to_string(), first_tok.lexeme)
        };
        self.consume(TokenType::RightParen, "Harapkan ')' setelah receiver metode")?;

        let method_tok = self.consume_identifier("Harapkan nama metode setelah receiver")?;
        let method_name = method_tok.lexeme;
        let _type_params = self.parse_optional_generic_params()?;

        self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
        let (params, mut body) = self.parse_function_params_with_defaults()?;
        self.consume(TokenType::RightParen, "Harapkan ')' setelah parameter metode")?;

        if self.match_token(&[TokenType::Minus]) {
            if self.match_token(&[TokenType::Greater]) {
                let _ = self.parse_type_annotation()?;
            } else {
                self.current -= 1;
            }
        }

        if recv_var != "ini" {
            body.insert(
                0,
                Stmt::VarDecl {
                    name: recv_var,
                    is_const: false,
                    initializer: Some(Expr::This(span.clone())),
                    span: span.clone(),
                },
            );
        }

        let parsed_body = self.block_statement_list()?;
        body.extend(parsed_body);

        Ok(Stmt::ImplDecl {
            trait_name: None,
            target_name,
            methods: vec![StructMethod {
                name: method_name,
                params,
                body,
                span: span.clone(),
            }],
            span,
        })
    }

    fn parse_function_params_with_defaults(&mut self) -> Result<(Vec<String>, Vec<Stmt>), Galat> {
        let mut params = Vec::new();
        let mut default_stmts = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let is_variadic = self.match_token(&[TokenType::Star, TokenType::DotDot]);
                let param = self.consume_identifier("Harapkan nama parameter fungsi")?;
                let param_span = param.span.clone();
                let raw_name = param.lexeme.clone();
                let stored_name = if is_variadic {
                    format!("*{}", raw_name)
                } else {
                    raw_name.clone()
                };
                params.push(stored_name);

                // Optional parameter type annotation: param: Tipe
                if self.match_token(&[TokenType::Colon]) {
                    self.parse_type_annotation()?;
                }

                // Optional default parameter value: param = nilai_bawaan
                if self.match_token(&[TokenType::Assign]) {
                    let default_expr = self.expression()?;
                    default_stmts.push(Stmt::If {
                        condition: Expr::Binary {
                            left: Box::new(Expr::Identifier(raw_name.clone(), param_span.clone())),
                            op: BinaryOp::Equal,
                            right: Box::new(Expr::Nil(param_span.clone())),
                            span: param_span.clone(),
                        },
                        then_branch: Box::new(Stmt::Assignment {
                            target: AssignTarget::Variable(raw_name),
                            op: AssignOp::Assign,
                            value: default_expr,
                            span: param_span.clone(),
                        }),
                        elif_branches: Vec::new(),
                        else_branch: None,
                        span: param_span,
                    });
                }

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        Ok((params, default_stmts))
    }

    fn function_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama fungsi")?;
        let name = name_token.lexeme.clone();

        // Optional generic type parameters: <T, U: Sifat>
        let _type_params = self.parse_optional_generic_params()?;

        self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama fungsi")?;
        let (params, mut body) = self.parse_function_params_with_defaults()?;
        self.consume(TokenType::RightParen, "Harapkan ')' setelah daftar parameter")?;

        // Optional return type annotation: -> Tipe / Hasil<T, E>
        if self.match_token(&[TokenType::Minus]) {
            if self.match_token(&[TokenType::Greater]) {
                let _ret_ty = self.parse_type_annotation()?;
            } else {
                self.current -= 1;
            }
        }

        // Optional where clause: dimana T: Sifat, U: Sifat2
        if self.check_identifier() && (self.peek().lexeme == "dimana" || self.peek().lexeme == "where") {
            self.advance();
            self.parse_where_clause()?;
        }

        let parsed_body = self.block_statement_list()?;
        body.extend(parsed_body);
        Ok(Stmt::FunctionDecl {
            name,
            params,
            body,
            attributes,
            span,
        })
    }

    fn struct_declaration(&mut self, mut attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama struktur")?;
        let name = name_token.lexeme.clone();

        // Optional generic type parameters: struktur Kantong<T: Sama>
        let _type_params = self.parse_optional_generic_params()?;

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi struktur")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }

            if self.match_token(&[TokenType::Fungsi]) {
                let method_span = self.previous().span.clone();
                let method_name = self.consume_identifier("Harapkan nama metode")?.lexeme;
                let _method_type_params = self.parse_optional_generic_params()?;
                self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
                let (params, mut body) = self.parse_function_params_with_defaults()?;
                self.consume(TokenType::RightParen, "Harapkan ')'")?;
                if self.match_token(&[TokenType::Minus]) {
                    if self.match_token(&[TokenType::Greater]) {
                        self.parse_type_annotation()?;
                    } else {
                        self.current -= 1;
                    }
                }
                let parsed_body = self.block_statement_list()?;
                body.extend(parsed_body);
                methods.push(StructMethod {
                    name: method_name,
                    params,
                    body,
                    span: method_span,
                });
            } else if self.check_identifier() {
                let field_token = self.advance();
                let fname = field_token.lexeme.clone();
                let fspan = field_token.span.clone();
                fields.push(fname.clone());
                // Optional field type annotation: field_name: Tipe (: bit_width)
                if self.match_token(&[TokenType::Colon]) {
                    let ftype = self.parse_type_annotation()?;
                    let mut bit_str = String::new();
                    if self.match_token(&[TokenType::Colon]) {
                        if let TokenType::Number(n) = self.peek().token_type {
                            self.advance();
                            bit_str = format!("{}", n as u8);
                        }
                    }
                    attributes.push(Attribute {
                        name: "__c_field__".to_string(),
                        arguments: vec![fname, ftype, bit_str],
                        span: fspan,
                    });
                }
                self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
            } else {
                return Err(Galat::sintaks(
                    "Harapkan nama field atau metode 'fungsi' di dalam struktur",
                    &self.peek().span,
                ));
            }
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup struktur")?;
        Ok(Stmt::StructDecl {
            name,
            fields,
            methods,
            attributes,
            span,
        })
    }

    fn union_declaration(&mut self) -> Result<Stmt, Galat> {
        let span = self.advance().span.clone(); // consume 'union' / 'gabungan'
        let name_token = self.consume_identifier("Harapkan nama union")?;
        let name = name_token.lexeme;
        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi union")?;
        let mut field_pairs = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }
            let f_tok = self.consume_identifier("Harapkan nama field union")?;
            let f_type = if self.match_token(&[TokenType::Colon]) {
                self.parse_type_annotation()?
            } else {
                "u64".to_string()
            };
            field_pairs.push((
                Expr::String(f_tok.lexeme, f_tok.span.clone()),
                Expr::String(f_type, f_tok.span),
            ));
            self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
        }
        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup union")?;
        Ok(Stmt::VarDecl {
            name: name.clone(),
            is_const: false,
            initializer: Some(Expr::Call {
                callee: Box::new(Expr::Identifier("_definisi_union_c".to_string(), span.clone())),
                arguments: vec![
                    Expr::String(name, span.clone()),
                    Expr::Map(field_pairs, span.clone()),
                ],
                span: span.clone(),
            }),
            span,
        })
    }

    fn trait_declaration(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama sifat (trait)")?;
        let name = name_token.lexeme;

        // Optional generic type parameters: sifat Pembanding<T>
        let _type_params = self.parse_optional_generic_params()?;

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi sifat")?;
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }

            let _ = self.match_token(&[TokenType::Fungsi]);
            let m_span = self.peek().span.clone();
            let m_name = self.consume_identifier("Harapkan nama metode pada sifat")?.lexeme;
            let _m_type_params = self.parse_optional_generic_params()?;
            self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
            let mut params = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    let param = self.consume_identifier("Harapkan nama parameter")?;
                    params.push(param.lexeme);
                    if self.match_token(&[TokenType::Colon]) {
                        self.parse_type_annotation()?;
                    }
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Harapkan ')'")?;
            if self.match_token(&[TokenType::Minus]) {
                if self.match_token(&[TokenType::Greater]) {
                    self.parse_type_annotation()?;
                } else {
                    self.current -= 1;
                }
            }
            self.match_token(&[TokenType::Semicolon]);
            methods.push(TraitMethodSignature {
                name: m_name,
                params,
                span: m_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup sifat")?;
        Ok(Stmt::TraitDecl {
            name,
            methods,
            span,
        })
    }

    fn impl_declaration(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let _impl_type_params = self.parse_optional_generic_params()?;
        let first_id = self.consume_identifier("Harapkan nama sifat atau struktur setelah 'terapkan'")?.lexeme;
        let _first_gen_args = self.parse_optional_generic_args()?;

        let (trait_name, target_name) = if self.match_token(&[TokenType::Untuk]) {
            let target_id = self.consume_identifier("Harapkan nama struktur target setelah 'untuk'")?.lexeme;
            let _target_gen_args = self.parse_optional_generic_args()?;
            (Some(first_id), target_id)
        } else {
            (None, first_id)
        };

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada blok 'terapkan'")?;
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }

            self.consume(TokenType::Fungsi, "Harapkan kata kunci 'fungsi' dalam blok terapkan")?;
            let method_span = self.previous().span.clone();
            let method_name = self.consume_identifier("Harapkan nama metode")?.lexeme;
            let _m_type_params = self.parse_optional_generic_params()?;
            self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
            let (params, mut body) = self.parse_function_params_with_defaults()?;
            self.consume(TokenType::RightParen, "Harapkan ')'")?;
            if self.match_token(&[TokenType::Minus]) {
                if self.match_token(&[TokenType::Greater]) {
                    self.parse_type_annotation()?;
                } else {
                    self.current -= 1;
                }
            }
            let parsed_body = self.block_statement_list()?;
            body.extend(parsed_body);
            methods.push(StructMethod {
                name: method_name,
                params,
                body,
                span: method_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok terapkan")?;
        Ok(Stmt::ImplDecl {
            trait_name,
            target_name,
            methods,
            span,
        })
    }

    fn enum_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama enum setelah kata kunci 'enum'")?;
        let name = name_token.lexeme.clone();

        // Optional generic type parameters: enum Hasil<T, E: Sama>
        let _type_params = self.parse_optional_generic_params()?;

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi enum")?;
        let mut variants = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Comma, TokenType::Semicolon]) {
                continue;
            }

            let var_span = self.peek().span.clone();
            let var_name = self.consume_identifier("Harapkan nama varian enum")?.lexeme;

            let mut fields_count = 0;
            if self.match_token(&[TokenType::LeftParen]) {
                if !self.check(&TokenType::RightParen) {
                    loop {
                        // Allow type annotation or identifier with colon
                        if self.check_identifier() {
                            let _field_id = self.advance();
                            if self.match_token(&[TokenType::Colon]) {
                                self.parse_type_annotation()?;
                            }
                        } else {
                            self.parse_type_annotation()?;
                        }
                        fields_count += 1;
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah parameter varian enum")?;
            }

            variants.push(EnumVariantDecl {
                name: var_name,
                fields_count,
                span: var_span,
            });

            self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup enum")?;
        Ok(Stmt::EnumDecl {
            name,
            variants,
            attributes,
            span,
        })
    }

    fn external_block(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let abi = if let TokenType::StringLiteral(s) = &self.peek().token_type {
            let val = s.clone();
            self.advance();
            val
        } else {
            "C".to_string()
        };

        self.consume(TokenType::LeftBrace, "Harapkan '{' setelah blok 'eksternal'")?;
        let mut functions = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon]) {
                continue;
            }
            self.consume(TokenType::Fungsi, "Harapkan deklarasi 'fungsi' dalam blok eksternal")?;
            let fn_token = self.consume_identifier("Harapkan nama fungsi eksternal")?;
            let fn_name = fn_token.lexeme.clone();
            let fn_span = fn_token.span.clone();

            self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama fungsi eksternal")?;
            let mut params = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    let p = self.consume_identifier("Harapkan nama parameter fungsi eksternal")?;
                    params.push(p.lexeme);
                    // Optional type annotation: param: Tipe
                    if self.match_token(&[TokenType::Colon]) {
                        let _type_name = self.consume_identifier("Harapkan tipe parameter")?;
                    }
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Harapkan ')' setelah daftar parameter")?;

            // Optional return type annotation: -> Tipe
            let mut return_type = None;
            if self.match_token(&[TokenType::Minus]) {
                if self.match_token(&[TokenType::Greater]) {
                    let ret_id = self.consume_identifier("Harapkan tipe kembalian fungsi eksternal")?;
                    return_type = Some(ret_id.lexeme);
                }
            }

            self.match_token(&[TokenType::Semicolon]);
            functions.push(ExternalFunctionDecl {
                name: fn_name,
                params,
                return_type,
                span: fn_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok eksternal")?;
        Ok(Stmt::ExternalBlock {
            abi,
            functions,
            span,
        })
    }

    fn parse_module_path_spec(&mut self) -> Result<(String, Option<String>), Galat> {
        let path_token = self.peek().clone();
        match &path_token.token_type {
            TokenType::StringLiteral(s) => {
                self.advance();
                let default_alias = if s.starts_with("std::") || s.starts_with("std.") || s.starts_with("std:") {
                    s.split(|c| c == ':' || c == '.' || c == '/').filter(|p| !p.is_empty()).last().map(|x| x.to_string())
                } else {
                    None
                };
                Ok((s.clone(), default_alias))
            }
            TokenType::Identifier(_) => {
                let first = self.advance().lexeme;
                let mut segments = vec![first];
                loop {
                    if self.match_token(&[TokenType::Dot]) {
                        let seg = self.consume_property_name("Harapkan nama sub-modul setelah '.'")?.lexeme;
                        segments.push(seg);
                    } else if self.match_token(&[TokenType::Colon]) {
                        self.consume(TokenType::Colon, "Harapkan '::' pada path modul")?;
                        let seg = self.consume_property_name("Harapkan nama sub-modul setelah '::'")?.lexeme;
                        segments.push(seg);
                    } else {
                        break;
                    }
                }
                let last_seg = segments.last().cloned();
                let full_path = if segments.len() == 1 && !segments[0].ends_with(".wya") && !segments[0].ends_with(".widya") {
                    format!("std::{}", segments[0])
                } else {
                    segments.join("::")
                };
                Ok((full_path, last_seg))
            }
            _ => Err(Galat::sintaks(
                "Harapkan path berkas string atau namespace modul (misal std::matematika) setelah 'impor'/'muat'",
                &path_token.span,
            )),
        }
    }

    fn import_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let (path, default_alias) = self.parse_module_path_spec()?;

        let alias = if self.match_token(&[TokenType::Sebagai]) {
            let alias_token = self.consume_identifier("Harapkan nama alias setelah 'sebagai'")?;
            Some(alias_token.lexeme.clone())
        } else {
            default_alias
        };

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Import { path, alias, span })
    }

    fn is_from_import_at_current(&self) -> bool {
        let mut idx = self.current + 1;
        while idx < self.tokens.len() {
            match &self.tokens[idx].token_type {
                TokenType::Impor => return true,
                TokenType::StringLiteral(_)
                | TokenType::Identifier(_)
                | TokenType::Dot
                | TokenType::Colon => {
                    idx += 1;
                }
                _ => return false,
            }
        }
        false
    }

    fn from_import_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.advance().span.clone(); // consume 'dari' / 'from'
        let (path, _) = self.parse_module_path_spec()?;
        self.consume(TokenType::Impor, "Harapkan 'impor' setelah nama modul pada pernyataan 'dari'")?;

        let hidden_mod = format!("_mod_dari_{}", span.line);
        let mut stmts = vec![Stmt::Import {
            path,
            alias: Some(hidden_mod.clone()),
            span: span.clone(),
        }];

        if self.match_token(&[TokenType::Star]) {
            // dari modul impor * -> flat import
            if let Some(Stmt::Import { alias, .. }) = stmts.first_mut() {
                *alias = None;
            }
        } else {
            loop {
                let item_tok = self.consume_identifier("Harapkan nama simbol yang diimpor")?;
                let item_name = item_tok.lexeme.clone();
                let bind_name = if self.match_token(&[TokenType::Sebagai]) {
                    self.consume_identifier("Harapkan alias setelah 'sebagai'")?.lexeme
                } else {
                    item_name.clone()
                };
                stmts.push(Stmt::VarDecl {
                    name: bind_name,
                    is_const: false,
                    initializer: Some(Expr::Index {
                        target: Box::new(Expr::Identifier(hidden_mod.clone(), span.clone())),
                        index: Box::new(Expr::String(item_name, span.clone())),
                        span: span.clone(),
                    }),
                    span: span.clone(),
                });
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.match_token(&[TokenType::Semicolon]);
        // Emit without creating an isolated child block scope so bindings stay in current scope:
        // Wait: Stmt::Block creates a child scope in Interpreter, so how can we keep bindings in current scope?
        // Using Stmt::TryCatch with a non-throwing block? No, TryCatch try_block executes self.execute(try_block)!
        // Wait: in Interpreter, Stmt::If with condition Expr::Boolean(true) executes `then_branch` directly!
        //Wait: if `then_branch` is `Stmt::Block`, `Stmt::Block` creates a child scope.
        // What if we use `DestructureDecl`! Look at `Interpreter::execute` lines 245-268:
        // `Stmt::DestructureDecl` evaluates `initializer` (which can be an IIFE returning a Map of the imported items!) and defines every `name` in `names` directly in `self.environment.borrow_mut().define(name, val, is_const)`!
        // Let's check that! `Stmt::DestructureDecl` defines variables directly in `self.environment` (the current scope)!
        if stmts.len() == 1 {
            return Ok(stmts.remove(0));
        }
        let mut names = Vec::new();
        let mut pairs = Vec::new();
        for s in stmts.iter().skip(1) {
            if let Stmt::VarDecl { name, initializer: Some(init_expr), .. } = s {
                names.push(name.clone());
                pairs.push((Expr::String(name.clone(), span.clone()), init_expr.clone()));
            }
        }
        let mut iife_body = vec![stmts.remove(0)];
        iife_body.push(Stmt::Return {
            value: Some(Expr::Map(pairs, span.clone())),
            span: span.clone(),
        });
        Ok(Stmt::DestructureDecl {
            names,
            initializer: Expr::Call {
                callee: Box::new(Expr::FunctionExpr {
                    params: Vec::new(),
                    body: iife_body,
                    span: span.clone(),
                }),
                arguments: Vec::new(),
                span: span.clone(),
            },
            is_const: false,
            span,
        })
    }

    fn try_catch_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let try_block = Box::new(self.block_or_single_statement()?);

        self.consume(TokenType::Tangkap, "Harapkan 'tangkap' setelah blok 'coba'")?;
        let has_paren = self.match_token(&[TokenType::LeftParen]);
        let error_var = if self.check_identifier() {
            self.advance().lexeme
        } else {
            "galat".to_string()
        };
        if has_paren {
            self.consume(TokenType::RightParen, "Harapkan ')' setelah variabel galat")?;
        }

        let catch_block = Box::new(self.block_or_single_statement()?);
        Ok(Stmt::TryCatch {
            try_block,
            error_var,
            catch_block,
            span,
        })
    }

    fn throw_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let expr = self.expression()?;
        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Throw { expr, span })
    }

    fn with_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.advance().span.clone(); // consume 'dengan' / 'with'
        let resource_expr = self.expression()?;
        let var_name = if self.match_token(&[TokenType::Sebagai]) {
            self.consume_identifier("Harapkan nama variabel setelah 'sebagai' pada 'dengan'")?.lexeme
        } else {
            "_sumber_konteks".to_string()
        };
        let body_stmt = self.block_or_single_statement()?;
        let cleanup_call = Stmt::Expression(Expr::Call {
            callee: Box::new(Expr::Identifier("_konteks_keluar".to_string(), span.clone())),
            arguments: vec![Expr::Identifier(var_name.clone(), span.clone())],
            span: span.clone(),
        });
        let enter_call = Expr::Call {
            callee: Box::new(Expr::Identifier("_konteks_masuk".to_string(), span.clone())),
            arguments: vec![resource_expr],
            span: span.clone(),
        };
        Ok(Stmt::Block(
            vec![
                Stmt::VarDecl {
                    name: var_name.clone(),
                    is_const: false,
                    initializer: Some(enter_call),
                    span: span.clone(),
                },
                Stmt::TryCatch {
                    try_block: Box::new(body_stmt),
                    error_var: "_galat_konteks".to_string(),
                    catch_block: Box::new(Stmt::Block(
                        vec![
                            cleanup_call.clone(),
                            Stmt::Throw {
                                expr: Expr::Identifier("_galat_konteks".to_string(), span.clone()),
                                span: span.clone(),
                            },
                        ],
                        span.clone(),
                    )),
                    span: span.clone(),
                },
                cleanup_call,
            ],
            span,
        ))
    }

    fn statement(&mut self) -> Result<Stmt, Galat> {
        if self.match_token(&[TokenType::Jika]) {
            self.if_statement()
        } else if self.check(&TokenType::Cocokkan) && self.is_match_construct_at_current() {
            self.advance();
            let expr = self.match_expression_with_keyword_consumed(self.previous().span.clone())?;
            self.match_token(&[TokenType::Semicolon]);
            Ok(Stmt::Expression(expr))
        } else if self.check_identifier() && (self.peek().lexeme == "dengan" || self.peek().lexeme == "with") {
            self.with_statement()
        } else if self.check_identifier() && (self.peek().lexeme == "tidak_aman" || self.peek().lexeme == "unsafe") {
            self.advance(); // consume 'tidak_aman' / 'unsafe'
            self.block_or_single_statement()
        } else if self.check_identifier()
            && (self.peek().lexeme == "tunda" || self.peek().lexeme == "defer")
            && self.current + 1 < self.tokens.len()
            && !matches!(
                self.tokens[self.current + 1].token_type,
                TokenType::Assign
                    | TokenType::Colon
                    | TokenType::Dot
                    | TokenType::Comma
                    | TokenType::Semicolon
                    | TokenType::RightParen
            )
        {
            let span = self.advance().span.clone(); // consume 'tunda' / 'defer'
            let deferred_stmt = self.block_or_single_statement()?;
            Ok(Stmt::Expression(Expr::Call {
                callee: Box::new(Expr::Identifier("_daftar_tunda".to_string(), span.clone())),
                arguments: vec![Expr::FunctionExpr {
                    params: Vec::new(),
                    body: vec![deferred_stmt],
                    span: span.clone(),
                }],
                span,
            }))
        } else if self.check_identifier()
            && (self.peek().lexeme == "jalan" || self.peek().lexeme == "go")
            && self.current + 1 < self.tokens.len()
            && !matches!(
                self.tokens[self.current + 1].token_type,
                TokenType::Assign
                    | TokenType::Colon
                    | TokenType::Dot
                    | TokenType::Comma
                    | TokenType::Semicolon
                    | TokenType::RightParen
                    | TokenType::LeftParen
            )
        {
            let span = self.advance().span.clone(); // consume 'jalan' / 'go'
            let goroutine_stmt = self.block_or_single_statement()?;
            Ok(Stmt::Expression(Expr::Call {
                callee: Box::new(Expr::Identifier("_jalan_goroutine".to_string(), span.clone())),
                arguments: vec![Expr::FunctionExpr {
                    params: Vec::new(),
                    body: vec![goroutine_stmt],
                    span: span.clone(),
                }],
                span,
            }))
        } else if self.check_identifier()
            && (self.peek().lexeme == "pilih_saluran" || self.peek().lexeme == "select")
            && self.current + 1 < self.tokens.len()
            && self.tokens[self.current + 1].token_type == TokenType::LeftBrace
        {
            self.select_channel_statement()
        } else if self.match_token(&[TokenType::Volatil]) {
            let span = self.previous().span.clone();
            let _ = self.match_token(&[TokenType::Star]);
            let address = self.term()?;
            if self.match_token(&[TokenType::Assign]) {
                let value = self.expression()?;
                self.match_token(&[TokenType::Semicolon]);
                Ok(Stmt::VolatileWrite {
                    address,
                    value,
                    span,
                })
            } else {
                self.match_token(&[TokenType::Semicolon]);
                Ok(Stmt::Expression(Expr::VolatileRead {
                    address: Box::new(address),
                    span,
                }))
            }
        } else if self.match_token(&[TokenType::Selama]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::Untuk]) {
            self.for_in_statement()
        } else if self.match_token(&[TokenType::Ulang]) {
            self.loop_statement()
        } else if self.match_token(&[TokenType::Kembalikan]) {
            self.return_statement()
        } else if self.match_token(&[TokenType::Berhenti]) {
            let span = self.previous().span.clone();
            self.match_token(&[TokenType::Semicolon]);
            Ok(Stmt::Break(span))
        } else if self.match_token(&[TokenType::Lanjut]) {
            let span = self.previous().span.clone();
            self.match_token(&[TokenType::Semicolon]);
            Ok(Stmt::Continue(span))
        } else if self.check(&TokenType::LeftBrace) {
            let span = self.peek().span.clone();
            let stmts = self.block_statement_list()?;
            Ok(Stmt::Block(stmts, span))
        } else {
            self.assignment_or_expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();

        // Check for 'if let' pattern: jika misal <pola> = <nilai> { ... }
        if self.match_token(&[TokenType::Misal]) {
            let pattern = self.match_pattern()?;
            self.consume(TokenType::Assign, "Harapkan '=' setelah pola pada 'jika misal'")?;
            let value = self.expression()?;
            let then_branch = Box::new(self.block_or_single_statement()?);
            let else_branch = if self.match_token(&[TokenType::Lainnya]) {
                Some(Box::new(self.block_or_single_statement()?))
            } else {
                None
            };
            return Ok(Stmt::IfLet {
                pattern,
                value,
                then_branch,
                else_branch,
                span,
            });
        }

        let condition = self.expression()?;
        let then_branch = Box::new(self.block_or_single_statement()?);

        let mut elif_branches = Vec::new();
        while self.match_token(&[TokenType::Kalau]) || (self.check(&TokenType::Lainnya) && self.check_next_is_jika()) {
            if self.check(&TokenType::Lainnya) {
                self.advance(); // consume 'lainnya' / 'jika_tidak'
                self.advance(); // consume 'jika'
            }
            let elif_cond = self.expression()?;
            let elif_body = self.block_or_single_statement()?;
            elif_branches.push((elif_cond, elif_body));
        }

        let else_branch = if self.match_token(&[TokenType::Lainnya]) {
            Some(Box::new(self.block_or_single_statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
            span,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let condition = self.expression()?;
        let body = Box::new(self.block_or_single_statement()?);
        Ok(Stmt::While {
            condition,
            body,
            span,
        })
    }

    fn for_in_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let var_token = self.consume_identifier("Harapkan nama variabel pada perulangan 'untuk'")?;
        let var_name = var_token.lexeme.clone();

        self.consume(TokenType::Dalam, "Harapkan 'dalam' setelah nama variabel pada perulangan 'untuk'")?;
        let iterable = self.expression()?;
        let body = Box::new(self.block_or_single_statement()?);

        Ok(Stmt::ForIn {
            var_name,
            iterable,
            body,
            span,
        })
    }

    fn loop_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let body = Box::new(self.block_or_single_statement()?);
        Ok(Stmt::Loop { body, span })
    }

    fn select_channel_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.advance().span.clone(); // consume 'pilih_saluran' / 'select'
        self.consume(TokenType::LeftBrace, "Harapkan '{' setelah 'pilih_saluran'/'select'")?;
        let mut arm_maps = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }
            let arm_span = self.peek().span.clone();
            // Optional 'kasus' / 'case'
            if self.check_identifier()
                && (self.peek().lexeme == "kasus" || self.peek().lexeme == "case")
            {
                self.advance();
            }

            // 1. Default arm: bawaan / default / _
            if self.check_identifier()
                && (self.peek().lexeme == "bawaan"
                    || self.peek().lexeme == "default"
                    || self.peek().lexeme == "_")
            {
                self.advance();
                let _ = self.match_token(&[TokenType::FatArrow, TokenType::Colon]);
                let body_stmt = self.block_or_single_statement()?;
                let _ = self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
                arm_maps.push(Expr::Map(
                    vec![
                        (
                            Expr::String("jenis".to_string(), arm_span.clone()),
                            Expr::String("bawaan".to_string(), arm_span.clone()),
                        ),
                        (
                            Expr::String("aksi".to_string(), arm_span.clone()),
                            Expr::FunctionExpr {
                                params: Vec::new(),
                                body: vec![body_stmt],
                                span: arm_span.clone(),
                            },
                        ),
                    ],
                    arm_span,
                ));
                continue;
            }

            // 2. Timeout arm: setelah_waktu(ms) / timeout(ms) / after(ms)
            if self.check_identifier()
                && (self.peek().lexeme == "setelah_waktu"
                    || self.peek().lexeme == "timeout"
                    || self.peek().lexeme == "after")
                && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].token_type == TokenType::LeftParen
            {
                self.advance(); // consume keyword
                self.consume(TokenType::LeftParen, "Harapkan '(' setelah 'setelah_waktu'")?;
                let ms_expr = self.expression()?;
                self.consume(TokenType::RightParen, "Harapkan ')' setelah durasi timeout")?;
                let _ = self.match_token(&[TokenType::FatArrow, TokenType::Colon]);
                let body_stmt = self.block_or_single_statement()?;
                let _ = self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
                arm_maps.push(Expr::Map(
                    vec![
                        (
                            Expr::String("jenis".to_string(), arm_span.clone()),
                            Expr::String("timeout".to_string(), arm_span.clone()),
                        ),
                        (Expr::String("nilai".to_string(), arm_span.clone()), ms_expr),
                        (
                            Expr::String("aksi".to_string(), arm_span.clone()),
                            Expr::FunctionExpr {
                                params: Vec::new(),
                                body: vec![body_stmt],
                                span: arm_span.clone(),
                            },
                        ),
                    ],
                    arm_span,
                ));
                continue;
            }

            // 3. Receive with binding: [buat/misal] var_name [:= | =] <-ch_expr
            let _ = self.match_token(&[TokenType::Misal, TokenType::Tetap]);
            if self.check_identifier()
                && self.current + 1 < self.tokens.len()
                && (self.tokens[self.current + 1].token_type == TokenType::Assign
                    || (self.tokens[self.current + 1].token_type == TokenType::Colon
                        && self.current + 2 < self.tokens.len()
                        && self.tokens[self.current + 2].token_type == TokenType::Assign))
            {
                let var_name = self.advance().lexeme;
                let _ = self.match_token(&[TokenType::Colon]);
                self.consume(TokenType::Assign, "Harapkan '=' pada kasus pilih_saluran")?;
                if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__chan_op__")
                {
                    self.advance(); // consume '<-'
                }
                let ch_expr = self.postfix()?;
                let _ = self.match_token(&[TokenType::FatArrow, TokenType::Colon]);
                let body_stmt = self.block_or_single_statement()?;
                let _ = self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
                arm_maps.push(Expr::Map(
                    vec![
                        (
                            Expr::String("jenis".to_string(), arm_span.clone()),
                            Expr::String("terima".to_string(), arm_span.clone()),
                        ),
                        (Expr::String("saluran".to_string(), arm_span.clone()), ch_expr),
                        (
                            Expr::String("aksi".to_string(), arm_span.clone()),
                            Expr::FunctionExpr {
                                params: vec![var_name],
                                body: vec![body_stmt],
                                span: arm_span.clone(),
                            },
                        ),
                    ],
                    arm_span,
                ));
                continue;
            }

            // 4. Receive without binding: <-ch_expr => body
            if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__chan_op__") {
                self.advance(); // consume '<-'
                let ch_expr = self.postfix()?;
                let _ = self.match_token(&[TokenType::FatArrow, TokenType::Colon]);
                let body_stmt = self.block_or_single_statement()?;
                let _ = self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
                arm_maps.push(Expr::Map(
                    vec![
                        (
                            Expr::String("jenis".to_string(), arm_span.clone()),
                            Expr::String("terima".to_string(), arm_span.clone()),
                        ),
                        (Expr::String("saluran".to_string(), arm_span.clone()), ch_expr),
                        (
                            Expr::String("aksi".to_string(), arm_span.clone()),
                            Expr::FunctionExpr {
                                params: vec!["_".to_string()],
                                body: vec![body_stmt],
                                span: arm_span.clone(),
                            },
                        ),
                    ],
                    arm_span,
                ));
                continue;
            }

            // 5. Send arm: ch_expr <- val_expr => body
            let ch_expr = self.postfix()?;
            if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__chan_op__") {
                self.advance(); // consume '<-'
            }
            let val_expr = self.expression()?;
            let _ = self.match_token(&[TokenType::FatArrow, TokenType::Colon]);
            let body_stmt = self.block_or_single_statement()?;
            let _ = self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
            arm_maps.push(Expr::Map(
                vec![
                    (
                        Expr::String("jenis".to_string(), arm_span.clone()),
                        Expr::String("kirim".to_string(), arm_span.clone()),
                    ),
                    (Expr::String("saluran".to_string(), arm_span.clone()), ch_expr),
                    (Expr::String("nilai".to_string(), arm_span.clone()), val_expr),
                    (
                        Expr::String("aksi".to_string(), arm_span.clone()),
                        Expr::FunctionExpr {
                            params: Vec::new(),
                            body: vec![body_stmt],
                            span: arm_span.clone(),
                        },
                    ),
                ],
                arm_span,
            ));
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup 'pilih_saluran'")?;
        Ok(Stmt::Expression(Expr::Call {
            callee: Box::new(Expr::Identifier(
                "_eksekusi_pilih_saluran".to_string(),
                span.clone(),
            )),
            arguments: arm_maps,
            span,
        }))
    }

    fn return_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let value = if self.check(&TokenType::Semicolon)
            || self.check(&TokenType::RightBrace)
            || self.is_at_end()
        {
            None
        } else {
            let first = self.expression()?;
            if self.match_token(&[TokenType::Comma]) {
                let mut items = vec![first];
                loop {
                    items.push(self.expression()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
                Some(Expr::Array(items, span.clone()))
            } else {
                Some(first)
            }
        };

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Return { value, span })
    }

    fn block_statement_list(&mut self) -> Result<Vec<Stmt>, Galat> {
        self.consume(TokenType::LeftBrace, "Harapkan '{' untuk memulai blok")?;
        let mut stmts = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon]) {
                continue;
            }
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok")?;
        Ok(stmts)
    }

    fn block_or_single_statement(&mut self) -> Result<Stmt, Galat> {
        if self.check(&TokenType::LeftBrace) {
            let span = self.peek().span.clone();
            let stmts = self.block_statement_list()?;
            Ok(Stmt::Block(stmts, span))
        } else {
            self.statement()
        }
    }

    fn assignment_or_expression_statement(&mut self) -> Result<Stmt, Galat> {
        let expr = self.expression()?;

        // Go short variable declaration: `x := nilai`
        if let Expr::Identifier(ref var_name, ref id_span) = expr {
            if self.check(&TokenType::Colon)
                && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].token_type == TokenType::Assign
            {
                self.advance(); // consume ':'
                self.advance(); // consume '='
                let rhs = self.expression()?;
                self.match_token(&[TokenType::Semicolon]);
                return Ok(Stmt::VarDecl {
                    name: var_name.clone(),
                    is_const: false,
                    initializer: Some(rhs),
                    span: id_span.clone(),
                });
            }

            // Go multi-variable short declaration or assignment: `a, b := expr` or `a, b = expr1, expr2`
            if self.match_token(&[TokenType::Comma]) {
                let mut names = vec![var_name.clone()];
                loop {
                    let next_id = self.consume_identifier("Harapkan nama variabel setelah ','")?;
                    names.push(next_id.lexeme);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
                let _ = self.match_token(&[TokenType::Colon]);
                self.consume(TokenType::Assign, "Harapkan ':=' atau '=' setelah daftar variabel")?;
                let initializer = self.parse_destructure_rhs(names.len(), id_span)?;
                self.match_token(&[TokenType::Semicolon]);
                return Ok(Stmt::DestructureDecl {
                    names,
                    initializer,
                    is_const: false,
                    span: id_span.clone(),
                });
            }
        }

        if self.match_token(&[
            TokenType::Assign,
            TokenType::PlusAssign,
            TokenType::MinusAssign,
            TokenType::StarAssign,
            TokenType::SlashAssign,
        ]) {
            let op_token = self.previous().clone();
            let op = match op_token.token_type {
                TokenType::Assign => AssignOp::Assign,
                TokenType::PlusAssign => AssignOp::PlusAssign,
                TokenType::MinusAssign => AssignOp::MinusAssign,
                TokenType::StarAssign => AssignOp::StarAssign,
                TokenType::SlashAssign => AssignOp::SlashAssign,
                _ => unreachable!(),
            };

            // Dereference pointer write: *ptr = nilai
            if let Expr::Call { ref callee, ref arguments, .. } = expr {
                if let Expr::Identifier(ref fn_name, _) = **callee {
                    if fn_name == "_deref_baca" && arguments.len() == 1 {
                        let value = self.expression()?;
                        self.match_token(&[TokenType::Semicolon]);
                        return Ok(Stmt::Expression(Expr::Call {
                            callee: Box::new(Expr::Identifier("_deref_tulis".to_string(), op_token.span.clone())),
                            arguments: vec![arguments[0].clone(), value],
                            span: op_token.span,
                        }));
                    }
                }
            }

            let target = match expr {
                Expr::Identifier(name, _) => AssignTarget::Variable(name),
                Expr::Index {
                    target,
                    index,
                    span: _,
                } => AssignTarget::Index { target, index },
                _ => {
                    return Err(Galat::sintaks(
                        "Target penugasan tidak valid (hanya variabel, elemen indeks, atau dereferensi pointer *ptr)",
                        expr.span(),
                    ));
                }
            };

            let value = self.expression()?;
            self.match_token(&[TokenType::Semicolon]);
            return Ok(Stmt::Assignment {
                target,
                op,
                value,
                span: op_token.span,
            });
        }

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Expression(expr))
    }

    // --- Expressions (Pratt Precedence) ---

    pub fn expression(&mut self) -> Result<Expr, Galat> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.and()?;

        if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__chan_op__") {
            let span = self.advance().span.clone();
            let val_expr = self.expression()?;
            return Ok(Expr::Call {
                callee: Box::new(Expr::Identifier("saluran_kirim".to_string(), span.clone())),
                arguments: vec![expr, val_expr],
                span,
            });
        }

        while self.match_token(&[TokenType::Atau]) {
            let span = self.previous().span.clone();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.bitwise_or()?;

        while self.match_token(&[TokenType::Dan]) {
            let span = self.previous().span.clone();
            let right = self.bitwise_or()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn bitwise_or(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.bitwise_xor()?;

        while matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__bitor__") {
            let span = self.advance().span.clone();
            let right = self.bitwise_xor()?;
            expr = Expr::Call {
                callee: Box::new(Expr::Identifier("_bit_atau".to_string(), span.clone())),
                arguments: vec![expr, right],
                span,
            };
        }

        Ok(expr)
    }

    fn bitwise_xor(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.bitwise_and()?;

        loop {
            if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__bitxor__") {
                let span = self.advance().span.clone();
                let right = self.bitwise_and()?;
                expr = Expr::Call {
                    callee: Box::new(Expr::Identifier("_bit_xor".to_string(), span.clone())),
                    arguments: vec![expr, right],
                    span,
                };
            } else if self.check(&TokenType::Caret)
                && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].token_type == TokenType::Caret
            {
                let span = self.advance().span.clone();
                self.advance(); // consume second '^'
                let right = self.bitwise_and()?;
                expr = Expr::Call {
                    callee: Box::new(Expr::Identifier("_bit_xor".to_string(), span.clone())),
                    arguments: vec![expr, right],
                    span,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn bitwise_and(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::Ampersand]) {
            let span = self.previous().span.clone();
            let right = self.equality()?;
            expr = Expr::Call {
                callee: Box::new(Expr::Identifier("_bit_dan".to_string(), span.clone())),
                arguments: vec![expr, right],
                span,
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.bitwise_shift()?;

        loop {
            if self.match_token(&[
                TokenType::Less,
                TokenType::LessEqual,
                TokenType::Greater,
                TokenType::GreaterEqual,
            ]) {
                let token = self.previous().clone();
                let op = match token.token_type {
                    TokenType::Less => BinaryOp::Less,
                    TokenType::LessEqual => BinaryOp::LessEqual,
                    TokenType::Greater => BinaryOp::Greater,
                    TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                    _ => unreachable!(),
                };
                let right = self.bitwise_shift()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                    span: token.span,
                };
            } else if self.match_token(&[TokenType::Dalam]) {
                let span = self.previous().span.clone();
                let right = self.bitwise_shift()?;
                expr = Expr::Call {
                    callee: Box::new(Expr::Identifier("_apakah_dalam".to_string(), span.clone())),
                    arguments: vec![expr, right],
                    span,
                };
            } else if self.check(&TokenType::Bukan)
                && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].token_type == TokenType::Dalam
            {
                let span = self.advance().span.clone(); // consume 'bukan'/'tidak'/'not'
                self.advance(); // consume 'dalam'/'in'
                let right = self.bitwise_shift()?;
                let in_call = Expr::Call {
                    callee: Box::new(Expr::Identifier("_apakah_dalam".to_string(), span.clone())),
                    arguments: vec![expr, right],
                    span: span.clone(),
                };
                expr = Expr::Unary {
                    op: UnaryOp::Not,
                    right: Box::new(in_call),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn bitwise_shift(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.range()?;

        loop {
            let is_shl = matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__shl__");
            let is_shr = matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__shr__");
            if is_shl || is_shr {
                let span = self.advance().span.clone();
                let fn_name = if is_shl { "_bit_kiri" } else { "_bit_kanan" };
                let right = self.range()?;
                expr = Expr::Call {
                    callee: Box::new(Expr::Identifier(fn_name.to_string(), span.clone())),
                    arguments: vec![expr, right],
                    span,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn range(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.term()?;

        if self.match_token(&[TokenType::DotDot]) {
            let span = self.previous().span.clone();
            let end = self.term()?;
            expr = Expr::Range {
                start: Box::new(expr),
                end: Box::new(end),
                span,
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.power()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Caret]) {
            let span = self.previous().span.clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Power,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Galat> {
        if self.match_token(&[TokenType::TungguHasil]) {
            let span = self.previous().span.clone();
            let right = self.unary()?;
            return Ok(Expr::Await(Box::new(right), span));
        }

        if self.match_token(&[TokenType::Volatil]) {
            let span = self.previous().span.clone();
            let _ = self.match_token(&[TokenType::Star]);
            let addr = self.unary()?;
            return Ok(Expr::VolatileRead {
                address: Box::new(addr),
                span,
            });
        }

        // Go Channel Receive: <-ch
        if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__chan_op__") {
            let span = self.advance().span.clone();
            let right = self.unary()?;
            return Ok(Expr::Call {
                callee: Box::new(Expr::Identifier("saluran_terima".to_string(), span.clone())),
                arguments: vec![right],
                span,
            });
        }

        // Bitwise NOT: ~expr
        if matches!(&self.peek().token_type, TokenType::Identifier(s) if s == "__bitnot__") {
            let span = self.advance().span.clone();
            let right = self.unary()?;
            return Ok(Expr::Call {
                callee: Box::new(Expr::Identifier("_bit_bukan".to_string(), span.clone())),
                arguments: vec![right],
                span,
            });
        }

        // Pointer Dereference: *ptr
        if self.match_token(&[TokenType::Star]) {
            let span = self.previous().span.clone();
            let right = self.unary()?;
            return Ok(Expr::Call {
                callee: Box::new(Expr::Identifier("_deref_baca".to_string(), span.clone())),
                arguments: vec![right],
                span,
            });
        }

        // Address-of / Borrow: &var or &mut var or pinjam var
        if self.match_token(&[TokenType::Ampersand, TokenType::Pinjam]) {
            let span = self.previous().span.clone();
            let _is_mut = self.match_token(&[TokenType::Mutabel]);
            let right = self.unary()?;
            return Ok(Expr::Call {
                callee: Box::new(Expr::Identifier("_alamat_dari".to_string(), span.clone())),
                arguments: vec![right],
                span,
            });
        }

        if self.match_token(&[TokenType::Minus, TokenType::Bukan]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Minus => UnaryOp::Negate,
                TokenType::Bukan => UnaryOp::Not,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op,
                right: Box::new(right),
                span: token.span,
            });
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.primary()?;

        loop {
            // Allow macro-style call `asm!(...)` / `rakitan!(...)` where `!` precedes `(`
            if self.check(&TokenType::Bukan)
                && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].token_type == TokenType::LeftParen
            {
                self.advance(); // consume '!'
            }
            if self.match_token(&[TokenType::LeftParen]) {
                let span = self.previous().span.clone();
                let mut arguments = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        // Support keyword argument: nama=nilai
                        if self.check_identifier()
                            && self.current + 1 < self.tokens.len()
                            && self.tokens[self.current + 1].token_type == TokenType::Assign
                        {
                            let kw_tok = self.advance();
                            let kw_span = kw_tok.span.clone();
                            self.advance(); // consume '='
                            let val_expr = self.expression()?;
                            arguments.push(Expr::Map(
                                vec![
                                    (
                                        Expr::String("_widya_kwarg".to_string(), kw_span.clone()),
                                        Expr::Boolean(true, kw_span.clone()),
                                    ),
                                    (
                                        Expr::String(kw_tok.lexeme, kw_span.clone()),
                                        val_expr,
                                    ),
                                ],
                                kw_span,
                            ));
                        } else {
                            arguments.push(self.expression()?);
                        }
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah argumen fungsi")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    arguments,
                    span,
                };
            } else if self.check(&TokenType::LeftBrace)
                && matches!(&expr, Expr::Identifier(s_name, _) if s_name.chars().next().is_some_and(|c| c.is_ascii_uppercase()))
                && self.current + 2 < self.tokens.len()
                && self.tokens[self.current + 2].token_type == TokenType::Colon
            {
                let span = self.advance().span.clone(); // consume '{'
                let mut arguments = Vec::new();
                while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                    let f_tok = self.consume_property_name("Harapkan nama field pada inisialisasi struktur")?;
                    let f_span = f_tok.span.clone();
                    self.consume(TokenType::Colon, "Harapkan ':' setelah nama field struktur")?;
                    let val_expr = self.expression()?;
                    arguments.push(Expr::Map(
                        vec![
                            (
                                Expr::String("_widya_kwarg".to_string(), f_span.clone()),
                                Expr::Boolean(true, f_span.clone()),
                            ),
                            (
                                Expr::String(f_tok.lexeme, f_span.clone()),
                                val_expr,
                            ),
                        ],
                        f_span,
                    ));
                    if !self.match_token(&[TokenType::Comma, TokenType::Semicolon]) {
                        break;
                    }
                }
                self.consume(TokenType::RightBrace, "Harapkan '}' setelah inisialisasi struktur")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    arguments,
                    span,
                };
            } else if self.match_token(&[TokenType::LeftBracket]) {
                let span = self.previous().span.clone();
                // Check for slice starting with ':' -> target[:end]
                if self.match_token(&[TokenType::Colon]) {
                    let start_expr = Expr::Nil(span.clone());
                    let end_expr = if !self.check(&TokenType::RightBracket) && !self.check(&TokenType::Colon) {
                        self.expression()?
                    } else {
                        Expr::Nil(span.clone())
                    };
                    let step_expr = if self.match_token(&[TokenType::Colon]) {
                        if !self.check(&TokenType::RightBracket) {
                            self.expression()?
                        } else {
                            Expr::Number(1.0, span.clone())
                        }
                    } else {
                        Expr::Number(1.0, span.clone())
                    };
                    self.consume(TokenType::RightBracket, "Harapkan ']' setelah irisan (slice)")?;
                    expr = Expr::Call {
                        callee: Box::new(Expr::Identifier("_iris_koleksi".to_string(), span.clone())),
                        arguments: vec![expr, start_expr, end_expr, step_expr],
                        span,
                    };
                } else {
                    let first_expr = self.expression()?;
                    if self.match_token(&[TokenType::Colon]) {
                        let end_expr = if !self.check(&TokenType::RightBracket) && !self.check(&TokenType::Colon) {
                            self.expression()?
                        } else {
                            Expr::Nil(span.clone())
                        };
                        let step_expr = if self.match_token(&[TokenType::Colon]) {
                            if !self.check(&TokenType::RightBracket) {
                                self.expression()?
                            } else {
                                Expr::Number(1.0, span.clone())
                            }
                        } else {
                            Expr::Number(1.0, span.clone())
                        };
                        self.consume(TokenType::RightBracket, "Harapkan ']' setelah irisan (slice)")?;
                        expr = Expr::Call {
                            callee: Box::new(Expr::Identifier("_iris_koleksi".to_string(), span.clone())),
                            arguments: vec![expr, first_expr, end_expr, step_expr],
                            span,
                        };
                    } else {
                        self.consume(TokenType::RightBracket, "Harapkan ']' setelah indeks")?;
                        expr = Expr::Index {
                            target: Box::new(expr),
                            index: Box::new(first_expr),
                            span,
                        };
                    }
                }
            } else if self.match_token(&[TokenType::Dot]) {
                let span = self.previous().span.clone();
                let prop_token = self.consume_property_name("Harapkan nama properti setelah '.'")?;
                let index = Expr::String(prop_token.lexeme, span.clone());
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else if self.match_token(&[TokenType::Colon]) {
                // Check if it's double colon '::'
                if self.match_token(&[TokenType::Colon]) {
                    let span = self.previous().span.clone();
                    let variant_token = self.consume_identifier("Harapkan nama varian enum setelah '::'")?;
                    let index = Expr::String(variant_token.lexeme, span.clone());
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                } else {
                    // Single colon is not part of postfix, backtrack
                    self.current -= 1;
                    break;
                }
            } else if self.match_token(&[TokenType::Question]) {
                let span = self.previous().span.clone();
                expr = Expr::Try(Box::new(expr), span);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_fstring_interpolation(&self, raw: &str, span: &crate::error::Span) -> Result<Expr, Galat> {
        let chars: Vec<char> = raw.chars().collect();
        let mut i = 0;
        let mut parts: Vec<Expr> = Vec::new();
        let mut current_lit = String::new();

        while i < chars.len() {
            if chars[i] == '{' {
                if i + 1 < chars.len() && chars[i + 1] == '{' {
                    current_lit.push('{');
                    i += 2;
                    continue;
                }
                if !current_lit.is_empty() {
                    parts.push(Expr::String(std::mem::take(&mut current_lit), span.clone()));
                }
                i += 1; // skip '{'
                let mut depth = 1;
                let mut expr_src = String::new();
                while i < chars.len() && depth > 0 {
                    if chars[i] == '{' {
                        depth += 1;
                        expr_src.push('{');
                    } else if chars[i] == '}' {
                        depth -= 1;
                        if depth > 0 {
                            expr_src.push('}');
                        }
                    } else {
                        expr_src.push(chars[i]);
                    }
                    i += 1;
                }
                // Optional Python format spec after ':' at top level (e.g. {nilai:.2f})
                let clean_expr_src = if let Some((expr_part, _fmt_spec)) = expr_src.split_once(':') {
                    if !expr_part.contains('{') && !expr_part.contains('"') {
                        expr_part.trim()
                    } else {
                        expr_src.trim()
                    }
                } else {
                    expr_src.trim()
                };
                if !clean_expr_src.is_empty() {
                    let mut sub_lexer = crate::lexer::Lexer::new(clean_expr_src);
                    let sub_tokens = sub_lexer.scan_tokens()?;
                    let mut sub_parser = Parser::new(sub_tokens);
                    let parsed_expr = sub_parser.expression()?;
                    parts.push(Expr::Call {
                        callee: Box::new(Expr::Identifier("ke_teks".to_string(), span.clone())),
                        arguments: vec![parsed_expr],
                        span: span.clone(),
                    });
                }
            } else if chars[i] == '}' && i + 1 < chars.len() && chars[i + 1] == '}' {
                current_lit.push('}');
                i += 2;
            } else {
                current_lit.push(chars[i]);
                i += 1;
            }
        }

        if !current_lit.is_empty() || parts.is_empty() {
            parts.push(Expr::String(current_lit, span.clone()));
        }

        let mut iter = parts.into_iter();
        let mut combined = iter.next().unwrap();
        for next_part in iter {
            combined = Expr::Binary {
                left: Box::new(combined),
                op: BinaryOp::Add,
                right: Box::new(next_part),
                span: span.clone(),
            };
        }
        Ok(combined)
    }

    fn primary(&mut self) -> Result<Expr, Galat> {
        let token = self.peek().clone();

        match &token.token_type {
            TokenType::Cocokkan if self.is_match_construct_at_current() => self.match_expression(),
            TokenType::Cocokkan => {
                let tok = self.advance();
                Ok(Expr::Identifier(tok.lexeme.clone(), tok.span.clone()))
            }
            TokenType::Number(n) => {
                let val = *n;
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Number(val, span))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                let span = token.span.clone();
                self.advance();
                if let Some(fstr_body) = val.strip_prefix("\x01FSTR:") {
                    self.parse_fstring_interpolation(fstr_body, &span)
                } else {
                    Ok(Expr::String(val, span))
                }
            }
            TokenType::Benar => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Boolean(true, span))
            }
            TokenType::Salah => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Boolean(false, span))
            }
            TokenType::Nihil => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Nil(span))
            }
            TokenType::Ini => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::This(span))
            }
            TokenType::Identifier(name) if name == "lambda" => {
                let span = token.span.clone();
                self.advance(); // consume 'lambda'
                let mut params = Vec::new();
                if !self.check(&TokenType::Colon) {
                    loop {
                        let p = self.consume_identifier("Harapkan nama parameter pada lambda")?;
                        params.push(p.lexeme);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::Colon, "Harapkan ':' setelah parameter lambda")?;
                let body_expr = self.expression()?;
                Ok(Expr::FunctionExpr {
                    params,
                    body: vec![Stmt::Return {
                        value: Some(body_expr),
                        span: span.clone(),
                    }],
                    span,
                })
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Identifier(name, span))
            }
            TokenType::LeftParen => {
                let span = token.span.clone();
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Harapkan ')' setelah ekspresi")?;
                Ok(Expr::Grouping(Box::new(expr), span))
            }
            TokenType::LeftBracket => self.array_literal(),
            TokenType::LeftBrace => self.map_literal(),
            TokenType::Fungsi => self.function_expression(),
            _ => Err(Galat::sintaks(
                format!("Ekspresi tidak terduga '{}'", token.lexeme),
                &token.span,
            )),
        }
    }

    fn match_expression(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::Cocokkan, "Harapkan 'cocokkan' atau 'cocok'")?.span;
        self.match_expression_with_keyword_consumed(span)
    }

    fn match_expression_with_keyword_consumed(&mut self, span: crate::error::Span) -> Result<Expr, Galat> {
        let has_paren = self.match_token(&[TokenType::LeftParen]);
        let target = self.expression()?;
        if has_paren {
            self.consume(TokenType::RightParen, "Harapkan ')' setelah target ekspresi 'cocok'")?;
        }
        self.consume(TokenType::LeftBrace, "Harapkan '{' setelah target ekspresi 'cocok'")?;

        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Comma, TokenType::Semicolon]) {
                continue;
            }

            let pattern_span = self.peek().span.clone();
            let pattern = self.match_pattern()?;

            // Optional guard: pola jika kondisi => aksi
            let _guard = if self.match_token(&[TokenType::Jika]) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TokenType::FatArrow, "Harapkan '=>' setelah pola cocokkan")?;

            let body = if self.check(&TokenType::LeftBrace) {
                let block_span = self.peek().span.clone();
                let stmts = self.block_statement_list()?;
                // Wrap block as an IIFE (Immediately Invoked Function Expression)
                Expr::Call {
                    callee: Box::new(Expr::FunctionExpr {
                        params: Vec::new(),
                        body: stmts,
                        span: block_span.clone(),
                    }),
                    arguments: Vec::new(),
                    span: block_span,
                }
            } else {
                let expr = self.expression()?;
                self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
                expr
            };

            arms.push(MatchArm {
                pattern,
                body,
                span: pattern_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok 'cocok'")?;
        Ok(Expr::Match {
            target: Box::new(target),
            arms,
            span,
        })
    }

    fn match_pattern(&mut self) -> Result<MatchPattern, Galat> {
        let token = self.peek().clone();

        match &token.token_type {
            TokenType::Identifier(name) if name == "_" => {
                self.advance();
                Ok(MatchPattern::Wildcard(token.span))
            }
            TokenType::Number(n) => {
                let val = *n;
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Number(val, span)))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::String(val, span)))
            }
            TokenType::Benar => {
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Boolean(true, span)))
            }
            TokenType::Salah => {
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Boolean(false, span)))
            }
            TokenType::Nihil => {
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Nil(span)))
            }
            TokenType::LeftParen => {
                let span = token.span.clone();
                self.advance();
                let mut bindings = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        let id = self.consume_identifier("Harapkan variabel dalam pola tuple")?;
                        bindings.push(id.lexeme);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah pola tuple")?;
                Ok(MatchPattern::EnumVariant {
                    enum_name: None,
                    variant_name: "Tuple".to_string(),
                    bindings,
                    span,
                })
            }
            TokenType::Identifier(first_id) => {
                let first_name = first_id.clone();
                let span = token.span.clone();
                self.advance();

                // Check for Enum::Variant or Variant(args...)
                if self.match_token(&[TokenType::Colon]) {
                    if self.match_token(&[TokenType::Colon]) {
                        let variant_token = self.consume_identifier("Harapkan nama varian enum setelah '::'")?;
                        let variant_name = variant_token.lexeme;
                        let mut bindings = Vec::new();
                        if self.match_token(&[TokenType::LeftParen]) {
                            if !self.check(&TokenType::RightParen) {
                                loop {
                                    let b = self.consume_identifier("Harapkan nama variabel pengikat varian")?;
                                    bindings.push(b.lexeme);
                                    if !self.match_token(&[TokenType::Comma]) {
                                        break;
                                    }
                                }
                            }
                            self.consume(TokenType::RightParen, "Harapkan ')' setelah pengikat varian")?;
                        }
                        return Ok(MatchPattern::EnumVariant {
                            enum_name: Some(first_name),
                            variant_name,
                            bindings,
                            span,
                        });
                    } else {
                        self.current -= 1;
                    }
                }

                if self.match_token(&[TokenType::LeftParen]) {
                    let mut bindings = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            let b = self.consume_identifier("Harapkan nama variabel pengikat varian")?;
                            bindings.push(b.lexeme);
                            if !self.match_token(&[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Harapkan ')' setelah pengikat varian")?;
                    return Ok(MatchPattern::EnumVariant {
                        enum_name: None,
                        variant_name: first_name,
                        bindings,
                        span,
                    });
                }

                Ok(MatchPattern::Identifier(first_name, span))
            }
            _ => Err(Galat::sintaks(
                format!("Pola tidak valid pada 'cocokkan': '{}'", token.lexeme),
                &token.span,
            )),
        }
    }

    fn array_literal(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::LeftBracket, "Harapkan '['")?.span;
        let mut elements = Vec::new();
        if !self.check(&TokenType::RightBracket) {
            let first_elem = self.expression()?;
            // Check for Python-style List Comprehension: [expr untuk var dalam iterable (jika kondisi)]
            if self.match_token(&[TokenType::Untuk]) {
                let mut clauses = Vec::new();
                loop {
                    let var_name = self.consume_identifier("Harapkan nama variabel pada list comprehension")?.lexeme;
                    self.consume(TokenType::Dalam, "Harapkan 'dalam'/'in' pada list comprehension")?;
                    let iterable = self.expression()?;
                    let cond = if self.match_token(&[TokenType::Jika]) {
                        Some(self.expression()?)
                    } else {
                        None
                    };
                    clauses.push((var_name, iterable, cond));
                    if !self.match_token(&[TokenType::Untuk]) {
                        break;
                    }
                }
                self.consume(TokenType::RightBracket, "Harapkan ']' untuk menutup list comprehension")?;

                let push_stmt = Stmt::Expression(Expr::Call {
                    callee: Box::new(Expr::Identifier("tambah".to_string(), span.clone())),
                    arguments: vec![
                        Expr::Identifier("_hasil_komp".to_string(), span.clone()),
                        first_elem,
                    ],
                    span: span.clone(),
                });

                let mut inner_stmt = push_stmt;
                for (var_name, iterable, cond) in clauses.into_iter().rev() {
                    let loop_body = if let Some(c) = cond {
                        Stmt::If {
                            condition: c,
                            then_branch: Box::new(inner_stmt),
                            elif_branches: Vec::new(),
                            else_branch: None,
                            span: span.clone(),
                        }
                    } else {
                        inner_stmt
                    };
                    inner_stmt = Stmt::ForIn {
                        var_name,
                        iterable,
                        body: Box::new(loop_body),
                        span: span.clone(),
                    };
                }

                let iife_body = vec![
                    Stmt::VarDecl {
                        name: "_hasil_komp".to_string(),
                        is_const: false,
                        initializer: Some(Expr::Array(Vec::new(), span.clone())),
                        span: span.clone(),
                    },
                    inner_stmt,
                    Stmt::Return {
                        value: Some(Expr::Identifier("_hasil_komp".to_string(), span.clone())),
                        span: span.clone(),
                    },
                ];

                return Ok(Expr::Call {
                    callee: Box::new(Expr::FunctionExpr {
                        params: Vec::new(),
                        body: iife_body,
                        span: span.clone(),
                    }),
                    arguments: Vec::new(),
                    span,
                });
            }

            elements.push(first_elem);
            while self.match_token(&[TokenType::Comma]) {
                if self.check(&TokenType::RightBracket) {
                    break;
                }
                elements.push(self.expression()?);
            }
        }
        self.consume(TokenType::RightBracket, "Harapkan ']' untuk menutup daftar")?;
        Ok(Expr::Array(elements, span))
    }

    fn map_literal(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::LeftBrace, "Harapkan '{'")?.span;
        let mut pairs = Vec::new();
        if !self.check(&TokenType::RightBrace) {
            let first_key = if self.check_identifier()
                && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].token_type == TokenType::Colon
            {
                // Could be literal key `nama: "Budi"` OR dict comprehension `{x: x*2 untuk x dalam ...}`
                // Look ahead to see if `untuk` appears before `,` or `}`
                if self.is_dict_comprehension_ahead() {
                    self.expression()?
                } else {
                    let id = self.advance();
                    Expr::String(id.lexeme, id.span)
                }
            } else {
                self.expression()?
            };

            self.consume(TokenType::Colon, "Harapkan ':' antara kunci dan nilai kamus")?;
            let first_val = self.expression()?;

            // Check for Python-style Dict Comprehension: {k: v untuk var dalam iterable (jika kondisi)}
            if self.match_token(&[TokenType::Untuk]) {
                let var_name = self.consume_identifier("Harapkan nama variabel pada dict comprehension")?.lexeme;
                self.consume(TokenType::Dalam, "Harapkan 'dalam'/'in' pada dict comprehension")?;
                let iterable = self.expression()?;
                let cond = if self.match_token(&[TokenType::Jika]) {
                    Some(self.expression()?)
                } else {
                    None
                };
                self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup dict comprehension")?;

                let assign_stmt = Stmt::Assignment {
                    target: AssignTarget::Index {
                        target: Box::new(Expr::Identifier("_kamus_komp".to_string(), span.clone())),
                        index: Box::new(first_key),
                    },
                    op: AssignOp::Assign,
                    value: first_val,
                    span: span.clone(),
                };

                let loop_body = if let Some(c) = cond {
                    Stmt::If {
                        condition: c,
                        then_branch: Box::new(assign_stmt),
                        elif_branches: Vec::new(),
                        else_branch: None,
                        span: span.clone(),
                    }
                } else {
                    assign_stmt
                };

                let iife_body = vec![
                    Stmt::VarDecl {
                        name: "_kamus_komp".to_string(),
                        is_const: false,
                        initializer: Some(Expr::Map(Vec::new(), span.clone())),
                        span: span.clone(),
                    },
                    Stmt::ForIn {
                        var_name,
                        iterable,
                        body: Box::new(loop_body),
                        span: span.clone(),
                    },
                    Stmt::Return {
                        value: Some(Expr::Identifier("_kamus_komp".to_string(), span.clone())),
                        span: span.clone(),
                    },
                ];

                return Ok(Expr::Call {
                    callee: Box::new(Expr::FunctionExpr {
                        params: Vec::new(),
                        body: iife_body,
                        span: span.clone(),
                    }),
                    arguments: Vec::new(),
                    span,
                });
            }

            pairs.push((first_key, first_val));

            while self.match_token(&[TokenType::Comma]) {
                if self.check(&TokenType::RightBrace) {
                    break;
                }
                let key = if self.check_identifier() {
                    let id = self.advance();
                    Expr::String(id.lexeme, id.span)
                } else {
                    self.expression()?
                };

                self.consume(TokenType::Colon, "Harapkan ':' antara kunci dan nilai kamus")?;
                let val = self.expression()?;
                pairs.push((key, val));
            }
        }
        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup kamus")?;
        Ok(Expr::Map(pairs, span))
    }

    fn is_dict_comprehension_ahead(&self) -> bool {
        let mut idx = self.current;
        let mut depth = 0;
        while idx < self.tokens.len() {
            match &self.tokens[idx].token_type {
                TokenType::LeftBrace | TokenType::LeftBracket | TokenType::LeftParen => depth += 1,
                TokenType::RightBrace | TokenType::RightBracket | TokenType::RightParen => {
                    if depth == 0 {
                        return false;
                    }
                    depth -= 1;
                }
                TokenType::Comma if depth == 0 => return false,
                TokenType::Untuk if depth == 0 => return true,
                _ => {}
            }
            idx += 1;
        }
        false
    }

    fn function_expression(&mut self) -> Result<Expr, Galat> {
        let span = self.advance().span.clone();
        self.function_expression_with_keyword_consumed(span)
    }

    fn function_expression_with_keyword_consumed(&mut self, span: crate::error::Span) -> Result<Expr, Galat> {
        self.consume(TokenType::LeftParen, "Harapkan '(' setelah 'fungsi'")?;
        let (params, mut body) = self.parse_function_params_with_defaults()?;
        self.consume(TokenType::RightParen, "Harapkan ')' setelah daftar parameter")?;
        if self.match_token(&[TokenType::Minus]) {
            if self.match_token(&[TokenType::Greater]) {
                let _ = self.parse_type_annotation()?;
            } else {
                self.current -= 1;
            }
        }
        let parsed_body = self.block_statement_list()?;
        body.extend(parsed_body);
        Ok(Expr::FunctionExpr { params, body, span })
    }

    // --- Helpers for Generic Types and Type Annotations ---

    /// Parse optional generic type parameters like `<T, U: Sama>`
    fn parse_optional_generic_params(&mut self) -> Result<Vec<String>, Galat> {
        if !self.match_token(&[TokenType::Less]) {
            return Ok(Vec::new());
        }

        let mut params = Vec::new();
        if !self.check(&TokenType::Greater) {
            loop {
                let param_token = self.consume_identifier("Harapkan nama parameter tipe generik")?;
                let mut param_name = param_token.lexeme;
                // Optional trait bound: T: Sifat + Sifat2
                if self.match_token(&[TokenType::Colon]) {
                    let mut bounds = Vec::new();
                    loop {
                        let bound = self.consume_identifier("Harapkan nama sifat pada batas tipe generik")?;
                        bounds.push(bound.lexeme);
                        if !self.match_token(&[TokenType::Plus]) {
                            break;
                        }
                    }
                    param_name = format!("{}: {}", param_name, bounds.join(" + "));
                }
                params.push(param_name);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::Greater, "Harapkan '>' untuk menutup parameter tipe generik")?;
        Ok(params)
    }

    /// Parse optional generic arguments like `<Angka, String>`
    fn parse_optional_generic_args(&mut self) -> Result<Vec<String>, Galat> {
        if !self.match_token(&[TokenType::Less]) {
            return Ok(Vec::new());
        }

        let mut args = Vec::new();
        if !self.check(&TokenType::Greater) {
            loop {
                let arg = self.parse_type_annotation()?;
                args.push(arg);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::Greater, "Harapkan '>' untuk menutup argumen tipe generik")?;
        Ok(args)
    }

    /// Parse type annotation string recursively (e.g. `Angka`, `Daftar<String>`, `Hasil<T, Galat>`, `(Angka, String)`)
    fn parse_type_annotation(&mut self) -> Result<String, Galat> {
        if self.match_token(&[TokenType::LeftParen]) {
            let mut tuple_types = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    tuple_types.push(self.parse_type_annotation()?);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Harapkan ')' setelah tipe tuple")?;
            return Ok(format!("({})", tuple_types.join(", ")));
        }

        if self.match_token(&[TokenType::Star]) {
            let _ = self.match_token(&[TokenType::Mutabel, TokenType::Tetap]);
            let inner = self.parse_type_annotation()?;
            return Ok(format!("*{}", inner));
        }

        if self.match_token(&[TokenType::Ampersand]) {
            let _ = self.match_token(&[TokenType::Mutabel]);
            let inner = self.parse_type_annotation()?;
            return Ok(format!("&{}", inner));
        }

        if self.match_token(&[TokenType::LeftBracket]) {
            let inner = self.parse_type_annotation()?;
            let mut len_suffix = String::new();
            if self.match_token(&[TokenType::Semicolon]) {
                if let TokenType::Number(n) = self.peek().token_type {
                    self.advance();
                    len_suffix = format!("; {}", n as usize);
                }
            }
            self.consume(TokenType::RightBracket, "Harapkan ']' setelah tipe array")?;
            return Ok(format!("[{}{}]", inner, len_suffix));
        }

        let type_token = self.peek().clone();
        let base_token = if self.check_identifier() {
            self.advance()
        } else if matches!(
            self.peek().token_type,
            TokenType::Fungsi
                | TokenType::Struktur
                | TokenType::Enum
                | TokenType::Sifat
                | TokenType::Nihil
                | TokenType::Benar
                | TokenType::Salah
        ) {
            self.advance()
        } else {
            return Err(Galat::sintaks("Harapkan nama tipe", &type_token.span));
        };
        let mut type_str = base_token.lexeme;

        if self.check(&TokenType::Less) {
            let gen_args = self.parse_optional_generic_args()?;
            type_str = format!("{}<{}>", type_str, gen_args.join(", "));
        }

        Ok(type_str)
    }

    /// Parse optional where clause: `dimana T: Sifat, U: Sifat2`
    fn parse_where_clause(&mut self) -> Result<(), Galat> {
        loop {
            let _param = self.consume_identifier("Harapkan nama parameter tipe dalam klausa 'dimana'")?;
            self.consume(TokenType::Colon, "Harapkan ':' setelah parameter tipe")?;
            let _bound = self.consume_identifier("Harapkan nama sifat dalam batas klausa 'dimana'")?;
            if !self.match_token(&[TokenType::Comma]) {
                break;
            }
        }
        Ok(())
    }

    // --- Helpers ---

    fn is_match_construct_at_current(&self) -> bool {
        let mut idx = self.current + 1;
        if idx >= self.tokens.len() {
            return false;
        }
        if self.tokens[idx].token_type == TokenType::LeftParen {
            let mut depth = 1;
            idx += 1;
            while idx < self.tokens.len() && depth > 0 {
                match self.tokens[idx].token_type {
                    TokenType::LeftParen => depth += 1,
                    TokenType::RightParen => depth -= 1,
                    _ => {}
                }
                idx += 1;
            }
            if depth != 0 || idx >= self.tokens.len() {
                return false;
            }
        } else {
            let mut depth = 0;
            while idx < self.tokens.len() {
                match &self.tokens[idx].token_type {
                    TokenType::LeftBrace if depth == 0 => break,
                    TokenType::LeftParen | TokenType::LeftBracket => depth += 1,
                    TokenType::RightParen | TokenType::RightBracket => {
                        if depth > 0 { depth -= 1; } else { return false; }
                    }
                    TokenType::Semicolon | TokenType::Comma | TokenType::Assign | TokenType::EqualEqual | TokenType::BangEqual => {
                        if depth == 0 { return false; }
                    }
                    _ => {}
                }
                idx += 1;
            }
        }
        if idx < self.tokens.len() && self.tokens[idx].token_type == TokenType::LeftBrace {
            let mut b_depth = 1;
            let mut scan = idx + 1;
            while scan < self.tokens.len() && b_depth > 0 {
                match &self.tokens[scan].token_type {
                    TokenType::FatArrow if b_depth == 1 => return true,
                    TokenType::LeftBrace => b_depth += 1,
                    TokenType::RightBrace => b_depth -= 1,
                    _ => {}
                }
                scan += 1;
            }
        }
        false
    }

    fn check_identifier(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Identifier(_) | TokenType::Cocokkan)
    }

    fn check_next_is_jika(&self) -> bool {
        if self.current + 1 < self.tokens.len() {
            matches!(self.tokens[self.current + 1].token_type, TokenType::Jika)
        } else {
            false
        }
    }

    fn consume_identifier(&mut self, err_msg: &str) -> Result<Token, Galat> {
        if self.check_identifier() {
            Ok(self.advance())
        } else {
            Err(Galat::sintaks(err_msg, &self.peek().span))
        }
    }

    fn consume_property_name(&mut self, err_msg: &str) -> Result<Token, Galat> {
        if self.is_at_end() {
            return Err(Galat::sintaks(err_msg, &self.peek().span));
        }
        match self.peek().token_type {
            TokenType::RightBracket
            | TokenType::RightParen
            | TokenType::RightBrace
            | TokenType::Semicolon
            | TokenType::Comma
            | TokenType::Eof => Err(Galat::sintaks(err_msg, &self.peek().span)),
            _ => Ok(self.advance()),
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, err_msg: &str) -> Result<Token, Galat> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(Galat::sintaks(err_msg, &self.peek().span))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_source(source: &str) -> Result<Program, Galat> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_parse_generic_function() {
        let code = "fungsi tukar<T, U>(a: T, b: U) -> T { kembalikan a; }";
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse generic function: {:?}", program.err());
    }

    #[test]
    fn test_parse_generic_struct() {
        let code = "struktur Kantong<T: Sama> { isi: T, fungsi ambil() -> T { kembalikan ini.isi; } }";
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse generic struct: {:?}", program.err());
    }

    #[test]
    fn test_parse_generic_enum() {
        let code = "enum Hasil<T, E> { Ok(T), Err(E) }";
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse generic enum: {:?}", program.err());
    }

    #[test]
    fn test_parse_pattern_matching_expression() {
        let code = r#"
        misal pesan = cocok(nilai) {
            0 => "nol",
            1 => "satu",
            Hasil::Ok(v) => "sukses",
            _ => "lainnya",
        };
        "#;
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse pattern matching: {:?}", program.err());
    }

    #[test]
    fn test_parse_pattern_matching_statement_with_guards() {
        let code = r#"
        cocok(nilai) {
            x jika x > 100 => cetak("besar"),
            _ => cetak("kecil"),
        }
        "#;
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse match statement: {:?}", program.err());
    }

    #[test]
    fn test_parse_type_annotations_in_variables() {
        let code = "misal x: Angka = 42; tetap nama: String = \"Widya\";";
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse type annotations: {:?}", program.err());
    }

    #[test]
    fn test_parse_derived_traits_attribute() {
        let code = r#"
        #[turunkan(Tunjukkan, Sama)]
        struktur Titik {
            x: Angka,
            y: Angka,
        }
        "#;
        let program = parse_source(code);
        assert!(program.is_ok(), "Gagal parse derived traits: {:?}", program.err());
    }
}

