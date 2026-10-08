//! Sovereign Pure CLI Argument Parser (Engine Argumen Baris Perintah Murni)
//! 100% Bebas Ketergantungan Eksternal (Zero External Crates: Tanpa clap, structopt, argh).
//! Mendukung Subcommand, Flag Singkat (-v), Flag Panjang (--version), Opsi Nilai (--port 8080 / --port=8080),
//! Argumen Posisi, Bantuan Otomatis (Help Generator), dan Lokalisasi Dwi-Bahasa (Indonesia/Inggris).

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ArgType {
    Flag,          // Boolean toggle, e.g., --verbose, -v
    OptionValue,   // Key-value, e.g., --output <file>, -o <file>
    Positional,    // Positional argument, e.g., <script.wya>
}

#[derive(Debug, Clone)]
pub struct ArgDef {
    pub name: String,
    pub short: Option<char>,
    pub long: Option<String>,
    pub help_id: String,
    pub help_en: String,
    pub arg_type: ArgType,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CommandDef {
    pub name: String,
    pub description_id: String,
    pub description_en: String,
    pub args: Vec<ArgDef>,
    pub subcommands: HashMap<String, CommandDef>,
}

impl CommandDef {
    pub fn new(name: &str, desc_id: &str, desc_en: &str) -> Self {
        Self {
            name: name.to_string(),
            description_id: desc_id.to_string(),
            description_en: desc_en.to_string(),
            args: Vec::new(),
            subcommands: HashMap::new(),
        }
    }

    pub fn add_flag(mut self, name: &str, short: Option<char>, long: Option<&str>, desc_id: &str, desc_en: &str) -> Self {
        self.args.push(ArgDef {
            name: name.to_string(),
            short,
            long: long.map(|s| s.to_string()),
            help_id: desc_id.to_string(),
            help_en: desc_en.to_string(),
            arg_type: ArgType::Flag,
            required: false,
            default_value: None,
        });
        self
    }

    pub fn add_option(mut self, name: &str, short: Option<char>, long: Option<&str>, desc_id: &str, desc_en: &str, required: bool, default: Option<&str>) -> Self {
        self.args.push(ArgDef {
            name: name.to_string(),
            short,
            long: long.map(|s| s.to_string()),
            help_id: desc_id.to_string(),
            help_en: desc_en.to_string(),
            arg_type: ArgType::OptionValue,
            required,
            default_value: default.map(|s| s.to_string()),
        });
        self
    }

    pub fn add_positional(mut self, name: &str, desc_id: &str, desc_en: &str, required: bool) -> Self {
        self.args.push(ArgDef {
            name: name.to_string(),
            short: None,
            long: None,
            help_id: desc_id.to_string(),
            help_en: desc_en.to_string(),
            arg_type: ArgType::Positional,
            required,
            default_value: None,
        });
        self
    }

    pub fn add_subcommand(mut self, sub: CommandDef) -> Self {
        self.subcommands.insert(sub.name.clone(), sub);
        self
    }

