use crate::models::transaction::schema::Schema;

use super::schema_common_definition::schema_common_definition;

pub fn abstract_transaction_schema() -> Schema {
    Schema::new(schema_common_definition())
}
