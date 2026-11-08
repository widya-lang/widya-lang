use crate::ast::*;
use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::stdlib::register_stdlib;
use crate::value::{Value, WidyaFunction, WidyaInstance, WidyaStruct};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    imported_modules: HashSet<String>,
    pub source_dir_stack: Vec<std::path::PathBuf>,
    pub defer_stack: Vec<Vec<Value>>,
    pub active_panic: Option<String>,
    pub trait_registry: HashMap<String, Vec<String>>,
    pub trait_default_methods: HashMap<String, HashMap<String, StructMethod>>,
    pub goroutine_initial_snapshot: Option<HashMap<String, crate::stdlib::concurrency::ValueSerial>>,
    pub borrow_checker: crate::borrow_checker::BorrowChecker,
    pub scope_borrows_stack: Vec<Vec<(String, Option<String>)>>,
    pub scope_decl_order_stack: Vec<Vec<String>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        register_stdlib(&mut globals);
        let globals_rc = Rc::new(RefCell::new(globals));
        Self {
            globals: globals_rc.clone(),
            environment: globals_rc,
            imported_modules: HashSet::new(),
            source_dir_stack: Vec::new(),
            defer_stack: vec![Vec::new()],
            active_panic: None,
            trait_registry: HashMap::new(),
            trait_default_methods: HashMap::new(),
            goroutine_initial_snapshot: None,
            borrow_checker: crate::borrow_checker::BorrowChecker::new(),
            scope_borrows_stack: vec![Vec::new()],
            scope_decl_order_stack: vec![Vec::new()],
        }
    }

    pub fn call_value_public(
        &mut self,
        callee: Value,
        args: &[Value],
        span: &Span,
    ) -> Result<Value, Galat> {
        self.call_value(callee, args, span)
    }

    pub fn execute_goroutine_body(&mut self, statements: &[Stmt]) -> Result<Value, Galat> {
        self.defer_stack.push(Vec::new());
        let mut last = Value::Nil;
        let mut exec_err: Option<Galat> = None;
        for stmt in statements {
            match self.execute(stmt) {
                Ok(v) => last = v,
                Err(Galat::Kembalikan(v)) => {
                    last = v;
                    break;
                }
                Err(e) => {
                    exec_err = Some(e);
                    break;
                }
            }
        }
        let deferred = self.defer_stack.pop().unwrap_or_default();
        let dummy_span = Span::new(0, 0);
        if let Some(ref e) = exec_err {
            self.active_panic = Some(Self::extract_galat_msg(e));
        }
        for def_fn in deferred.into_iter().rev() {
            let _ = self.call_value(def_fn, &[], &dummy_span);
        }
        if let Some(e) = exec_err {
            if self.active_panic.is_none() {
                Ok(Value::Nil)
            } else {
                Err(e)
            }
        } else {
            Ok(last)
        }
    }

    pub fn flush_goroutine_dirty_vars(&mut self) {
        if let Some(ref mut initial) = self.goroutine_initial_snapshot {
            let current_flat = self.environment.borrow().flatten_chain();
            if let Ok(mut dirty) = crate::stdlib::concurrency::get_dirty_goroutine_vars().lock() {
                for (k, v) in current_flat {
                    if matches!(
                        v,
                        Value::Builtin(_) | Value::Function(_) | Value::StructDef(_)
                    ) {
                        continue;
                    }
                    let ser = crate::stdlib::concurrency::ValueSerial::from_value(&v);
                    if let Some(init_ser) = initial.get(&k) {
                        if &ser != init_ser {
                            dirty.insert(k.clone(), ser.clone());
                            initial.insert(k, ser);
                        }
                    }
                }
            }
        }
    }

    pub fn sync_goroutine_dirty_vars(&mut self) {
        crate::stdlib::concurrency::sync_dirty_vars_to_env(&self.environment);
        if let Some(ref mut initial) = self.goroutine_initial_snapshot {
            if let Ok(dirty) = crate::stdlib::concurrency::get_dirty_goroutine_vars().lock() {
                for (k, v_ser) in dirty.iter() {
                    if initial.contains_key(k) {
                        initial.insert(k.clone(), v_ser.clone());
                    }
                }
            }
        }
    }

    fn extract_galat_msg(e: &Galat) -> String {
        match e {
            Galat::Runtime { pesan, .. } => pesan.clone(),
            Galat::Sintaks { pesan, .. } => pesan.clone(),
            other => format!("{}", other),
        }
    }


    pub fn get_environment_bindings(&self) -> HashMap<String, Value> {
        let mut bindings = HashMap::new();
        let all_local = self.environment.borrow().get_all_local();
        for (k, v) in all_local {
            match v {
                Value::Builtin(_) => {}
                _ => {
                    bindings.insert(k, v);
                }
            }
        }
        bindings
    }

    pub fn get_global_variables(&self) -> HashMap<String, Value> {
        self.get_environment_bindings()
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Value, Galat> {
        let mut last_value = Value::Nil;
        let mut exec_err: Option<Galat> = None;
        for stmt in &program.statements {
            match self.execute(stmt) {
                Ok(v) => last_value = v,
                Err(Galat::Kembalikan(val)) => {
                    last_value = val;
                    break;
                }
                Err(e) => {
                    exec_err = Some(e);
                    break;
                }
            }
        }
        // Run top-level deferred statements in LIFO order
        if let Some(top_defers) = self.defer_stack.last_mut() {
            let deferred: Vec<Value> = top_defers.drain(..).collect();
            let dummy_span = Span::new(0, 0);
            if let Some(ref e) = exec_err {
                self.active_panic = Some(Self::extract_galat_msg(e));
            }
            for def_fn in deferred.into_iter().rev() {
                let _ = self.call_value(def_fn, &[], &dummy_span);
            }
        }
        if let Some(e) = exec_err {
            if self.active_panic.is_none() {
                Ok(last_value)
            } else {
                Err(e)
            }
        } else {
            Ok(last_value)
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<Value, Galat> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::VarDecl {
                name,
                is_const,
                initializer,
                ..
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate(init)?
                } else {
                    Value::Nil
                };
                self.environment
                    .borrow_mut()
                    .define(name.clone(), value, *is_const);
                self.borrow_checker.define_owned(name);
                if let Some(decl_frame) = self.scope_decl_order_stack.last_mut() {
                    if !decl_frame.contains(name) {
                        decl_frame.push(name.clone());
                    }
                }
                Ok(Value::Nil)
            }
            Stmt::Assignment {
                target,
                op,
                value,
                span,
            } => {
                let rhs_val = self.evaluate(value)?;
                match target {
                    AssignTarget::Variable(name) => {
                        self.borrow_checker.check_can_mutate(name, span)?;
                        let final_val = match op {
                            AssignOp::Assign => rhs_val,
                            _ => {
                                let current_val = self.environment.borrow().get(name, span)?;
                                self.apply_compound_assign(current_val, rhs_val, op, span)?
                            }
                        };
                        self.environment
                            .borrow_mut()
                            .assign(name, final_val.clone(), span)?;
                        Ok(final_val)
                    }
                    AssignTarget::Index { target, index } => {
                        let target_val = self.evaluate(target)?;
                        let index_val = self.evaluate(index)?;
                        self.assign_to_index(target_val, index_val, rhs_val, op, span)
                    }
                }
            }
            Stmt::Block(statements, block_span) => {
                let previous = self.environment.clone();
                let block_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                self.environment = block_env;
                self.scope_borrows_stack.push(Vec::new());
                self.scope_decl_order_stack.push(Vec::new());

                let mut result = Ok(Value::Nil);
                for statement in statements {
                    match self.execute(statement) {
                        Ok(v) => {
                            result = Ok(v);
                        }
                        Err(e) => {
                            result = Err(e);
                            break;
                        }
                    }
                }

                // Release all borrows created in this lexical scope (NLL / Lexical Scope Drop)
                let scope_borrows = self.scope_borrows_stack.pop().unwrap_or_default();
                for (b_var, b_field) in scope_borrows {
                    if let Some(f) = b_field {
                        self.borrow_checker.release_field_borrow(&b_var, &f);
                    } else {
                        self.borrow_checker.release_borrow(&b_var);
                    }
                }

                // Deterministic RAII Destructor (Drop / Hancurkan) in reverse declaration order (LIFO)
                let decl_order = self.scope_decl_order_stack.pop().unwrap_or_default();
                let local_snapshot = self.environment.borrow().get_all_local();
                for var_name in decl_order.into_iter().rev() {
                    if self.borrow_checker.is_moved(&var_name) {
                        continue;
                    }
                    if let Some(val) = local_snapshot.get(&var_name) {
                        match val {
                            Value::Instance(inst) => {
                                let drop_method_opt = {
                                    let b_inst = inst.borrow();
                                    b_inst
                                        .struct_def
                                        .methods
                                        .get("hancurkan")
                                        .or_else(|| b_inst.struct_def.methods.get("drop"))
                                        .cloned()
                                };
                                if let Some(drop_method) = drop_method_opt {
                                    let _ = self.call_method(inst.clone(), &drop_method, &[], block_span);
                                }
                            }
                            Value::Map(m) => {
                                let tipe_opt = m.borrow().get("_tipe").and_then(|v| {
                                    if let Value::String(s) = v {
                                        Some(s.clone())
                                    } else {
                                        None
                                    }
                                });
                                match tipe_opt.as_deref() {
                                    Some("GuardMutex") => {
                                        let already_released = matches!(
                                            m.borrow().get("_dilepas"),
                                            Some(Value::Bool(true))
                                        );
                                        if !already_released {
                                            let mtx_opt = m.borrow().get("_mutex").cloned();
                                            let new_val_opt = m.borrow().get("nilai").cloned();
                                            if let Some(Value::Map(ref mtx_map)) = mtx_opt {
                                                if let Some(nv) = new_val_opt {
                                                    mtx_map.borrow_mut().insert("nilai".to_string(), nv);
                                                }
                                            }
                                            m.borrow_mut().insert("_dilepas".to_string(), Value::Bool(true));
                                            if let Some(mtx_val) = mtx_opt {
                                                self.flush_goroutine_dirty_vars();
                                                let _ = crate::stdlib::concurrency::builtin_mutex_buka(
                                                    &[mtx_val],
                                                    block_span,
                                                );
                                            }
                                        }
                                    }
                                    Some("RefImut") => {
                                        let active = matches!(
                                            m.borrow().get("_aktif"),
                                            Some(Value::Bool(true))
                                        );
                                        if active {
                                            if let Some(Value::Map(sel)) = m.borrow().get("_sel").cloned() {
                                                let count = sel
                                                    .borrow()
                                                    .get("_pinjam_baca")
                                                    .and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None })
                                                    .unwrap_or(1);
                                                sel.borrow_mut().insert(
                                                    "_pinjam_baca".to_string(),
                                                    Value::Number(count.saturating_sub(1) as f64),
                                                );
                                            }
                                            m.borrow_mut().insert("_aktif".to_string(), Value::Bool(false));
                                        }
                                    }
                                    Some("RefMut") => {
                                        let active = matches!(
                                            m.borrow().get("_aktif"),
                                            Some(Value::Bool(true))
                                        );
                                        if active {
                                            if let Some(Value::Map(sel)) = m.borrow().get("_sel").cloned() {
                                                sel.borrow_mut().insert(
                                                    "_pinjam_tulis".to_string(),
                                                    Value::Bool(false),
                                                );
                                            }
                                            m.borrow_mut().insert("_aktif".to_string(), Value::Bool(false));
                                        }
                                    }
                                    Some("Referensi") => {
                                        let active = matches!(
                                            m.borrow().get("_aktif"),
                                            Some(Value::Bool(true))
                                        );
                                        if active {
                                            let b_var = m
                                                .borrow()
                                                .get("_var")
                                                .map(|v| v.to_string_repr())
                                                .unwrap_or_default();
                                            let b_field = m
                                                .borrow()
                                                .get("_field")
                                                .map(|v| v.to_string_repr())
                                                .unwrap_or_default();
                                            if !b_var.is_empty() {
                                                if !b_field.is_empty() {
                                                    self.borrow_checker.release_field_borrow(&b_var, &b_field);
                                                } else {
                                                    self.borrow_checker.release_borrow(&b_var);
                                                }
                                            }
                                            m.borrow_mut().insert("_aktif".to_string(), Value::Bool(false));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }

                self.environment = previous;
                result
            }
            Stmt::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
                ..
            } => {
                let cond_val = self.evaluate(condition)?;
                if cond_val.is_truthy() {
                    return self.execute(then_branch);
                }

                for (elif_cond, elif_body) in elif_branches {
                    let elif_val = self.evaluate(elif_cond)?;
                    if elif_val.is_truthy() {
                        return self.execute(elif_body);
                    }
                }

                if let Some(else_stmt) = else_branch {
                    return self.execute(else_stmt);
                }

                Ok(Value::Nil)
            }
            Stmt::While {
                condition, body, ..
            } => {
                let mut last = Value::Nil;
                while self.evaluate(condition)?.is_truthy() {
                    match self.execute(body) {
                        Ok(v) => last = v,
                        Err(Galat::Berhenti) => break,
                        Err(Galat::Lanjut) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last)
            }
            Stmt::ForIn {
                var_name,
                iterable,
                body,
                span,
            } => {
                let iter_val = self.evaluate(iterable)?;
                // Go Range-over-Channel: `untuk pesan dalam saluran_ch { ... }`
                if let Value::Map(ref m) = iter_val {
                    let is_channel = matches!(
                        m.borrow().get("_tipe"),
                        Some(Value::String(t)) if t == "Saluran"
                    );
                    if is_channel {
                        let ch_id = crate::stdlib::concurrency::extract_id_from_arg(&iter_val, "Saluran", span)?;
                        let ch = crate::stdlib::concurrency::get_channel_arc(ch_id, span)?;
                        let previous = self.environment.clone();
                        let loop_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                        self.environment = loop_env;
                        let mut last = Value::Nil;
                        loop {
                            let (val_opt, ok) = ch.recv_blocking();
                            if !ok {
                                break;
                            }
                            let item = val_opt.map(|v| v.to_value()).unwrap_or(Value::Nil);
                            self.environment.borrow_mut().define(var_name.clone(), item, false);
                            match self.execute(body) {
                                Ok(v) => last = v,
                                Err(Galat::Berhenti) => break,
                                Err(Galat::Lanjut) => continue,
                                Err(e) => {
                                    self.environment = previous;
                                    return Err(e);
                                }
                            }
                        }
                        self.environment = previous;
                        self.sync_goroutine_dirty_vars();
                        return Ok(last);
                    }
                }
                let items: Vec<Value> = match iter_val {
                    Value::Range(start, end) => {
                        if start <= end {
                            (start..=end).map(|n| Value::Number(n as f64)).collect()
                        } else {
                            (end..=start)
                                .rev()
                                .map(|n| Value::Number(n as f64))
                                .collect()
                        }
                    }
                    Value::Array(a) => a.borrow().clone(),
                    Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
                    Value::Map(m) => {
                        let map = m.borrow();
                        let mut keys: Vec<Value> =
                            map.keys().map(|k| Value::String(k.clone())).collect();
                        keys.sort_by(|a, b| a.to_string_repr().cmp(&b.to_string_repr()));
                        keys
                    }
                    other => {
                        return Err(Galat::runtime(
                            format!(
                                "Tipe '{}' tidak dapat diiterasi pada perulangan 'untuk'",
                                other.type_name()
                            ),
                            span,
                        ));
                    }
                };

                let previous = self.environment.clone();
                let loop_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                self.environment = loop_env;

                let mut last = Value::Nil;
                for item in items {
                    self.environment
                        .borrow_mut()
                        .define(var_name.clone(), item, false);
                    match self.execute(body) {
                        Ok(v) => last = v,
                        Err(Galat::Berhenti) => break,
                        Err(Galat::Lanjut) => continue,
                        Err(e) => {
                            self.environment = previous;
                            return Err(e);
                        }
                    }
                }

                self.environment = previous;
                Ok(last)
            }
            Stmt::Loop { body, .. } => {
                let mut last = Value::Nil;
                loop {
                    match self.execute(body) {
                        Ok(v) => last = v,
                        Err(Galat::Berhenti) => break,
                        Err(Galat::Lanjut) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last)
            }
            Stmt::DestructureDecl {
                names,
                initializer,
                is_const,
                span,
            } => {
                // Check for Go comma-ok map lookup: `buat nilai, ada = kamus["kunci"]`
                if names.len() == 2 {
                    if let Expr::Index { target, index, .. } = initializer {
                        let target_val = self.evaluate(target)?;
                        if let Value::Map(m) = target_val {
                            let idx_val = self.evaluate(index)?;
                            let key = idx_val.to_string_repr();
                            let map = m.borrow();
                            let exists = map.contains_key(&key);
                            let val = map.get(&key).cloned().unwrap_or(Value::Nil);
                            drop(map);
                            let mut env_mut = self.environment.borrow_mut();
                            if !*is_const && env_mut.assign(&names[0], val.clone(), span).is_ok() {
                            } else {
                                env_mut.define(names[0].clone(), val, *is_const);
                            }
                            if !*is_const && env_mut.assign(&names[1], Value::Bool(exists), span).is_ok() {
                            } else {
                                env_mut.define(names[1].clone(), Value::Bool(exists), *is_const);
                            }
                            return Ok(Value::Nil);
                        }
                    }
                }

                let init_val = self.evaluate(initializer)?;
                match init_val {
                    Value::Array(arr) => {
                        let items = arr.borrow().clone();
                        let mut env_mut = self.environment.borrow_mut();
                        for (i, name) in names.iter().enumerate() {
                            let val = items.get(i).cloned().unwrap_or(Value::Nil);
                            if !*is_const && env_mut.assign(name, val.clone(), span).is_ok() {
                            } else {
                                env_mut.define(name.clone(), val, *is_const);
                            }
                        }
                        Ok(Value::Nil)
                    }
                    Value::Map(m) => {
                        let map = m.borrow();
                        let mut env_mut = self.environment.borrow_mut();
                        for name in names {
                            let val = map.get(name).cloned().unwrap_or(Value::Nil);
                            env_mut.define(name.clone(), val, *is_const);
                        }
                        Ok(Value::Nil)
                    }
                    other => Err(Galat::runtime(
                        format!("Dekonstruksi memerlukan daftar atau kamus, ditemukan '{}'", other.type_name()),
                        span,
                    )),
                }
            }
            Stmt::FunctionDecl {
                name,
                params,
                body,
                attributes,
                span,
                ..
            } => {
                let function = WidyaFunction {
                    name: Some(name.clone()),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                let mut func_val = Value::Function(Rc::new(function));

                // Apply Python-style @decorators in reverse order (innermost first)
                for attr in attributes.iter().rev() {
                    if let Some(dec_name) = attr.name.strip_prefix('@') {
                        let decorator_fn = self.environment.borrow().get(dec_name, span)?;
                        func_val = self.call_value(decorator_fn, &[func_val], span)?;
                    }
                }

                self.environment.borrow_mut().define(
                    name.clone(),
                    func_val,
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::StructDecl {
                name,
                fields,
                methods,
                attributes,
                span: _,
            } => {
                let mut method_map = HashMap::new();
                for method in methods {
                    method_map.insert(method.name.clone(), method.clone());
                }

                // Process Procedural Macro attributes: #[turunkan(Json, Serialisasi, Debug, Duplikat, Sama, Bawaan)]
                for attr in attributes {
                    if attr.name == "turunkan" || attr.name == "derive" {
                        for trait_macro in &attr.arguments {
                            match trait_macro.to_lowercase().as_str() {
                                "json" | "serialisasi" | "serde" => {
                                    // Auto-generate .ke_json() method
                                    let dummy_span = attr.span.clone();
                                    method_map.insert("ke_json".to_string(), StructMethod {
                                        name: "ke_json".to_string(),
                                        params: Vec::new(),
                                        body: vec![Stmt::Return {
                                            value: Some(Expr::Call {
                                                callee: Box::new(Expr::Identifier("ke_json".to_string(), dummy_span.clone())),
                                                arguments: vec![Expr::This(dummy_span.clone())],
                                                span: dummy_span.clone(),
                                            }),
                                            span: dummy_span.clone(),
                                        }],
                                        span: dummy_span,
                                    });
                                }
                                "debug" | "tampilan" => {
                                    // Auto-generate .format_debug()
                                    let dummy_span = attr.span.clone();
                                    method_map.insert("format_debug".to_string(), StructMethod {
                                        name: "format_debug".to_string(),
                                        params: Vec::new(),
                                        body: vec![Stmt::Return {
                                            value: Some(Expr::Call {
                                                callee: Box::new(Expr::Identifier("ke_teks".to_string(), dummy_span.clone())),
                                                arguments: vec![Expr::This(dummy_span.clone())],
                                                span: dummy_span.clone(),
                                            }),
                                            span: dummy_span.clone(),
                                        }],
                                        span: dummy_span,
                                    });
                                }
                                "duplikat" | "clone" => {
                                    // Auto-generate .duplikat() & .clone() with deep field cloning
                                    let dummy_span = attr.span.clone();
                                    let clone_body = vec![Stmt::Return {
                                        value: Some(Expr::Call {
                                            callee: Box::new(Expr::Identifier("_klon_instans".to_string(), dummy_span.clone())),
                                            arguments: vec![Expr::This(dummy_span.clone())],
                                            span: dummy_span.clone(),
                                        }),
                                        span: dummy_span.clone(),
                                    }];
                                    method_map.insert("duplikat".to_string(), StructMethod {
                                        name: "duplikat".to_string(),
                                        params: Vec::new(),
                                        body: clone_body.clone(),
                                        span: dummy_span.clone(),
                                    });
                                    method_map.insert("clone".to_string(), StructMethod {
                                        name: "clone".to_string(),
                                        params: Vec::new(),
                                        body: clone_body,
                                        span: dummy_span,
                                    });
                                }
                                "sama" | "partialeq" | "eq" => {
                                    let dummy_span = attr.span.clone();
                                    let eq_body = vec![Stmt::Return {
                                        value: Some(Expr::Call {
                                            callee: Box::new(Expr::Identifier("_sama_instans".to_string(), dummy_span.clone())),
                                            arguments: vec![
                                                Expr::This(dummy_span.clone()),
                                                Expr::Identifier("lain".to_string(), dummy_span.clone()),
                                            ],
                                            span: dummy_span.clone(),
                                        }),
                                        span: dummy_span.clone(),
                                    }];
                                    method_map.insert("sama".to_string(), StructMethod {
                                        name: "sama".to_string(),
                                        params: vec!["lain".to_string()],
                                        body: eq_body.clone(),
                                        span: dummy_span.clone(),
                                    });
                                    method_map.insert("eq".to_string(), StructMethod {
                                        name: "eq".to_string(),
                                        params: vec!["lain".to_string()],
                                        body: eq_body,
                                        span: dummy_span,
                                    });
                                }
                                "bawaan" | "default" => {
                                    let dummy_span = attr.span.clone();
                                    let default_args: Vec<Expr> = fields
                                        .iter()
                                        .map(|_| Expr::Number(0.0, dummy_span.clone()))
                                        .collect();
                                    let def_body = vec![Stmt::Return {
                                        value: Some(Expr::Call {
                                            callee: Box::new(Expr::Identifier(name.clone(), dummy_span.clone())),
                                            arguments: default_args,
                                            span: dummy_span.clone(),
                                        }),
                                        span: dummy_span.clone(),
                                    }];
                                    method_map.insert("bawaan".to_string(), StructMethod {
                                        name: "bawaan".to_string(),
                                        params: Vec::new(),
                                        body: def_body.clone(),
                                        span: dummy_span.clone(),
                                    });
                                    method_map.insert("default".to_string(), StructMethod {
                                        name: "default".to_string(),
                                        params: Vec::new(),
                                        body: def_body,
                                        span: dummy_span,
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let mut is_packed = false;
                let mut c_field_map: HashMap<String, (crate::sys::c_abi::CType, Option<u8>)> = HashMap::new();
                for attr in attributes {
                    let aname = attr.name.trim_start_matches('@').to_lowercase();
                    if aname == "packed" || (aname == "repr" && attr.arguments.iter().any(|a| a.to_lowercase() == "packed")) {
                        is_packed = true;
                    }
                    if attr.name == "__c_field__" && attr.arguments.len() >= 2 {
                        let fname = attr.arguments[0].clone();
                        let ftype = crate::sys::c_abi::CType::from_name(&attr.arguments[1]);
                        let bit_w = attr.arguments.get(2).and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() });
                        c_field_map.insert(fname, (ftype, bit_w));
                    }
                }
                let mut c_specs = Vec::new();
                for f in fields {
                    if let Some((ct, bw)) = c_field_map.get(f) {
                        c_specs.push((f.clone(), ct.clone(), *bw));
                    } else {
                        c_specs.push((f.clone(), crate::sys::c_abi::CType::F64, None));
                    }
                }
                let c_layout = crate::sys::c_abi::CStructLayout::compute(name, &c_specs, is_packed);
                if let Ok(mut reg) = crate::sys::c_abi::get_c_type_registry().lock() {
                    reg.structs.insert(name.clone(), c_layout);
                }

                let struct_def = WidyaStruct {
                    name: name.clone(),
                    fields: fields.clone(),
                    methods: method_map,
                    closure: self.environment.clone(),
                };
                self.environment.borrow_mut().define(
                    name.clone(),
                    Value::StructDef(Rc::new(struct_def)),
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::TraitDecl { name, methods, .. } => {
                let method_names: Vec<String> = methods.iter().map(|m| m.name.clone()).collect();
                self.trait_registry.insert(name.clone(), method_names.clone());
                let mut trait_map = HashMap::new();
                trait_map.insert("_tipe".to_string(), Value::String("Sifat".to_string()));
                trait_map.insert("nama".to_string(), Value::String(name.clone()));
                trait_map.insert(
                    "_metode".to_string(),
                    Value::Array(Rc::new(RefCell::new(
                        method_names.into_iter().map(Value::String).collect(),
                    ))),
                );
                self.environment.borrow_mut().define(
                    name.clone(),
                    Value::Map(Rc::new(RefCell::new(trait_map))),
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::ImplDecl { trait_name, target_name, methods, span } => {
                if let Some(tr_raw) = trait_name {
                    if let Some(real_trait_name) = tr_raw.strip_prefix("__trait_def__:") {
                        let method_names: Vec<String> = methods.iter().map(|m| m.name.clone()).collect();
                        self.trait_registry
                            .insert(real_trait_name.to_string(), method_names.clone());
                        let mut default_map = HashMap::new();
                        for m in methods {
                            if !m.body.is_empty() {
                                default_map.insert(m.name.clone(), m.clone());
                            }
                        }
                        self.trait_default_methods
                            .insert(real_trait_name.to_string(), default_map);
                        let mut trait_map = HashMap::new();
                        trait_map.insert("_tipe".to_string(), Value::String("Sifat".to_string()));
                        trait_map.insert("nama".to_string(), Value::String(real_trait_name.to_string()));
                        trait_map.insert(
                            "_metode".to_string(),
                            Value::Array(Rc::new(RefCell::new(
                                method_names.into_iter().map(Value::String).collect(),
                            ))),
                        );
                        self.environment.borrow_mut().define(
                            real_trait_name.to_string(),
                            Value::Map(Rc::new(RefCell::new(trait_map))),
                            false,
                        );
                        return Ok(Value::Nil);
                    }
                }

                let current_val = self.environment.borrow().get(target_name, span)?;
                if let Value::StructDef(ref struct_def) = current_val {
                    let mut new_methods = struct_def.methods.clone();
                    // Inherit default methods from trait if implementing a trait
                    if let Some(tr_name) = trait_name {
                        if let Some(defaults) = self.trait_default_methods.get(tr_name) {
                            for (d_name, d_method) in defaults {
                                new_methods.insert(d_name.clone(), d_method.clone());
                            }
                        }
                    }
                    for m in methods {
                        new_methods.insert(m.name.clone(), m.clone());
                    }
                    let updated_struct = WidyaStruct {
                        name: struct_def.name.clone(),
                        fields: struct_def.fields.clone(),
                        methods: new_methods,
                        closure: self.environment.clone(),
                    };
                    self.environment.borrow_mut().assign(target_name, Value::StructDef(Rc::new(updated_struct)), span)?;
                }
                Ok(Value::Nil)
            }
            Stmt::IfLet {
                pattern,
                value,
                then_branch,
                else_branch,
                ..
            } => {
                let target_val = self.evaluate(value)?;
                if let Some(bindings) = self.match_pattern_check(pattern, &target_val)? {
                    let previous = self.environment.clone();
                    let then_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                    for (var_name, var_val) in bindings {
                        then_env.borrow_mut().define(var_name, var_val, false);
                    }
                    self.environment = then_env;
                    let result = self.execute(then_branch);
                    self.environment = previous;
                    result
                } else if let Some(else_b) = else_branch {
                    self.execute(else_b)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::EnumDecl { name, variants, .. } => {
                let mut enum_map = HashMap::new();
                for variant in variants {
                    let v_name = variant.name.clone();
                    let e_name = name.clone();
                    let f_count = variant.fields_count;

                    if f_count == 0 {
                        let enum_val = Value::EnumVariant(Rc::new(crate::value::WidyaEnumVariant {
                            enum_name: e_name,
                            variant_name: v_name.clone(),
                            fields: Vec::new(),
                        }));
                        enum_map.insert(v_name, enum_val);
                    } else {
                        // Constructor function for variant with fields
                        let ctor = Value::Function(Rc::new(WidyaFunction {
                            name: Some(format!("{}::{}", e_name, v_name)),
                            params: (0..f_count).map(|i| format!("_arg{}", i)).collect(),
                            body: Vec::new(), // Handled via special constructor
                            closure: self.environment.clone(),
                        }));
                        enum_map.insert(v_name, ctor);
                    }
                }
                self.environment.borrow_mut().define(
                    name.clone(),
                    Value::Map(Rc::new(RefCell::new(enum_map))),
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::Import { path, alias, span } => {
                // Built-in hierarchical std::* atau modul bawaan (math, json, sys, ai, tensor, python, dll.)
                if let Some((ns_name, ns_map)) = self.resolve_builtin_module(path) {
                    if let Some(alias_name) = alias {
                        self.environment.borrow_mut().define(
                            alias_name.clone(),
                            Value::Map(Rc::new(RefCell::new(ns_map))),
                            false,
                        );
                    } else {
                        let mut env_borrow = self.environment.borrow_mut();
                        if !ns_name.is_empty() {
                            env_borrow.define(
                                ns_name,
                                Value::Map(Rc::new(RefCell::new(ns_map.clone()))),
                                false,
                            );
                        }
                        for (k, v) in ns_map {
                            if !env_borrow.contains_key_local(&k) {
                                env_borrow.define(k, v, false);
                            }
                        }
                    }
                    return Ok(Value::Nil);
                }

                let mut resolved_path: Option<std::path::PathBuf> = None;
                if let Some(current_dir) = self.source_dir_stack.last() {
                    let candidate = current_dir.join(path);
                    if candidate.exists() {
                        resolved_path = Some(candidate);
                    } else if candidate.with_extension("wya").exists() {
                        resolved_path = Some(candidate.with_extension("wya"));
                    } else if candidate.with_extension("widya").exists() {
                        resolved_path = Some(candidate.with_extension("widya"));
                    }
                }

                if resolved_path.is_none() {
                    let p = Path::new(path);
                    if p.exists() {
                        resolved_path = Some(p.to_path_buf());
                    } else if p.with_extension("wya").exists() {
                        resolved_path = Some(p.with_extension("wya"));
                    } else if p.with_extension("widya").exists() {
                        resolved_path = Some(p.with_extension("widya"));
                    }
                }

                if resolved_path.is_none() {
                    // Cek direktori packages/ (WPM Official Packages) & direktori vendor lokal
                    let clean_pkg_name = if path.starts_with("@widya/") {
                        format!("widya-{}", &path["@widya/".len()..])
                    } else {
                        path.to_string()
                    };

                    let package_candidates = [
                        std::path::PathBuf::from("packages").join(&clean_pkg_name).join("src").join("lib.wya"),
                        std::path::PathBuf::from("packages").join(path).join("src").join("lib.wya"),
                        std::path::PathBuf::from("vendor").join(path).join("src").join("lib.wya"),
                        std::path::PathBuf::from("vendor").join(&clean_pkg_name).join("src").join("lib.wya"),
                        std::path::PathBuf::from("vendor").join(path).join("src").join("main.wya"),
                        std::path::PathBuf::from("vendor").join(path).join("lib.wya"),
                        std::path::PathBuf::from("vendor").join(format!("{}.wya", path)),
                        std::path::PathBuf::from(".widya").join("packages").join(path).join("src").join("lib.wya"),
                        std::path::PathBuf::from(".widya").join("packages").join(&clean_pkg_name).join("src").join("lib.wya"),
                    ];
                    for vc in &package_candidates {
                        if vc.exists() {
                            resolved_path = Some(vc.clone());
                            break;
                        }
                    }
                }

                let final_path_buf = match resolved_path {
                    Some(p) => p,
                    None => {
                        return Err(Galat::runtime(
                            format!("Berkas modul '{}' tidak ditemukan", path),
                            span,
                        ));
                    }
                };

                let canonical_key = final_path_buf
                    .canonicalize()
                    .unwrap_or_else(|_| final_path_buf.clone())
                    .display()
                    .to_string();

                if self.imported_modules.contains(&canonical_key) {
                    return Ok(Value::Nil);
                }
                self.imported_modules.insert(canonical_key);

                let source = match fs::read_to_string(&final_path_buf) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(Galat::runtime(
                            format!("Gagal membaca berkas modul '{}': {}", final_path_buf.display(), e),
                            span,
                        ));
                    }
                };

                let mut lexer = Lexer::new(&source);
                let tokens = lexer.scan_tokens()?;
                let mut parser = Parser::new(tokens);
                let program = parser.parse()?;

                let env_sebelum = self.environment.clone();
                let import_env = Rc::new(RefCell::new(Environment::with_parent(env_sebelum.clone())));
                self.environment = import_env.clone();

                let import_program = Program::new(program.statements.clone());
                let parent_dir = final_path_buf.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| std::path::PathBuf::from("."));
                self.source_dir_stack.push(parent_dir);
                let import_result = self.interpret(&import_program);
                self.source_dir_stack.pop();

                // Kembalikan environment ke sebelumnya
                self.environment = env_sebelum;

                if let Err(e) = import_result {
                    return Err(e);
                }


                // Ekstrak SEMUA yang didefinisikan di import_env (local level) untuk diexport ke caller
                let exported = import_env.borrow().get_all_local();

                if let Some(alias_name) = alias {
                    // ⚡ Untuk alias: SELALU buat namespace baru dari exported (tidak perlu skip existing)
                    self.environment.borrow_mut().define(
                        alias_name.clone(),
                        Value::Map(Rc::new(RefCell::new(exported))),
                        false,
                    );
                } else {
                    // ⚡ Untuk export FLAT: JIKA KEY SUDAH ADA DI CALLER ENV → SKIP OVERWRITE!
                    //    (supaya variabel mutable seperti _penyimpanan_db yang sudah punya data
                    //     TIDAK di-reset jadi {} / nil oleh import transitive berulang)
                    let mut env_borrow = self.environment.borrow_mut();
                    for (k, v) in exported {
                        if !env_borrow.contains_key_local(&k) {
                            env_borrow.define(k, v, false);
                        }
                    }
                }

                match import_result {
                    Ok(_) => Ok(Value::Nil),
                    Err(e) => Err(e),
                }
            }
            Stmt::TryCatch {
                try_block,
                error_var,
                catch_block,
                ..
            } => match self.execute(try_block) {
                Ok(v) => Ok(v),
                Err(Galat::Kembalikan(v)) => Err(Galat::Kembalikan(v)),
                Err(Galat::Berhenti) => Err(Galat::Berhenti),
                Err(Galat::Lanjut) => Err(Galat::Lanjut),
                Err(e) => {
                    let prev_env = self.environment.clone();
                    let catch_env =
                        Rc::new(RefCell::new(Environment::with_parent(prev_env.clone())));
                    self.environment = catch_env;

                    let error_msg = match &e {
                        Galat::Runtime { pesan, .. } => pesan.clone(),
                        Galat::Sintaks { pesan, .. } => pesan.clone(),
                        _ => format!("{}", e),
                    };

                    self.environment.borrow_mut().define(
                        error_var.clone(),
                        Value::String(error_msg),
                        false,
                    );

                    let res = self.execute(catch_block);
                    self.environment = prev_env;
                    res
                }
            },
            Stmt::Throw { expr, span } => {
                let err_val = self.evaluate(expr)?;
                Err(Galat::runtime(err_val.to_string_repr(), span))
            }
            Stmt::Return { value, .. } => {
                let val = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                Err(Galat::Kembalikan(val))
            }
            Stmt::ExternalBlock { abi: _, functions, span: _ } => {
                // Register mock FFI functions into current environment
                for func_decl in functions {
                    let f_name = func_decl.name.clone();
                    let builtin_fn: crate::value::BuiltinFn = match f_name.as_str() {
                        "puts" | "printf" => crate::stdlib::builtin_cetak,
                        "abs" => crate::stdlib::builtin_mutlak,
                        "sqrt" => crate::stdlib::builtin_akar,
                        "sin" => crate::stdlib::builtin_sin,
                        "cos" => crate::stdlib::builtin_cos,
                        _ => crate::stdlib::builtin_ffi_call_generic,
                    };
                    
                    let builtin = crate::value::BuiltinFunction {
                        name: f_name.clone(),
                        arity: Some(func_decl.params.len()),
                        func: builtin_fn,
                    };
                    self.environment.borrow_mut().define(
                        func_decl.name.clone(),
                        Value::Builtin(Rc::new(builtin)),
                        false,
                    );
                }
                Ok(Value::Nil)
            }
            Stmt::VolatileWrite { address, value, span } => {
                let addr_val = self.evaluate(address)?;
                let data_val = self.evaluate(value)?;
                let addr_num = match addr_val {
                    Value::Number(n) => n as usize,
                    _ => return Err(Galat::runtime("Alamat memori volatil harus berupa angka pointer", span)),
                };
                crate::stdlib::tulis_memori_volatil(addr_num, &data_val, span)?;
                Ok(Value::Nil)
            }
            Stmt::Break(_) => Err(Galat::Berhenti),
            Stmt::Continue(_) => Err(Galat::Lanjut),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, Galat> {
        match expr {
            Expr::Number(n, _) => Ok(Value::Number(*n)),
            Expr::String(s, _) => Ok(Value::String(s.clone())),
            Expr::Boolean(b, _) => Ok(Value::Bool(*b)),
            Expr::Nil(_) => Ok(Value::Nil),
            Expr::This(span) => self.environment.borrow().get("ini", span),
            Expr::Identifier(name, span) => self.environment.borrow().get(name, span),
            Expr::Grouping(e, _) => self.evaluate(e),
            Expr::Unary { op, right, span } => {
                let right_val = self.evaluate(right)?;
                match op {
                    UnaryOp::Negate => match right_val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(Galat::runtime(
                            format!(
                                "Operator '-' tidak dapat digunakan pada tipe '{}'",
                                right_val.type_name()
                            ),
                            span,
                        )),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!right_val.is_truthy())),
                }
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                if *op == BinaryOp::And {
                    let left_val = self.evaluate(left)?;
                    if !left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    return self.evaluate(right);
                }
                if *op == BinaryOp::Or {
                    let left_val = self.evaluate(left)?;
                    if left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    return self.evaluate(right);
                }

                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match op {
                    BinaryOp::Add => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                        (Value::String(a), _) => {
                            Ok(Value::String(format!("{}{}", a, right_val.to_string_repr())))
                        }
                        (_, Value::String(b)) => {
                            Ok(Value::String(format!("{}{}", left_val.to_string_repr(), b)))
                        }
                        (Value::Array(a), Value::Array(b)) => {
                            let mut combined = a.borrow().clone();
                            combined.extend(b.borrow().clone());
                            Ok(Value::Array(Rc::new(RefCell::new(combined))))
                        }
                        _ => Err(Galat::runtime(
                            format!(
                                "Operasi '+' tidak valid antara '{}' dan '{}'",
                                left_val.type_name(),
                                right_val.type_name()
                            ),
                            span,
                        )),
                    },
                    BinaryOp::Subtract => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                        _ => Err(Galat::runtime("Operasi '-' memerlukan dua angka", span)),
                    },
                    BinaryOp::Multiply => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                        (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                            let count = (*n as isize).max(0) as usize;
                            Ok(Value::String(s.repeat(count)))
                        }
                        (Value::Array(arr), Value::Number(n)) | (Value::Number(n), Value::Array(arr)) => {
                            let count = (*n as isize).max(0) as usize;
                            let base = arr.borrow().clone();
                            let mut repeated = Vec::with_capacity(base.len() * count);
                            for _ in 0..count {
                                repeated.extend(base.iter().cloned());
                            }
                            Ok(Value::Array(Rc::new(RefCell::new(repeated))))
                        }
                        _ => Err(Galat::runtime(
                            "Operasi '*' memerlukan angka (atau teks/daftar dengan angka)",
                            span,
                        )),
                    },
                    BinaryOp::Divide => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => {
                            if *b == 0.0 {
                                Err(Galat::runtime("Pembagian dengan nol", span))
                            } else {
                                Ok(Value::Number(a / b))
                            }
                        }
                        _ => Err(Galat::runtime("Operasi '/' memerlukan dua angka", span)),
                    },
                    BinaryOp::Modulo => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => {
                            if *b == 0.0 {
                                Err(Galat::runtime("Sisa bagi (modulo) dengan nol", span))
                            } else {
                                Ok(Value::Number(a % b))
                            }
                        }
                        _ => Err(Galat::runtime("Operasi '%' memerlukan dua angka", span)),
                    },
                    BinaryOp::Power => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
                        _ => Err(Galat::runtime("Operasi '^' memerlukan dua angka", span)),
                    },
                    BinaryOp::Equal => Ok(Value::Bool(left_val == right_val)),
                    BinaryOp::NotEqual => Ok(Value::Bool(left_val != right_val)),
                    BinaryOp::Less => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '<' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::LessEqual => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '<=' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::Greater => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '>' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::GreaterEqual => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '>=' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::And | BinaryOp::Or => unreachable!(),
                }
            }
            Expr::Array(elements, _) => {
                let mut vals = Vec::new();
                for elem in elements {
                    vals.push(self.evaluate(elem)?);
                }
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            }
            Expr::Map(pairs, _) => {
                let mut map = HashMap::new();
                for (k_expr, v_expr) in pairs {
                    let k_val = self.evaluate(k_expr)?;
                    let v_val = self.evaluate(v_expr)?;
                    let key_str = match k_val {
                        Value::String(s) => s,
                        other => other.to_string_repr(),
                    };
                    map.insert(key_str, v_val);
                }
                Ok(Value::Map(Rc::new(RefCell::new(map))))
            }
            Expr::Range { start, end, span } => {
                let start_val = self.evaluate(start)?;
                let end_val = self.evaluate(end)?;
                match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => {
                        Ok(Value::Range(s as i64, e as i64))
                    }
                    _ => Err(Galat::runtime("Rentang (..) memerlukan dua angka", span)),
                }
            }
            Expr::Index {
                target,
                index,
                span,
            } => {
                let target_val = self.evaluate(target)?;
                let index_val = self.evaluate(index)?;
                self.index_get(&target_val, &index_val, span)
            }
            Expr::Call {
                callee,
                arguments,
                span,
            } => {
                // Check if this is a method call on an instance or built-in type: obj.method(...)
                if let Expr::Index { target, index, .. } = &**callee {
                    let target_val = self.evaluate(target)?;
                    let index_val = self.evaluate(index)?;
                    let method_name = match &index_val {
                        Value::String(s) => s.clone(),
                        other => other.to_string_repr(),
                    };

                    let method_opt = if let Value::Instance(ref inst) = target_val {
                        inst.borrow().struct_def.methods.get(&method_name).cloned()
                    } else {
                        None
                    };

                    if let (Value::Instance(inst), Some(method)) = (target_val.clone(), method_opt) {
                        let mut evaluated_args = Vec::new();
                        for arg in arguments {
                            evaluated_args.push(self.evaluate(arg)?);
                        }
                        return self.call_method(inst, &method, &evaluated_args, span);
                    }

                    // Check promoted methods on embedded struct fields (Go Struct Embedding)
                    if let Value::Instance(ref inst) = target_val {
                        let promoted_opt = {
                            let b_inst = inst.borrow();
                            let mut found = None;
                            for fval in b_inst.fields.values() {
                                if let Value::Instance(sub_inst) = fval {
                                    if let Some(m) = sub_inst.borrow().struct_def.methods.get(&method_name).cloned() {
                                        found = Some((sub_inst.clone(), m));
                                        break;
                                    }
                                }
                            }
                            found
                        };
                        if let Some((sub_inst, method)) = promoted_opt {
                            let mut evaluated_args = Vec::new();
                            for arg in arguments {
                                evaluated_args.push(self.evaluate(arg)?);
                            }
                            return self.call_method(sub_inst, &method, &evaluated_args, span);
                        }
                    }

                    // Check built-in methods on Array, String, and Map
                    let is_map_callable_entry = if let Value::Map(ref m) = target_val {
                        matches!(
                            m.borrow().get(&method_name),
                            Some(Value::Function(_)) | Some(Value::Builtin(_)) | Some(Value::StructDef(_))
                        )
                    } else {
                        false
                    };

                    if !is_map_callable_entry
                        && matches!(target_val, Value::Array(_) | Value::String(_) | Value::Map(_))
                    {
                        let mut evaluated_args = Vec::new();
                        for arg in arguments {
                            evaluated_args.push(self.evaluate(arg)?);
                        }
                        if let Some(res) = self.call_builtin_type_method(
                            &target_val,
                            &method_name,
                            &evaluated_args,
                            span,
                        )? {
                            return Ok(res);
                        }
                    }

                    // Fallback to evaluating the resolved property as a function
                    let callee_val = self.index_get(&target_val, &index_val, span)?;
                    let mut evaluated_args = Vec::new();
                    for arg in arguments {
                        evaluated_args.push(self.evaluate(arg)?);
                    }
                    return self.call_value(callee_val, &evaluated_args, span);
                }

                let callee_val = self.evaluate(callee)?;
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    evaluated_args.push(self.evaluate(arg)?);
                }
                self.call_value(callee_val, &evaluated_args, span)
            }
            Expr::FunctionExpr { params, body, .. } => {
                let function = WidyaFunction {
                    name: None,
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                Ok(Value::Function(Rc::new(function)))
            }
            Expr::Match { target, arms, span } => {
                let target_val = self.evaluate(target)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern_check(&arm.pattern, &target_val)? {
                        let previous = self.environment.clone();
                        let arm_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                        for (var_name, var_val) in bindings {
                            arm_env.borrow_mut().define(var_name, var_val, false);
                        }
                        self.environment = arm_env;
                        let result = self.evaluate(&arm.body);
                        self.environment = previous;
                        return result;
                    }
                }
                Err(Galat::runtime("Tidak ada pola yang cocok pada ekspresi 'cocokkan'", span))
            }
            Expr::Try(inner, span) => {
                let val = self.evaluate(inner)?;
                match &val {
                    Value::Map(m) => {
                        let map = m.borrow();
                        if let Some(Value::Bool(true)) = map.get("apakah_ok") {
                            Ok(map.get("nilai").cloned().unwrap_or(Value::Nil))
                        } else if let Some(Value::Bool(false)) = map.get("apakah_ok") {
                            // Early return error object (propagasi galat)
                            Err(Galat::Kembalikan(val.clone()))
                        } else if let Some(Value::Bool(true)) = map.get("ada") {
                            Ok(map.get("nilai").cloned().unwrap_or(Value::Nil))
                        } else if let Some(Value::Bool(false)) = map.get("ada") {
                            Err(Galat::Kembalikan(val.clone()))
                        } else {
                            Ok(val.clone())
                        }
                    }
                    Value::EnumVariant(ev) => {
                        if ev.variant_name == "Ok" || ev.variant_name == "Ada" {
                            if let Some(first_field) = ev.fields.first() {
                                Ok(first_field.clone())
                            } else {
                                Ok(Value::Nil)
                            }
                        } else if ev.variant_name == "Err" || ev.variant_name == "Kosong" {
                            // Early return error enum variant
                            Err(Galat::Kembalikan(val.clone()))
                        } else {
                            Ok(val.clone())
                        }
                    }
                    Value::Nil => {
                        Err(Galat::runtime("Operator '?' tidak dapat digunakan pada nilai nihil", span))
                    }
                    other => Ok(other.clone()),
                }
            }
            Expr::Await(inner, span) => {
                let val = self.evaluate(inner)?;
                match &val {
                    Value::Map(m) => {
                        let map = m.borrow();
                        // If it's a Thread handle object (Utas), join it
                        if let Some(Value::String(t)) = map.get("_tipe") {
                            if t == "Utas" {
                                if let Some(Value::Number(id)) = map.get("id") {
                                    return crate::stdlib::builtin_gabung_utas(&[Value::Number(*id)], span);
                                }
                            }
                        }
                        Ok(val.clone())
                    }
                    Value::Function(f) => {
                        // If awaiting a function, invoke it
                        self.call_value(Value::Function(f.clone()), &[], span)
                    }
                    other => Ok(other.clone()),
                }
            }
            Expr::VolatileRead { address, span } => {
                let addr_val = self.evaluate(address)?;
                let addr_num = match addr_val {
                    Value::Number(n) => n as usize,
                    _ => return Err(Galat::runtime("Alamat memori volatil harus berupa angka pointer", span)),
                };
                crate::stdlib::baca_memori_volatil(addr_num, span)
            }
        }
    }

    fn index_get(&self, target: &Value, index: &Value, span: &Span) -> Result<Value, Galat> {
        match target {
            Value::Array(a) => {
                let items = a.borrow();
                if let Value::String(prop) = index {
                    if prop == "panjang" || prop == "len" || prop == "length" {
                        return Ok(Value::Number(items.len() as f64));
                    }
                }
                let idx = match index {
                    Value::Number(n) => *n as i64,
                    _ => return Err(Galat::runtime("Indeks daftar harus berupa angka", span)),
                };
                let actual_idx = if idx < 0 {
                    (items.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                if actual_idx < items.len() {
                    Ok(items[actual_idx].clone())
                } else {
                    Err(Galat::runtime(
                        format!(
                            "Indeks {} di luar batas daftar (panjang: {})",
                            idx,
                            items.len()
                        ),
                        span,
                    ))
                }
            }
            Value::String(s) => {
                if let Value::String(prop) = index {
                    if prop == "panjang" || prop == "len" || prop == "length" {
                        return Ok(Value::Number(s.chars().count() as f64));
                    }
                }
                let chars: Vec<char> = s.chars().collect();
                let idx = match index {
                    Value::Number(n) => *n as i64,
                    _ => return Err(Galat::runtime("Indeks teks harus berupa angka", span)),
                };
                let actual_idx = if idx < 0 {
                    (chars.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                if actual_idx < chars.len() {
                    Ok(Value::String(chars[actual_idx].to_string()))
                } else {
                    Err(Galat::runtime(
                        format!(
                            "Indeks {} di luar jangkauan teks (panjang: {})",
                            idx,
                            chars.len()
                        ),
                        span,
                    ))
                }
            }
            Value::Map(m) => {
                let key = match index {
                    Value::String(s) => s.clone(),
                    other => other.to_string_repr(),
                };
                let map = m.borrow();
                if let Some(Value::String(union_name)) = map.get("_tipe_c_union") {
                    if let Some(Value::Array(bytes_arr)) = map.get("_bytes") {
                        let field_ctype = crate::sys::c_abi::get_c_type_registry()
                            .lock()
                            .ok()
                            .and_then(|reg| reg.unions.get(union_name).and_then(|u| u.fields.get(&key).cloned()));
                        if let Some(ct) = field_ctype {
                            let raw: Vec<u8> = bytes_arr
                                .borrow()
                                .iter()
                                .map(|v| match v {
                                    Value::Number(n) => (*n as i64) as u8,
                                    _ => 0,
                                })
                                .collect();
                            return Ok(crate::sys::c_abi::read_bytes_as_ctype(&raw, 0, &ct));
                        }
                    }
                }
                Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
            }
            Value::Instance(inst) => {
                let key = match index {
                    Value::String(s) => s.clone(),
                    other => other.to_string_repr(),
                };
                let instance = inst.borrow();
                if let Some(val) = instance.fields.get(&key) {
                    return Ok(val.clone());
                }
                if let Some(method) = instance.struct_def.methods.get(&key) {
                    // Return a bound function with 'ini' bound
                    let method_fn = WidyaFunction {
                        name: Some(format!("{}.{}", instance.struct_def.name, method.name)),
                        params: method.params.clone(),
                        body: method.body.clone(),
                        closure: {
                            let method_env = Rc::new(RefCell::new(Environment::with_parent(
                                instance.struct_def.closure.clone(),
                            )));
                            method_env
                                .borrow_mut()
                                .define("ini".to_string(), Value::Instance(inst.clone()), false);
                            method_env
                        },
                    };
                    return Ok(Value::Function(Rc::new(method_fn)));
                }
                // Check promoted fields & methods on embedded struct instances (Go Struct Embedding)
                for fval in instance.fields.values() {
                    if let Value::Instance(sub_inst) = fval {
                        let sub_borrow = sub_inst.borrow();
                        if let Some(promoted_val) = sub_borrow.fields.get(&key) {
                            return Ok(promoted_val.clone());
                        }
                        if let Some(method) = sub_borrow.struct_def.methods.get(&key) {
                            let method_fn = WidyaFunction {
                                name: Some(format!("{}.{}", sub_borrow.struct_def.name, method.name)),
                                params: method.params.clone(),
                                body: method.body.clone(),
                                closure: {
                                    let method_env = Rc::new(RefCell::new(Environment::with_parent(
                                        sub_borrow.struct_def.closure.clone(),
                                    )));
                                    method_env
                                        .borrow_mut()
                                        .define("ini".to_string(), Value::Instance(sub_inst.clone()), false);
                                    method_env
                                },
                            };
                            return Ok(Value::Function(Rc::new(method_fn)));
                        }
                    }
                }
                Ok(Value::Nil)
            }
            other => Err(Galat::runtime(
                format!(
                    "Tipe '{}' tidak mendukung pengaksesan dengan indeks [] atau .",
                    other.type_name()
                ),
                span,
            )),
        }
    }

    fn assign_to_index(
        &mut self,
        target_val: Value,
        index_val: Value,
        rhs_val: Value,
        op: &AssignOp,
        span: &Span,
    ) -> Result<Value, Galat> {
        match target_val {
            Value::Array(a) => {
                let mut items = a.borrow_mut();
                let idx = match index_val {
                    Value::Number(n) => n as i64,
                    _ => return Err(Galat::runtime("Indeks daftar harus berupa angka", span)),
                };
                let actual_idx = if idx < 0 {
                    (items.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                if actual_idx < items.len() {
                    let final_val = match op {
                        AssignOp::Assign => rhs_val,
                        _ => {
                            let curr = items[actual_idx].clone();
                            self.apply_compound_assign(curr, rhs_val, op, span)?
                        }
                    };
                    items[actual_idx] = final_val.clone();
                    Ok(final_val)
                } else {
                    Err(Galat::runtime(
                        format!(
                            "Indeks {} di luar batas daftar (panjang: {})",
                            idx,
                            items.len()
                        ),
                        span,
                    ))
                }
            }
            Value::Map(m) => {
                let key = match index_val {
                    Value::String(s) => s,
                    other => other.to_string_repr(),
                };
                let mut map = m.borrow_mut();
                let final_val = match op {
                    AssignOp::Assign => rhs_val,
                    _ => {
                        let curr = map.get(&key).cloned().unwrap_or(Value::Nil);
                        self.apply_compound_assign(curr, rhs_val, op, span)?
                    }
                };
                if let Some(Value::String(union_name)) = map.get("_tipe_c_union").cloned() {
                    if let Some(Value::Array(bytes_arr)) = map.get("_bytes").cloned() {
                        let field_ctype = crate::sys::c_abi::get_c_type_registry()
                            .lock()
                            .ok()
                            .and_then(|reg| reg.unions.get(&union_name).and_then(|u| u.fields.get(&key).cloned()));
                        if let Some(ct) = field_ctype {
                            let mut raw: Vec<u8> = bytes_arr
                                .borrow()
                                .iter()
                                .map(|v| match v {
                                    Value::Number(n) => (*n as i64) as u8,
                                    _ => 0,
                                })
                                .collect();
                            crate::sys::c_abi::write_ctype_to_bytes(&mut raw, 0, &ct, &final_val);
                            let mut b_mut = bytes_arr.borrow_mut();
                            for (i, byte) in raw.into_iter().enumerate() {
                                if i < b_mut.len() {
                                    b_mut[i] = Value::Number(byte as f64);
                                }
                            }
                            return Ok(final_val);
                        }
                    }
                }
                map.insert(key, final_val.clone());
                Ok(final_val)
            }
            Value::Instance(inst) => {
                let key = match index_val {
                    Value::String(s) => s,
                    other => other.to_string_repr(),
                };
                let mut instance = inst.borrow_mut();
                let mut final_val = match op {
                    AssignOp::Assign => rhs_val,
                    _ => {
                        let curr = instance.fields.get(&key).cloned().unwrap_or(Value::Nil);
                        self.apply_compound_assign(curr, rhs_val, op, span)?
                    }
                };
                if let Value::Number(n) = final_val {
                    if let Ok(reg) = crate::sys::c_abi::get_c_type_registry().lock() {
                        if let Some(s_layout) = reg.structs.get(&instance.struct_def.name) {
                            if let Some(f_layout) = s_layout.field_map.get(&key) {
                                let mut casted = f_layout.ctype.cast_value(n);
                                if let Some(bits) = f_layout.bit_width {
                                    let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                                    casted = ((casted as i64 as u64) & mask) as f64;
                                }
                                final_val = Value::Number(casted);
                            }
                        }
                    }
                }
                if !instance.fields.contains_key(&key) && !instance.struct_def.fields.contains(&key) {
                    for fval in instance.fields.values() {
                        if let Value::Instance(sub_inst) = fval {
                            let has_field = sub_inst.borrow().fields.contains_key(&key)
                                || sub_inst.borrow().struct_def.fields.contains(&key);
                            if has_field {
                                sub_inst.borrow_mut().fields.insert(key, final_val.clone());
                                return Ok(final_val);
                            }
                        }
                    }
                }
                instance.fields.insert(key, final_val.clone());
                Ok(final_val)
            }
            other => Err(Galat::runtime(
                format!(
                    "Tidak dapat mengubah elemen pada tipe '{}'",
                    other.type_name()
                ),
                span,
            )),
        }
    }

    fn apply_compound_assign(
        &self,
        left: Value,
        right: Value,
        op: &AssignOp,
        span: &Span,
    ) -> Result<Value, Galat> {
        match op {
            AssignOp::PlusAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), _) => {
                    Ok(Value::String(format!("{}{}", a, right.to_string_repr())))
                }
                (Value::Array(a), Value::Array(b)) => {
                    let mut combined = a.borrow().clone();
                    combined.extend(b.borrow().clone());
                    Ok(Value::Array(Rc::new(RefCell::new(combined))))
                }
                _ => Err(Galat::runtime(
                    "Operasi '+=' memerlukan angka, teks, atau daftar",
                    span,
                )),
            },
            AssignOp::MinusAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(Galat::runtime("Operasi '-=' memerlukan angka", span)),
            },
            AssignOp::StarAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(Galat::runtime("Operasi '*=' memerlukan angka", span)),
            },
            AssignOp::SlashAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(Galat::runtime("Pembagian dengan nol pada '/='", span))
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                _ => Err(Galat::runtime("Operasi '/=' memerlukan angka", span)),
            },
            AssignOp::Assign => unreachable!(),
        }
    }

    fn split_pos_and_kwargs(args: &[Value]) -> (Vec<Value>, HashMap<String, Value>) {
        let mut pos_args = Vec::new();
        let mut kwargs = HashMap::new();
        for arg in args {
            let mut is_kwarg = false;
            if let Value::Map(m) = arg {
                let borrow = m.borrow();
                if let Some(Value::Bool(true)) = borrow.get("_widya_kwarg") {
                    if let (Some(Value::String(k)), Some(v)) = (borrow.get("kunci"), borrow.get("nilai")) {
                        kwargs.insert(k.clone(), v.clone());
                        is_kwarg = true;
                    } else {
                        for (k, v) in borrow.iter() {
                            if k != "_widya_kwarg" {
                                kwargs.insert(k.clone(), v.clone());
                                is_kwarg = true;
                            }
                        }
                    }
                }
            }
            if !is_kwarg {
                pos_args.push(arg.clone());
            }
        }
        (pos_args, kwargs)
    }

    fn bind_params_into_env(
        params: &[String],
        pos_args: &[Value],
        kwargs: &HashMap<String, Value>,
        env: &Rc<RefCell<Environment>>,
        fn_label: &str,
        span: &Span,
    ) -> Result<(), Galat> {
        let variadic_pos = params.iter().position(|p| p.starts_with('*'));
        if variadic_pos.is_none() && pos_args.len() > params.len() {
            return Err(Galat::runtime(
                format!(
                    "Fungsi '{}' mengharapkan paling banyak {} argumen, tetapi menerima {}",
                    fn_label,
                    params.len(),
                    pos_args.len()
                ),
                span,
            ));
        }

        let mut env_mut = env.borrow_mut();
        if let Some(var_idx) = variadic_pos {
            for (i, param) in params.iter().enumerate() {
                if i < var_idx {
                    let val = pos_args.get(i).cloned().unwrap_or(Value::Nil);
                    env_mut.define(param.clone(), val, false);
                } else if i == var_idx {
                    let var_name = param.trim_start_matches('*').to_string();
                    let rest: Vec<Value> = if pos_args.len() > var_idx {
                        pos_args[var_idx..].to_vec()
                    } else {
                        Vec::new()
                    };
                    env_mut.define(
                        var_name,
                        Value::Array(Rc::new(RefCell::new(rest))),
                        false,
                    );
                } else {
                    let clean_name = param.trim_start_matches('*').to_string();
                    env_mut.define(clean_name, Value::Nil, false);
                }
            }
        } else {
            for (i, param) in params.iter().enumerate() {
                let arg = pos_args.get(i).cloned().unwrap_or(Value::Nil);
                env_mut.define(param.clone(), arg, false);
            }
        }

        for (k, v) in kwargs {
            env_mut.define(k.clone(), v.clone(), false);
        }

        Ok(())
    }

    fn call_method(
        &mut self,
        instance: Rc<RefCell<WidyaInstance>>,
        method: &StructMethod,
        args: &[Value],
        span: &Span,
    ) -> Result<Value, Galat> {
        let (pos_args, kwargs) = Self::split_pos_and_kwargs(args);
        let closure_env = instance.borrow().struct_def.closure.clone();
        let method_env = Rc::new(RefCell::new(Environment::with_parent(closure_env)));

        // Bind 'ini' to the instance
        method_env
            .borrow_mut()
            .define("ini".to_string(), Value::Instance(instance), false);

        Self::bind_params_into_env(
            &method.params,
            &pos_args,
            &kwargs,
            &method_env,
            &method.name,
            span,
        )?;

        let previous = self.environment.clone();
        self.environment = method_env;
        self.defer_stack.push(Vec::new());

        let mut return_val = Value::Nil;
        let mut exec_err: Option<Galat> = None;
        for statement in &method.body {
            match self.execute(statement) {
                Ok(_) => {}
                Err(Galat::Kembalikan(val)) => {
                    return_val = val;
                    break;
                }
                Err(e) => {
                    exec_err = Some(e);
                    break;
                }
            }
        }

        let deferred = self.defer_stack.pop().unwrap_or_default();
        let prev_panic = if let Some(ref e) = exec_err {
            self.active_panic.replace(Self::extract_galat_msg(e))
        } else {
            self.active_panic.clone()
        };
        for def_fn in deferred.into_iter().rev() {
            if let Ok(v) = self.call_value(def_fn, &[], span) {
                if v != Value::Nil && return_val == Value::Nil {
                    return_val = v;
                }
            }
        }

        self.environment = previous;
        if let Some(e) = exec_err {
            if self.active_panic.is_none() {
                self.active_panic = prev_panic;
                Ok(return_val)
            } else {
                self.active_panic = prev_panic;
                Err(e)
            }
        } else {
            Ok(return_val)
        }
    }

    pub fn call_value(
        &mut self,
        callee: Value,
        args: &[Value],
        span: &Span,
    ) -> Result<Value, Galat> {
        let (pos_args, kwargs) = Self::split_pos_and_kwargs(args);

        match callee {
            Value::StructDef(struct_def) => {
                let mut field_values = HashMap::new();

                // Bind positional arguments to fields if matching
                for (i, field_name) in struct_def.fields.iter().enumerate() {
                    let raw_val = if let Some(arg) = pos_args.get(i) {
                        arg.clone()
                    } else if let Some(kw_val) = kwargs.get(field_name) {
                        kw_val.clone()
                    } else {
                        // Go Struct Embedding: auto-initialize embedded struct if field_name matches a StructDef in scope
                        let embedded_def_opt = self
                            .environment
                            .borrow()
                            .get(field_name, span)
                            .ok()
                            .and_then(|v| if let Value::StructDef(sd) = v { Some(sd) } else { None });
                        if let Some(sub_def) = embedded_def_opt {
                            if sub_def.name != struct_def.name {
                                self.call_value(Value::StructDef(sub_def), &[], span)
                                    .unwrap_or(Value::Nil)
                            } else {
                                Value::Nil
                            }
                        } else {
                            Value::Nil
                        }
                    };
                    let coerced = if let Value::Number(n) = raw_val {
                        if let Ok(reg) = crate::sys::c_abi::get_c_type_registry().lock() {
                            if let Some(s_layout) = reg.structs.get(&struct_def.name) {
                                if let Some(f_layout) = s_layout.field_map.get(field_name) {
                                    let mut casted = f_layout.ctype.cast_value(n);
                                    if let Some(bits) = f_layout.bit_width {
                                        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                                        casted = ((casted as i64 as u64) & mask) as f64;
                                    }
                                    Value::Number(casted)
                                } else {
                                    Value::Number(n)
                                }
                            } else {
                                Value::Number(n)
                            }
                        } else {
                            Value::Number(n)
                        }
                    } else {
                        raw_val
                    };
                    field_values.insert(field_name.clone(), coerced);
                }

                let instance = Rc::new(RefCell::new(WidyaInstance {
                    struct_def: struct_def.clone(),
                    fields: field_values,
                }));

                // Check for constructor method: 'inisialisasi' or 'init' or struct_name
                let constructor = struct_def
                    .methods
                    .get("inisialisasi")
                    .or_else(|| struct_def.methods.get("init"))
                    .or_else(|| struct_def.methods.get(&struct_def.name))
                    .cloned();

                if let Some(init_method) = constructor {
                    self.call_method(instance.clone(), &init_method, args, span)?;
                }

                Ok(Value::Instance(instance))
            }
            Value::Builtin(b) => {
                // Intercept higher-order functions & context manager protocol hooks that may invoke closures/methods
                match b.name.as_str() {
                    "peta" | "map" | "petakan" => {
                        if pos_args.len() != 2 {
                            return Err(Galat::runtime(
                                "Fungsi 'peta' mengharapkan 2 argumen (daftar, fungsi)",
                                span,
                            ));
                        }
                        if let Value::Array(arr) = &pos_args[0] {
                            let items = arr.borrow().clone();
                            let func = pos_args[1].clone();
                            let mut mapped = Vec::with_capacity(items.len());
                            for item in items {
                                mapped.push(self.call_value(func.clone(), &[item], span)?);
                            }
                            return Ok(Value::Array(Rc::new(RefCell::new(mapped))));
                        } else if let Value::Array(arr) = &pos_args[1] {
                            // Also support Python order: map(fn, iterable)
                            let items = arr.borrow().clone();
                            let func = pos_args[0].clone();
                            let mut mapped = Vec::with_capacity(items.len());
                            for item in items {
                                mapped.push(self.call_value(func.clone(), &[item], span)?);
                            }
                            return Ok(Value::Array(Rc::new(RefCell::new(mapped))));
                        }
                    }
                    "saring" | "filter" => {
                        if pos_args.len() != 2 {
                            return Err(Galat::runtime(
                                "Fungsi 'saring' mengharapkan 2 argumen (daftar, fungsi)",
                                span,
                            ));
                        }
                        if let Value::Array(arr) = &pos_args[0] {
                            let items = arr.borrow().clone();
                            let func = pos_args[1].clone();
                            let mut filtered = Vec::new();
                            for item in items {
                                if self.call_value(func.clone(), &[item.clone()], span)?.is_truthy() {
                                    filtered.push(item);
                                }
                            }
                            return Ok(Value::Array(Rc::new(RefCell::new(filtered))));
                        } else if let Value::Array(arr) = &pos_args[1] {
                            // Also support Python order: filter(fn, iterable)
                            let items = arr.borrow().clone();
                            let func = pos_args[0].clone();
                            let mut filtered = Vec::new();
                            for item in items {
                                if self.call_value(func.clone(), &[item.clone()], span)?.is_truthy() {
                                    filtered.push(item);
                                }
                            }
                            return Ok(Value::Array(Rc::new(RefCell::new(filtered))));
                        }
                    }
                    "_konteks_masuk" => {
                        if let Some(Value::Instance(inst)) = pos_args.first() {
                            let enter_opt = {
                                let b_inst = inst.borrow();
                                b_inst
                                    .struct_def
                                    .methods
                                    .get("masuk")
                                    .or_else(|| b_inst.struct_def.methods.get("__enter__"))
                                    .or_else(|| b_inst.struct_def.methods.get("buka"))
                                    .cloned()
                            };
                            if let Some(enter_method) = enter_opt {
                                let res = self.call_method(inst.clone(), &enter_method, &[], span)?;
                                if res == Value::Nil {
                                    return Ok(Value::Instance(inst.clone()));
                                }
                                return Ok(res);
                            }
                            return Ok(Value::Instance(inst.clone()));
                        }
                    }
                    "_konteks_keluar" => {
                        if let Some(Value::Instance(inst)) = pos_args.first() {
                            let exit_opt = {
                                let b_inst = inst.borrow();
                                b_inst
                                    .struct_def
                                    .methods
                                    .get("keluar")
                                    .or_else(|| b_inst.struct_def.methods.get("__exit__"))
                                    .or_else(|| b_inst.struct_def.methods.get("tutup"))
                                    .cloned()
                            };
                            if let Some(exit_method) = exit_opt {
                                return self.call_method(inst.clone(), &exit_method, &[], span);
                            }
                            return Ok(Value::Nil);
                        }
                    }
                    "impor_header_c" | "urai_header_c" => {
                        if let Some(arg0) = pos_args.first() {
                            let input = arg0.to_string_repr();
                            let header_src = if input.ends_with(".h") && std::path::Path::new(&input).exists() {
                                std::fs::read_to_string(&input).map_err(|e| {
                                    Galat::runtime(format!("Gagal membaca header C '{}': {}", input, e), span)
                                })?
                            } else {
                                input
                            };
                            let parsed = crate::ffi::CHeaderParser::parse_header_str(&header_src);
                            for (k, v) in &parsed.defines {
                                self.environment.borrow_mut().define(k.clone(), v.clone(), true);
                            }
                            for (vname, vval) in &parsed.enums {
                                self.environment
                                    .borrow_mut()
                                    .define(vname.clone(), Value::Number(*vval as f64), true);
                            }
                            for sig in &parsed.functions {
                                let c_fn = Value::Builtin(Rc::new(crate::value::BuiltinFunction {
                                    name: format!("__c_ffi::{}", sig.name),
                                    arity: if sig.is_variadic { None } else { Some(sig.param_types.len()) },
                                    func: |_args, _span| Ok(Value::Nil),
                                }));
                                self.environment.borrow_mut().define(sig.name.clone(), c_fn, false);
                            }
                            return Ok(parsed.to_widya_map());
                        }
                    }
                    other_name if other_name.starts_with("__c_ffi::") => {
                        let c_fn_name = &other_name["__c_ffi::".len()..];
                        let mut mgr = crate::ffi::get_ffi_manager().lock().unwrap();
                        return mgr.call_foreign_function(c_fn_name, &pos_args);
                    }
                    other_name if other_name.starts_with("__batal_konteks::") => {
                        if let Ok(ctx_id) = other_name["__batal_konteks::".len()..].parse::<u64>() {
                            crate::stdlib::concurrency::cancel_context_by_id(ctx_id, "context canceled");
                        }
                        return Ok(Value::Nil);
                    }
                    "_daftar_tunda" => {
                        if let Some(fn_val) = pos_args.first() {
                            if let Some(frame) = self.defer_stack.last_mut() {
                                frame.push(fn_val.clone());
                            }
                        }
                        return Ok(Value::Nil);
                    }
                    "pulihkan" | "recover" => {
                        if let Some(msg) = self.active_panic.take() {
                            return Ok(Value::String(msg));
                        }
                        return Ok(Value::Nil);
                    }
                    "memenuhi_sifat" | "implements" | "apakah_memenuhi" => {
                        if pos_args.len() < 2 {
                            return Ok(Value::Bool(false));
                        }
                        let satisfied = self.check_structural_interface(&pos_args[0], &pos_args[1]);
                        return Ok(Value::Bool(satisfied));
                    }
                    "pastikan_sifat" | "type_assert" => {
                        if pos_args.len() < 2 {
                            return Ok(Value::Array(Rc::new(RefCell::new(vec![
                                Value::Nil,
                                Value::Bool(false),
                            ]))));
                        }
                        let satisfied = self.check_structural_interface(&pos_args[0], &pos_args[1]);
                        let out_val = if satisfied {
                            pos_args[0].clone()
                        } else {
                            Value::Nil
                        };
                        return Ok(Value::Array(Rc::new(RefCell::new(vec![
                            out_val,
                            Value::Bool(satisfied),
                        ]))));
                    }
                    "_eksekusi_pilih_saluran" => {
                        return self.execute_select_channels(&pos_args, span);
                    }
                    "wg_tunggu" | "gabung_utas" | "saluran_terima" | "saluran_terima_ok" => {
                        let effective_args = if kwargs.is_empty() { args } else { &pos_args };
                        let res = (b.func)(effective_args, span)?;
                        self.sync_goroutine_dirty_vars();
                        return Ok(res);
                    }
                    "mutex_kunci" => {
                        let effective_args = if kwargs.is_empty() { args } else { &pos_args };
                        let res = (b.func)(effective_args, span)?;
                        self.sync_goroutine_dirty_vars();
                        return Ok(res);
                    }
                    "mutex_buka" => {
                        self.flush_goroutine_dirty_vars();
                        let effective_args = if kwargs.is_empty() { args } else { &pos_args };
                        return (b.func)(effective_args, span);
                    }
                    _ => {}
                }

                let effective_args = if kwargs.is_empty() { args } else { &pos_args };
                if let Some(expected_arity) = b.arity {
                    if effective_args.len() != expected_arity {
                        return Err(Galat::runtime(
                            format!(
                                "Fungsi bawaan '{}' mengharapkan {} argumen, tetapi menerima {}",
                                b.name,
                                expected_arity,
                                effective_args.len()
                            ),
                            span,
                        ));
                    }
                }
                (b.func)(effective_args, span)
            }
            Value::Function(func) => {
                // Check if this is a synthetic enum constructor
                if let Some(ref fn_name) = func.name {
                    if fn_name.contains("::") && func.body.is_empty() {
                        let parts: Vec<&str> = fn_name.split("::").collect();
                        if parts.len() == 2 {
                            return Ok(Value::EnumVariant(Rc::new(crate::value::WidyaEnumVariant {
                                enum_name: parts[0].to_string(),
                                variant_name: parts[1].to_string(),
                                fields: pos_args,
                            })));
                        }
                    }
                }

                let fn_name = func.name.as_deref().unwrap_or("anonim");
                let call_env = Rc::new(RefCell::new(Environment::with_parent(func.closure.clone())));
                Self::bind_params_into_env(
                    &func.params,
                    &pos_args,
                    &kwargs,
                    &call_env,
                    fn_name,
                    span,
                )?;

                let previous = self.environment.clone();
                self.environment = call_env;
                self.defer_stack.push(Vec::new());

                let mut return_val = Value::Nil;
                let mut exec_err: Option<Galat> = None;
                for statement in &func.body {
                    match self.execute(statement) {
                        Ok(_) => {}
                        Err(Galat::Kembalikan(val)) => {
                            return_val = val;
                            break;
                        }
                        Err(e) => {
                            exec_err = Some(e);
                            break;
                        }
                    }
                }

                let deferred = self.defer_stack.pop().unwrap_or_default();
                let prev_panic = if let Some(ref e) = exec_err {
                    self.active_panic.replace(Self::extract_galat_msg(e))
                } else {
                    self.active_panic.clone()
                };
                for def_fn in deferred.into_iter().rev() {
                    if let Ok(v) = self.call_value(def_fn, &[], span) {
                        if v != Value::Nil && return_val == Value::Nil {
                            return_val = v;
                        }
                    }
                }

                self.environment = previous;
                if let Some(e) = exec_err {
                    if self.active_panic.is_none() {
                        self.active_panic = prev_panic;
                        Ok(return_val)
                    } else {
                        self.active_panic = prev_panic;
                        Err(e)
                    }
                } else {
                    Ok(return_val)
                }
            }
            Value::Map(m) => {
                let u_name_opt = m.borrow().get("_definisi_c_union").and_then(|v| {
                    if let Value::String(s) = v { Some(s.clone()) } else { None }
                });
                if let Some(u_name) = u_name_opt {
                    let inst = crate::sys::c_abi::create_c_union_instance(&u_name)?;
                    for (k, v) in &kwargs {
                        let _ = self.assign_to_index(inst.clone(), Value::String(k.clone()), v.clone(), &AssignOp::Assign, span);
                    }
                    return Ok(inst);
                }
                Err(Galat::runtime("Tipe 'kamus' tidak dapat dipanggil sebagai fungsi", span))
            }
            other => Err(Galat::runtime(
                format!("Tipe '{}' tidak dapat dipanggil sebagai fungsi", other.type_name()),
                span,
            )),
        }
    }

    fn call_builtin_type_method(
        &mut self,
        target: &Value,
        method_name: &str,
        args: &[Value],
        span: &Span,
    ) -> Result<Option<Value>, Galat> {
        let (pos_args, _kwargs) = Self::split_pos_and_kwargs(args);
        match target {
            Value::Array(arr) => match method_name {
                "tambah" | "append" | "push" => {
                    let mut items = arr.borrow_mut();
                    for a in &pos_args {
                        items.push(a.clone());
                    }
                    Ok(Some(Value::Array(arr.clone())))
                }
                "pop" | "ambil_akhir" | "hapus_akhir" => {
                    let mut items = arr.borrow_mut();
                    if let Some(Value::Number(idx_num)) = pos_args.first() {
                        let idx = *idx_num as i64;
                        let actual = if idx < 0 {
                            (items.len() as i64 + idx) as usize
                        } else {
                            idx as usize
                        };
                        if actual < items.len() {
                            Ok(Some(items.remove(actual)))
                        } else {
                            Ok(Some(Value::Nil))
                        }
                    } else {
                        Ok(Some(items.pop().unwrap_or(Value::Nil)))
                    }
                }
                "extend" | "perluas" | "gabungkan" => {
                    if let Some(Value::Array(other)) = pos_args.first() {
                        let other_items = other.borrow().clone();
                        arr.borrow_mut().extend(other_items);
                    }
                    Ok(Some(Value::Array(arr.clone())))
                }
                "insert" | "sisipkan" => {
                    if pos_args.len() >= 2 {
                        if let Value::Number(idx_num) = pos_args[0] {
                            let mut items = arr.borrow_mut();
                            let idx = (idx_num as isize).max(0) as usize;
                            let clamped = idx.min(items.len());
                            items.insert(clamped, pos_args[1].clone());
                        }
                    }
                    Ok(Some(Value::Array(arr.clone())))
                }
                "remove" | "hapus_nilai" => {
                    if let Some(target_val) = pos_args.first() {
                        let mut items = arr.borrow_mut();
                        if let Some(pos) = items.iter().position(|x| x == target_val) {
                            items.remove(pos);
                        }
                    }
                    Ok(Some(Value::Array(arr.clone())))
                }
                "clear" | "bersihkan" => {
                    arr.borrow_mut().clear();
                    Ok(Some(Value::Array(arr.clone())))
                }
                "sort" | "urutkan" => {
                    arr.borrow_mut().sort_by(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        _ => a.to_string_repr().cmp(&b.to_string_repr()),
                    });
                    Ok(Some(Value::Array(arr.clone())))
                }
                "reverse" | "balikkan" | "balik" => {
                    arr.borrow_mut().reverse();
                    Ok(Some(Value::Array(arr.clone())))
                }
                "copy" | "salin" | "duplikat" => {
                    let cloned = arr.borrow().clone();
                    Ok(Some(Value::Array(Rc::new(RefCell::new(cloned)))))
                }
                "count" | "hitung" => {
                    if let Some(needle) = pos_args.first() {
                        let c = arr.borrow().iter().filter(|x| *x == needle).count();
                        Ok(Some(Value::Number(c as f64)))
                    } else {
                        Ok(Some(Value::Number(arr.borrow().len() as f64)))
                    }
                }
                "index" | "indeks" | "cari" => {
                    if let Some(needle) = pos_args.first() {
                        let idx = arr
                            .borrow()
                            .iter()
                            .position(|x| x == needle)
                            .map(|i| i as f64)
                            .unwrap_or(-1.0);
                        Ok(Some(Value::Number(idx)))
                    } else {
                        Ok(Some(Value::Number(-1.0)))
                    }
                }
                "contains" | "memuat" | "mengandung" => {
                    if let Some(needle) = pos_args.first() {
                        Ok(Some(Value::Bool(arr.borrow().contains(needle))))
                    } else {
                        Ok(Some(Value::Bool(false)))
                    }
                }
                "join" | "gabung" => {
                    let sep = pos_args
                        .first()
                        .map(|v| v.to_string_repr())
                        .unwrap_or_default();
                    let joined: Vec<String> =
                        arr.borrow().iter().map(|v| v.to_string_repr()).collect();
                    Ok(Some(Value::String(joined.join(&sep))))
                }
                "len" | "panjang" => Ok(Some(Value::Number(arr.borrow().len() as f64))),
                "sum" | "jumlah" => {
                    let total: f64 = arr
                        .borrow()
                        .iter()
                        .filter_map(|v| match v {
                            Value::Number(n) => Some(*n),
                            _ => None,
                        })
                        .sum();
                    Ok(Some(Value::Number(total)))
                }
                "map" | "petakan" | "peta" => {
                    if let Some(func) = pos_args.first() {
                        let items = arr.borrow().clone();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            out.push(self.call_value(func.clone(), &[item], span)?);
                        }
                        Ok(Some(Value::Array(Rc::new(RefCell::new(out)))))
                    } else {
                        Err(Galat::runtime("Metode 'petakan'/'map' memerlukan argumen fungsi", span))
                    }
                }
                "filter" | "saring" => {
                    if let Some(func) = pos_args.first() {
                        let items = arr.borrow().clone();
                        let mut out = Vec::new();
                        for item in items {
                            if self.call_value(func.clone(), &[item.clone()], span)?.is_truthy() {
                                out.push(item);
                            }
                        }
                        Ok(Some(Value::Array(Rc::new(RefCell::new(out)))))
                    } else {
                        Err(Galat::runtime("Metode 'saring'/'filter' memerlukan argumen fungsi", span))
                    }
                }
                "reduce" | "lipat" => {
                    if let Some(func) = pos_args.first() {
                        let items = arr.borrow().clone();
                        let mut iter = items.into_iter();
                        let mut acc = if let Some(init) = pos_args.get(1) {
                            init.clone()
                        } else {
                            iter.next().unwrap_or(Value::Nil)
                        };
                        for item in iter {
                            acc = self.call_value(func.clone(), &[acc, item], span)?;
                        }
                        Ok(Some(acc))
                    } else {
                        Err(Galat::runtime("Metode 'lipat'/'reduce' memerlukan argumen fungsi", span))
                    }
                }
                "any" | "ada_yang" => {
                    let items = arr.borrow().clone();
                    if let Some(func) = pos_args.first() {
                        for item in items {
                            if self.call_value(func.clone(), &[item], span)?.is_truthy() {
                                return Ok(Some(Value::Bool(true)));
                            }
                        }
                        Ok(Some(Value::Bool(false)))
                    } else {
                        Ok(Some(Value::Bool(items.iter().any(|x| x.is_truthy()))))
                    }
                }
                "all" | "semua" => {
                    let items = arr.borrow().clone();
                    if let Some(func) = pos_args.first() {
                        for item in items {
                            if !self.call_value(func.clone(), &[item], span)?.is_truthy() {
                                return Ok(Some(Value::Bool(false)));
                            }
                        }
                        Ok(Some(Value::Bool(true)))
                    } else {
                        Ok(Some(Value::Bool(items.iter().all(|x| x.is_truthy()))))
                    }
                }
                _ => Ok(None),
            },
            Value::String(s) => match method_name {
                "upper" | "besar" | "huruf_besar" => Ok(Some(Value::String(s.to_uppercase()))),
                "lower" | "kecil" | "huruf_kecil" => Ok(Some(Value::String(s.to_lowercase()))),
                "strip" | "trim" | "hapus_spasi" | "pangkas" => {
                    if let Some(Value::String(chars_to_trim)) = pos_args.first() {
                        let pat: Vec<char> = chars_to_trim.chars().collect();
                        Ok(Some(Value::String(s.trim_matches(pat.as_slice()).to_string())))
                    } else {
                        Ok(Some(Value::String(s.trim().to_string())))
                    }
                }
                "lstrip" | "pangkas_kiri" => Ok(Some(Value::String(s.trim_start().to_string()))),
                "rstrip" | "pangkas_kanan" => Ok(Some(Value::String(s.trim_end().to_string()))),
                "split" | "pisah" => {
                    let parts: Vec<Value> = if let Some(sep_val) = pos_args.first() {
                        let sep = sep_val.to_string_repr();
                        s.split(&sep).map(|p| Value::String(p.to_string())).collect()
                    } else {
                        s.split_whitespace()
                            .map(|p| Value::String(p.to_string()))
                            .collect()
                    };
                    Ok(Some(Value::Array(Rc::new(RefCell::new(parts)))))
                }
                "replace" | "ganti" => {
                    if pos_args.len() >= 2 {
                        let from = pos_args[0].to_string_repr();
                        let to = pos_args[1].to_string_repr();
                        if let Some(Value::Number(count)) = pos_args.get(2) {
                            Ok(Some(Value::String(s.replacen(&from, &to, *count as usize))))
                        } else {
                            Ok(Some(Value::String(s.replace(&from, &to))))
                        }
                    } else {
                        Err(Galat::runtime("Metode 'ganti'/'replace' memerlukan 2 argumen", span))
                    }
                }
                "startswith" | "diawali" | "mulai_dengan" => {
                    let prefix = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    Ok(Some(Value::Bool(s.starts_with(&prefix))))
                }
                "endswith" | "diakhiri" | "akhir_dengan" => {
                    let suffix = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    Ok(Some(Value::Bool(s.ends_with(&suffix))))
                }
                "find" | "cari" | "index" | "indeks" => {
                    let sub = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    let pos = s.find(&sub).map(|i| i as f64).unwrap_or(-1.0);
                    Ok(Some(Value::Number(pos)))
                }
                "count" | "hitung" => {
                    let sub = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    Ok(Some(Value::Number(s.matches(&sub).count() as f64)))
                }
                "contains" | "mengandung" | "memuat" => {
                    let sub = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    Ok(Some(Value::Bool(s.contains(&sub))))
                }
                "join" | "gabung" => {
                    if let Some(Value::Array(arr)) = pos_args.first() {
                        let joined: Vec<String> =
                            arr.borrow().iter().map(|v| v.to_string_repr()).collect();
                        Ok(Some(Value::String(joined.join(s))))
                    } else {
                        Ok(Some(Value::String(s.clone())))
                    }
                }
                "isdigit" | "apakah_angka" => {
                    Ok(Some(Value::Bool(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))))
                }
                "isalpha" | "apakah_huruf" => {
                    Ok(Some(Value::Bool(!s.is_empty() && s.chars().all(|c| c.is_alphabetic()))))
                }
                "isalnum" | "apakah_alfanumerik" => {
                    Ok(Some(Value::Bool(!s.is_empty() && s.chars().all(|c| c.is_alphanumeric()))))
                }
                "title" | "judul" => {
                    let titled: Vec<String> = s
                        .split_whitespace()
                        .map(|w| {
                            let mut ch = w.chars();
                            match ch.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>()
                                        + &ch.as_str().to_lowercase()
                                }
                            }
                        })
                        .collect();
                    Ok(Some(Value::String(titled.join(" "))))
                }
                "capitalize" | "kapital" => {
                    let mut ch = s.chars();
                    let cap = match ch.next() {
                        None => String::new(),
                        Some(first) => {
                            first.to_uppercase().collect::<String>() + &ch.as_str().to_lowercase()
                        }
                    };
                    Ok(Some(Value::String(cap)))
                }
                "repeat" | "ulangi" => {
                    let n = match pos_args.first() {
                        Some(Value::Number(num)) => (*num as isize).max(0) as usize,
                        _ => 1,
                    };
                    Ok(Some(Value::String(s.repeat(n))))
                }
                "chars" | "karakter" => {
                    let chars: Vec<Value> =
                        s.chars().map(|c| Value::String(c.to_string())).collect();
                    Ok(Some(Value::Array(Rc::new(RefCell::new(chars)))))
                }
                "len" | "panjang" => Ok(Some(Value::Number(s.chars().count() as f64))),
                "format" => {
                    let mut formatted = s.clone();
                    for (idx, val) in pos_args.iter().enumerate() {
                        let repr = val.to_string_repr();
                        let indexed_placeholder = format!("{{{}}}", idx);
                        if formatted.contains(&indexed_placeholder) {
                            formatted = formatted.replace(&indexed_placeholder, &repr);
                        } else if formatted.contains("{}") {
                            formatted = formatted.replacen("{}", &repr, 1);
                        }
                    }
                    Ok(Some(Value::String(formatted)))
                }
                _ => Ok(None),
            },
            Value::Map(m) => {
                let map_tipe = m
                    .borrow()
                    .get("_tipe")
                    .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
                if let Some(ref tipe) = map_tipe {
                    match tipe.as_str() {
                        "Saluran" => match method_name {
                            "kirim" | "send" => {
                                let mut full_args = vec![target.clone()];
                                full_args.extend(pos_args);
                                return crate::stdlib::concurrency::builtin_saluran_kirim(&full_args, span).map(Some);
                            }
                            "terima" | "recv" => {
                                let res = crate::stdlib::concurrency::builtin_saluran_terima(&[target.clone()], span)?;
                                self.sync_goroutine_dirty_vars();
                                return Ok(Some(res));
                            }
                            "tutup" | "close" => {
                                return crate::stdlib::concurrency::builtin_tutup_saluran(&[target.clone()], span).map(Some);
                            }
                            "panjang" | "len" => {
                                let ch_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Saluran", span)?;
                                let ch = crate::stdlib::concurrency::get_channel_arc(ch_id, span)?;
                                return Ok(Some(Value::Number(ch.len() as f64)));
                            }
                            "kapasitas" | "cap" => {
                                let ch_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Saluran", span)?;
                                let ch = crate::stdlib::concurrency::get_channel_arc(ch_id, span)?;
                                return Ok(Some(Value::Number(ch.cap() as f64)));
                            }
                            "apakah_tutup" | "is_closed" => {
                                let ch_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Saluran", span)?;
                                let ch = crate::stdlib::concurrency::get_channel_arc(ch_id, span)?;
                                return Ok(Some(Value::Bool(ch.is_closed())));
                            }
                            _ => {}
                        },
                        "GrupTunggu" => match method_name {
                            "tambah" | "add" => {
                                let mut full_args = vec![target.clone()];
                                full_args.extend(pos_args);
                                return crate::stdlib::concurrency::builtin_wg_tambah(&full_args, span).map(Some);
                            }
                            "selesai" | "done" => {
                                self.flush_goroutine_dirty_vars();
                                return crate::stdlib::concurrency::builtin_wg_selesai(&[target.clone()], span).map(Some);
                            }
                            "tunggu" | "wait" => {
                                let res = crate::stdlib::concurrency::builtin_wg_tunggu(&[target.clone()], span)?;
                                self.sync_goroutine_dirty_vars();
                                return Ok(Some(res));
                            }
                            _ => {}
                        },
                        "KunciMutex" => match method_name {
                            "kunci" | "lock" => {
                                let res = crate::stdlib::concurrency::builtin_mutex_kunci(&[target.clone()], span)?;
                                self.sync_goroutine_dirty_vars();
                                return Ok(Some(res));
                            }
                            "buka_kunci" | "buka" | "unlock" => {
                                self.flush_goroutine_dirty_vars();
                                return crate::stdlib::concurrency::builtin_mutex_buka(&[target.clone()], span).map(Some);
                            }
                            _ => {}
                        },
                        "Konteks" => match method_name {
                            "batalkan" | "batal" | "cancel" => {
                                let ctx_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Konteks", span)?;
                                crate::stdlib::concurrency::cancel_context_by_id(ctx_id, "context canceled");
                                return Ok(Some(Value::Nil));
                            }
                            "apakah_batal" | "is_cancelled" | "dibatalkan" => {
                                let ctx_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Konteks", span)?;
                                return Ok(Some(Value::Bool(crate::stdlib::concurrency::context_is_cancelled(ctx_id))));
                            }
                            "galat" | "err" => {
                                let ctx_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Konteks", span)?;
                                return Ok(Some(crate::stdlib::concurrency::context_err_value(ctx_id)));
                            }
                            "nilai" | "value" => {
                                let ctx_id = crate::stdlib::concurrency::extract_id_from_arg(target, "Konteks", span)?;
                                let key = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                                return Ok(Some(crate::stdlib::concurrency::context_get_value(ctx_id, &key)));
                            }
                            "selesai" | "done" => {
                                let done_ch = m.borrow().get("_saluran_selesai").cloned().unwrap_or(Value::Nil);
                                return Ok(Some(done_ch));
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                match method_name {
                "keys" | "kunci" => {
                    let map = m.borrow();
                    let mut keys: Vec<String> = map.keys().cloned().collect();
                    keys.sort();
                    let vals: Vec<Value> = keys.into_iter().map(Value::String).collect();
                    Ok(Some(Value::Array(Rc::new(RefCell::new(vals)))))
                }
                "values" | "nilai" => {
                    let map = m.borrow();
                    let mut keys: Vec<String> = map.keys().cloned().collect();
                    keys.sort();
                    let vals: Vec<Value> = keys
                        .into_iter()
                        .filter_map(|k| map.get(&k).cloned())
                        .collect();
                    Ok(Some(Value::Array(Rc::new(RefCell::new(vals)))))
                }
                "items" | "pasangan" | "entri" => {
                    let map = m.borrow();
                    let mut keys: Vec<String> = map.keys().cloned().collect();
                    keys.sort();
                    let mut pairs = Vec::with_capacity(keys.len());
                    for k in keys {
                        if let Some(v) = map.get(&k) {
                            pairs.push(Value::Array(Rc::new(RefCell::new(vec![
                                Value::String(k),
                                v.clone(),
                            ]))));
                        }
                    }
                    Ok(Some(Value::Array(Rc::new(RefCell::new(pairs)))))
                }
                "get" | "ambil" => {
                    let key = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    let default_val = pos_args.get(1).cloned().unwrap_or(Value::Nil);
                    let val = m.borrow().get(&key).cloned().unwrap_or(default_val);
                    Ok(Some(val))
                }
                "pop" | "hapus" => {
                    let key = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    let default_val = pos_args.get(1).cloned().unwrap_or(Value::Nil);
                    let val = m.borrow_mut().remove(&key).unwrap_or(default_val);
                    Ok(Some(val))
                }
                "update" | "perbarui" => {
                    if let Some(Value::Map(other)) = pos_args.first() {
                        let entries: Vec<(String, Value)> = other
                            .borrow()
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();
                        let mut borrow = m.borrow_mut();
                        for (k, v) in entries {
                            borrow.insert(k, v);
                        }
                    }
                    Ok(Some(Value::Map(m.clone())))
                }
                "contains" | "memuat" | "ada_kunci" | "has_key" => {
                    let key = pos_args.first().map(|v| v.to_string_repr()).unwrap_or_default();
                    Ok(Some(Value::Bool(m.borrow().contains_key(&key))))
                }
                "clear" | "bersihkan" => {
                    m.borrow_mut().clear();
                    Ok(Some(Value::Map(m.clone())))
                }
                "copy" | "salin" => {
                    let cloned = m.borrow().clone();
                    Ok(Some(Value::Map(Rc::new(RefCell::new(cloned)))))
                }
                "len" | "panjang" => Ok(Some(Value::Number(m.borrow().len() as f64))),
                _ => Ok(None),
                }
            },
            _ => Ok(None),
        }
    }

    fn resolve_builtin_module(&self, raw_path: &str) -> Option<(String, HashMap<String, Value>)> {
        let normalized = raw_path
            .trim()
            .replace("::", ".")
            .replace('/', ".");
        let parts: Vec<&str> = normalized.split('.').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return None;
        }

        let is_std_prefix = parts[0] == "std" || parts[0] == "widya";
        let submod = if is_std_prefix {
            parts.get(1).copied().unwrap_or("std")
        } else if parts.len() == 1 {
            match parts[0] {
                "matematika" | "math" | "teks" | "string" | "koleksi" | "collections" | "io"
                | "waktu" | "time" | "kripto" | "crypto" | "json" | "ai" | "tensor" | "python"
                | "gpu" | "sistem" | "sys" | "os" => parts[0],
                _ => return None,
            }
        } else {
            return None;
        };

        let all_globals = self.globals.borrow().get_all_local();
        let mut ns_map = HashMap::new();

        // Check if std already has a pre-built sub-map in globals
        if let Some(Value::Map(std_root)) = all_globals.get("std") {
            if submod == "std" {
                return Some(("std".to_string(), std_root.borrow().clone()));
            }
            if let Some(Value::Map(sub_rc)) = std_root.borrow().get(submod) {
                let mut map_clone = sub_rc.borrow().clone();
                // Also include any matching globals
                for (k, v) in &all_globals {
                    if Self::symbol_belongs_to_submodule(k, submod) {
                        map_clone.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
                return Some((submod.to_string(), map_clone));
            }
        }

        for (k, v) in &all_globals {
            if submod == "std" || Self::symbol_belongs_to_submodule(k, submod) {
                ns_map.insert(k.clone(), v.clone());
            }
        }

        // Provide standard aliases inside the module map (e.g. math.sqrt, math.pi, json.dumps, etc.)
        match submod {
            "matematika" | "math" => {
                if let Some(v) = all_globals.get("akar") {
                    ns_map.insert("sqrt".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("mutlak") {
                    ns_map.insert("abs".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("bulat_bawah") {
                    ns_map.insert("floor".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("bulat_atas") {
                    ns_map.insert("ceil".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("bulatkan") {
                    ns_map.insert("round".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("PI") {
                    ns_map.insert("pi".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("E") {
                    ns_map.insert("e".to_string(), v.clone());
                }
            }
            "json" => {
                if let Some(v) = all_globals.get("ke_json") {
                    ns_map.insert("dumps".to_string(), v.clone());
                    ns_map.insert("stringify".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("dari_json").or_else(|| all_globals.get("urai_json")) {
                    ns_map.insert("loads".to_string(), v.clone());
                    ns_map.insert("parse".to_string(), v.clone());
                    ns_map.insert("urai_json".to_string(), v.clone());
                }
            }
            "python" => {
                if let Some(v) = all_globals.get("impor_python") {
                    ns_map.insert("import_module".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("eval_python") {
                    ns_map.insert("eval".to_string(), v.clone());
                }
                if let Some(v) = all_globals.get("panggil_python") {
                    ns_map.insert("call".to_string(), v.clone());
                }
            }
            _ => {}
        }

        if ns_map.is_empty() {
            None
        } else {
            Some((submod.to_string(), ns_map))
        }
    }

    fn symbol_belongs_to_submodule(sym: &str, submod: &str) -> bool {
        match submod {
            "matematika" | "math" => matches!(
                sym,
                "PI" | "E"
                    | "akar"
                    | "pangkat"
                    | "mutlak"
                    | "bulatkan"
                    | "bulat_bawah"
                    | "bulat_atas"
                    | "sin"
                    | "cos"
                    | "tan"
                    | "asin"
                    | "acos"
                    | "atan"
                    | "log"
                    | "log10"
                    | "log2"
                    | "eksponen"
                    | "acak"
                    | "acak_antara"
                    | "min"
                    | "maks"
                    | "apakah_nan"
                    | "apakah_tak_hingga"
                    | "kpk"
                    | "fpb"
                    | "faktorial"
                    | "derajat_ke_radian"
                    | "radian_ke_derajat"
                    | "hipotenusa"
            ),
            "teks" | "string" => matches!(
                sym,
                "huruf_besar"
                    | "huruf_kecil"
                    | "pisah"
                    | "gabung"
                    | "ganti"
                    | "hapus_spasi"
                    | "apakah_diawali"
                    | "apakah_diakhiri"
                    | "mengandung"
                    | "cari_indeks"
                    | "potong"
                    | "ulangi_teks"
                    | "balik_teks"
                    | "panjang_karakter"
            ),
            "koleksi" | "collections" => matches!(
                sym,
                "panjang"
                    | "len"
                    | "tambah"
                    | "hapus_akhir"
                    | "hapus_pada"
                    | "sisipkan"
                    | "gabung_daftar"
                    | "balik_daftar"
                    | "balik"
                    | "reversed"
                    | "urutkan"
                    | "urut"
                    | "sorted"
                    | "kunci_kamus"
                    | "nilai_kamus"
                    | "hapus_kunci"
                    | "ada_kunci"
                    | "rentang"
                    | "range"
                    | "enumerasi"
                    | "enumerate"
                    | "pasangkan"
                    | "zip"
                    | "semua"
                    | "all"
                    | "ada_yang"
                    | "any"
                    | "jumlah"
                    | "sum"
                    | "peta"
                    | "map"
                    | "saring"
                    | "filter"
                    | "iris"
                    | "slice"
            ),
            "io" => matches!(
                sym,
                "cetak"
                    | "cetak_baris"
                    | "tulis"
                    | "baca_baris"
                    | "baca_berkas"
                    | "tulis_berkas"
                    | "tambah_berkas"
                    | "apakah_berkas_ada"
                    | "hapus_berkas"
                    | "daftar_direktori"
                    | "buat_direktori"
            ),
            "waktu" | "time" => matches!(
                sym,
                "waktu_sekarang"
                    | "waktu_ms"
                    | "waktu_ns"
                    | "tidur"
                    | "format_waktu"
            ),
            "kripto" | "crypto" => matches!(
                sym,
                "sha256"
                    | "sha512"
                    | "blake3"
                    | "hmac_sha256"
                    | "enkripsi_aes"
                    | "dekripsi_aes"
                    | "base64_encode"
                    | "base64_decode"
                    | "hex_encode"
                    | "hex_decode"
                    | "uuid_v4"
            ),
            "json" => matches!(sym, "ke_json" | "dari_json" | "urai_json" | "format_json"),
            "ai" | "tensor" | "gpu" => {
                sym.starts_with("tensor_")
                    || sym.starts_with("gpu_")
                    || sym.starts_with("ai_")
                    || matches!(
                        sym,
                        "info_perangkat_gpu"
                            | "alokasi_gpu"
                            | "salin_ke_gpu"
                            | "salin_dari_gpu"
                            | "luncurkan_kernel"
                    )
            }
            "python" => matches!(
                sym,
                "impor_python" | "panggil_python" | "eval_python" | "tensor_ke_dlpack"
            ),
            "sistem" | "sys" | "os" => matches!(
                sym,
                "argumen_cli"
                    | "ambil_env"
                    | "atur_env"
                    | "keluar"
                    | "nama_os"
                    | "arsitektur_cpu"
                    | "jumlah_inti_cpu"
                    | "direktori_kerja"
                    | "jalankan_perintah"
            ),
            _ => false,
        }
    }

    fn match_pattern_check(
        &mut self,
        pattern: &MatchPattern,
        target: &Value,
    ) -> Result<Option<Vec<(String, Value)>>, Galat> {
        match pattern {
            MatchPattern::Wildcard(_) => Ok(Some(Vec::new())),
            MatchPattern::Literal(expr) => {
                let lit_val = self.evaluate(expr)?;
                if &lit_val == target {
                    Ok(Some(Vec::new()))
                } else {
                    Ok(None)
                }
            }
            MatchPattern::Identifier(name, _) => {
                // If identifier matches an Enum variant with 0 fields in scope, check equality
                if let Ok(val) = self.environment.borrow().get(name, &crate::error::Span::new(0, 0)) {
                    if let Value::EnumVariant(ev) = &val {
                        if let Value::EnumVariant(target_ev) = target {
                            if ev.variant_name == target_ev.variant_name && ev.fields.is_empty() && target_ev.fields.is_empty() {
                                return Ok(Some(Vec::new()));
                            }
                        }
                    }
                }
                // Otherwise it's a variable binding pattern
                Ok(Some(vec![(name.clone(), target.clone())]))
            }
            MatchPattern::EnumVariant {
                enum_name,
                variant_name,
                bindings,
                ..
            } => {
                if let Value::EnumVariant(target_ev) = target {
                    let enum_matches = if let Some(e_name) = enum_name {
                        &target_ev.enum_name == e_name
                    } else {
                        true
                    };

                    if enum_matches && &target_ev.variant_name == variant_name {
                        if bindings.len() == target_ev.fields.len() {
                            let mut bound_vars = Vec::new();
                            for (var_name, field_val) in bindings.iter().zip(target_ev.fields.iter()) {
                                bound_vars.push((var_name.clone(), field_val.clone()));
                            }
                            return Ok(Some(bound_vars));
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    fn check_structural_interface(&self, target: &Value, trait_spec: &Value) -> bool {
        let required_methods: Vec<String> = match trait_spec {
            Value::Map(m) => {
                if let Some(Value::Array(arr)) = m.borrow().get("_metode") {
                    arr.borrow().iter().map(|v| v.to_string_repr()).collect()
                } else {
                    Vec::new()
                }
            }
            Value::String(trait_name) => {
                if trait_name == "any" || trait_name == "kosong" || trait_name == "interface{}" {
                    return true;
                }
                if let Some(ms) = self.trait_registry.get(trait_name) {
                    ms.clone()
                } else if let Ok(Value::Map(m)) =
                    self.environment.borrow().get(trait_name, &Span::new(0, 0))
                {
                    if let Some(Value::Array(arr)) = m.borrow().get("_metode") {
                        arr.borrow().iter().map(|v| v.to_string_repr()).collect()
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        };

        if required_methods.is_empty() {
            return true;
        }

        // Collect all methods available on target (including promoted methods from Go struct embedding)
        let mut available_methods = HashSet::new();
        match target {
            Value::Instance(inst) => {
                let b_inst = inst.borrow();
                for mname in b_inst.struct_def.methods.keys() {
                    available_methods.insert(mname.clone());
                }
                // Also check latest StructDef in environment in case Go receiver methods were attached after
                if let Ok(Value::StructDef(latest_sd)) = self
                    .environment
                    .borrow()
                    .get(&b_inst.struct_def.name, &Span::new(0, 0))
                {
                    for mname in latest_sd.methods.keys() {
                        available_methods.insert(mname.clone());
                    }
                }
                for (fname, fval) in &b_inst.fields {
                    if matches!(fval, Value::Function(_) | Value::Builtin(_)) {
                        available_methods.insert(fname.clone());
                    }
                    if let Value::Instance(sub_inst) = fval {
                        let sub_b = sub_inst.borrow();
                        for mname in sub_b.struct_def.methods.keys() {
                            available_methods.insert(mname.clone());
                        }
                        if let Ok(Value::StructDef(latest_sub)) = self
                            .environment
                            .borrow()
                            .get(&sub_b.struct_def.name, &Span::new(0, 0))
                        {
                            for mname in latest_sub.methods.keys() {
                                available_methods.insert(mname.clone());
                            }
                        }
                    }
                }
            }
            Value::Map(m) => {
                for (k, v) in m.borrow().iter() {
                    if matches!(v, Value::Function(_) | Value::Builtin(_)) {
                        available_methods.insert(k.clone());
                    }
                }
            }
            _ => return false,
        }

        required_methods
            .iter()
            .all(|req| available_methods.contains(req))
    }

    fn execute_select_channels(&mut self, arms: &[Value], span: &Span) -> Result<Value, Galat> {
        let mut default_action: Option<Value> = None;
        let mut timeout_spec: Option<(u64, Value)> = None;

        for arm in arms {
            if let Value::Map(m) = arm {
                let mb = m.borrow();
                let jenis = mb
                    .get("jenis")
                    .map(|v| v.to_string_repr())
                    .unwrap_or_default();
                if jenis == "bawaan" {
                    default_action = mb.get("aksi").cloned();
                } else if jenis == "timeout" {
                    let ms = mb
                        .get("nilai")
                        .and_then(|v| if let Value::Number(n) = v { Some((*n).max(0.0) as u64) } else { None })
                        .unwrap_or(50);
                    if let Some(aksi) = mb.get("aksi").cloned() {
                        timeout_spec = Some((ms, aksi));
                    }
                }
            }
        }

        let max_wait = if default_action.is_some() {
            std::time::Duration::from_millis(0)
        } else if let Some((ms, _)) = timeout_spec {
            std::time::Duration::from_millis(ms)
        } else {
            std::time::Duration::from_millis(5000)
        };
        let deadline = std::time::Instant::now() + max_wait;

        loop {
            for arm in arms {
                if let Value::Map(m) = arm {
                    let (jenis, ch_opt, val_opt, aksi_opt) = {
                        let mb = m.borrow();
                        (
                            mb.get("jenis").map(|v| v.to_string_repr()).unwrap_or_default(),
                            mb.get("saluran").cloned(),
                            mb.get("nilai").cloned(),
                            mb.get("aksi").cloned(),
                        )
                    };
                    if jenis == "terima" {
                        if let (Some(ch_val), Some(aksi)) = (ch_opt, aksi_opt) {
                            let ch_id = crate::stdlib::concurrency::extract_id_from_arg(&ch_val, "Saluran", span)?;
                            let ch = crate::stdlib::concurrency::get_channel_arc(ch_id, span)?;
                            if let Some((msg_opt, _ok)) = ch.try_recv() {
                                let received = msg_opt.map(|v| v.to_value()).unwrap_or(Value::Nil);
                                self.sync_goroutine_dirty_vars();
                                return self.call_value(aksi, &[received], span);
                            }
                        }
                    } else if jenis == "kirim" {
                        if let (Some(ch_val), Some(send_val), Some(aksi)) = (ch_opt, val_opt, aksi_opt) {
                            let ch_id = crate::stdlib::concurrency::extract_id_from_arg(&ch_val, "Saluran", span)?;
                            let ch = crate::stdlib::concurrency::get_channel_arc(ch_id, span)?;
                            let ser = crate::stdlib::concurrency::ValueSerial::from_value(&send_val);
                            if let Ok(true) = ch.try_send(ser) {
                                return self.call_value(aksi, &[], span);
                            }
                        }
                    }
                }
            }

            if let Some(def_fn) = default_action {
                return self.call_value(def_fn, &[], span);
            }

            if std::time::Instant::now() >= deadline {
                if let Some((_, to_fn)) = timeout_spec {
                    return self.call_value(to_fn, &[], span);
                }
                return Ok(Value::Nil);
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}
