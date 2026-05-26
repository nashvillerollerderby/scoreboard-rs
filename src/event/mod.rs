mod event_provider;
mod listener;

pub use event_provider::*;
pub use listener::*;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone)]
pub struct ScoreBoardEvent<T>
where
    T: Clone + Display,
{
    provider: Arc<dyn ScoreBoardEventProvider<T>>,
    property: Property<T>,
    value: Option<T>,
    previous_value: Option<T>,
    remove: bool,
}

impl<T> ScoreBoardEvent<T>
where
    T: Clone + Display,
{
    pub fn new(
        event_provider: Arc<dyn ScoreBoardEventProvider<T>>,
        property: Property<T>,
        value: Option<T>,
        prev: Option<T>,
    ) -> Self {
        Self {
            provider: event_provider,
            property,
            value,
            previous_value: prev,
            remove: false,
        }
    }

    pub fn new_with_remove(
        event_provider: Arc<dyn ScoreBoardEventProvider<T>>,
        property: Property<T>,
        value: Option<T>,
        remove: bool,
    ) -> Self {
        Self {
            provider: event_provider,
            property,
            value,
            previous_value: None,
            remove,
        }
    }

    pub fn get_provider(&self) -> Arc<dyn ScoreBoardEventProvider<T>> {
        self.provider.clone()
    }
    pub fn get_property(&self) -> Property<T> {
        self.property.clone()
    }
    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }
    pub fn get_previous_value(&self) -> Option<T> {
        self.previous_value.clone()
    }
    pub fn is_remove(&self) -> bool {
        self.remove
    }
}

impl<T> PartialEq for ScoreBoardEvent<T>
where
    T: Clone + Display,
{
    fn eq(&self, other: &Self) -> bool {
        if self.provider.get_provider_name() != other.provider.get_provider_name() {
            return false;
        }
        if self.property != other.property {
            return false;
        }
        true
    }
}
impl<T> Eq for ScoreBoardEvent<T> where T: Clone + Display {}

impl<T> Hash for ScoreBoardEvent<T>
where
    T: Clone + Display,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.provider.get_provider_name().into_bytes());
        state.write(&self.property.to_string().into_bytes());
        state.write(
            &match &self.value {
                Some(v) => v.to_string(),
                None => "None".to_string(),
            }
            .into_bytes(),
        );
        state.write(
            &match &self.previous_value {
                Some(v) => v.to_string(),
                None => "None".to_string(),
            }
            .into_bytes(),
        )
    }
}

impl<T> Display for ScoreBoardEvent<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.provider.get_provider_name(),
            self.property.to_string(),
            self.value
                .as_ref()
                .map_or("None".to_string(), |v| v.to_string()),
            self.previous_value
                .as_ref()
                .map_or("None".to_string(), |v| v.to_string())
        )
    }
}
