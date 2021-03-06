use super::super::model::*;
use super::parser::*;

struct BoolExprVisitor {

}

impl AstVisitor for BoolExprVisitor {
    fn enter_create(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_match(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_node(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_relationship(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_property(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_integer_value(&mut self, value: Option<i64>) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_float_value(&mut self, value: Option<f64>) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_string_value(&mut self, value: Option<&str>) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_bool_value(&mut self, value: Option<bool>) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_identifier(&mut self, key: &str) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_variable(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_label(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_query(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_return(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_function(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_function_arg(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_item(&mut self) -> AstVisitorResult<bool> {
        Ok(true)
    }
    fn enter_where(&mut self, node: &AstTagNode) -> AstVisitorResult<bool> {
        Ok(true)
    }
}

pub fn extract_mandatory_conditions_from_bool_expr(ast: &Box<dyn Ast>) -> Vec<BoolCondition> {
    let mut res = Vec::new();

    res
}