    pub fn format_help(&self, is_id: bool) -> String {
        let mut out = String::new();
        let desc = if is_id { &self.description_id } else { &self.description_en };
        out.push_str(&format!("{}: {}\n\n", self.name, desc));

        if is_id {
            out.push_str("PENGGUNAAN:\n");
            out.push_str(&format!("    widya {} [OPSI] [ARGUMEN]\n\n", self.name));
        } else {
            out.push_str("USAGE:\n");
            out.push_str(&format!("    widya {} [OPTIONS] [ARGUMENTS]\n\n", self.name));
        }

        if !self.subcommands.is_empty() {
            out.push_str(if is_id { "SUBPERINTAH TERSEDIA:\n" } else { "AVAILABLE SUBCOMMANDS:\n" });
            let mut subs: Vec<&CommandDef> = self.subcommands.values().collect();
            subs.sort_by(|a, b| a.name.cmp(&b.name));
            for s in subs {
                let s_desc = if is_id { &s.description_id } else { &s.description_en };
                out.push_str(&format!("    {:<18} {}\n", s.name, s_desc));
            }
            out.push('\n');
        }

        if !self.args.is_empty() {
            out.push_str(if is_id { "OPSI & ARGUMEN:\n" } else { "OPTIONS & ARGUMENTS:\n" });
            for a in &self.args {
                let opt_str = match (&a.short, &a.long) {
                    (Some(s), Some(l)) => format!("-{}, --{}", s, l),
                    (None, Some(l)) => format!("    --{}", l),
                    (Some(s), None) => format!("-{}", s),
                    (None, None) => format!("    <{}>", a.name),
                };
                let a_help = if is_id { &a.help_id } else { &a.help_en };
                out.push_str(&format!("    {:<24} {}\n", opt_str, a_help));
            }
        }

        out
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedArgs {
    pub subcommand: Option<String>,
    pub flags: HashMap<String, bool>,
    pub options: HashMap<String, String>,
    pub positionals: Vec<String>,
    pub sub_parsed: Option<Box<ParsedArgs>>,
}

impl ParsedArgs {
    pub fn get_flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    pub fn get_option(&self, name: &str) -> Option<&str> {
        self.options.get(name).map(|s| s.as_str())
    }

    pub fn get_positional(&self, index: usize) -> Option<&str> {
        self.positionals.get(index).map(|s| s.as_str())
    }
}

pub struct PureArgParser {
    pub root: CommandDef,
}

impl PureArgParser {
    pub fn new(root: CommandDef) -> Self {
        Self { root }
    }

    pub fn parse(&self, raw_args: &[String]) -> Result<ParsedArgs, String> {
        self.parse_command(&self.root, raw_args)
    }

    fn parse_command(&self, cmd: &CommandDef, raw_args: &[String]) -> Result<ParsedArgs, String> {
        let mut parsed = ParsedArgs::default();

        // Terapkan nilai default terlebih dahulu
        for arg in &cmd.args {
            if let Some(ref def) = arg.default_value {
                parsed.options.insert(arg.name.clone(), def.clone());
            }
        }

        let mut i = 0;
        let mut pos_idx = 0;

        while i < raw_args.len() {
            let item = &raw_args[i];

            // Cek apakah item ini adalah subcommand terdaftar
            if pos_idx == 0 && !item.starts_with('-') && cmd.subcommands.contains_key(item) {
                let sub_cmd = &cmd.subcommands[item];
                parsed.subcommand = Some(item.clone());
                let sub_res = self.parse_command(sub_cmd, &raw_args[i + 1..])?;
                parsed.sub_parsed = Some(Box::new(sub_res));
                return Ok(parsed);
            }

            // Flag atau Opsi Panjang (--option / --flag)
            if item.starts_with("--") {
                let body = &item[2..];
                if let Some(eq_pos) = body.find('=') {
                    let key = &body[..eq_pos];
                    let val = &body[eq_pos + 1..];
                    if let Some(arg_def) = cmd.args.iter().find(|a| a.long.as_deref() == Some(key)) {
                        parsed.options.insert(arg_def.name.clone(), val.to_string());
                    } else {
                        return Err(format!("Opsi tidak dikenal: '--{}'", key));
                    }
                } else if let Some(arg_def) = cmd.args.iter().find(|a| a.long.as_deref() == Some(body)) {
                    match arg_def.arg_type {
                        ArgType::Flag => {
                            parsed.flags.insert(arg_def.name.clone(), true);
                        }
                        ArgType::OptionValue => {
                            if i + 1 >= raw_args.len() {
                                return Err(format!("Opsi '--{}' memerlukan nilai argumen", body));
                            }
                            i += 1;
                            parsed.options.insert(arg_def.name.clone(), raw_args[i].clone());
                        }
                        ArgType::Positional => unreachable!(),
                    }
                } else {
                    return Err(format!("Flag/Opsi tidak dikenal: '--{}'", body));
                }
            } else if item.starts_with('-') && item.len() > 1 {
                // Short flag / option
                let short_char = item.chars().nth(1).unwrap();
                if let Some(arg_def) = cmd.args.iter().find(|a| a.short == Some(short_char)) {
                    match arg_def.arg_type {
                        ArgType::Flag => {
                            parsed.flags.insert(arg_def.name.clone(), true);
                        }
                        ArgType::OptionValue => {
                            if item.len() > 2 {
                                // Nilai menempel, misal -p8080
                                let val = &item[2..];
                                parsed.options.insert(arg_def.name.clone(), val.to_string());
                            } else {
                                if i + 1 >= raw_args.len() {
                                    return Err(format!("Opsi '-{}' memerlukan nilai argumen", short_char));
                                }
                                i += 1;
                                parsed.options.insert(arg_def.name.clone(), raw_args[i].clone());
                            }
                        }
                        ArgType::Positional => unreachable!(),
                    }
                } else {
                    return Err(format!("Flag singkat tidak dikenal: '-{}'", short_char));
                }
            } else {
                // Positional argument
                parsed.positionals.push(item.clone());
                pos_idx += 1;
            }

            i += 1;
        }

        // Validasi argumen yang wajib diisi (required)
        for arg in &cmd.args {
            if arg.required {
                match arg.arg_type {
                    ArgType::OptionValue => {
                        if !parsed.options.contains_key(&arg.name) {
                            return Err(format!("Argumen wajib belum disediakan: '--{}'", arg.long.as_deref().unwrap_or(&arg.name)));
                        }
                    }
                    ArgType::Positional => {
                        if parsed.positionals.is_empty() {
                            return Err(format!("Argumen posisi wajib belum disediakan: <{}>", arg.name));
                        }
                    }
                    ArgType::Flag => {}
                }
            }
        }

        Ok(parsed)
    }
}

// ============================================================================
// UNIT TESTS UNTUK PURE CLI PARSER
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_cli_flags_and_options() {
        let root = CommandDef::new("widya", "Bahasa Pemrograman Berdaulat", "Sovereign Programming Language")
            .add_flag("verbose", Some('v'), Some("verbose"), "Tampilkan log terperinci", "Show verbose logs")
            .add_option("output", Some('o'), Some("output"), "Tentukan path berkas luaran", "Specify output file", false, Some("a.out"))
            .add_positional("berkas", "Berkas skrip Widya yang akan dieksekusi", "Widya script file to execute", false);

        let parser = PureArgParser::new(root);

        let args = vec!["-v".to_string(), "--output=binari_resmi".to_string(), "utama.wya".to_string()];
        let parsed = parser.parse(&args).expect("Gagal mem-parse CLI");

        assert!(parsed.get_flag("verbose"));
        assert_eq!(parsed.get_option("output"), Some("binari_resmi"));
        assert_eq!(parsed.get_positional(0), Some("utama.wya"));
    }

    #[test]
    fn test_pure_cli_subcommands() {
        let wpm = CommandDef::new("wpm", "Widya Package Manager", "Widya Package Manager")
            .add_flag("audit", None, Some("audit"), "Audit integritas dependensi", "Audit dependency integrity")
            .add_positional("package", "Nama paket", "Package name", false);

        let root = CommandDef::new("widya", "Root CLI", "Root CLI")
            .add_subcommand(wpm);

        let parser = PureArgParser::new(root);

        let args = vec!["wpm".to_string(), "--audit".to_string(), "widya-base".to_string()];
        let parsed = parser.parse(&args).expect("Gagal mem-parse subcommand");

        assert_eq!(parsed.subcommand.as_deref(), Some("wpm"));
        let sub = parsed.sub_parsed.expect("Subcommand parsed harus ada");
        assert!(sub.get_flag("audit"));
        assert_eq!(sub.get_positional(0), Some("widya-base"));
    }

    #[test]
    fn test_pure_cli_bilingual_help() {
        let root = CommandDef::new("widya", "Sistem Runtime Kedaulatan Digital", "Digital Sovereignty Runtime")
            .add_flag("versi", Some('V'), Some("versi"), "Cetak versi Widya-Lang", "Print Widya-Lang version");

        let help_id = root.format_help(true);
        assert!(help_id.contains("PENGGUNAAN:"));
        assert!(help_id.contains("Cetak versi"));

        let help_en = root.format_help(false);
        assert!(help_en.contains("USAGE:"));
        assert!(help_en.contains("Print Widya-Lang"));
    }
}
