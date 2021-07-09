use futures_util::future::{self, FutureExt, TryFutureExt};

use crate::{Backend, Entity};

use super::{
    GetEntityFuture, ListEntitiesFuture, RemoveEntitiesFuture, RemoveEntityFuture,
    UpsertEntitiesFuture, UpsertEntityFuture,
};

pub trait Repository<E: Entity, B: Backend> {
    fn backend(&self) -> B;

    fn get(&self, entity_id: E::Id) -> GetEntityFuture<'_, E, B::Error>;

    fn list(&self) -> ListEntitiesFuture<'_, E, B::Error>;

    fn remove(&self, entity_id: E::Id) -> RemoveEntityFuture<'_, B::Error>;

    fn remove_bulk<T: Iterator<Item = E::Id>>(
        &self,
        entity_ids: T,
    ) -> RemoveEntitiesFuture<'_, B::Error> {
        future::try_join_all(entity_ids.map(|id| self.remove(id)))
            .map_ok(|_| ())
            .boxed()
    }

    fn upsert(&self, entity: E) -> UpsertEntityFuture<'_, B::Error>;

    fn upsert_bulk<T: Iterator<Item = E> + Send>(
        &self,
        entities: T,
    ) -> UpsertEntitiesFuture<'_, B::Error> {
        Box::pin(future::try_join_all(entities.map(|entity| self.upsert(entity))).map_ok(|_| ()))
    }
}

pub trait SingleEntityRepository<E: Entity, B: Backend> {
    fn backend(&self) -> B;

    fn get(&self) -> GetEntityFuture<'_, E, B::Error>;

    fn remove(&self) -> RemoveEntityFuture<'_, B::Error>;

    fn upsert(&self) -> UpsertEntityFuture<'_, B::Error>;
}
