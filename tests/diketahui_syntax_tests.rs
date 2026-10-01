use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::value::Value;

#[test]
fn test_diketahui_and_diket_variable_declarations() {
    let source = r#"
        // Formulasi Fisika: Menghitung Gaya (F = m * a)
        diketahui massa = 50;
        diket percepatan = 2;
        misal gaya = massa * percepatan;

        // Formulasi Geometri: Luas Lingkaran
        diketahui jari_jari = 7;
        diket pi = 3.14;
        misal luas = pi * jari_jari * jari_jari;

        // Mutasi variabel
        gaya = gaya + 100;
        
        gaya;
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program).expect("Interpreter gagal");

    assert_eq!(result, Value::Number(200.0));
}

#[test]
fn test_diketahui_inside_functions() {
    let source = r#"
        fungsi hitung_energi_kinetik(m, v) {
            diketahui massa = m;
            diket kecepatan = v;
            kembalikan 0.5 * massa * (kecepatan * kecepatan);
        }

        hitung_energi_kinetik(10, 4);
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer gagal");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser gagal");

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&program).expect("Interpreter gagal");

    assert_eq!(result, Value::Number(80.0));
}
