pub trait ValueWithId {
    fn get_id(&self) -> String;

    fn get_value(&self) -> String;
}
