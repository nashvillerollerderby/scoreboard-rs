use crate::state::StateTrie;

pub trait JSONStateListener {
    /// A snapshot of the current state, and which keys in it have changed.
    ///
    /// Keys with a value of null are considered deleted, and will only be present in changed, not in state.
    fn send_updates(state: StateTrie, changes: StateTrie);
}
