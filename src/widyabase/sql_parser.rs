// ============================================================================
// WIDYABASE BILINGUAL SQL LEXER & PARSER (sql_parser.rs)
// ============================================================================
// Mendukung kueri dwibahasa secara simetris:
// - Dialek Nusantara : PILIH, DARI, DIMANA, SISIP_KE, NILAI, PERBARUI, ATUR, HAPUS_DARI, dll.
// - Standar Global   : SELECT, FROM, WHERE, INSERT INTO, VALUES, UPDATE, SET, DELETE, dll.
// Keduanya menghasilkan Abstract Syntax Tree (AST) kanonikal yang 100% identik!
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Select,
    From,
    Where,
    Insert,
    InsertOrIgnore,  // SISIP_ATAU_ABAIKAN / INSERT OR IGNORE
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Drop,
    If,      // IF / JIKA
    Exists,  // EXISTS / ADA
    Join,
    On,
    Group,
    Order,
    By,
    Asc,
    Desc,
    Limit,
    Offset,
    And,
    Or,
    Not,
    Like,
    Between,
    In,
    Null,
    PrimaryKey,
    Begin,
    Commit,
    Rollback,

    // Pilar Kueri Skala Industri
    Having,
    Distinct,
    Case,
    When,
    Then,
    Else,
    End,
    Is,
    Union,
    UnionAll,
    All,
    Left,
    Right,
    Outer,
    LeftJoin,
    RightJoin,

    // Aggregates & Functions
    Count,
    Sum,
    Avg,
    Min,
    Max,

    // Pilar Enterprise / Titan
    With,
    Over,
    Partition,
    View,
    Index,
    Alter,
    Add,
    Column,
    Savepoint,
    Release,
    To,
    RollbackTo,
    Foreign,
    Key,
    References,
    Cascade,
    DropView,
    DropIndex,
    DropColumn,

    // Standar Industri Baru
    Serial,          // SERIAL / AUTO_INCREMENT / OTOMATIS
    Default,         // DEFAULT / BAWAAN
    UniqueConstraint,// UNIQUE (constraint kolom) — berbeda dari DISTINCT
    Returning,       // RETURNING / KEMBALIKAN
    Truncate,        // TRUNCATE / KOSONGKAN
    Vacuum,          // VACUUM / BERSIHKAN
    Conflict,        // CONFLICT / KONFLIK (untuk ON CONFLICT)
    Do,              // DO / LAKUKAN (untuk ON CONFLICT DO)
    Nothing,         // NOTHING / TIDAK_ADA_AKSI (DO NOTHING)
    Rename,          // RENAME / GANTI_NAMA
    Type,            // TYPE / TIPE (ALTER COLUMN TYPE)
    Intersect,       // INTERSECT / IRISAN
    Except,          // EXCEPT / KECUALI
    Full,            // FULL (FULL OUTER JOIN)
    Cross,           // CROSS (CROSS JOIN)
    Natural,         // NATURAL (NATURAL JOIN)
    Unique,          // UNIQUE keyword untuk CREATE UNIQUE INDEX
    Percent,         // % / MOD operator
    Show,            // SHOW / TAMPILKAN (SHOW TABLES)
    Tables,          // TABLES (SHOW TABLES)
    Columns,         // COLUMNS (SHOW COLUMNS)
    Explain,         // EXPLAIN / JELASKAN
    Analyze,         // ANALYZE / ANALISIS
    DropTableIfExists,  // DROP TABLE IF EXISTS (combined)
    DropViewIfExists,   // DROP VIEW IF EXISTS
    DropIndexIfExists,  // DROP INDEX IF EXISTS
    InsertIntoSelect,   // INSERT INTO ... SELECT marker

    // Literals & Identifiers
    Ident(String),
    StringLit(String),
    NumberLit(f64),
    BooleanLit(bool),

    // Symbols
    Asterisk,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Slash,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Dot,
}

pub struct SqlLexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> SqlLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        SqlLexer {
            input,
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];

            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }

            // Komentar baris (-- atau //)
            if (c == '-' && self.peek_next() == Some('-')) || (c == '/' && self.peek_next() == Some('/')) {
                while self.pos < self.chars.len() && self.chars[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue;
            }

            match c {
                '+' => { tokens.push(Token::Plus); self.pos += 1; }
                '-' => {
                    let is_prev_operand = match tokens.last() {
                        Some(Token::Ident(_)) | Some(Token::NumberLit(_)) | Some(Token::RightParen) | Some(Token::StringLit(_)) => true,
                        _ => false,
                    };
                    if !is_prev_operand && self.peek_next().map(|n| n.is_ascii_digit()).unwrap_or(false) {
                        tokens.push(self.read_number()?);
                    } else {
                        tokens.push(Token::Minus);
                        self.pos += 1;
                    }
                }
                '/' => { tokens.push(Token::Slash); self.pos += 1; }
                '*' => { tokens.push(Token::Asterisk); self.pos += 1; }
                ',' => { tokens.push(Token::Comma); self.pos += 1; }
                ';' => { tokens.push(Token::Semicolon); self.pos += 1; }
                '(' => { tokens.push(Token::LeftParen); self.pos += 1; }
                ')' => { tokens.push(Token::RightParen); self.pos += 1; }
                '=' => { tokens.push(Token::Equal); self.pos += 1; }
                '!' if self.peek_next() == Some('=') => { tokens.push(Token::NotEqual); self.pos += 2; }
                '<' if self.peek_next() == Some('>') => { tokens.push(Token::NotEqual); self.pos += 2; }
                '>' if self.peek_next() == Some('=') => { tokens.push(Token::GreaterEqual); self.pos += 2; }
                '<' if self.peek_next() == Some('=') => { tokens.push(Token::LessEqual); self.pos += 2; }
                '>' => { tokens.push(Token::GreaterThan); self.pos += 1; }
                '<' => { tokens.push(Token::LessThan); self.pos += 1; }
                '.' => { tokens.push(Token::Dot); self.pos += 1; }
                '\'' | '"' => {
                    tokens.push(self.read_string(c)?);
                }
                _ if c.is_ascii_digit() => {
                    tokens.push(self.read_number()?);
                }
                _ if c.is_alphabetic() || c == '_' || c == ':' => {
                    tokens.push(self.read_identifier_or_keyword());
                }
                _ => {
                    return Err(format!("Karakter tak dikenal pada SQL: '{}'", c));
                }
            }
        }

        Ok(tokens)
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.chars.len() {
            Some(self.chars[self.pos + 1])
        } else {
            None
        }
    }

    fn read_string(&mut self, quote: char) -> Result<Token, String> {
        self.pos += 1; // lewati quote pembuka
        let mut s = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == quote {
                self.pos += 1; // lewati quote penutup
                return Ok(Token::StringLit(s));
            }
            s.push(c);
            self.pos += 1;
        }
        Err("String SQL tidak ditutup dengan benar".to_string())
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut s = String::new();
        if self.chars[self.pos] == '-' {
            s.push('-');
            self.pos += 1;
        }
        while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.') {
            s.push(self.chars[self.pos]);
            self.pos += 1;
        }
        s.parse::<f64>()
            .map(Token::NumberLit)
            .map_err(|e| format!("Gagal mengurai angka '{}': {}", s, e))
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut s = String::new();
        while self.pos < self.chars.len() && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_' || self.chars[self.pos] == ':' || self.chars[self.pos] == '-') {
            s.push(self.chars[self.pos]);
            self.pos += 1;
        }

        let upper = s.to_uppercase();
        match upper.as_str() {
            // Dialek Nusantara & Standar Global
            "SELECT" | "PILIH" => Token::Select,
            "FROM" | "DARI" => Token::From,
            "WHERE" | "DIMANA" | "DI_MANA" => Token::Where,
            "INSERT" | "SISIP" | "MASUKKAN" | "SISIP_KE" | "MASUKKAN_KE" | "INSERT_INTO" => Token::Insert,
            "SISIP_ATAU_ABAIKAN" | "INSERT_OR_IGNORE" | "INSERT_IGNORE" | "MASUKKAN_ATAU_ABAIKAN" => Token::InsertOrIgnore,
            "INTO" | "KE" | "KE_DALAM" => Token::Into,
            "VALUES" | "NILAI" => Token::Values,
            "UPDATE" | "PERBARUI" | "UBAH" => Token::Update,
            "SET" | "ATUR" => Token::Set,
            "DELETE" | "HAPUS" | "HAPUS_DARI" | "DELETE_FROM" => Token::Delete,
            "CREATE" | "BUAT" | "CREATE_TABLE" | "BUAT_TABEL" => Token::Create,
            "TABLE" | "TABEL" => Token::Table,
            "DROP" | "BUANG" | "HAPUS_TABEL" | "BUANG_TABEL" | "DROP_TABLE" => Token::Drop,
            // IF NOT EXISTS / JIKA TIDAK ADA
            "IF" | "JIKA" => Token::If,
            "EXISTS" | "ADA" => Token::Exists,
            "JOIN" | "GABUNG" | "HUBUNGKAN" | "INNER_JOIN" | "GABUNG_DENGAN" => Token::Join,
            "ON" | "PADA" => Token::On,
            "GROUP" | "KELOMPOKKAN" | "GROUP_BY" | "KELOMPOKKAN_BERDASARKAN" => Token::Group,
            "ORDER" | "URUTKAN" | "ORDER_BY" | "URUTKAN_BERDASARKAN" => Token::Order,
            "BY" | "BERDASARKAN" => Token::By,
            "ASC" | "NAIK" => Token::Asc,
            "DESC" | "TURUN" => Token::Desc,
            "LIMIT" | "BATAS" => Token::Limit,
            "OFFSET" | "LEWATI" => Token::Offset,
            "AND" | "DAN" => Token::And,
            "OR" | "ATAU" => Token::Or,
            "NOT" | "BUKAN" | "TIDAK" => Token::Not,
            "LIKE" | "SEPERTI" | "MIRIP" => Token::Like,
            "BETWEEN" | "ANTARA" => Token::Between,
            "IN" | "DI_DALAM" | "DIDALAM" => Token::In,
            "NULL" | "NIHIL" => Token::Null,
            "TRUE" | "BENAR" => Token::BooleanLit(true),
            "FALSE" | "SALAH" => Token::BooleanLit(false),
            "PRIMARY_KEY" | "KUNCI_UTAMA" => Token::PrimaryKey,
            "BEGIN" | "MULAI" => Token::Begin,
            "COMMIT" | "KOMIT" | "SIMPAN" => Token::Commit,
            "ROLLBACK" | "BATALKAN" | "GUGURKAN" => Token::Rollback,

            // Pilar Kueri Skala Industri
            "HAVING" | "MEMILIKI" => Token::Having,
            "DISTINCT" | "BERBEDA" | "UNIK" => Token::Distinct,
            "UNIQUE" | "KOLOM_UNIK" | "UNIK_KOLOM" => Token::UniqueConstraint,
            "CASE" | "KASUS" => Token::Case,
            "WHEN" | "KETIKA" => Token::When,
            "THEN" | "MAKA" => Token::Then,
            "ELSE" | "LAINNYA" | "SELAIN_ITU" => Token::Else,
            "END" | "SELESAI" => Token::End,
            "IS" | "ADALAH" => Token::Is,
            "UNION" | "GABUNGAN" => Token::Union,
            "UNION_ALL" | "GABUNGAN_SEMUA" => Token::UnionAll,
            "ALL" | "SEMUA" => Token::All,
            "LEFT" | "KIRI" => Token::Left,
            "RIGHT" | "KANAN" => Token::Right,
            "OUTER" | "LUAR" => Token::Outer,
            "LEFT_JOIN" | "GABUNG_KIRI" => Token::LeftJoin,
            "RIGHT_JOIN" | "GABUNG_KANAN" => Token::RightJoin,
            "FULL" | "PENUH" | "FULL_JOIN" | "FULL_OUTER_JOIN" | "GABUNG_PENUH" => Token::Full,
            "CROSS" | "SILANG" | "CROSS_JOIN" | "GABUNG_SILANG" => Token::Cross,
            "NATURAL" | "ALAMI" | "NATURAL_JOIN" | "GABUNG_ALAMI" => Token::Natural,

            // Aggregates
            "COUNT" | "HITUNG" => Token::Count,
            "SUM" | "JUMLAH" => Token::Sum,
            "AVG" | "RATA_RATA" => Token::Avg,
            "MIN" => Token::Min,
            "MAX" | "MAKS" => Token::Max,

            // Pilar Enterprise / Titan
            "WITH" | "DENGAN" => Token::With,
            "OVER" | "LEWAT" => Token::Over,
            "PARTITION" | "PARTISI" | "PARTITION_BY" | "PARTISI_BERDASARKAN" => Token::Partition,
            "VIEW" | "TAMPILAN" | "CREATE_VIEW" | "BUAT_TAMPILAN" => Token::View,
            "INDEX" | "INDEKS" | "CREATE_INDEX" | "BUAT_INDEKS" => Token::Index,
            "DROP_VIEW" | "HAPUS_TAMPILAN" | "BUANG_TAMPILAN" => Token::DropView,
            "DROP_INDEX" | "HAPUS_INDEKS" | "BUANG_INDEKS" => Token::DropIndex,
            "ALTER" | "ALTER_TABLE" | "UBAH_TABEL" => Token::Alter,
            "ADD" | "TAMBAH" | "TAMBAH_KOLOM" => Token::Add,
            "COLUMN" | "KOLOM" => Token::Column,
            "DROP_COLUMN" | "HAPUS_KOLOM" | "BUANG_KOLOM" => Token::DropColumn,
            "SAVEPOINT" | "TITIK_SIMPAN" => Token::Savepoint,
            "RELEASE" | "LEPAS" | "LEPAS_TITIK_SIMPAN" | "RELEASE_SAVEPOINT" => Token::Release,
            "TO" => Token::To,
            "ROLLBACK_TO" | "BATALKAN_KE" | "GUGURKAN_KE" => Token::RollbackTo,
            "FOREIGN" | "FOREIGN_KEY" | "KUNCI_ASING" => Token::Foreign,
            "KEY" | "KUNCI" => Token::Key,
            "REFERENCES" | "REFERENSI" | "MERUJUK_PADA" => Token::References,
            "CASCADE" | "KASKADE" => Token::Cascade,

            // Standar Industri Baru
            "SERIAL" | "AUTO_INCREMENT" | "OTOMATIS" | "SERIAL_PK" => Token::Serial,
            "DEFAULT" | "BAWAAN" | "NILAI_BAWAAN" => Token::Default,
            "RETURNING" | "KEMBALIKAN" | "HASILKAN" => Token::Returning,
            "TRUNCATE" | "KOSONGKAN" | "TRUNCATE_TABLE" => Token::Truncate,
            "VACUUM" | "BERSIHKAN_MVCC" | "KOMPAKKAN" => Token::Vacuum,
            "CONFLICT" | "KONFLIK" => Token::Conflict,
            "DO" | "LAKUKAN" => Token::Do,
            "NOTHING" | "TIDAK_ADA_AKSI" | "ABAIKAN" => Token::Nothing,
            "RENAME" | "GANTI_NAMA" => Token::Rename,
            "TYPE" | "TIPE" => Token::Type,
            "INTERSECT" | "IRISAN" | "POTONG" => Token::Intersect,
            "EXCEPT" | "KECUALI" | "MINUS" => Token::Except,
            "SHOW" | "TAMPILKAN" | "DAFTAR" => Token::Show,
            "TABLES" | "TABEL_SEMUA" | "DAFTAR_TABEL" => Token::Tables,
            "COLUMNS" | "KOLOM_SEMUA" | "DAFTAR_KOLOM" => Token::Columns,
            "EXPLAIN" | "JELASKAN" | "RENCANA_KUERI" => Token::Explain,
            "ANALYZE" | "ANALISIS" => Token::Analyze,
            "UNIQUE_INDEX" | "INDEKS_UNIK" | "BUAT_INDEKS_UNIK" => Token::Unique,

            _ => Token::Ident(s),
        }
    }
}

