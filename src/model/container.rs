use derivative::Derivative;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// A sequential container with type-checked indices and id string based access.
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct Container<T> {
    values: Vec<T>,
    ids: HashMap<String, Id<T>>,
}

///
#[derive(Derivative)]
#[derivative(
    Clone(bound = ""),
    Copy(bound = ""),
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Id<T>(pub(crate) usize, PhantomData<T>);

impl<T> Container<T> {
    /// Create a new empty Conatiner.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new value with id `id_str`. The id can not alias other ids.
    pub(crate) fn insert(&mut self, id_str: impl ToString, value: T) -> Id<T> {
        let id = Id::new(self.values.len());

        let id_str = id_str.to_string();
        assert!(self.ids.get(&id_str).is_none());
        self.ids.insert(id_str, id);

        self.values.push(value);
        id
    }

    /// Get an id based on the id string.
    pub fn id(&self, id_str: &str) -> Id<T> {
        self.ids[id_str]
    }

    /// Get a reference to the value with the give index.
    pub fn get(&self, id: Id<T>) -> &T {
        &self.values[id.0]
    }

    /// Get a mutable reference to the value with the give index.
    pub(crate) fn get_mut(&mut self, id: Id<T>) -> &mut T {
        &mut self.values[id.0]
    }

    /// Iterate over references to all values in the container.
    pub fn iter(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.values
            .iter()
            .enumerate()
            .map(|(id, v)| (Id::new(id), v))
    }
}

impl<T> Index<Id<T>> for Container<T> {
    type Output = T;

    fn index(&self, index: Id<T>) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<Id<T>> for Container<T> {
    fn index_mut(&mut self, index: Id<T>) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> Id<T> {
    pub(crate) fn new(id: usize) -> Self {
        Self(id, PhantomData)
    }
}
