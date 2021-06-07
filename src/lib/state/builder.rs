use super::{EventHandler, State};
use twilight_cache_inmemory::InMemoryCacheBuilder as CacheBuilder;
use twilight_gateway::{cluster::ClusterBuilder, Intents};
use twilight_http::client::ClientBuilder as HttpBuilder;
use twilight_standby::Standby;

#[derive(Default)]
pub struct StateBuilder {
    cluster: Option<ClusterBuilder>,
    cache: Option<CacheBuilder>,
    http: Option<HttpBuilder>,
    token: Option<String>,
    intents: Option<Intents>,
    event_handler: Option<Box<dyn EventHandler + 'static>>,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self {
            cluster: None,
            cache: None,
            http: None,
            token: None,
            intents: None,
            event_handler: None,
        }
    }

    pub fn token(mut self, token: impl AsRef<str>) -> Self {
        let token = token.as_ref().trim();

        let token = if token.starts_with("Bot ") {
            token.to_string()
        } else {
            format!("Bot {}", token)
        };

        self.token = Some(token.clone());

        self.http = if self.http.is_none() {
            Some(HttpBuilder::new().token(token))
        } else {
            self.http
        };

        self
    }

    pub const fn intents(mut self, intents: Intents) -> Self {
        self.intents = Some(intents);

        self
    }

    pub fn cluster_builder<F>(mut self, cluster_fn: F) -> Self
    where
        F: FnOnce(ClusterBuilder) -> ClusterBuilder,
    {
        let intents = self.intents.expect("Need intents to build cluster");
        let token = self.token.clone().expect("Need token to build cluster");

        let cluster = cluster_fn((token, intents).into());

        self.cluster = Some(cluster);

        self
    }

    pub fn cache_builder<F>(mut self, cache_fn: F) -> Self
    where
        F: FnOnce(CacheBuilder) -> CacheBuilder,
    {
        let built = cache_fn(CacheBuilder::default());

        self.cache = Some(built);

        self
    }

    pub fn event_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.event_handler = Some(Box::new(handler));

        self
    }

    pub fn http_builder<F>(mut self, http_fn: F) -> Self
    where
        F: FnOnce(HttpBuilder) -> HttpBuilder,
    {
        let token = self.token.clone().expect("Need token to build http");
        let http_builder = self
            .http
            .map_or_else(move || HttpBuilder::new().token(token), |builder| builder);
        let http = http_fn(http_builder);

        self.http = Some(http);

        self
    }

    pub async fn build(self) -> super::super::GenericResult<State> {
        let token = self.token.unwrap_or_default();
        let http_builder = self.http.unwrap_or_default();
        let cluster_builder = self.cluster.expect("Need cluster to build state");
        let cache_builder = self.cache.unwrap_or_default();

        let http = http_builder.token(token).build();
        let cache = cache_builder.build();
        let cluster = cluster_builder.http_client(http.clone()).build().await?;
        let standby = Standby::new();
        let event_handler = self.event_handler;

        Ok(State {
            cache,
            cluster,
            http,
            standby,
            event_handler,
        })
    }
}
