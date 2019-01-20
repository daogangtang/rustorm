use rustorm_dao::TableName;
use rustorm_dao::{Rows, Value};
use entity::EntityManager;
use table::SchemaContent;
use table::Table;
use users::User;
use users::Role;

use error::DbError;
use rustorm_dao::FromDao;

/// The current database name and its comment
#[derive(Serialize, FromDao)]
pub struct DatabaseName{
    name: String,
    description: Option<String>,
}

pub trait Database {
    fn execute_sql_with_return(&self, sql: &str, param: &[&Value]) -> Result<Rows, DbError>;

    fn get_table(&self, em: &EntityManager, table_name: &TableName) -> Result<Table, DbError>;

    fn get_all_tables(&self, em: &EntityManager) -> Result<Vec<Table>, DbError>;

    fn get_grouped_tables(&self, em: &EntityManager) -> Result<Vec<SchemaContent>, DbError>;

    fn get_users(&self, em: &EntityManager) -> Result<Vec<User>, DbError>;

    fn get_roles(&self, em: &EntityManager, username: &str) -> Result<Vec<Role>, DbError>;

    fn get_database_name(&self, em: &EntityManager) -> Result<Option<DatabaseName>, DbError>;
}
