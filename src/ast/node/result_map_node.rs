use core::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::DerefMut;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::{create_deep, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::result_map_id_node::ResultMapIdNode;
use crate::ast::node::result_map_result_node::ResultMapResultNode;
use rbatis_core::convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultMapNode {
    pub id: String,
    pub table: Option<String>,
    pub id_node: Option<ResultMapIdNode>,
    pub results: Vec<ResultMapResultNode>,
    pub column_map: HashMap<String, ResultMapResultNode>,
    //Map<Column,Node>
    pub delete_node: Option<ResultMapResultNode>,
    pub version_node: Option<ResultMapResultNode>,
}

impl ResultMapNode {
    pub fn new(id: String, table_str: String, id_node: Option<ResultMapIdNode>, results: Vec<ResultMapResultNode>) -> Self {
        let mut column_map = HashMap::new();
        let mut delete_node = Option::None;
        let mut version_node = Option::None;
        let mut table = Option::None;
        if !table_str.is_empty() {
            table = Option::Some(table_str);
        }
        for item in &results {
            column_map.insert(item.column.clone(), item.clone());
            if item.logic_enable.eq("true") {
                delete_node = Option::Some(item.clone());
            }
            if item.version_enable.eq("true") {
                version_node = Option::Some(item.clone());
            }
        }
        let data = Self {
            id,
            table,
            id_node,
            results,
            column_map,
            delete_node,
            version_node,
        };
        return data;
    }

    pub fn find_delete_flag(&self) -> &ResultMapResultNode {
        unimplemented!()
    }
}

impl RbatisAST for ResultMapNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, rbatis_core::Error> {
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for ResultMapNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<result_map id=\"" + self.id.as_str() + "\">";
        if self.id_node.is_some() {
            result = result + self.id_node.as_ref().unwrap().print(deep).as_str();
        }
        if self.results.len() > 0 {
            result = result + print_child(self.results.as_ref(), deep + 1).as_str();
        }
        result = result + create_deep(deep).as_str() + "</result_map>";
        return result;
    }
}

