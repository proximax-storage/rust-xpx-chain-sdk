use std::any::Any;
use serde_json::Value;
use crate::transaction::{CommonTransaction, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfigTransaction {
    pub common: CommonTransaction,
    pub network_config: String,
}

#[typetag::serde]
impl Transaction for NetworkConfigTransaction {
    fn size(&self) -> usize {
        todo!()
    }

    fn as_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    fn get_common_transaction(&self) -> CommonTransaction {
        self.common.to_owned()
    }

    fn to_serializer(&self) -> Vec<u8> {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn box_clone(&self) -> Box<dyn Transaction + 'static> {
        Box::new((*self).clone())
    }
}
