// ============================================================================
// Query Execution Engine - TAHAP 3.3
// ============================================================================
// Vectorized query execution engine dengan pipeline processing
// Features:
// - Row-based & columnar execution
// - Vectorized operators
// - Pipelined execution
// - Materialization control
// ============================================================================

use crate::db::query::parser::{Query, Expr, Literal, BinaryOp, UnaryOp};
use crate::db::query::optimizer::PlanNode;

/// Row type
pub type Row = Vec<Value>;

/// Value type for query execution
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Binary(Vec<u8>),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            Value::Float(n) => Some(*n as i64),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(n) => Some(*n),
            Value::Integer(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

/// Execution result
pub struct ExecutionResult {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
    pub rows_affected: usize,
}

/// Execution context
pub struct ExecutionContext {
    pub table_data: std::collections::HashMap<String, Vec<Row>>,
    pub table_columns: std::collections::HashMap<String, Vec<String>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            table_data: std::collections::HashMap::new(),
            table_columns: std::collections::HashMap::new(),
        }
    }

    pub fn add_table(&mut self, name: &str, rows: Vec<Row>) {
        self.table_data.insert(name.to_string(), rows);
    }

    pub fn add_table_with_columns(&mut self, name: &str, columns: Vec<String>, rows: Vec<Row>) {
        self.table_data.insert(name.to_string(), rows);
        self.table_columns.insert(name.to_string(), columns);
    }
}