// ============================================================================
// ABSTRACT SYNTAX TREE (AST) UNTUK WIDYABASE
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    Like,
    NotLike,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlExpr {
    Column(String),
    Literal(SqlValue),
    Binary {
        left: Box<SqlExpr>,
        op: BinaryOp,
        right: Box<SqlExpr>,
    },
    FunctionCall {
        name: String,
        args: Vec<SqlExpr>,
    },
    Between {
        expr: Box<SqlExpr>,
        low: Box<SqlExpr>,
        high: Box<SqlExpr>,
        negated: bool,
    },
    InList {
        expr: Box<SqlExpr>,
        list: Vec<SqlExpr>,
        negated: bool,
    },
    IsNull {
        expr: Box<SqlExpr>,
        negated: bool,
    },
    Case {
        conditions: Vec<(SqlExpr, SqlExpr)>,
        else_branch: Option<Box<SqlExpr>>,
    },
    InSubquery {
        expr: Box<SqlExpr>,
        subquery: Box<SqlStatement>,
        negated: bool,
    },
    Subquery(Box<SqlStatement>),
    Alias {
        expr: Box<SqlExpr>,
        alias: String,
    },
    WindowFunction {
        func: String,
        args: Vec<SqlExpr>,
        partition_by: Option<String>,
        order_by: Option<OrderByClause>,
    },
    Wildcard,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ForeignKeyConstraint {
    pub column: String,
    pub foreign_table: String,
    pub foreign_column: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CteDefinition {
    pub name: String,
    pub query: Box<SqlStatement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SqlValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Text(String),
}

impl SqlValue {
    pub fn as_text(&self) -> String {
        match self {
            SqlValue::Null => "NULL".to_string(),
            SqlValue::Boolean(b) => b.to_string(),
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Text(s) => s.clone(),
        }
    }
}

impl std::fmt::Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderByClause {
    pub column: String,
    pub is_desc: bool,
    pub secondary: Vec<(String, bool)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    FullOuter, // FULL OUTER JOIN
    Cross,     // CROSS JOIN
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: String,
    pub table_alias: Option<String>,
    pub on_left: String,
    pub on_right: String,
    pub on_expr: Option<Box<SqlExpr>>, // ekspresi ON yang lebih kompleks
}

/// Definisi kolom lengkap dengan constraint standar industri
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub col_type: String,
    pub not_null: bool,           // NOT NULL / TIDAK_NIHIL
    pub default_value: Option<String>, // DEFAULT <value> / BAWAAN <value>
    pub is_unique: bool,          // UNIQUE constraint / KOLOM_UNIK
    pub is_serial: bool,          // SERIAL / AUTO_INCREMENT
    pub is_primary_key: bool,     // PRIMARY KEY inline
    pub references: Option<(String, String)>, // REFERENCES tabel(kolom)
}

/// Opsi ON CONFLICT untuk INSERT
#[derive(Debug, Clone, PartialEq)]
pub enum OnConflict {
    DoNothing,                    // ON CONFLICT DO NOTHING
    DoUpdate {
        conflict_target: Option<String>,           // kolom yang konflik
        assignments: Vec<(String, SqlExpr)>,  // SET col = val
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlStatement {
    Select {
        columns: Vec<SqlExpr>,
        from_table: String,
        from_alias: Option<String>,
        join: Option<JoinClause>,
        joins: Vec<JoinClause>,           // Multi-JOIN support
        where_clause: Option<SqlExpr>,
        group_by: Vec<String>,            // Multi-column GROUP BY
        having: Option<SqlExpr>,
        order_by: Option<OrderByClause>,
        limit: Option<usize>,
        offset: Option<usize>,
        is_distinct: bool,
    },
    Insert {
        table: String,
        columns: Vec<String>,
        values: Vec<SqlExpr>,
        returning: Option<Vec<SqlExpr>>,  // RETURNING kolom1, kolom2
        on_conflict: Option<OnConflict>,  // ON CONFLICT DO ...
    },
    InsertSelect {
        table: String,
        columns: Vec<String>,
        select: Box<SqlStatement>,        // INSERT INTO t SELECT ...
        returning: Option<Vec<SqlExpr>>,
        on_conflict: Option<OnConflict>,
    },
    Update {
        table: String,
        assignments: Vec<(String, SqlExpr)>,
        where_clause: Option<SqlExpr>,
        returning: Option<Vec<SqlExpr>>,  // RETURNING
    },
    Delete {
        table: String,
        where_clause: Option<SqlExpr>,
        returning: Option<Vec<SqlExpr>>,  // RETURNING
    },
    CreateTable {
        name: String,
        columns: Vec<(String, String)>,   // backward compat (nama, tipe)
        column_defs: Vec<ColumnDef>,      // definisi kolom lengkap
        primary_key: Option<String>,
        foreign_keys: Vec<ForeignKeyConstraint>,
        if_not_exists: bool,
    },
    InsertOrIgnore {
        table: String,
        columns: Vec<String>,
        values: Vec<SqlExpr>,
    },
    TruncateTable {
        name: String,
    },
    VacuumDb {
        table: Option<String>,            // VACUUM [table] — opsional
    },
    DropTable {
        name: String,
        if_exists: bool,                  // DROP TABLE IF EXISTS
    },
    DropView {
        name: String,
        if_exists: bool,
    },
    DropIndex {
        name: String,
        if_exists: bool,
    },
    BeginTransaction,
    CommitTransaction,
    RollbackTransaction,
    Union {
        left: Box<SqlStatement>,
        right: Box<SqlStatement>,
        is_all: bool,
    },
    Intersect {
        left: Box<SqlStatement>,
        right: Box<SqlStatement>,
    },
    Except {
        left: Box<SqlStatement>,
        right: Box<SqlStatement>,
    },
    With {
        ctes: Vec<CteDefinition>,
        body: Box<SqlStatement>,
    },
    CreateView {
        name: String,
        query: Box<SqlStatement>,
        if_not_exists: bool,
    },
    CreateIndex {
        name: String,
        table: String,
        column: String,
        is_unique: bool,                  // CREATE UNIQUE INDEX
        if_not_exists: bool,
    },
    AlterTableAddColumn {
        table: String,
        column: String,
        col_type: String,
        not_null: bool,
        default_value: Option<String>,
    },
    AlterTableDropColumn {
        table: String,
        column: String,
    },
    AlterTableRenameColumn {
        table: String,
        old_name: String,
        new_name: String,
    },
    AlterTableChangeType {
        table: String,
        column: String,
        new_type: String,
    },
    ShowTables,
    ShowColumns {
        table: String,
    },
    ExplainStatement {
        stmt: Box<SqlStatement>,
        analyze: bool,
    },
    Savepoint {
        name: String,
    },
    RollbackToSavepoint {
        name: String,
    },
    ReleaseSavepoint {
        name: String,
    },

    // ========================================================================
    // PILAR ANALITIK KOLUMNAR (ClickHouse-Compatible)
    // ========================================================================
    CreateColumnarTable {
        name: String,
        columns: Vec<(String, String)>,
        engine: String,
        order_by: Vec<String>,
        if_not_exists: bool,
    },
    InsertColumnar {
        table: String,
        columns: Vec<String>,
        rows: Vec<Vec<SqlValue>>,
    },
    ColumnarAggregation {
        table: String,
        aggregates: Vec<(String, String, Option<String>)>, // (func, col, alias)
        group_by: Vec<String>,
        where_col: Option<(String, String, SqlValue)>,
        order_by: Option<(String, bool)>,
        limit: Option<usize>,
    },

    // ========================================================================
    // PILAR IN-MEMORY CACHE & KV (Redis-Compatible)
    // ========================================================================
    CacheSet {
        key: String,
        value: String,
        ttl_secs: Option<u64>,
    },
    CacheGet {
        key: String,
    },
    CacheDel {
        key: String,
    },
    CacheExpire {
        key: String,
        ttl_secs: u64,
    },
    CacheTtl {
        key: String,
    },
    CacheIncr {
        key: String,
        by: i64,
    },
    CacheHSet {
        key: String,
        field: String,
        value: String,
    },
    CacheHGet {
        key: String,
        field: String,
    },
    CacheHGetAll {
        key: String,
    },
    CacheLPush {
        key: String,
        values: Vec<String>,
    },
    CacheRPush {
        key: String,
        values: Vec<String>,
    },
    CacheLPop {
        key: String,
    },
    CacheRPop {
        key: String,
    },
    CacheLRange {
        key: String,
        start: i64,
        stop: i64,
    },
    CacheSAdd {
        key: String,
        members: Vec<String>,
    },
    CacheSMembers {
        key: String,
    },
    CacheSRem {
        key: String,
        members: Vec<String>,
    },
    CacheZAdd {
        key: String,
        score: f64,
        member: String,
    },
    CacheZRange {
        key: String,
        start: i64,
        stop: i64,
        with_scores: bool,
    },
    CachePublish {
        channel: String,
        message: String,
    },
}

pub struct SqlParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl SqlParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        SqlParser { tokens, pos: 0 }
    }

    pub fn parse_statement(&mut self) -> Result<SqlStatement, String> {
        let mut stmt = self.parse_single_statement()?;

        while self.pos < self.tokens.len() {
            if self.tokens[self.pos] == Token::Union || self.tokens[self.pos] == Token::UnionAll {
                let is_union_all_token = self.tokens[self.pos] == Token::UnionAll;
                self.pos += 1;
                let mut is_all = is_union_all_token;
                if !is_all && self.pos < self.tokens.len() && self.tokens[self.pos] == Token::All {
                    is_all = true;
                    self.pos += 1;
                }
                let right = self.parse_single_statement()?;
                stmt = SqlStatement::Union {
                    left: Box::new(stmt),
                    right: Box::new(right),
                    is_all,
                };
            } else if self.tokens[self.pos] == Token::Intersect {
                self.pos += 1;
                let right = self.parse_single_statement()?;
                stmt = SqlStatement::Intersect {
                    left: Box::new(stmt),
                    right: Box::new(right),
                };
            } else if self.tokens[self.pos] == Token::Except {
                self.pos += 1;
                let right = self.parse_single_statement()?;
                stmt = SqlStatement::Except {
                    left: Box::new(stmt),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Semicolon {
            self.pos += 1;
        }

        Ok(stmt)
    }

    pub fn parse_single_statement(&mut self) -> Result<SqlStatement, String> {
        if self.pos >= self.tokens.len() {
            return Err("Kueri SQL kosong".to_string());
        }

        match &self.tokens[self.pos] {
            Token::Select => self.parse_select(),
            Token::Insert => self.parse_insert(),
            Token::InsertOrIgnore => self.parse_insert_or_ignore(),
            Token::Update => self.parse_update(),
            Token::Delete => self.parse_delete(),
            Token::Truncate => self.parse_truncate_table(),
            Token::Vacuum => self.parse_vacuum(),
            Token::Show => self.parse_show(),
            Token::Explain => self.parse_explain(),
            Token::Create => {
                if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::View {
                    self.pos += 1; // lewati CREATE
                    self.parse_create_view()
                } else if self.pos + 1 < self.tokens.len() && (self.tokens[self.pos + 1] == Token::Index || self.tokens[self.pos + 1] == Token::Unique) {
                    self.pos += 1; // lewati CREATE
                    self.parse_create_index()
                } else {
                    self.parse_create_table()
                }
            }
            Token::View => self.parse_create_view(),
            Token::Index => self.parse_create_index(),
            Token::Drop => {
                if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::View {
                    self.pos += 1; // lewati DROP
                    self.parse_drop_view()
                } else if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Index {
                    self.pos += 1; // lewati DROP
                    self.parse_drop_index()
                } else {
                    self.parse_drop_table()
                }
            }
            Token::DropView => self.parse_drop_view(),
            Token::DropIndex => self.parse_drop_index(),
            Token::Alter => self.parse_alter_table(),
            Token::Savepoint => self.parse_savepoint(),
            Token::Release => self.parse_release_savepoint(),
            Token::With => self.parse_with(),
            Token::Begin => { self.pos += 1; Ok(SqlStatement::BeginTransaction) }
            Token::Commit => { self.pos += 1; Ok(SqlStatement::CommitTransaction) }
            Token::Rollback => {
                self.pos += 1;
                if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::To || self.tokens[self.pos] == Token::Into) {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Savepoint {
                        self.pos += 1;
                    }
                    let name = match self.tokens.get(self.pos) {
                        Some(Token::Ident(n)) => { let res = n.clone(); self.pos += 1; res }
                        _ => return Err("Diharapkan nama titik simpan setelah ROLLBACK TO".to_string()),
                    };
                    return Ok(SqlStatement::RollbackToSavepoint { name });
                }
                Ok(SqlStatement::RollbackTransaction)
            }
            Token::RollbackTo => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Savepoint {
                    self.pos += 1;
                }
                let name = match self.tokens.get(self.pos) {
                    Some(Token::Ident(n)) => { let res = n.clone(); self.pos += 1; res }
                    _ => return Err("Diharapkan nama titik simpan setelah ROLLBACK TO".to_string()),
                };
                Ok(SqlStatement::RollbackToSavepoint { name })
            }
            Token::Set => self.parse_cache_set(),
            Token::Ident(cmd) => {
                let upper = cmd.to_uppercase();
                match upper.as_str() {
                    "SET" | "ATUR_CACHE" | "SET_CACHE" => self.parse_cache_set(),
                    "GET" | "AMBIL_CACHE" | "GET_CACHE" => self.parse_cache_get(),
                    "DEL" | "DELETE_CACHE" | "HAPUS_CACHE" => self.parse_cache_del(),
                    "EXPIRE" | "KADALUWARSA" => self.parse_cache_expire(),
                    "TTL" | "SISA_WAKTU" => self.parse_cache_ttl(),
                    "INCR" | "TAMBAH_SATU" | "INCRBY" | "TAMBAH_NILAI" => self.parse_cache_incr(),
                    "HSET" | "ATUR_HASH" => self.parse_cache_hset(),
                    "HGET" | "AMBIL_HASH" => self.parse_cache_hget(),
                    "HGETALL" | "AMBIL_SEMUA_HASH" => self.parse_cache_hgetall(),
                    "LPUSH" | "DORONG_KIRI" => self.parse_cache_lpush(),
                    "RPUSH" | "DORONG_KANAN" => self.parse_cache_rpush(),
                    "LPOP" | "TARIK_KIRI" => self.parse_cache_lpop(),
                    "RPOP" | "TARIK_KANAN" => self.parse_cache_rpop(),
                    "LRANGE" | "RENTANG_LIST" => self.parse_cache_lrange(),
                    "SADD" | "TAMBAH_SET" => self.parse_cache_sadd(),
                    "SMEMBERS" | "ANGGOTA_SET" => self.parse_cache_smembers(),
                    "SREM" | "HAPUS_SET" => self.parse_cache_srem(),
                    "ZADD" | "TAMBAH_ZSET" => self.parse_cache_zadd(),
                    "ZRANGE" | "RENTANG_ZSET" => self.parse_cache_zrange(),
                    "PUBLISH" | "SIARKAN" => self.parse_cache_publish(),
                    _ => Err(format!("Pernyataan SQL/Perintah tidak didukung di awal kueri: {}", cmd)),
                }
            }
            other => Err(format!("Pernyataan SQL tidak didukung di awal kueri: {:?}", other)),
        }
    }

    fn parse_returning_clause(&mut self) -> Result<Option<Vec<SqlExpr>>, String> {
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Returning {
            self.pos += 1; // lewati RETURNING
            let mut exprs = Vec::new();
            loop {
                if self.pos >= self.tokens.len() {
                    break;
                }
                if self.tokens[self.pos] == Token::Asterisk {
                    self.pos += 1;
                    exprs.push(SqlExpr::Wildcard);
                } else {
                    exprs.push(self.parse_expression()?);
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            Ok(Some(exprs))
        } else {
            Ok(None)
        }
    }

    fn parse_on_conflict_clause(&mut self) -> Result<Option<OnConflict>, String> {
        if self.pos >= self.tokens.len() {
            return Ok(None);
        }

        let mut is_conflict = false;
        if self.tokens[self.pos] == Token::On {
            self.pos += 1;
            if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Conflict || matches!(&self.tokens[self.pos], Token::Ident(c) if c.eq_ignore_ascii_case("CONFLICT") || c.eq_ignore_ascii_case("KONFLIK"))) {
                self.pos += 1;
                is_conflict = true;
            } else {
                self.pos -= 1;
            }
        } else if self.tokens[self.pos] == Token::Conflict {
            self.pos += 1;
            is_conflict = true;
        } else if self.tokens[self.pos] == Token::If {
            self.pos += 1;
            if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Conflict || matches!(&self.tokens[self.pos], Token::Ident(c) if c.eq_ignore_ascii_case("CONFLICT") || c.eq_ignore_ascii_case("KONFLIK"))) {
                self.pos += 1;
                is_conflict = true;
            } else {
                self.pos -= 1;
            }
        } else if let Token::Ident(w) = &self.tokens[self.pos] {
            let up = w.to_uppercase();
            if up == "JIKA_KONFLIK" || up == "ON_CONFLICT" || up == "PADA_KONFLIK" {
                self.pos += 1;
                is_conflict = true;
            } else if up == "JIKA" || up == "PADA" {
                if self.pos + 1 < self.tokens.len() {
                    let next_is_conflict = match &self.tokens[self.pos + 1] {
                        Token::Conflict => true,
                        Token::Ident(c) => c.eq_ignore_ascii_case("KONFLIK") || c.eq_ignore_ascii_case("CONFLICT"),
                        _ => false,
                    };
                    if next_is_conflict {
                        self.pos += 2;
                        is_conflict = true;
                    }
                }
            }
        }

        if !is_conflict {
            return Ok(None);
        }

        // Kolom target konflik opsional: (kolom)
        let mut conflict_target = None;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
            self.pos += 1;
            conflict_target = Some(self.parse_column_name()?);
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                self.pos += 1;
            }
        }

        if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Do || matches!(&self.tokens[self.pos], Token::Ident(d) if d.eq_ignore_ascii_case("DO") || d.eq_ignore_ascii_case("LAKUKAN"))) {
            self.pos += 1;
        }

        if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Nothing || matches!(&self.tokens[self.pos], Token::Ident(n) if n.eq_ignore_ascii_case("NOTHING") || n.eq_ignore_ascii_case("ABAIKAN") || n.eq_ignore_ascii_case("TIDAK_ADA_AKSI"))) {
            self.pos += 1;
            return Ok(Some(OnConflict::DoNothing));
        } else if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Update || matches!(&self.tokens[self.pos], Token::Ident(u) if u.eq_ignore_ascii_case("UPDATE") || u.eq_ignore_ascii_case("PERBARUI") || u.eq_ignore_ascii_case("UBAH"))) {
            self.pos += 1;
            if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Set || matches!(&self.tokens[self.pos], Token::Ident(s) if s.eq_ignore_ascii_case("SET") || s.eq_ignore_ascii_case("ATUR"))) {
                self.pos += 1;
            }
            let mut assignments = Vec::new();
            loop {
                let col = match self.tokens.get(self.pos) {
                    Some(Token::Ident(c)) => {
                        let col_name = c.clone();
                        self.pos += 1;
                        col_name
                    }
                    _ => break,
                };
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Equal {
                    return Err("Diharapkan '=' pada DO UPDATE SET".to_string());
                }
                self.pos += 1;
                let val_expr = self.parse_single_value_expr()?;
                assignments.push((col, val_expr));
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            return Ok(Some(OnConflict::DoUpdate {
                conflict_target,
                assignments,
            }));
        }

        Ok(None)
    }

    fn parse_alias_name(&mut self) -> Option<String> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let name = match &self.tokens[self.pos] {
            Token::Ident(s) => Some(s.clone()),
            Token::Union => Some("gabungan".to_string()),
            Token::UnionAll => Some("gabungan_semua".to_string()),
            Token::Values => Some("nilai".to_string()),
            Token::Set => Some("atur".to_string()),
            Token::Limit => Some("limit".to_string()),
            Token::Offset => Some("offset".to_string()),
            Token::Order => Some("order".to_string()),
            Token::Group => Some("group".to_string()),
            Token::Where => Some("where".to_string()),
            Token::Case => Some("case".to_string()),
            Token::When => Some("when".to_string()),
            Token::Then => Some("then".to_string()),
            Token::Else => Some("else".to_string()),
            Token::End => Some("end".to_string()),
            Token::Distinct => Some("distinct".to_string()),
            Token::Having => Some("having".to_string()),
            Token::Left => Some("left".to_string()),
            Token::Right => Some("right".to_string()),
            Token::Outer => Some("outer".to_string()),
            Token::All => Some("all".to_string()),
            Token::Count => Some("count".to_string()),
            Token::Sum => Some("sum".to_string()),
            Token::Avg => Some("avg".to_string()),
            Token::Min => Some("min".to_string()),
            Token::Max => Some("max".to_string()),
            Token::With => Some("dengan".to_string()),
            Token::Over => Some("lewat".to_string()),
            Token::Partition => Some("partisi".to_string()),
            Token::View => Some("tampilan".to_string()),
            Token::Index => Some("indeks".to_string()),
            Token::Alter => Some("ubah".to_string()),
            Token::Add => Some("tambah".to_string()),
            Token::Column => Some("kolom".to_string()),
            Token::Savepoint => Some("titik_simpan".to_string()),
            Token::Release => Some("lepas".to_string()),
            Token::To => Some("ke".to_string()),
            Token::Foreign => Some("asing".to_string()),
            Token::Key => Some("kunci".to_string()),
            Token::References => Some("referensi".to_string()),
            Token::Cascade => Some("kaskade".to_string()),
            _ => None,
        };
        if name.is_some() {
            self.pos += 1;
        }
        name
    }

    fn parse_column_name(&mut self) -> Result<String, String> {
        let first = match self.tokens.get(self.pos) {
            Some(Token::Ident(id)) => {
                let s = id.clone();
                self.pos += 1;
                s
            }
            Some(Token::Values) => {
                self.pos += 1;
                "nilai".to_string()
            }
            Some(Token::Key) => {
                self.pos += 1;
                "kunci".to_string()
            }
            Some(Token::Index) => {
                self.pos += 1;
                "indeks".to_string()
            }
            Some(Token::View) => {
                self.pos += 1;
                "tampilan".to_string()
            }
            Some(Token::Column) => {
                self.pos += 1;
                "kolom".to_string()
            }
            _ => return Err("Diharapkan nama kolom".to_string()),
        };

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Dot {
            self.pos += 1;
            if let Some(Token::Ident(second)) = self.tokens.get(self.pos) {
                let res = format!("{}.{}", first, second);
                self.pos += 1;
                return Ok(res);
            } else if let Some(Token::Values) = self.tokens.get(self.pos) {
                let res = format!("{}.nilai", first);
                self.pos += 1;
                return Ok(res);
            } else if let Some(Token::Key) = self.tokens.get(self.pos) {
                let res = format!("{}.kunci", first);
                self.pos += 1;
                return Ok(res);
            } else {
                return Err("Diharapkan nama kolom setelah '.'".to_string());
            }
        }
        Ok(first)
    }

    fn parse_select(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SELECT / PILIH

        // Cek BERBEDA / DISTINCT / UNIK
        let mut is_distinct = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Distinct {
            is_distinct = true;
            self.pos += 1;
        }

        // Parsing kolom / ekspresi
        let mut columns = Vec::new();
        loop {
            if self.pos >= self.tokens.len() {
                return Err("Kueri SELECT tidak lengkap, diharapkan DARI / FROM".to_string());
            }

            let mut expr = if self.tokens[self.pos] == Token::Asterisk {
                self.pos += 1;
                SqlExpr::Wildcard
            } else {
                self.parse_additive()?
            };

            // Tangani alias opsional: AS / SEBAGAI <alias> atau langsung <alias>
            if self.pos < self.tokens.len() {
                if let Token::Ident(w) = &self.tokens[self.pos] {
                    if w.eq_ignore_ascii_case("AS") || w.eq_ignore_ascii_case("SEBAGAI") {
                        self.pos += 1;
                        if let Some(alias) = self.parse_alias_name() {
                            expr = SqlExpr::Alias { expr: Box::new(expr), alias };
                        }
                    }
                }
            }

            columns.push(expr);

            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                self.pos += 1;
            } else {
                break;
            }
        }

        // DARI / FROM opsional (misal: SELECT NOW() atau PILIH SEKARANG())
        let mut from_table = String::new();
        let mut from_alias = None;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::From {
            self.pos += 1;
            from_table = match self.tokens.get(self.pos) {
                Some(Token::Ident(tbl)) => {
                    let t = tbl.clone();
                    self.pos += 1;
                    t
                }
                _ => return Err("Diharapkan nama tabel setelah klausul DARI / FROM".to_string()),
            };

            // Tangani alias tabel: AS <alias>, SEBAGAI <alias>, atau langsung <alias>
            if self.pos < self.tokens.len() {
                if let Token::Ident(w) = &self.tokens[self.pos] {
                    if w.eq_ignore_ascii_case("AS") || w.eq_ignore_ascii_case("SEBAGAI") {
                        self.pos += 1;
                        from_alias = self.parse_alias_name();
                    } else if !matches!(self.tokens[self.pos], Token::Where | Token::Group | Token::Order | Token::Limit | Token::Offset | Token::Join | Token::LeftJoin | Token::RightJoin | Token::Left | Token::Right | Token::Having | Token::Union | Token::UnionAll | Token::Semicolon) {
                        from_alias = self.parse_alias_name();
                    }
                }
            }
        }

        // Multi-JOIN support
        let mut joins = Vec::new();
        while self.pos < self.tokens.len() {
            let mut detected_join_type = None;

            if self.tokens[self.pos] == Token::LeftJoin {
                self.pos += 1;
                detected_join_type = Some(JoinType::Left);
            } else if self.tokens[self.pos] == Token::RightJoin {
                self.pos += 1;
                detected_join_type = Some(JoinType::Right);
            } else if self.tokens[self.pos] == Token::Left {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Outer {
                    self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                    self.pos += 1;
                }
                detected_join_type = Some(JoinType::Left);
            } else if self.tokens[self.pos] == Token::Right {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Outer {
                    self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                    self.pos += 1;
                }
                detected_join_type = Some(JoinType::Right);
            } else if self.tokens[self.pos] == Token::Full {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Outer {
                    self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                    self.pos += 1;
                }
                detected_join_type = Some(JoinType::FullOuter);
            } else if self.tokens[self.pos] == Token::Cross {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                    self.pos += 1;
                }
                detected_join_type = Some(JoinType::Cross);
            } else if self.tokens[self.pos] == Token::Join {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Left {
                    self.pos += 1;
                    detected_join_type = Some(JoinType::Left);
                } else if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Right {
                    self.pos += 1;
                    detected_join_type = Some(JoinType::Right);
                } else {
                    detected_join_type = Some(JoinType::Inner);
                }
            } else if let Token::Ident(w) = &self.tokens[self.pos] {
                if w.eq_ignore_ascii_case("INNER") || w.eq_ignore_ascii_case("DALAM") {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                        self.pos += 1;
                        detected_join_type = Some(JoinType::Inner);
                    }
                } else if w.eq_ignore_ascii_case("FULL") || w.eq_ignore_ascii_case("PENUH") {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Outer {
                        self.pos += 1;
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                        self.pos += 1;
                    }
                    detected_join_type = Some(JoinType::FullOuter);
                } else if w.eq_ignore_ascii_case("CROSS") || w.eq_ignore_ascii_case("SILANG") {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Join {
                        self.pos += 1;
                    }
                    detected_join_type = Some(JoinType::Cross);
                }
            }

            if let Some(j_type) = detected_join_type {
                let join_table = match self.tokens.get(self.pos) {
                    Some(Token::Ident(tbl)) => {
                        let t = tbl.clone();
                        self.pos += 1;
                        t
                    }
                    _ => return Err("Diharapkan nama tabel setelah klausa GABUNG / JOIN".to_string()),
                };

                let mut join_alias = None;
                if self.pos < self.tokens.len() {
                    if let Token::Ident(w) = &self.tokens[self.pos] {
                        if w.eq_ignore_ascii_case("AS") || w.eq_ignore_ascii_case("SEBAGAI") {
                            self.pos += 1;
                            join_alias = self.parse_alias_name();
                        } else if self.tokens[self.pos] != Token::On {
                            join_alias = self.parse_alias_name();
                        }
                    }
                }

                let mut left_col = String::new();
                let mut right_col = String::new();

                if j_type != JoinType::Cross {
                    if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::On {
                        return Err("Diharapkan kata kunci PADA / ON setelah tabel GABUNG / JOIN".to_string());
                    }
                    self.pos += 1;

                    left_col = self.parse_column_name()?;
                    if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Equal {
                        return Err("Diharapkan operator '=' pada klausa PADA / ON".to_string());
                    }
                    self.pos += 1;
                    right_col = self.parse_column_name()?;
                }

                joins.push(JoinClause {
                    join_type: j_type,
                    table: join_table,
                    table_alias: join_alias,
                    on_left: left_col,
                    on_right: right_col,
                    on_expr: None,
                });
            } else {
                break;
            }
        }

        let join = joins.first().cloned();

        // Klausul opsional: WHERE, GROUP BY, HAVING, ORDER BY, LIMIT, OFFSET
        let mut where_clause = None;
        let mut group_by = Vec::new();
        let mut having = None;
        let mut order_by = None;
        let mut limit = None;
        let mut offset = None;

        while self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Where if where_clause.is_none() => {
                    self.pos += 1;
                    where_clause = Some(self.parse_expression()?);
                }
                Token::Group if group_by.is_empty() => {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::By {
                        self.pos += 1;
                    }
                    while self.pos < self.tokens.len() {
                        let col = self.parse_column_name()?;
                        group_by.push(col);
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                }
                Token::Having if having.is_none() => {
                    self.pos += 1;
                    having = Some(self.parse_expression()?);
                }
                Token::Order if order_by.is_none() => {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::By {
                        self.pos += 1;
                    }
                    if self.pos < self.tokens.len() && matches!(self.tokens.get(self.pos), Some(Token::Ident(_))) {
                        let c = self.parse_column_name()?;
                        let mut is_desc = false;
                        if self.pos < self.tokens.len() {
                            if self.tokens[self.pos] == Token::Desc {
                                is_desc = true;
                                self.pos += 1;
                            } else if self.tokens[self.pos] == Token::Asc {
                                self.pos += 1;
                            }
                        }
                        let mut secondary = Vec::new();
                        while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                            self.pos += 1;
                            if self.pos < self.tokens.len() && matches!(self.tokens.get(self.pos), Some(Token::Ident(_))) {
                                let c2 = self.parse_column_name()?;
                                let mut d2 = false;
                                if self.pos < self.tokens.len() {
                                    if self.tokens[self.pos] == Token::Desc {
                                        d2 = true;
                                        self.pos += 1;
                                    } else if self.tokens[self.pos] == Token::Asc {
                                        self.pos += 1;
                                    }
                                }
                                secondary.push((c2, d2));
                            } else {
                                break;
                            }
                        }
                        order_by = Some(OrderByClause { column: c, is_desc, secondary });
                    }
                }
                Token::Limit if limit.is_none() => {
                    self.pos += 1;
                    if let Some(Token::NumberLit(n)) = self.tokens.get(self.pos) {
                        limit = Some(*n as usize);
                        self.pos += 1;
                    }
                }
                Token::Offset if offset.is_none() => {
                    self.pos += 1;
                    if let Some(Token::NumberLit(n)) = self.tokens.get(self.pos) {
                        offset = Some(*n as usize);
                        self.pos += 1;
                    }
                }
                _ => break,
            }
        }

        Ok(SqlStatement::Select {
            columns,
            from_table,
            from_alias,
            join,
            joins,
            where_clause,
            group_by,
            having,
            order_by,
            limit,
            offset,
            is_distinct,
        })
    }

    fn parse_insert(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati INSERT / SISIP / MASUKKAN

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Into {
            self.pos += 1;
        }

        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => {
                let t = tbl.clone();
                self.pos += 1;
                t
            }
            _ => return Err("Diharapkan nama tabel pada klausa INSERT INTO / SISIP_KE".to_string()),
        };

        // Kolom opsional (kolom1, kolom2)
        let mut columns = Vec::new();
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
            self.pos += 1;
            while self.pos < self.tokens.len() {
                if let Token::Ident(c) = &self.tokens[self.pos] {
                    columns.push(c.clone());
                    self.pos += 1;
                } else if self.tokens[self.pos] == Token::Values {
                    columns.push("nilai".to_string());
                    self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                self.pos += 1;
            }
        }

        // Cek apakah INSERT INTO ... SELECT
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Select {
            let select_stmt = self.parse_select()?;
            let returning = self.parse_returning_clause()?;
            let on_conflict = self.parse_on_conflict_clause()?;
            return Ok(SqlStatement::InsertSelect {
                table,
                columns,
                select: Box::new(select_stmt),
                returning,
                on_conflict,
            });
        }

        // VALUES / NILAI
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Values {
            return Err("Diharapkan kata kunci VALUES / NILAI".to_string());
        }
        self.pos += 1;

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
            return Err("Diharapkan tanda '(' setelah VALUES / NILAI".to_string());
        }
        self.pos += 1;

        let mut values = Vec::new();
        while self.pos < self.tokens.len() {
            let expr = self.parse_single_value_expr()?;
            values.push(expr);
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                self.pos += 1;
            } else {
                break;
            }
        }

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
            self.pos += 1;
        }

        let on_conflict = self.parse_on_conflict_clause()?;
        let returning = self.parse_returning_clause()?;

        Ok(SqlStatement::Insert {
            table,
            columns,
            values,
            returning,
            on_conflict,
        })
    }

    fn parse_update(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati UPDATE / PERBARUI / UBAH
        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => {
                let t = tbl.clone();
                self.pos += 1;
                t
            }
            _ => return Err("Diharapkan nama tabel pada klausa UPDATE / PERBARUI".to_string()),
        };

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Set {
            return Err("Diharapkan klausa SET / ATUR".to_string());
        }
        self.pos += 1;

        let mut assignments = Vec::new();
        loop {
            let col = match self.tokens.get(self.pos) {
                Some(Token::Ident(c)) => {
                    let col_name = c.clone();
                    self.pos += 1;
                    col_name
                }
                Some(Token::Values) => {
                    self.pos += 1;
                    "nilai".to_string()
                }
                _ => break,
            };

            if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Equal {
                return Err("Diharapkan tanda '=' pada assignment UPDATE".to_string());
            }
            self.pos += 1;

            let val_expr = self.parse_single_value_expr()?;
            assignments.push((col, val_expr));

            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                self.pos += 1;
            } else {
                break;
            }
        }

        let mut where_clause = None;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Where {
            self.pos += 1;
            where_clause = Some(self.parse_expression()?);
        }

        let returning = self.parse_returning_clause()?;

        Ok(SqlStatement::Update {
            table,
            assignments,
            where_clause,
            returning,
        })
    }

    fn parse_delete(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati DELETE / HAPUS
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::From {
            self.pos += 1;
        }

        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => {
                let t = tbl.clone();
                self.pos += 1;
                t
            }
            _ => return Err("Diharapkan nama tabel pada DELETE FROM / HAPUS_DARI".to_string()),
        };

        let mut where_clause = None;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Where {
            self.pos += 1;
            where_clause = Some(self.parse_expression()?);
        }

        let returning = self.parse_returning_clause()?;

        Ok(SqlStatement::Delete {
            table,
            where_clause,
            returning,
        })
    }

    fn parse_insert_or_ignore(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SISIP_ATAU_ABAIKAN
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Into {
            self.pos += 1;
        }
        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => { let t = tbl.clone(); self.pos += 1; t }
            _ => return Err("Diharapkan nama tabel pada SISIP_ATAU_ABAIKAN".to_string()),
        };
        let mut columns = Vec::new();
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
            self.pos += 1;
            while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                if let Token::Ident(c) = &self.tokens[self.pos] {
                    columns.push(c.clone()); self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma { self.pos += 1; } else { break; }
            }
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen { self.pos += 1; }
        }
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Values {
            return Err("Diharapkan NILAI / VALUES pada SISIP_ATAU_ABAIKAN".to_string());
        }
        self.pos += 1;
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
            return Err("Diharapkan '(' setelah VALUES".to_string());
        }
        self.pos += 1;
        let mut values = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
            values.push(self.parse_single_value_expr()?);
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma { self.pos += 1; } else { break; }
        }
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen { self.pos += 1; }
        Ok(SqlStatement::InsertOrIgnore { table, columns, values })
    }

    fn parse_create_table(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati CREATE / BUAT
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Table {
            self.pos += 1;
        }

        // Tangani IF NOT EXISTS / JIKA TIDAK ADA
        let mut if_not_exists = false;
        if self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::If => {
                    self.pos += 1; // lewati IF / JIKA
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Not {
                        self.pos += 1; // lewati NOT / TIDAK
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Exists {
                        self.pos += 1; // lewati EXISTS / ADA
                    }
                    if_not_exists = true;
                }
                Token::Ident(word) => {
                    let w = word.to_uppercase();
                    if w == "JIKA_TIDAK_ADA" || w == "IF_NOT_EXISTS" {
                        self.pos += 1;
                        if_not_exists = true;
                    }
                }
                _ => {}
            }
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => {
                let t = tbl.clone();
                self.pos += 1;
                t
            }
            _ => return Err("Diharapkan nama tabel setelah CREATE TABLE / BUAT_TABEL".to_string()),
        };

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
            return Err("Diharapkan '(' setelah nama tabel".to_string());
        }
        self.pos += 1;

        let mut columns = Vec::new();
        let mut column_defs = Vec::new();
        let mut primary_key = None;
        let mut foreign_keys = Vec::new();

        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
            // Cek jika deklarasi FOREIGN KEY tingkat tabel
            if self.tokens[self.pos] == Token::Foreign {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Key {
                    self.pos += 1;
                }
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
                    return Err("Diharapkan '(' setelah FOREIGN KEY / KUNCI_ASING".to_string());
                }
                self.pos += 1;
                let fk_col = self.parse_column_name()?;
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RightParen {
                    return Err("Diharapkan ')' setelah nama kolom FOREIGN KEY".to_string());
                }
                self.pos += 1;
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::References {
                    return Err("Diharapkan kata kunci REFERENCES / REFERENSI".to_string());
                }
                self.pos += 1;
                let ref_tbl = match self.tokens.get(self.pos) {
                    Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
                    _ => return Err("Diharapkan nama tabel rujukan pada FOREIGN KEY".to_string()),
                };
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
                    return Err("Diharapkan '(' setelah nama tabel rujukan".to_string());
                }
                self.pos += 1;
                let ref_col = self.parse_column_name()?;
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RightParen {
                    return Err("Diharapkan ')' setelah nama kolom rujukan".to_string());
                }
                self.pos += 1;

                foreign_keys.push(ForeignKeyConstraint {
                    column: fk_col,
                    foreign_table: ref_tbl,
                    foreign_column: ref_col,
                });

                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                }
                continue;
            }

            // Deklarasi PRIMARY KEY tingkat tabel: PRIMARY KEY (col)
            let is_table_pk = if self.tokens[self.pos] == Token::PrimaryKey {
                self.pos += 1;
                true
            } else if self.pos + 1 < self.tokens.len()
                && (matches!(&self.tokens[self.pos], Token::Ident(p) if p.eq_ignore_ascii_case("PRIMARY")) && (self.tokens[self.pos + 1] == Token::Key || matches!(&self.tokens[self.pos + 1], Token::Ident(k) if k.eq_ignore_ascii_case("KEY"))))
            {
                self.pos += 2;
                true
            } else if self.pos + 1 < self.tokens.len()
                && (matches!(&self.tokens[self.pos], Token::Ident(k) if k.eq_ignore_ascii_case("KUNCI")) && matches!(&self.tokens[self.pos + 1], Token::Ident(u) if u.eq_ignore_ascii_case("UTAMA")))
            {
                self.pos += 2;
                true
            } else {
                false
            };

            if is_table_pk {
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                    let pk_col = self.parse_column_name()?;
                    primary_key = Some(pk_col);
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    }
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                }
                continue;
            }

            let col_name = match self.tokens.get(self.pos) {
                Some(Token::Ident(c)) => {
                    let cn = c.clone();
                    self.pos += 1;
                    cn
                }
                Some(Token::Values) => {
                    self.pos += 1;
                    "nilai".to_string()
                }
                _ => return Err("Diharapkan nama kolom pada definisi tabel".to_string()),
            };

            let mut col_type = "TEXT".to_string();
            let mut is_serial = false;

            if self.pos < self.tokens.len() {
                if self.tokens[self.pos] == Token::Serial {
                    is_serial = true;
                    col_type = "INTEGER".to_string();
                    self.pos += 1;
                } else if let Token::Ident(t) = &self.tokens[self.pos] {
                    let t_up = t.to_uppercase();
                    if t_up == "SERIAL" || t_up == "BERURUTAN" || t_up == "AUTO_INCREMENT" {
                        is_serial = true;
                        col_type = "INTEGER".to_string();
                        self.pos += 1;
                    } else {
                        col_type = t.clone();
                        self.pos += 1;
                    }
                }
            }

            let mut not_null = false;
            let mut default_value = None;
            let mut is_unique = false;
            let mut is_col_pk = false;
            let mut references = None;

            // Loop membaca constraint kolom: PRIMARY KEY, NOT NULL, DEFAULT, UNIQUE, REFERENCES
            while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Comma && self.tokens[self.pos] != Token::RightParen {
                if self.tokens[self.pos] == Token::PrimaryKey {
                    is_col_pk = true;
                    primary_key = Some(col_name.clone());
                    self.pos += 1;
                } else if self.pos + 1 < self.tokens.len()
                    && (matches!(&self.tokens[self.pos], Token::Ident(p) if p.eq_ignore_ascii_case("PRIMARY")) && (self.tokens[self.pos + 1] == Token::Key || matches!(&self.tokens[self.pos + 1], Token::Ident(k) if k.eq_ignore_ascii_case("KEY"))))
                {
                    is_col_pk = true;
                    primary_key = Some(col_name.clone());
                    self.pos += 2;
                } else if self.pos + 1 < self.tokens.len()
                    && (matches!(&self.tokens[self.pos], Token::Ident(k) if k.eq_ignore_ascii_case("KUNCI")) && matches!(&self.tokens[self.pos + 1], Token::Ident(u) if u.eq_ignore_ascii_case("UTAMA")))
                {
                    is_col_pk = true;
                    primary_key = Some(col_name.clone());
                    self.pos += 2;
                } else if self.tokens[self.pos] == Token::Not {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::Null || matches!(&self.tokens[self.pos], Token::Ident(n) if n.eq_ignore_ascii_case("NIHIL") || n.eq_ignore_ascii_case("KOSONG") || n.eq_ignore_ascii_case("NULL"))) {
                        self.pos += 1;
                        not_null = true;
                    }
                } else if self.tokens[self.pos] == Token::UniqueConstraint || self.tokens[self.pos] == Token::Unique {
                    is_unique = true;
                    self.pos += 1;
                } else if self.tokens[self.pos] == Token::Default {
                    self.pos += 1;
                    if self.pos < self.tokens.len() {
                        let def_expr = self.parse_single_value_expr()?;
                        default_value = match def_expr {
                            SqlExpr::Literal(val) => Some(val.as_text()),
                            _ => Some("NULL".to_string()),
                        };
                    }
                } else if self.tokens[self.pos] == Token::References {
                    self.pos += 1;
                    let ref_tbl = match self.tokens.get(self.pos) {
                        Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
                        _ => return Err("Diharapkan nama tabel rujukan pada REFERENCES".to_string()),
                    };
                    if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
                        return Err("Diharapkan '(' setelah nama tabel rujukan".to_string());
                    }
                    self.pos += 1;
                    let ref_col = self.parse_column_name()?;
                    if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RightParen {
                        return Err("Diharapkan ')' setelah nama kolom rujukan".to_string());
                    }
                    self.pos += 1;

                    references = Some((ref_tbl.clone(), ref_col.clone()));
                    foreign_keys.push(ForeignKeyConstraint {
                        column: col_name.clone(),
                        foreign_table: ref_tbl,
                        foreign_column: ref_col,
                    });
                } else if let Token::Ident(w) = &self.tokens[self.pos] {
                    let w_up = w.to_uppercase();
                    if w_up == "NOT_NULL" || w_up == "TIDAK_NIHIL" {
                        not_null = true;
                        self.pos += 1;
                    } else if w_up == "UNIQUE" || w_up == "UNIK" {
                        is_unique = true;
                        self.pos += 1;
                    } else if w_up == "DEFAULT" || w_up == "BAWAAN" {
                        self.pos += 1;
                        if self.pos < self.tokens.len() {
                            let def_expr = self.parse_single_value_expr()?;
                            default_value = match def_expr {
                                SqlExpr::Literal(val) => Some(val.as_text()),
                                _ => Some("NULL".to_string()),
                            };
                        }
                    } else if w_up == "PRIMARY_KEY" || w_up == "KUNCI_UTAMA" {
                        is_col_pk = true;
                        primary_key = Some(col_name.clone());
                        self.pos += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            columns.push((col_name.clone(), col_type.clone()));
            column_defs.push(ColumnDef {
                name: col_name,
                col_type,
                not_null,
                default_value,
                is_unique,
                is_serial,
                is_primary_key: is_col_pk,
                references,
            });

            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                self.pos += 1;
            } else {
                break;
            }
        }

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
            self.pos += 1;
        }

        let mut engine = "Standard".to_string();
        let mut order_by = Vec::new();
        let mut is_columnar = false;

        while self.pos < self.tokens.len() {
            if let Token::Ident(w) = &self.tokens[self.pos] {
                let w_up = w.to_uppercase();
                if w_up == "ENGINE" || w_up == "MESIN" {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Equal {
                        self.pos += 1;
                    }
                    if let Some(Token::Ident(eng_name)) = self.tokens.get(self.pos) {
                        engine = eng_name.clone();
                        if eng_name.eq_ignore_ascii_case("MergeTree") || eng_name.eq_ignore_ascii_case("Kolumnar") || eng_name.eq_ignore_ascii_case("Columnar") {
                            is_columnar = true;
                        }
                        self.pos += 1;
                    }
                } else if w_up == "ORDER" || w_up == "URUTKAN" || w_up == "ORDER_BY" || w_up == "URUTKAN_BERDASARKAN" {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && (self.tokens[self.pos] == Token::By || matches!(&self.tokens[self.pos], Token::Ident(b) if b.eq_ignore_ascii_case("BY") || b.eq_ignore_ascii_case("BERDASARKAN"))) {
                        self.pos += 1;
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                        self.pos += 1;
                        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                            if let Token::Ident(c) = &self.tokens[self.pos] {
                                order_by.push(c.clone());
                                self.pos += 1;
                            }
                            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                            self.pos += 1;
                        }
                    } else if let Some(Token::Ident(c)) = self.tokens.get(self.pos) {
                        order_by.push(c.clone());
                        self.pos += 1;
                    }
                } else {
                    break;
                }
            } else if self.tokens[self.pos] == Token::Order {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::By {
                    self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                    while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                        if let Token::Ident(c) = &self.tokens[self.pos] {
                            order_by.push(c.clone());
                            self.pos += 1;
                        }
                        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    }
                } else if let Some(Token::Ident(c)) = self.tokens.get(self.pos) {
                    order_by.push(c.clone());
                    self.pos += 1;
                }
            } else {
                break;
            }
        }

        if is_columnar {
            Ok(SqlStatement::CreateColumnarTable {
                name,
                columns,
                engine,
                order_by,
                if_not_exists,
            })
        } else {
            Ok(SqlStatement::CreateTable {
                name,
                columns,
                column_defs,
                primary_key,
                foreign_keys,
                if_not_exists,
            })
        }
    }

    fn parse_create_view(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati VIEW / TAMPILAN

        let mut if_not_exists = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::If {
            self.pos += 1;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Not {
                self.pos += 1;
            }
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Exists {
                self.pos += 1;
            }
            if_not_exists = true;
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(v)) => { let res = v.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama tampilan (view) setelah CREATE VIEW / BUAT_TAMPILAN".to_string()),
        };

        if self.pos < self.tokens.len() {
            if let Token::Ident(w) = &self.tokens[self.pos] {
                if w.eq_ignore_ascii_case("AS") || w.eq_ignore_ascii_case("SEBAGAI") {
                    self.pos += 1;
                }
            }
        }

        let query = if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
            self.pos += 1;
            let q = self.parse_single_statement()?;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                self.pos += 1;
            }
            q
        } else {
            self.parse_single_statement()?
        };

        Ok(SqlStatement::CreateView {
            name,
            query: Box::new(query),
            if_not_exists,
        })
    }

    fn parse_drop_view(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati DROP_VIEW atau VIEW
        let mut if_exists = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::If {
            self.pos += 1;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Exists {
                self.pos += 1;
            }
            if_exists = true;
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(v)) => { let res = v.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama tampilan setelah DROP VIEW / HAPUS_TAMPILAN".to_string()),
        };
        Ok(SqlStatement::DropView { name, if_exists })
    }

    fn parse_create_index(&mut self) -> Result<SqlStatement, String> {
        let mut is_unique = false;
        if self.tokens[self.pos] == Token::Unique {
            is_unique = true;
            self.pos += 1;
        }

        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Index {
            self.pos += 1;
        }

        let mut if_not_exists = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::If {
            self.pos += 1;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Not {
                self.pos += 1;
            }
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Exists {
                self.pos += 1;
            }
            if_not_exists = true;
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(i)) => { let res = i.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama indeks setelah CREATE INDEX / BUAT_INDEKS".to_string()),
        };

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::On {
            return Err("Diharapkan kata kunci ON / PADA setelah nama indeks".to_string());
        }
        self.pos += 1; // lewati ON / PADA

        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama tabel setelah ON / PADA".to_string()),
        };

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
            return Err("Diharapkan '(' setelah nama tabel".to_string());
        }
        self.pos += 1;

        let column = self.parse_column_name()?;

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RightParen {
            return Err("Diharapkan ')' setelah nama kolom indeks".to_string());
        }
        self.pos += 1;

        Ok(SqlStatement::CreateIndex {
            name,
            table,
            column,
            is_unique,
            if_not_exists,
        })
    }

    fn parse_drop_index(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati DROP_INDEX atau INDEX
        let mut if_exists = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::If {
            self.pos += 1;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Exists {
                self.pos += 1;
            }
            if_exists = true;
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(i)) => { let res = i.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama indeks setelah DROP INDEX / HAPUS_INDEKS".to_string()),
        };
        Ok(SqlStatement::DropIndex { name, if_exists })
    }

    fn parse_alter_table(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati ALTER / UBAH
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Table {
            self.pos += 1;
        }

        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama tabel pada ALTER TABLE / UBAH_TABEL".to_string()),
        };

        if self.pos >= self.tokens.len() {
            return Err("ALTER TABLE memerlukan tindakan (ADD COLUMN, DROP COLUMN, RENAME, dll.)".to_string());
        }

        match &self.tokens[self.pos] {
            Token::Add => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Column {
                    self.pos += 1;
                }
                let column = self.parse_column_name()?;
                let col_type = match self.tokens.get(self.pos) {
                    Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
                    _ => "TEXT".to_string(),
                };
                let mut not_null = false;
                let mut default_value = None;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Default {
                    self.pos += 1;
                    let expr = self.parse_single_value_expr()?;
                    default_value = match expr {
                        SqlExpr::Literal(val) => Some(val.as_text()),
                        _ => None,
                    };
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Not {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Null {
                        self.pos += 1;
                        not_null = true;
                    }
                }
                Ok(SqlStatement::AlterTableAddColumn {
                    table,
                    column,
                    col_type,
                    not_null,
                    default_value,
                })
            }
            Token::Drop | Token::DropColumn | Token::Delete => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Column {
                    self.pos += 1;
                }
                let column = self.parse_column_name()?;
                Ok(SqlStatement::AlterTableDropColumn { table, column })
            }
            Token::Rename => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Column {
                    self.pos += 1;
                }
                let old_name = self.parse_column_name()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::To {
                    self.pos += 1;
                }
                let new_name = self.parse_column_name()?;
                Ok(SqlStatement::AlterTableRenameColumn {
                    table,
                    old_name,
                    new_name,
                })
            }
            Token::Alter => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Column {
                    self.pos += 1;
                }
                let column = self.parse_column_name()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Type {
                    self.pos += 1;
                }
                let new_type = match self.tokens.get(self.pos) {
                    Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
                    _ => "TEXT".to_string(),
                };
                Ok(SqlStatement::AlterTableChangeType {
                    table,
                    column,
                    new_type,
                })
            }
            other => Err(format!("Tindakan ALTER TABLE tidak didukung: {:?}", other)),
        }
    }

    fn parse_truncate_table(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati TRUNCATE / KOSONGKAN_TABEL
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Table {
            self.pos += 1;
        }
        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => { let t = tbl.clone(); self.pos += 1; t }
            _ => return Err("Diharapkan nama tabel setelah TRUNCATE TABLE".to_string()),
        };
        Ok(SqlStatement::TruncateTable { name })
    }

    fn parse_vacuum(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati VACUUM / BERSIHKAN_MVCC
        let table = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => { let t = tbl.clone(); self.pos += 1; Some(t) }
            _ => None,
        };
        Ok(SqlStatement::VacuumDb { table })
    }

    fn parse_show(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SHOW / TAMPILKAN
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Tables {
            self.pos += 1;
            Ok(SqlStatement::ShowTables)
        } else if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Columns {
            self.pos += 1;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::From {
                self.pos += 1;
            }
            let table = match self.tokens.get(self.pos) {
                Some(Token::Ident(tbl)) => { let t = tbl.clone(); self.pos += 1; t }
                _ => return Err("Diharapkan nama tabel setelah SHOW COLUMNS FROM".to_string()),
            };
            Ok(SqlStatement::ShowColumns { table })
        } else {
            Err("Diharapkan TABLES atau COLUMNS setelah SHOW".to_string())
        }
    }

    fn parse_explain(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati EXPLAIN / JELASKAN
        let mut analyze = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Analyze {
            analyze = true;
            self.pos += 1;
        }
        let stmt = self.parse_single_statement()?;
        Ok(SqlStatement::ExplainStatement {
            stmt: Box::new(stmt),
            analyze,
        })
    }

    fn parse_savepoint(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SAVEPOINT / TITIK_SIMPAN
        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(n)) => { let res = n.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama titik simpan setelah SAVEPOINT / TITIK_SIMPAN".to_string()),
        };
        Ok(SqlStatement::Savepoint { name })
    }

    fn parse_release_savepoint(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati RELEASE / LEPAS
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Savepoint {
            self.pos += 1;
        }
        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(n)) => { let res = n.clone(); self.pos += 1; res }
            _ => return Err("Diharapkan nama titik simpan setelah RELEASE SAVEPOINT / LEPAS_TITIK_SIMPAN".to_string()),
        };
        Ok(SqlStatement::ReleaseSavepoint { name })
    }

    fn parse_with(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati WITH / DENGAN
        let mut ctes = Vec::new();

        loop {
            let name = match self.tokens.get(self.pos) {
                Some(Token::Ident(n)) => { let res = n.clone(); self.pos += 1; res }
                _ => return Err("Diharapkan nama CTE setelah WITH / DENGAN".to_string()),
            };

            if self.pos < self.tokens.len() {
                if let Token::Ident(w) = &self.tokens[self.pos] {
                    if w.eq_ignore_ascii_case("AS") || w.eq_ignore_ascii_case("SEBAGAI") {
                        self.pos += 1;
                    }
                }
            }

            if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
                return Err("Diharapkan '(' sebelum kueri CTE".to_string());
            }
            self.pos += 1;

            let query = self.parse_single_statement()?;

            if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RightParen {
                return Err("Diharapkan ')' setelah kueri CTE".to_string());
            }
            self.pos += 1;

            ctes.push(CteDefinition { name, query: Box::new(query) });

            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                self.pos += 1;
            } else {
                break;
            }
        }

        let body = self.parse_single_statement()?;
        Ok(SqlStatement::With { ctes, body: Box::new(body) })
    }

    fn parse_window_spec(&mut self, func_name: String, args: Vec<SqlExpr>) -> Result<SqlExpr, String> {
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LeftParen {
            return Err("Diharapkan '(' setelah klausa OVER / LEWAT".to_string());
        }
        self.pos += 1; // lewati '('

        let mut partition_by = None;
        let mut order_by = None;

        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
            match self.tokens[self.pos] {
                Token::Partition => {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::By {
                        self.pos += 1;
                    }
                    partition_by = Some(self.parse_column_name()?);
                }
                Token::Order => {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::By {
                        self.pos += 1;
                    }
                    let col = self.parse_column_name()?;
                    let mut is_desc = false;
                    if self.pos < self.tokens.len() {
                        if self.tokens[self.pos] == Token::Desc {
                            is_desc = true;
                            self.pos += 1;
                        } else if self.tokens[self.pos] == Token::Asc {
                            self.pos += 1;
                        }
                    }
                    let mut secondary = Vec::new();
                    while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                        self.pos += 1;
                        if self.pos < self.tokens.len() && matches!(self.tokens.get(self.pos), Some(Token::Ident(_)) | Some(Token::Values) | Some(Token::Key)) {
                            let c2 = self.parse_column_name()?;
                            let mut d2 = false;
                            if self.pos < self.tokens.len() {
                                if self.tokens[self.pos] == Token::Desc {
                                    d2 = true;
                                    self.pos += 1;
                                } else if self.tokens[self.pos] == Token::Asc {
                                    self.pos += 1;
                                }
                            }
                            secondary.push((c2, d2));
                        } else {
                            break;
                        }
                    }
                    order_by = Some(OrderByClause { column: col, is_desc, secondary });
                }
                _ => break,
            }
        }

        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RightParen {
            return Err("Diharapkan ')' penutup klausa OVER / LEWAT".to_string());
        }
        self.pos += 1; // lewati ')'

        Ok(SqlExpr::WindowFunction {
            func: func_name,
            args,
            partition_by,
            order_by,
        })
    }

    fn parse_drop_table(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati DROP / BUANG
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Table {
            self.pos += 1;
        }

        let mut if_exists = false;
        if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::If {
            self.pos += 1;
            if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Exists {
                self.pos += 1;
            }
            if_exists = true;
        }

        let name = match self.tokens.get(self.pos) {
            Some(Token::Ident(tbl)) => {
                let t = tbl.clone();
                self.pos += 1;
                t
            }
            _ => return Err("Diharapkan nama tabel setelah DROP TABLE / BUANG_TABEL".to_string()),
        };

        Ok(SqlStatement::DropTable { name, if_exists })
    }

    fn parse_expression(&mut self) -> Result<SqlExpr, String> {
        let mut left = self.parse_logical_and()?;

        while self.pos < self.tokens.len() {
            if self.tokens[self.pos] == Token::Or {
                self.pos += 1;
                let right = self.parse_logical_and()?;
                left = SqlExpr::Binary {
                    left: Box::new(left),
                    op: BinaryOp::Or,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<SqlExpr, String> {
        let mut left = self.parse_single_comparison()?;

        while self.pos < self.tokens.len() {
            if self.tokens[self.pos] == Token::And {
                self.pos += 1;
                let right = self.parse_single_comparison()?;
                left = SqlExpr::Binary {
                    left: Box::new(left),
                    op: BinaryOp::And,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_single_comparison(&mut self) -> Result<SqlExpr, String> {
        let left_val = self.parse_additive()?;

        if self.pos < self.tokens.len() {
            // Cek IS NULL / IS NOT NULL / ADALAH NIHIL / ADALAH BUKAN NIHIL
            if self.tokens[self.pos] == Token::Is {
                self.pos += 1;
                let mut negated = false;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Not {
                    negated = true;
                    self.pos += 1;
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Null {
                    self.pos += 1;
                    return Ok(SqlExpr::IsNull {
                        expr: Box::new(left_val),
                        negated,
                    });
                } else {
                    return Err("Diharapkan kata kunci NULL / NIHIL setelah IS / ADALAH".to_string());
                }
            }

            // Cek BUKAN NIHIL / TIDAK NIHIL
            if self.tokens[self.pos] == Token::Not && self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Null {
                self.pos += 2;
                return Ok(SqlExpr::IsNull {
                    expr: Box::new(left_val),
                    negated: true,
                });
            }

            // Cek LIKE / SEPERTI / MIRIP
            if self.tokens[self.pos] == Token::Like {
                self.pos += 1;
                let right_val = self.parse_additive()?;
                return Ok(SqlExpr::Binary {
                    left: Box::new(left_val),
                    op: BinaryOp::Like,
                    right: Box::new(right_val),
                });
            }

            // Cek NOT LIKE / BUKAN SEPERTI
            if self.tokens[self.pos] == Token::Not && self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Like {
                self.pos += 2;
                let right_val = self.parse_additive()?;
                return Ok(SqlExpr::Binary {
                    left: Box::new(left_val),
                    op: BinaryOp::NotLike,
                    right: Box::new(right_val),
                });
            }

            // Cek BETWEEN / ANTARA
            if self.tokens[self.pos] == Token::Between {
                self.pos += 1;
                let low = self.parse_additive()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::And {
                    self.pos += 1;
                } else {
                    return Err("Diharapkan kata kunci DAN / AND pada klausa BETWEEN / ANTARA".to_string());
                }
                let high = self.parse_additive()?;
                return Ok(SqlExpr::Between {
                    expr: Box::new(left_val),
                    low: Box::new(low),
                    high: Box::new(high),
                    negated: false,
                });
            }

            // Cek NOT BETWEEN / BUKAN ANTARA
            if self.tokens[self.pos] == Token::Not && self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Between {
                self.pos += 2;
                let low = self.parse_additive()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::And {
                    self.pos += 1;
                } else {
                    return Err("Diharapkan kata kunci DAN / AND pada klausa BETWEEN / ANTARA".to_string());
                }
                let high = self.parse_additive()?;
                return Ok(SqlExpr::Between {
                    expr: Box::new(left_val),
                    low: Box::new(low),
                    high: Box::new(high),
                    negated: true,
                });
            }

            // Cek IN / DI_DALAM
            if self.tokens[self.pos] == Token::In {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                } else {
                    return Err("Diharapkan '(' setelah kata kunci IN / DI_DALAM".to_string());
                }

                // Cek Subquery di dalam IN: IN (SELECT ...)
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Select {
                    let subquery = self.parse_statement()?;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    } else {
                        return Err("Diharapkan ')' penutup setelah subkueri IN".to_string());
                    }
                    return Ok(SqlExpr::InSubquery {
                        expr: Box::new(left_val),
                        subquery: Box::new(subquery),
                        negated: false,
                    });
                }

                let mut list = Vec::new();
                while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                    list.push(self.parse_additive()?);
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                    self.pos += 1;
                }
                return Ok(SqlExpr::InList {
                    expr: Box::new(left_val),
                    list,
                    negated: false,
                });
            }

            // Cek NOT IN / BUKAN DI_DALAM
            if self.tokens[self.pos] == Token::Not && self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::In {
                self.pos += 2;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                } else {
                    return Err("Diharapkan '(' setelah NOT IN / BUKAN DI_DALAM".to_string());
                }

                // Cek Subquery di dalam NOT IN: NOT IN (SELECT ...)
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Select {
                    let subquery = self.parse_statement()?;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    } else {
                        return Err("Diharapkan ')' penutup setelah subkueri NOT IN".to_string());
                    }
                    return Ok(SqlExpr::InSubquery {
                        expr: Box::new(left_val),
                        subquery: Box::new(subquery),
                        negated: true,
                    });
                }

                let mut list = Vec::new();
                while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                    list.push(self.parse_additive()?);
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                    self.pos += 1;
                }
                return Ok(SqlExpr::InList {
                    expr: Box::new(left_val),
                    list,
                    negated: true,
                });
            }

            // Relasional standar
            let op = match &self.tokens[self.pos] {
                Token::Equal => Some(BinaryOp::Eq),
                Token::NotEqual => Some(BinaryOp::Neq),
                Token::GreaterThan => Some(BinaryOp::Gt),
                Token::LessThan => Some(BinaryOp::Lt),
                Token::GreaterEqual => Some(BinaryOp::Gte),
                Token::LessEqual => Some(BinaryOp::Lte),
                _ => None,
            };

            if let Some(op_code) = op {
                self.pos += 1;
                let right_val = self.parse_additive()?;
                return Ok(SqlExpr::Binary {
                    left: Box::new(left_val),
                    op: op_code,
                    right: Box::new(right_val),
                });
            }
        }

        Ok(left_val)
    }

    pub fn parse_additive(&mut self) -> Result<SqlExpr, String> {
        let mut left = self.parse_multiplicative()?;
        while self.pos < self.tokens.len() {
            let op = match &self.tokens[self.pos] {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.pos += 1;
            let right = self.parse_multiplicative()?;
            left = SqlExpr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    pub fn parse_multiplicative(&mut self) -> Result<SqlExpr, String> {
        let mut left = self.parse_primary()?;
        while self.pos < self.tokens.len() {
            let op = match &self.tokens[self.pos] {
                Token::Asterisk => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => break,
            };
            self.pos += 1;
            let right = self.parse_primary()?;
            left = SqlExpr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    pub fn parse_primary(&mut self) -> Result<SqlExpr, String> {
        if self.pos >= self.tokens.len() {
            return Err("Ekspresi tidak lengkap".to_string());
        }

        match &self.tokens[self.pos] {
            Token::LeftParen => {
                self.pos += 1;
                // Cek jika subkueri di dalam kurung: (SELECT ...)
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Select {
                    let subquery = self.parse_statement()?;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    } else {
                        return Err("Diharapkan tanda ')' penutup setelah subkueri".to_string());
                    }
                    return Ok(SqlExpr::Subquery(Box::new(subquery)));
                }

                let expr = self.parse_expression()?;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                    self.pos += 1;
                } else {
                    return Err("Diharapkan tanda ')' penutup".to_string());
                }
                Ok(expr)
            }
            Token::Case => {
                self.pos += 1; // lewati CASE / KASUS
                let mut conditions = Vec::new();
                while self.pos < self.tokens.len() && self.tokens[self.pos] == Token::When {
                    self.pos += 1; // lewati WHEN / KETIKA
                    let cond = self.parse_expression()?;
                    if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Then {
                        return Err("Diharapkan MAKA / THEN setelah kondisi KETIKA / WHEN".to_string());
                    }
                    self.pos += 1; // lewati THEN / MAKA
                    let res = self.parse_expression()?;
                    conditions.push((cond, res));
                }
                if conditions.is_empty() {
                    return Err("Klausul KASUS / CASE harus memiliki minimal satu cabang KETIKA / WHEN".to_string());
                }
                let mut else_branch = None;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Else {
                    self.pos += 1; // lewati ELSE / LAINNYA
                    else_branch = Some(Box::new(self.parse_expression()?));
                }
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::End {
                    return Err("Diharapkan kata kunci SELESAI / END untuk menutup KASUS / CASE".to_string());
                }
                self.pos += 1; // lewati END / SELESAI
                Ok(SqlExpr::Case { conditions, else_branch })
            }
            Token::Count | Token::Sum | Token::Avg | Token::Min | Token::Max => {
                let func_name = match self.tokens[self.pos] {
                    Token::Count => "COUNT",
                    Token::Sum => "SUM",
                    Token::Avg => "AVG",
                    Token::Min => "MIN",
                    Token::Max => "MAX",
                    _ => "",
                }.to_string();
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                    let mut args = Vec::new();
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Asterisk {
                        self.pos += 1;
                        args.push(SqlExpr::Wildcard);
                    } else if self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                        args.push(self.parse_expression()?);
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Over {
                        self.pos += 1;
                        return self.parse_window_spec(func_name, args);
                    }
                    Ok(SqlExpr::FunctionCall { name: func_name, args })
                } else {
                    Err("Diharapkan '(' setelah fungsi agregat".to_string())
                }
            }
            Token::Ident(id) if (id.eq_ignore_ascii_case("CAST") || id.eq_ignore_ascii_case("KONVERSI") || id.eq_ignore_ascii_case("UBAH_TIPE")) && self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::LeftParen => {
                self.pos += 2; // lewati nama fungsi dan '('
                let expr = self.parse_expression()?;
                if self.pos < self.tokens.len() {
                    if let Token::Ident(w) = &self.tokens[self.pos] {
                        if w.eq_ignore_ascii_case("AS") || w.eq_ignore_ascii_case("SEBAGAI") || w.eq_ignore_ascii_case("KE") {
                            self.pos += 1;
                        }
                    }
                }
                let target_type = match self.tokens.get(self.pos) {
                    Some(Token::Ident(t)) => { let res = t.clone(); self.pos += 1; res }
                    Some(Token::Type) => { self.pos += 1; "TYPE".to_string() }
                    _ => return Err("Diharapkan tipe target pada CAST / KONVERSI".to_string()),
                };
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                    self.pos += 1;
                }
                Ok(SqlExpr::FunctionCall {
                    name: "CAST".to_string(),
                    args: vec![expr, SqlExpr::Literal(SqlValue::Text(target_type))],
                })
            }
            Token::Ident(id) if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::LeftParen => {
                let func_name = id.clone();
                self.pos += 2; // lewati nama fungsi dan '('
                let mut args = Vec::new();
                while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::RightParen {
                    if self.tokens[self.pos] == Token::Asterisk {
                        self.pos += 1;
                        args.push(SqlExpr::Wildcard);
                    } else {
                        args.push(self.parse_expression()?);
                    }
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                    self.pos += 1;
                }
                let upper_func = func_name.to_uppercase();
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Over {
                    self.pos += 1;
                    return self.parse_window_spec(upper_func, args);
                }
                Ok(SqlExpr::FunctionCall { name: upper_func, args })
            }
            Token::Ident(id) if id.eq_ignore_ascii_case("CURRENT_DATE") || id.eq_ignore_ascii_case("HARI_INI") || id.eq_ignore_ascii_case("TANGGAL_SEKARANG") => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    }
                }
                Ok(SqlExpr::FunctionCall { name: "CURRENT_DATE".to_string(), args: Vec::new() })
            }
            Token::Ident(id) if id.eq_ignore_ascii_case("NOW") || id.eq_ignore_ascii_case("SEKARANG") => {
                self.pos += 1;
                if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::LeftParen {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::RightParen {
                        self.pos += 1;
                    }
                }
                Ok(SqlExpr::FunctionCall { name: "NOW".to_string(), args: Vec::new() })
            }
            Token::StringLit(s) => {
                let val = SqlExpr::Literal(SqlValue::Text(s.clone()));
                self.pos += 1;
                Ok(val)
            }
            Token::NumberLit(n) => {
                let val = if n.fract() == 0.0 {
                    SqlExpr::Literal(SqlValue::Integer(*n as i64))
                } else {
                    SqlExpr::Literal(SqlValue::Float(*n))
                };
                self.pos += 1;
                Ok(val)
            }
            Token::BooleanLit(b) => {
                let val = SqlExpr::Literal(SqlValue::Boolean(*b));
                self.pos += 1;
                Ok(val)
            }
            Token::Null => {
                self.pos += 1;
                Ok(SqlExpr::Literal(SqlValue::Null))
            }
            Token::Ident(_) | Token::Values => {
                let col_name = self.parse_column_name()?;
                Ok(SqlExpr::Column(col_name))
            }
            Token::Asterisk => {
                self.pos += 1;
                Ok(SqlExpr::Wildcard)
            }
            other => Err(format!("Token ekspresi tidak didukung: {:?}", other)),
        }
    }

    fn parse_single_value_expr(&mut self) -> Result<SqlExpr, String> {
        self.parse_primary()
    }

    fn parse_cache_token_as_string(&mut self) -> Result<String, String> {
        if self.pos >= self.tokens.len() {
            return Err("Argumen perintah cache tidak lengkap".to_string());
        }
        if self.tokens[self.pos] == Token::Minus {
            self.pos += 1;
            let next_str = self.parse_cache_token_as_string()?;
            return Ok(format!("-{}", next_str));
        }
        let s = match &self.tokens[self.pos] {
            Token::Ident(s) => s.clone(),
            Token::StringLit(s) => s.clone(),
            Token::NumberLit(n) => {
                if n.fract() == 0.0 {
                    (*n as i64).to_string()
                } else {
                    n.to_string()
                }
            }
            Token::BooleanLit(b) => b.to_string(),
            Token::Values => "nilai".to_string(),
            other => format!("{:?}", other),
        };
        self.pos += 1;
        Ok(s)
    }

    fn parse_cache_set(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SET / ATUR_CACHE
        let key = self.parse_cache_token_as_string()?;
        let value = self.parse_cache_token_as_string()?;
        let mut ttl_secs = None;

        if self.pos < self.tokens.len() {
            if let Token::Ident(w) = &self.tokens[self.pos] {
                if w.eq_ignore_ascii_case("EX") || w.eq_ignore_ascii_case("TTL") || w.eq_ignore_ascii_case("DETIK") {
                    self.pos += 1;
                    let ttl_str = self.parse_cache_token_as_string()?;
                    ttl_secs = ttl_str.parse::<u64>().ok();
                }
            }
        }

        Ok(SqlStatement::CacheSet { key, value, ttl_secs })
    }

    fn parse_cache_get(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati GET / AMBIL_CACHE
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheGet { key })
    }

    fn parse_cache_del(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati DEL / HAPUS_CACHE
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheDel { key })
    }

    fn parse_cache_expire(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati EXPIRE / KADALUWARSA
        let key = self.parse_cache_token_as_string()?;
        let ttl_str = self.parse_cache_token_as_string()?;
        let ttl_secs = ttl_str.parse::<u64>().map_err(|e| format!("Nilai TTL detik tidak valid: {}", e))?;
        Ok(SqlStatement::CacheExpire { key, ttl_secs })
    }

    fn parse_cache_ttl(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati TTL / SISA_WAKTU
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheTtl { key })
    }

    fn parse_cache_incr(&mut self) -> Result<SqlStatement, String> {
        let is_incrby = match &self.tokens[self.pos] {
            Token::Ident(cmd) => cmd.eq_ignore_ascii_case("INCRBY") || cmd.eq_ignore_ascii_case("TAMBAH_NILAI"),
            _ => false,
        };
        self.pos += 1;
        let key = self.parse_cache_token_as_string()?;
        let by = if is_incrby {
            let by_str = self.parse_cache_token_as_string()?;
            by_str.parse::<i64>().unwrap_or(1)
        } else {
            1
        };
        Ok(SqlStatement::CacheIncr { key, by })
    }

    fn parse_cache_hset(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati HSET / ATUR_HASH
        let key = self.parse_cache_token_as_string()?;
        let field = self.parse_cache_token_as_string()?;
        let value = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheHSet { key, field, value })
    }

    fn parse_cache_hget(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati HGET / AMBIL_HASH
        let key = self.parse_cache_token_as_string()?;
        let field = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheHGet { key, field })
    }

    fn parse_cache_hgetall(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati HGETALL / AMBIL_SEMUA_HASH
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheHGetAll { key })
    }

    fn parse_cache_lpush(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati LPUSH / DORONG_KIRI
        let key = self.parse_cache_token_as_string()?;
        let mut values = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Semicolon {
            values.push(self.parse_cache_token_as_string()?);
        }
        Ok(SqlStatement::CacheLPush { key, values })
    }

    fn parse_cache_rpush(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati RPUSH / DORONG_KANAN
        let key = self.parse_cache_token_as_string()?;
        let mut values = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Semicolon {
            values.push(self.parse_cache_token_as_string()?);
        }
        Ok(SqlStatement::CacheRPush { key, values })
    }

    fn parse_cache_lpop(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati LPOP / TARIK_KIRI
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheLPop { key })
    }

    fn parse_cache_rpop(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati RPOP / TARIK_KANAN
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheRPop { key })
    }

    fn parse_cache_lrange(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati LRANGE / RENTANG_LIST
        let key = self.parse_cache_token_as_string()?;
        let start_str = self.parse_cache_token_as_string()?;
        let stop_str = self.parse_cache_token_as_string()?;
        let start = start_str.parse::<i64>().unwrap_or(0);
        let stop = stop_str.parse::<i64>().unwrap_or(-1);
        Ok(SqlStatement::CacheLRange { key, start, stop })
    }

    fn parse_cache_sadd(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SADD / TAMBAH_SET
        let key = self.parse_cache_token_as_string()?;
        let mut members = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Semicolon {
            members.push(self.parse_cache_token_as_string()?);
        }
        Ok(SqlStatement::CacheSAdd { key, members })
    }

    fn parse_cache_smembers(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SMEMBERS / ANGGOTA_SET
        let key = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheSMembers { key })
    }

    fn parse_cache_srem(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati SREM / HAPUS_SET
        let key = self.parse_cache_token_as_string()?;
        let mut members = Vec::new();
        while self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Semicolon {
            members.push(self.parse_cache_token_as_string()?);
        }
        Ok(SqlStatement::CacheSRem { key, members })
    }

    fn parse_cache_zadd(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati ZADD / TAMBAH_ZSET
        let key = self.parse_cache_token_as_string()?;
        let score_str = self.parse_cache_token_as_string()?;
        let score = score_str.parse::<f64>().map_err(|e| format!("Skor ZADD tidak valid: {}", e))?;
        let member = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CacheZAdd { key, score, member })
    }

    fn parse_cache_zrange(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati ZRANGE / RENTANG_ZSET
        let key = self.parse_cache_token_as_string()?;
        let start_str = self.parse_cache_token_as_string()?;
        let stop_str = self.parse_cache_token_as_string()?;
        let start = start_str.parse::<i64>().unwrap_or(0);
        let stop = stop_str.parse::<i64>().unwrap_or(-1);
        let mut with_scores = false;
        if self.pos < self.tokens.len() {
            match &self.tokens[self.pos] {
                Token::Ident(w) if w.eq_ignore_ascii_case("WITHSCORES") || w.eq_ignore_ascii_case("DENGAN_SKOR") || w.eq_ignore_ascii_case("WITH_SCORES") || w.eq_ignore_ascii_case("DENGAN_SKOR") => {
                    with_scores = true;
                    self.pos += 1;
                }
                Token::With => {
                    self.pos += 1;
                    if self.pos < self.tokens.len() && matches!(&self.tokens[self.pos], Token::Ident(s) if s.eq_ignore_ascii_case("SCORES") || s.eq_ignore_ascii_case("SKOR")) {
                        self.pos += 1;
                    }
                    with_scores = true;
                }
                _ => {}
            }
        }
        Ok(SqlStatement::CacheZRange { key, start, stop, with_scores })
    }

    fn parse_cache_publish(&mut self) -> Result<SqlStatement, String> {
        self.pos += 1; // lewati PUBLISH / SIARKAN
        let channel = self.parse_cache_token_as_string()?;
        let message = self.parse_cache_token_as_string()?;
        Ok(SqlStatement::CachePublish { channel, message })
    }
}
