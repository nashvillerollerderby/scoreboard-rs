use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Typ {
    Boolean,
    Integer,
    Long,
    String,
    Time,
}