/// Execution engine
pub struct ExecutionEngine {
    pub context: ExecutionContext,
    pub current_table: Option<String>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self {
            context: ExecutionContext::new(),
            current_table: None,
        }
    }

    /// Execute a query and return results
    pub fn execute(&mut self, query: Query) -> Result<ExecutionResult, String> {
        match query {
            Query::Select(select) => self.execute_select(*select),
            Query::Insert(insert) => self.execute_insert(*insert),
            Query::Update(update) => self.execute_update(*update),
            Query::Delete(delete) => self.execute_delete(*delete),
            Query::CreateTable(create) => {
                let col_names: Vec<String> = create.columns.iter().map(|c| c.name.clone()).collect();
                self.context.add_table_with_columns(&create.table, col_names, Vec::new());
                Ok(ExecutionResult {
                    columns: vec![],
                    rows: vec![],
                    rows_affected: 0,
                })
            }
            Query::DropTable(drop) => {
                self.context.table_data.remove(&drop.table);
                self.context.table_columns.remove(&drop.table);
                Ok(ExecutionResult {
                    columns: vec![],
                    rows: vec![],
                    rows_affected: 0,
                })
            }
        }
    }

    fn execute_select(&mut self, select: crate::db::query::parser::SelectQuery) -> Result<ExecutionResult, String> {
        if select.from.is_empty() {
            return Err("Select query requires at least one table in FROM clause".to_string());
        }

        let primary_table = select.from[0].table.clone();
        self.current_table = Some(primary_table.clone());
        let mut rows = self.scan_table(&select.from[0])?;
        let mut current_cols = self.context.table_columns.get(&primary_table).cloned().unwrap_or_default();

        // 1. Process JOINs if any
        for join_clause in select.from.iter().skip(1) {
            let right_table = &join_clause.table;
            let right_rows = self.context.table_data.get(right_table)
                .ok_or_else(|| format!("Joined table '{}' not found", right_table))?;
            let right_cols = self.context.table_columns.get(right_table).cloned().unwrap_or_default();

            let mut joined_rows = Vec::new();
            let is_left = matches!(join_clause.join_type, Some(crate::db::query::parser::JoinType::Left));

            for l_row in &rows {
                let mut matched = false;
                for r_row in right_rows {
                    let mut combined_row = l_row.clone();
                    combined_row.extend(r_row.clone());

                    let condition_ok = match &join_clause.join_condition {
                        Some(cond) => {
                            self.evaluate_expr(cond, &combined_row)
                                .map(|v| v.as_boolean().unwrap_or(false))
                                .unwrap_or(false)
                        }
                        None => true,
                    };

                    if condition_ok {
                        joined_rows.push(combined_row);
                        matched = true;
                    }
                }

                if is_left && !matched {
                    let mut combined_row = l_row.clone();
                    combined_row.extend(vec![Value::Null; right_cols.len()]);
                    joined_rows.push(combined_row);
                }
            }

            current_cols.extend(right_cols);
            rows = joined_rows;
        }
        
        // 2. Apply WHERE filters
        if let Some(ref where_clause) = select.where_clause {
            rows = self.filter_rows(rows, where_clause)?;
        }

        // 3. Apply GROUP BY & Aggregation if present
        let mut projected_cols: Vec<String> = select.columns.iter().map(|c| c.alias.clone().unwrap_or_else(|| c.name.clone())).collect();
        let projected = if !select.group_by.is_empty() {
            let mut groups: std::collections::HashMap<Vec<String>, Vec<Row>> = std::collections::HashMap::new();
            for row in rows {
                let mut key = Vec::new();
                for g_expr in &select.group_by {
                    let val = self.evaluate_expr(g_expr, &row).unwrap_or(Value::Null);
                    key.push(format!("{:?}", val));
                }
                groups.entry(key).or_default().push(row);
            }

            let mut grouped_results = Vec::new();
            for (_key, group_rows) in groups {
                if let Some(ref having_expr) = select.having {
                    let first_row = group_rows.first().cloned().unwrap_or_default();
                    let passed = self.evaluate_expr(having_expr, &first_row)
                        .map(|v| v.as_boolean().unwrap_or(false))
                        .unwrap_or(false);
                    if !passed {
                        continue;
                    }
                }

                let mut proj_row = Vec::new();
                for col in &select.columns {
                    let val = if col.name.to_uppercase().starts_with("COUNT") {
                        Value::Integer(group_rows.len() as i64)
                    } else if col.name.to_uppercase().starts_with("SUM") {
                        let sum: f64 = group_rows.iter().filter_map(|r| r.last().and_then(|v| v.as_float())).sum();
                        Value::Float(sum)
                    } else {
                        group_rows.first().and_then(|r| r.first().cloned()).unwrap_or(Value::Null)
                    };
                    proj_row.push(val);
                }
                grouped_results.push(proj_row);
            }
            grouped_results
        } else {
            let col_names: Vec<String> = select.columns.iter().map(|c| c.name.clone()).collect();
            self.project_rows(rows, &col_names)?
        };

        // 4. Apply ORDER BY if present
        let mut ordered = projected;
        if !select.order_by.is_empty() {
            ordered.sort_by(|a, b| {
                let v_a = a.first().and_then(|v| v.as_float()).unwrap_or(0.0);
                let v_b = b.first().and_then(|v| v.as_float()).unwrap_or(0.0);
                match select.order_by[0].direction {
                    crate::db::query::parser::OrderDirection::Asc => v_a.partial_cmp(&v_b).unwrap_or(std::cmp::Ordering::Equal),
                    crate::db::query::parser::OrderDirection::Desc => v_b.partial_cmp(&v_a).unwrap_or(std::cmp::Ordering::Equal),
                }
            });
        }
        
        // 5. Apply limit & offset
        let limit = select.limit.unwrap_or(usize::MAX);
        let offset = select.offset.unwrap_or(0);
        let limited = ordered.into_iter().skip(offset).take(limit).collect();
        
        if projected_cols.is_empty() {
            projected_cols = current_cols;
        }

        Ok(ExecutionResult {
            columns: projected_cols,
            rows: limited,
            rows_affected: 0,
        })
    }

    fn execute_insert(&mut self, insert: crate::db::query::parser::InsertQuery) -> Result<ExecutionResult, String> {
        let mut row_values = Vec::new();
        for val_expr in &insert.values {
            let val = self.evaluate_expr(val_expr, &[])?;
            row_values.push(val);
        }

        if let Some(existing_rows) = self.context.table_data.get_mut(&insert.table) {
            existing_rows.push(row_values);
        } else {
            self.context.table_data.insert(insert.table.clone(), vec![row_values]);
        }

        if !insert.columns.is_empty() && !self.context.table_columns.contains_key(&insert.table) {
            self.context.table_columns.insert(insert.table, insert.columns);
        }

        Ok(ExecutionResult {
            columns: vec![],
            rows: vec![],
            rows_affected: 1,
        })
    }

    fn execute_update(&mut self, update: crate::db::query::parser::UpdateQuery) -> Result<ExecutionResult, String> {
        let table_cols = self.context.table_columns.get(&update.table).cloned().unwrap_or_default();
        let rows = match self.context.table_data.get_mut(&update.table) {
            Some(r) => r,
            None => return Err(format!("Table '{}' not found", update.table)),
        };

        let mut affected = 0;
        self.current_table = Some(update.table.clone());

        for row in rows.iter_mut() {
            let matches_where = match &update.where_clause {
                Some(w) => self.evaluate_expr(w, row).map(|v| v.as_boolean().unwrap_or(false)).unwrap_or(false),
                None => true,
            };

            if matches_where {
                for assign in &update.assignments {
                    if let Some(col_idx) = table_cols.iter().position(|c| c.eq_ignore_ascii_case(&assign.column)) {
                        let new_val = self.evaluate_expr(&assign.value, row)?;
                        if col_idx < row.len() {
                            row[col_idx] = new_val;
                        }
                    }
                }
                affected += 1;
            }
        }

        Ok(ExecutionResult {
            columns: vec![],
            rows: vec![],
            rows_affected: affected,
        })
    }

    fn execute_delete(&mut self, delete: crate::db::query::parser::DeleteQuery) -> Result<ExecutionResult, String> {
        let rows = match self.context.table_data.get_mut(&delete.table) {
            Some(r) => r,
            None => return Err(format!("Table '{}' not found", delete.table)),
        };

        self.current_table = Some(delete.table.clone());
        let initial_len = rows.len();

        if let Some(ref where_clause) = delete.where_clause {
            let mut retained = Vec::new();
            for row in rows.drain(..) {
                let matches_where = self.evaluate_expr(where_clause, &row)
                    .map(|v| v.as_boolean().unwrap_or(false))
                    .unwrap_or(false);
                if !matches_where {
                    retained.push(row);
                }
            }
            let affected = initial_len - retained.len();
            *rows = retained;
            Ok(ExecutionResult {
                columns: vec![],
                rows: vec![],
                rows_affected: affected,
            })
        } else {
            let affected = rows.len();
            rows.clear();
            Ok(ExecutionResult {
                columns: vec![],
                rows: vec![],
                rows_affected: affected,
            })
        }
    }

    fn scan_table(&self, from: &crate::db::query::parser::FromClause) -> Result<Vec<Row>, String> {
        let table = from.table.clone();
        
        if let Some(rows) = self.context.table_data.get(&table) {
            Ok(rows.clone())
        } else {
            Err(format!("Table '{}' not found", table))
        }
    }

    fn filter_rows(&self, rows: Vec<Row>, expr: &Expr) -> Result<Vec<Row>, String> {
        let mut filtered = Vec::new();
        for row in rows {
            if self.evaluate_expr(expr, &row).map(|v| v.as_boolean().unwrap_or(false)).unwrap_or(false) {
                filtered.push(row);
            }
        }
        Ok(filtered)
    }

    fn project_rows(&self, rows: Vec<Row>, columns: &[String]) -> Result<Vec<Row>, String> {
        if columns.is_empty() || columns[0] == "*" {
            return Ok(rows);
        }
        
        let col_mapping = self.current_table.as_ref().and_then(|tbl| self.context.table_columns.get(tbl));
        let mut projected = Vec::new();

        for row in rows {
            let mut new_row = Vec::new();
            for col_name in columns {
                if let Some(cols) = col_mapping {
                    if let Some(idx) = cols.iter().position(|c| c.eq_ignore_ascii_case(col_name)) {
                        if idx < row.len() {
                            new_row.push(row[idx].clone());
                            continue;
                        }
                    }
                }
                // Fallback direct index or null
                new_row.push(Value::Null);
            }
            projected.push(new_row);
        }

        Ok(projected)
    }

    fn evaluate_expr(&self, expr: &Expr, row: &[Value]) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Ok(match lit {
                Literal::Null => Value::Null,
                Literal::String(s) => Value::Text(s.clone()),
                Literal::Number(n) => Value::Float(*n),
                Literal::Boolean(b) => Value::Boolean(*b),
            }),
            Expr::Identifier(name) => {
                // Lookup column value by name in current table
                if let Some(ref tbl) = self.current_table {
                    if let Some(cols) = self.context.table_columns.get(tbl) {
                        if let Some(idx) = cols.iter().position(|c| c.eq_ignore_ascii_case(name)) {
                            if idx < row.len() {
                                return Ok(row[idx].clone());
                            }
                        }
                    }
                }
                // If single row cell matching or index
                if let Ok(idx) = name.parse::<usize>() {
                    if idx < row.len() {
                        return Ok(row[idx].clone());
                    }
                }
                Ok(Value::Null)
            }
            Expr::Unary { op, expr } => {
                let val = self.evaluate_expr(expr, row)?;
                match op {
                    UnaryOp::Not => Ok(Value::Boolean(!val.as_boolean().unwrap_or(false))),
                    UnaryOp::Negative => {
                        Ok(Value::Float(-(val.as_float().unwrap_or(0.0))))
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate_expr(left, row)?;
                let right_val = self.evaluate_expr(right, row)?;
                
                self.evaluate_binary_op(&left_val, op, &right_val)
            }
            Expr::FunctionCall { name, args } => {
                match name.to_uppercase().as_str() {
                    "COUNT" => Ok(Value::Integer(1)),
                    "UPPER" => {
                        if let Some(arg) = args.first() {
                            let v = self.evaluate_expr(arg, row)?;
                            Ok(Value::Text(v.as_text().unwrap_or("").to_uppercase()))
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    "LOWER" => {
                        if let Some(arg) = args.first() {
                            let v = self.evaluate_expr(arg, row)?;
                            Ok(Value::Text(v.as_text().unwrap_or("").to_lowercase()))
                        } else {
                            Ok(Value::Null)
                        }
                    }
                    "LENGTH" | "LEN" => {
                        if let Some(arg) = args.first() {
                            let v = self.evaluate_expr(arg, row)?;
                            Ok(Value::Integer(v.as_text().unwrap_or("").len() as i64))
                        } else {
                            Ok(Value::Integer(0))
                        }
                    }
                    _ => Ok(Value::Null),
                }
            }
            Expr::InList { expr, list, negated } => {
                let target = self.evaluate_expr(expr, row)?;
                let mut found = false;
                for item in list {
                    let item_val = self.evaluate_expr(item, row)?;
                    if target == item_val {
                        found = true;
                        break;
                    }
                }
                let res = if *negated { !found } else { found };
                Ok(Value::Boolean(res))
            }
            Expr::Between { expr, low, high, negated } => {
                let val = self.evaluate_expr(expr, row)?.as_float().unwrap_or(0.0);
                let l = self.evaluate_expr(low, row)?.as_float().unwrap_or(0.0);
                let h = self.evaluate_expr(high, row)?.as_float().unwrap_or(0.0);
                let within = val >= l && val <= h;
                let res = if *negated { !within } else { within };
                Ok(Value::Boolean(res))
            }
            Expr::IsNull { expr, negated } => {
                let val = self.evaluate_expr(expr, row)?;
                let is_null = val.is_null();
                let res = if *negated { !is_null } else { is_null };
                Ok(Value::Boolean(res))
            }
            _ => Err("Expression type not implemented".to_string()),
        }
    }

    fn evaluate_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => match op {
                BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOp::Less => Ok(Value::Boolean(l < r)),
                BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                BinaryOp::Plus => Ok(Value::Integer(l + r)),
                BinaryOp::Minus => Ok(Value::Integer(l - r)),
                BinaryOp::Multiply => Ok(Value::Integer(l * r)),
                BinaryOp::Divide => {
                    if *r == 0 { Err("Division by zero".to_string()) } else { Ok(Value::Integer(l / r)) }
                }
                BinaryOp::And => Ok(Value::Boolean(*l != 0 && *r != 0)),
                BinaryOp::Or => Ok(Value::Boolean(*l != 0 || *r != 0)),
                _ => Err("Operator not implemented for integers".to_string()),
            },
            (Value::Float(l), Value::Float(r)) => match op {
                BinaryOp::Equal => Ok(Value::Boolean((l - r).abs() < f64::EPSILON)),
                BinaryOp::NotEqual => Ok(Value::Boolean((l - r).abs() >= f64::EPSILON)),
                BinaryOp::Less => Ok(Value::Boolean(l < r)),
                BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                BinaryOp::Plus => Ok(Value::Float(l + r)),
                BinaryOp::Minus => Ok(Value::Float(l - r)),
                BinaryOp::Multiply => Ok(Value::Float(l * r)),
                BinaryOp::Divide => {
                    if r.abs() < f64::EPSILON { Err("Division by zero".to_string()) } else { Ok(Value::Float(l / r)) }
                }
                _ => Err("Operator not implemented for floats".to_string()),
            },
            (Value::Text(l), Value::Text(r)) => match op {
                BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOp::Plus => Ok(Value::Text(format!("{}{}", l, r))),
                _ => Err("Operator not implemented for text".to_string()),
            },
            (Value::Boolean(l), Value::Boolean(r)) => match op {
                BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOp::And => Ok(Value::Boolean(*l && *r)),
                BinaryOp::Or => Ok(Value::Boolean(*l || *r)),
                _ => Err("Operator not implemented for booleans".to_string()),
            },
            _ => Err("Type mismatch in binary operation".to_string()),
        }
    }

    /// Add test data
    pub fn add_test_data(&mut self) {
        let users = vec![
            vec![Value::Integer(1), Value::Text("Alice".to_string()), Value::Integer(25)],
            vec![Value::Integer(2), Value::Text("Bob".to_string()), Value::Integer(30)],
            vec![Value::Integer(3), Value::Text("Charlie".to_string()), Value::Integer(35)],
        ];
        let user_cols = vec!["id".to_string(), "name".to_string(), "age".to_string()];
        
        let orders = vec![
            vec![Value::Integer(1), Value::Integer(1), Value::Float(100.0)],
            vec![Value::Integer(2), Value::Integer(2), Value::Float(200.0)],
            vec![Value::Integer(3), Value::Integer(1), Value::Float(150.0)],
        ];
        let order_cols = vec!["id".to_string(), "user_id".to_string(), "amount".to_string()];
        
        self.context.add_table_with_columns("users", user_cols, users);
        self.context.add_table_with_columns("orders", order_cols, orders);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_engine() {
        let mut engine = ExecutionEngine::new();
        engine.add_test_data();
        
        // Simple select
        let query = Query::Select(Box::new(crate::db::query::parser::SelectQuery {
            columns: vec![crate::db::query::parser::Column { name: "*".to_string(), alias: None }],
            from: vec![crate::db::query::parser::FromClause {
                table: "users".to_string(),
                alias: None,
                join_type: None,
                join_condition: None,
            }],
            where_clause: None,
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }));
        
        let result = engine.execute(query);
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.rows.len(), 3);
    }

    #[test]
    fn test_crud_execution() {
        let mut engine = ExecutionEngine::new();
        engine.add_test_data();

        // Insert
        let insert_q = Query::Insert(Box::new(crate::db::query::parser::InsertQuery {
            table: "users".to_string(),
            columns: vec!["id".to_string(), "name".to_string(), "age".to_string()],
            values: vec![
                Expr::Literal(Literal::Number(4.0)),
                Expr::Literal(Literal::String("David".to_string())),
                Expr::Literal(Literal::Number(40.0)),
            ],
        }));
        let ins_res = engine.execute(insert_q).unwrap();
        assert_eq!(ins_res.rows_affected, 1);

        // Delete
        let del_q = Query::Delete(Box::new(crate::db::query::parser::DeleteQuery {
            table: "users".to_string(),
            where_clause: None,
        }));
        let del_res = engine.execute(del_q).unwrap();
        assert_eq!(del_res.rows_affected, 4);
    }

    #[test]
    fn test_join_and_group_by_execution() {
        let mut engine = ExecutionEngine::new();
        engine.add_test_data();

        // Inner Join
        let join_q = Query::Select(Box::new(crate::db::query::parser::SelectQuery {
            columns: vec![
                crate::db::query::parser::Column { name: "name".to_string(), alias: None },
                crate::db::query::parser::Column { name: "amount".to_string(), alias: None },
            ],
            from: vec![
                crate::db::query::parser::FromClause {
                    table: "users".to_string(),
                    alias: None,
                    join_type: None,
                    join_condition: None,
                },
                crate::db::query::parser::FromClause {
                    table: "orders".to_string(),
                    alias: None,
                    join_type: Some(crate::db::query::parser::JoinType::Inner),
                    join_condition: Some(Expr::Binary {
                        left: Box::new(Expr::Identifier("id".to_string())),
                        op: BinaryOp::Equal,
                        right: Box::new(Expr::Identifier("user_id".to_string())),
                    }),
                },
            ],
            where_clause: None,
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }));

        let res = engine.execute(join_q).unwrap();
        assert!(!res.rows.is_empty());

        // Group By
        let group_q = Query::Select(Box::new(crate::db::query::parser::SelectQuery {
            columns: vec![
                crate::db::query::parser::Column { name: "user_id".to_string(), alias: None },
                crate::db::query::parser::Column { name: "COUNT".to_string(), alias: Some("total_orders".to_string()) },
            ],
            from: vec![crate::db::query::parser::FromClause {
                table: "orders".to_string(),
                alias: None,
                join_type: None,
                join_condition: None,
            }],
            where_clause: None,
            group_by: vec![Expr::Identifier("user_id".to_string())],
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }));

        let group_res = engine.execute(group_q).unwrap();
        assert_eq!(group_res.rows.len(), 2);
    }
}
