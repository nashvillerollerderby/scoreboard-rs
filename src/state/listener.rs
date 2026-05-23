use crate::state::StateTrie;

pub trait JSONStateListener {
    /// A snapshot of the current state, and which keys in it have changed.
    ///
    /// Keys with a value of null are considered deleted, and will only be present in changed, not in state.
    ///
    /// From the JSONStateListener.java, but with borrowed StateTries and the method is async.
    async fn send_updates(&mut self, state: &StateTrie, changes: &StateTrie);
}
