use crate::event::ScoreBoardListener;
use crate::util::ValueWithId;
use std::any::Any;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::atomic::AtomicIsize;

#[derive(Clone)]
pub enum Property<T = Box<dyn Any>> {
    Value {
        json_name: String,
        value: T,
    },
    Child {
        json_name: String,
        value: Arc<dyn ValueWithId>,
    },
    Command {
        json_name: String,
        value: bool,
    },
}

impl<T> Display for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Property::Value { json_name, .. }
            | Property::Child { json_name, .. }
            | Property::Command { json_name, .. } => json_name.clone(),
        };
        write!(f, "{}", str)
    }
}

impl<T> PartialEq for Property<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Property::Value { json_name, .. },
                Property::Value {
                    json_name: other_json_name,
                    ..
                },
            )
            | (
                Property::Child { json_name, .. },
                Property::Child {
                    json_name: other_json_name,
                    ..
                },
            )
            | (
                Property::Command { json_name, .. },
                Property::Command {
                    json_name: other_json_name,
                    ..
                },
            ) => json_name == other_json_name,
            (_, _) => false,
        }
    }
}
impl<T> Eq for Property<T> {}

impl<T> Property<T> {
    pub fn get_json_name(&self) -> String {
        match self {
            Property::Value { json_name, .. }
            | Property::Child { json_name, .. }
            | Property::Command { json_name, .. } => json_name.clone(),
        }
    }
}

pub trait Source {
    fn is_internal(&self) -> bool;
    fn is_file(&self) -> bool;
}

pub enum SourceEnum {
    WS,
    AutoSave,
    JSON,
    InverseReference,
    Copy,
    Recalculate,
    Unlink,
    Renumber,
    Other,
    AnyInternal,
    AnyFile,
    NonWS,
}

impl Source for SourceEnum {
    fn is_internal(&self) -> bool {
        match self {
            SourceEnum::WS => false,
            SourceEnum::AutoSave => false,
            SourceEnum::JSON => false,
            SourceEnum::InverseReference => true,
            SourceEnum::Copy => true,
            SourceEnum::Recalculate => true,
            SourceEnum::Unlink => true,
            SourceEnum::Renumber => true,
            SourceEnum::Other => true,
            SourceEnum::AnyInternal => true,
            SourceEnum::AnyFile => false,
            SourceEnum::NonWS => true,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            SourceEnum::WS => false,
            SourceEnum::AutoSave => true,
            SourceEnum::JSON => true,
            SourceEnum::InverseReference => false,
            SourceEnum::Copy => false,
            SourceEnum::Recalculate => false,
            SourceEnum::Unlink => false,
            SourceEnum::Renumber => false,
            SourceEnum::Other => false,
            SourceEnum::AnyInternal => false,
            SourceEnum::AnyFile => true,
            SourceEnum::NonWS => true,
        }
    }
}

pub struct SourceInstance(bool, bool);

impl Source for SourceInstance {
    fn is_internal(&self) -> bool {
        self.0
    }

    fn is_file(&self) -> bool {
        self.1
    }
}

pub enum Flag {
    Change,
    Reset,
    SpecialCase,
}

pub trait ScoreBoardEventProvider<T>: ValueWithId
where
    T: Clone + Display,
{
    /// This is the frontend string (i.e. path component) for the Child enum value corresponding to
    /// this type in its parent element.
    fn get_provider_name(&self) -> String;

    /// This should return the class or (usually) interface that this type will be accessed
    /// through by event receivers.
    fn get_provider_class(&self) -> String;

    /// ID to be used in order to identify this element amongst its siblings. (Could
    ///  e.g. be a Period/Jam/etc number or a UUID.)
    fn get_provider_id(&self) -> String;

    fn get_parent(&self) -> Option<Arc<dyn ScoreBoardEventProvider<T>>>;
    fn is_ancestor_of(&self, other: Arc<dyn ScoreBoardEventProvider<T>>);

    fn delete(&self);
    fn delete_source(&self, source: dyn Source);

    fn add_scoreboard_listener(&self, listener: dyn ScoreBoardListener<T>);
    fn remove_scoreboard_listener(&self, listener: dyn ScoreBoardListener<T>);

    // fn value_from_string(&self, prop: Property<T>, value: String) -> T;
}

pub trait ScoreBoardEventProviderProperties {
    fn get_properties<T>(&self) -> Vec<Property<T>>;
    fn get_property<T>(&self, json_name: String) -> Option<Property<T>>;
}

pub trait OrderedScoreBoardEventProvider<T>: ScoreBoardEventProvider<T>
where
    T: Clone + Display,
{
    const NUMBER: AtomicIsize = AtomicIsize::new(0);

    fn get_number(&self) -> i64;

    fn get_previous(&self) -> T;
    fn has_previous(&self) -> bool;
    fn set_previous(&self, prev: T);

    fn get_next(&self) -> T;
    fn has_next(&self) -> bool;
    fn set_next(&self, next: T);
}
