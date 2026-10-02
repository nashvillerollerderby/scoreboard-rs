use crate::state::JSONStateManager;
use crate::utils::ValueWithId;
use std::any::{Any, TypeId};
use std::sync::Arc;

pub trait Property<T> {
    fn get_type(&self) -> TypeId;

    fn get_json_name(&self) -> String;
}

pub struct Value<T> {
    typ: TypeId,
    json_name: String,
    default_value: T,
    props_to_add_to: Vec<Box<dyn Property<dyn Any>>>,
}

impl<T> Property<T> for Value<T> {
    fn get_type(&self) -> TypeId {
        self.typ
    }

    fn get_json_name(&self) -> String {
        self.json_name.clone()
    }
}

pub struct Child {
    typ: TypeId,
    json_name: String,
    props_to_add_to: Vec<Box<dyn Property<dyn Any>>>,
}

impl<T> Property<T> for Child {
    fn get_type(&self) -> TypeId {
        self.typ
    }

    fn get_json_name(&self) -> String {
        self.json_name.clone()
    }
}

pub struct Command {
    json_name: String,
    props_to_add_to: Vec<Box<dyn Property<dyn Any>>>,
}

impl Property<bool> for Command {
    fn get_type(&self) -> TypeId {
        TypeId::of::<bool>()
    }

    fn get_json_name(&self) -> String {
        self.json_name.clone()
    }
}

pub trait EventProvider {}

pub trait ScoreBoardEventProvider: EventProvider + ValueWithId + Ord {
    fn get_provider_name(&self) -> String;
    fn get_provider_class(&self) -> TypeId;
    fn get_provider_id(&self) -> String;
    fn get_parent(&self) -> Option<Arc<impl ScoreBoardEventProvider>>;
    fn is_ancestor_of(&self, other: Arc<impl ScoreBoardEventProvider>) -> bool;
    fn delete(&self);
    fn delete_with_source(&self, source: Source);

    fn get_properties(&self) -> Vec<Box<dyn Property<dyn Any>>>;
    fn get_property(&self, json_name: String) -> Option<Box<dyn Property<dyn Any>>>;

    fn add_score_board_listener(&self, listener: impl ScoreBoardListener);
    fn remove_score_board_listener(&self, listener: impl ScoreBoardListener);

    fn value_from_string<T>(&self, prop: Value<T>, s_value: String) -> T;

    fn get<T>(&self, prop: Value<T>) -> T;

    fn set<T>(&self, prop: Value<T>, value: T) -> bool;
    fn set_with_flag<T>(&self, prop: Value<T>, value: T, flag: Flag) -> bool;
    fn set_with_source<T>(&self, prop: Value<T>, value: T, source: Source) -> bool;
    fn set_with_source_and_flag<T>(
        &self,
        prop: Value<T>,
        value: T,
        source: Source,
        flag: Flag,
    ) -> bool;

    fn run_in_batch(&self, r: impl Fn());

    fn child_from_string<T: ValueWithId, C: Property<T>>(
        &self,
        child: C,
        id: String,
        s_value: String,
    ) -> T;
    fn get_child<T: ValueWithId, C: Property<T>>(&self, prop: C, id: String) -> Option<T>;
    fn get_or_create_child<T: EventProvider, C: Property<T>>(&self, prop: C, id: String) -> T;
    fn get_or_create_child_with_source<T: EventProvider, C: Property<T>>(
        &self,
        prop: C,
        id: String,
        source: Source,
    ) -> T;
    fn get_all_children<T: ValueWithId, C: Property<T>>(&self, prop: C) -> Vec<T>;
    fn number_of_children(&self, prop: Child) -> i32;

    fn add_child<T: ValueWithId, C: Property<T>>(&self, prop: C, item: T);
    fn add_child_with_source<T: ValueWithId, C: Property<T>>(
        &self,
        prop: C,
        item: T,
        source: Source,
    );
    fn remove_child(&self, prop: Child, id: String);
    fn remove_child_with_source(&self, prop: Child, id: String, source: Source);
    fn remove_item<T: ValueWithId, C: Property<T>>(&self, prop: C, item: T);
    fn remove_item_with_source<T: ValueWithId, C: Property<T>>(
        &self,
        prop: C,
        item: T,
        source: Source,
    );
    fn remove_all(&self, prop: Child);
    fn remove_all_with_source(&self, prop: Child, source: Source);

    fn create(&self, prop: Child, id: String, source: Source);
    // fn get_min_number();
    // fn get_max_number();
    fn execute(&self, prop: Command);
    fn execute_with_source(&self, prop: Command, source: Source);
    fn get_scoreboard() -> impl ScoreBoard;

    fn get_element<T: ValueWithId>(&self, typ: TypeId, id: String) -> T;
    fn check_property<T>(&self, prop: Box<dyn Property<T>>);
    fn cleanup_aliases(&self);
}

pub trait ScoreBoard: ScoreBoardEventProvider {
    fn post_autosave_update(&self);

    // fn get_timeout_owner(&self) -> TimeoutOwner;
    // fn get_settings(&self) -> Settings;
    // fn get_rulesets(&self) -> Rulesets;
    // fn get_media(&self) -> Media;
    // fn get_clients(&self) -> Clients;
    // fn get_game(&self) -> Game;
    // fn get_prepared_team(&self) -> PreparedTeam;
    // fn get_current_game(&self) -> CurrentGame;
    fn get_jsm(&self) -> JSONStateManager;
    fn use_metrics(&self) -> bool;
    fn is_initial_load_done(&self) -> bool;
}

pub struct ScoreBoardEvent<T> {
    // provider: Arc<Box<dyn ScoreBoardEventProvider>>,
    property: Option<Box<dyn Property<T>>>,
    value: Option<T>,
    previous_value: Option<T>,
    remove: bool,
}

pub trait ScoreBoardListener {
    fn scoreboard_change(event: ScoreBoardEvent<Box<dyn Any>>);
}

// Source: src/com/carolinarollergirls/scoreboard/event/ScoreBoardEventProvider.java
pub enum Source {
    WS,
    Autosave,
    JSON,
    InverseReference,
    Copy,
    Recalculate,
    Unlink,
    Renumber,
    Other,

    // the following are intended for use as writeProtection Override only;
    AnyInternal,
    AnyFile,
    NonWS,
}

impl Source {
    pub fn internal(&self) -> bool {
        match self {
            Source::WS => false,
            Source::Autosave => false,
            Source::JSON => false,
            Source::InverseReference => true,
            Source::Copy => true,
            Source::Recalculate => true,
            Source::Unlink => true,
            Source::Renumber => true,
            Source::Other => true,
            Source::AnyInternal => true,
            Source::AnyFile => false,
            Source::NonWS => true,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Source::WS => false,
            Source::Autosave => true,
            Source::JSON => true,
            Source::InverseReference => false,
            Source::Copy => false,
            Source::Recalculate => false,
            Source::Unlink => false,
            Source::Renumber => false,
            Source::Other => false,
            Source::AnyInternal => false,
            Source::AnyFile => true,
            Source::NonWS => true,
        }
    }
}

// Source: src/com/carolinarollergirls/scoreboard/event/ScoreBoardEventProvider.java
/// Flags that affect how an update from the frontend is processed
pub enum Flag {
    Change,
    Reset,
    SpecialCase,
}
