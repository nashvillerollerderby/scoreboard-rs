use std::fmt::Display;

pub enum RuleType {
    Boolean {
        true_value: &'static str,
        false_value: &'static str,
    },
    Integer,
    Long,
    String,
    Time,
}

impl Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            RuleType::Boolean {
                true_value: _,
                false_value: _,
            } => "Boolean",
            RuleType::Integer => "Integer",
            RuleType::Long => "Long",
            RuleType::String => "String",
            RuleType::Time => "Time",
        };

        f.write_str(ty)
    }
}

pub struct RuleDefinition {
    pub name: &'static str,
    pub rule_type: RuleType,
    pub description: &'static str,
    pub default_value: &'static str,
}
