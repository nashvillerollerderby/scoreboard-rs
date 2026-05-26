use serde_json::Value;

/// Base class for types held in `Child` properties
pub trait ValueWithId {
    /// ID to be used in order to identify this element amongst all elements of its
    /// type. Used when the element is referenced by elements other than its parent.
    /// (Typically a UUID.)
    fn get_id(&self) -> String;

    /// String representation of the element's value to be used in JSON. For complex types this will usually be the ID.
    fn get_value(&self) -> Value;
}
