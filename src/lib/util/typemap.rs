use std::{
    any::{Any, TypeId},
    collections::hash_map::{
        Entry as HashMapEntry, HashMap, OccupiedEntry as HashMapOccupiedEntry,
        VacantEntry as HashMapVacantEntry,
    },
    marker::PhantomData,
};

pub trait Key: Any {
    type Value: Send + Sync;
}

#[derive(Debug, Default)]
pub struct TypeMap {
    data: HashMap<TypeId, Box<(dyn Any + Send + Sync)>>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn contains_key<T>(&self) -> bool
    where
        T: Key,
    {
        self.data.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T>(&mut self, value: T::Value)
    where
        T: Key,
    {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn entry<T>(&mut self) -> Entry<'_, T>
    where
        T: Key,
    {
        self.data.entry(TypeId::of::<T>()).into()
    }

    pub fn get<T>(&self) -> Option<&T::Value>
    where
        T: Key,
    {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T::Value>())
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut T::Value>
    where
        T: Key,
    {
        self.data
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut::<T::Value>())
    }

    pub fn remove<T>(&mut self) -> Option<T::Value>
    where
        T: Key,
    {
        self.data
            .remove(&TypeId::of::<T>())
            .and_then(|b| (b as Box<dyn Any>).downcast::<T::Value>().ok())
            .map(|b| *b)
    }
}

pub enum Entry<'a, K>
where
    K: Key,
{
    Occupied(OccupiedEntry<'a, K>),
    Vacant(VacantEntry<'a, K>),
}

impl<'a, K> Entry<'a, K>
where
    K: Key,
{
    pub fn or_insert(self, value: K::Value) -> &'a mut K::Value {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(value),
        }
    }

    pub fn or_insert_with<F>(self, f: F) -> &'a mut K::Value
    where
        F: FnOnce() -> K::Value,
    {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(f()),
        }
    }

    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut K::Value),
    {
        if let Self::Occupied(mut entry) = self {
            f(entry.get_mut());
            Self::Occupied(entry)
        } else {
            self
        }
    }
}

impl<'a, K> Entry<'a, K>
where
    K: Key,
    K::Value: Default,
{
    pub fn or_default(self) -> &'a mut K::Value {
        self.or_insert_with(<K::Value as Default>::default)
    }
}

impl<'a, K> From<HashMapEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>> for Entry<'a, K>
where
    K: Key,
{
    fn from(entry: HashMapEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>) -> Self {
        match entry {
            HashMapEntry::Occupied(entry) => Self::Occupied(entry.into()),
            HashMapEntry::Vacant(entry) => Self::Vacant(entry.into()),
        }
    }
}

pub struct OccupiedEntry<'a, K>
where
    K: Key,
{
    entry: HashMapOccupiedEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>,
    _marker: PhantomData<&'a K::Value>,
}

impl<'a, K> OccupiedEntry<'a, K>
where
    K: Key,
{
    pub fn get(&self) -> &K::Value {
        self.entry.get().downcast_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut K::Value {
        self.entry.get_mut().downcast_mut().unwrap()
    }

    pub fn into_mut(self) -> &'a mut K::Value {
        self.entry.into_mut().downcast_mut().unwrap()
    }

    pub fn insert(&mut self, value: K::Value) {
        self.entry.insert(Box::new(value));
    }

    pub fn remove(self) {
        self.entry.remove();
    }
}

impl<'a, K> From<HashMapOccupiedEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>>
    for OccupiedEntry<'a, K>
where
    K: Key,
{
    fn from(entry: HashMapOccupiedEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>) -> Self {
        Self {
            entry,
            _marker: PhantomData,
        }
    }
}

pub struct VacantEntry<'a, K>
where
    K: Key,
{
    entry: HashMapVacantEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>,
    _marker: PhantomData<&'a K::Value>,
}

impl<'a, K> VacantEntry<'a, K>
where
    K: Key,
{
    pub fn insert(self, value: K::Value) -> &'a mut K::Value {
        self.entry.insert(Box::new(value)).downcast_mut().unwrap()
    }
}

impl<'a, K> From<HashMapVacantEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>>
    for VacantEntry<'a, K>
where
    K: Key,
{
    fn from(entry: HashMapVacantEntry<'a, TypeId, Box<(dyn Any + Send + Sync)>>) -> Self {
        Self {
            entry,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Key, TypeMap};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(TypeMap: Debug, Default);

    struct Counter;

    impl Key for Counter {
        type Value = u64;
    }

    #[test]
    fn counter() {
        let mut map = TypeMap::new();

        map.insert::<Counter>(0);

        assert_eq!(*map.get::<Counter>().unwrap(), 0);

        for _ in 0..100 {
            *map.get_mut::<Counter>().unwrap() += 1;
        }

        assert_eq!(*map.get::<Counter>().unwrap(), 100);
    }

    #[test]
    fn entry() {
        let mut map = TypeMap::new();

        assert_eq!(map.get::<Counter>(), None);
        *map.entry::<Counter>().or_insert(0) += 42;
        assert_eq!(*map.get::<Counter>().unwrap(), 42);
    }

    struct Text;

    impl Key for Text {
        type Value = String;
    }

    #[test]
    fn remove() {
        let mut map = TypeMap::new();

        map.insert::<Text>(String::from("foobar"));
        assert_eq!(map.get::<Text>().unwrap(), "foobar");

        let original = map.remove::<Text>().unwrap();
        assert_eq!(original, "foobar");

        assert!(map.get::<Text>().is_none());
    }

    struct Foo;

    impl Key for Foo {
        type Value = u32;
    }

    #[test]
    fn default() {
        let mut map = TypeMap::new();

        assert_eq!(map.entry::<Foo>().or_default(), &mut 0);
    }
}
