/// In this folder the redux pattern will not necessarily be one on one, considering actions and reducers.
/// Some Actions will use reducers from multiple different features.
/// Some Reducers will be used by actions across multiple features.
/// Therefore, the actions and reducers are not necessarily grouped together in this folder.

pub mod actions;
pub mod reducers;
