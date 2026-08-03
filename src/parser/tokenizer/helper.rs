pub(super) fn current<T>(state_history: &[T]) -> &T {
    state_history.last().unwrap()
}
