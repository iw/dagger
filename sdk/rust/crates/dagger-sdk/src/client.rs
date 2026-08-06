//! Owned public client facade and the temporary generated-binding migration shim.
//!
//! The stable facade is a cloneable lease on one shared session. Its raw execution and
//! close behavior are available with generated bindings disabled. The callback helper
//! remains hidden while generated handles move to the same lease; it is not a second
//! stable lifecycle model.

use crate::config::ClientConfig;
use crate::connector::{ConnectionRequest, Connector, DefaultConnector};
use crate::errors::{CloseError, ConnectError, RequestError};
use crate::graphql::{RawRequest, RawResponse};
use crate::lifecycle::SessionHandle;
use crate::preflight::{ConnectionPlan, preflight};

/// A cloneable owned lease on one Dagger engine session.
///
/// Cloning a client does not open another connection. Explicit [`Client::close`] calls
/// across all clones share one terminal result; dropping the final clone starts
/// non-blocking cleanup when close was not called.
#[derive(Clone)]
pub struct Client {
    session: SessionHandle,
}

impl Client {
    /// Executes one raw GraphQL request through this client's shared session.
    ///
    /// Requests admitted before close may finish or return
    /// [`RequestError::InterruptedByClose`]. Once close begins, new requests fail with
    /// [`RequestError::ClientClosed`] without invoking the connection.
    pub async fn execute(&self, request: RawRequest) -> Result<RawResponse, RequestError> {
        self.session.execute(request).await
    }

    /// Gracefully closes the shared session and returns its reusable terminal result.
    ///
    /// This method is cancellation-safe: abandoning one waiter does not cancel the
    /// SDK-owned shutdown attempt, and a later caller observes the same result.
    pub async fn close(&self) -> Result<(), CloseError> {
        self.session.close().await
    }

    #[cfg(test)]
    pub(crate) fn lifecycle_state(&self) -> &'static str {
        self.session.lifecycle_state()
    }
}

/// Connects with the documented default configuration.
pub async fn connect() -> Result<Client, ConnectError> {
    connect_with(ClientConfig::default()).await
}

/// Consumes a validated configuration and returns one owned client.
pub async fn connect_with(config: ClientConfig) -> Result<Client, ConnectError> {
    connect_with_connector(config, &DefaultConnector).await
}

pub(crate) async fn connect_with_connector(
    config: ClientConfig,
    connector: &dyn Connector,
) -> Result<Client, ConnectError> {
    let plan = preflight(config)?;
    if let ConnectionPlan::Explicit {
        connection,
        execution_timeout,
    } = plan
    {
        // The explicit path intentionally returns before constructing a connector
        // request. This preserves caller injection as a complete abstraction and
        // prevents environment, discovery, process, or network side effects.
        return Ok(Client {
            session: SessionHandle::new(connection, None, execution_timeout),
        });
    }

    let request = ConnectionRequest::try_from(plan).map_err(|_| internal_connect_error())?;
    let (startup_timeout, http_connect_timeout, execution_timeout) = request.timeouts();
    let connection = tokio::time::timeout(startup_timeout, connector.connect(request))
        .await
        .map_err(|_| ConnectError::StartupTimeout {
            duration: startup_timeout,
        })??;

    Ok(Client {
        session: SessionHandle::new(connection, Some(http_connect_timeout), execution_timeout),
    })
}

fn internal_connect_error() -> ConnectError {
    ConnectError::Connection(crate::EngineConnectionError::new(
        crate::EngineConnectionErrorKind::Other,
    ))
}

#[cfg(feature = "gen")]
use std::sync::Arc;

#[cfg(feature = "gen")]
use crate::core::config::Config;
#[cfg(feature = "gen")]
use crate::core::engine::Engine as DaggerEngine;
#[cfg(feature = "gen")]
use crate::core::graphql_client::DefaultGraphQLClient;
#[cfg(feature = "gen")]
use crate::errors::DaggerError;
#[cfg(feature = "gen")]
use crate::r#gen::{Id, Query};
#[cfg(feature = "gen")]
use crate::id::IntoID;
#[cfg(feature = "gen")]
use crate::loadable::Loadable;
#[cfg(feature = "gen")]
use crate::logging::StdLogger;
#[cfg(feature = "gen")]
use crate::querybuilder::query;

#[cfg(feature = "gen")]
pub type DaggerConn = Query;

/// Type-erased failure returned by the transitional callback facade.
#[cfg(feature = "gen")]
pub type ConnectCallbackError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Result expected from the transitional [`connect_legacy`] callback.
#[cfg(feature = "gen")]
pub type ConnectCallbackResult = Result<(), ConnectCallbackError>;

/// Runs the beta generated client within its callback-scoped lifecycle.
///
/// This compatibility shim exists only until generated handles use [`Client`]. New raw
/// integrations should use [`connect`] or [`connect_with`].
#[doc(hidden)]
#[cfg(feature = "gen")]
pub async fn connect_legacy<F, Fut>(dagger: F) -> Result<(), ConnectError>
where
    F: FnOnce(DaggerConn) -> Fut + 'static,
    Fut: futures::Future<Output = ConnectCallbackResult> + 'static,
{
    let cfg = Config::builder()
        .logger(Arc::new(StdLogger::default()))
        .build();

    connect_opts(cfg, dagger).await
}

#[doc(hidden)]
#[cfg(feature = "gen")]
pub async fn connect_opts<F, Fut>(cfg: Config, dagger: F) -> Result<(), ConnectError>
where
    F: FnOnce(DaggerConn) -> Fut + 'static,
    Fut: futures::Future<Output = ConnectCallbackResult> + 'static,
{
    let (conn, proc) = DaggerEngine::new()
        .start(&cfg)
        .await
        .map_err(ConnectError::from_legacy_connection)?;

    let proc = proc.map(Arc::new);
    let client = Query {
        proc: proc.clone(),
        selection: query(),
        graphql_client: Arc::new(DefaultGraphQLClient::new(&conn, &cfg)),
    };

    dagger(client).await.map_err(|source| {
        ConnectError::CallbackFailed(crate::EngineConnectionError::with_boxed_source(
            crate::EngineConnectionErrorKind::Other,
            source,
        ))
    })?;

    if let Some(proc) = &proc {
        proc.shutdown()
            .await
            .map_err(ConnectError::from_legacy_close)?;
    }

    Ok(())
}

#[cfg(feature = "gen")]
impl Query {
    /// Returns a lazy reference to a node by its ID without making a network call.
    ///
    /// The returned value can be used to chain further queries.
    ///
    /// ```ignore
    /// let ctr: Container = client.r#ref(id);
    /// let out = ctr.with_exec(vec!["echo", "hi"]).stdout().await?;
    /// ```
    pub fn r#ref<T: Loadable>(&self, id: impl IntoID<Id>) -> T {
        let selection = self
            .selection
            .select("node")
            .arg_lazy(
                "id",
                Box::new(move || {
                    let id = id.clone();
                    Box::pin(async move {
                        let resolved = id.into_id().await.unwrap();
                        format!("\"{}\"", resolved.0)
                    })
                }),
            )
            .inline_fragment(T::graphql_type());

        T::from_query(self.proc.clone(), selection, self.graphql_client.clone())
    }

    /// Loads a node by ID after verifying that it exists with the expected type.
    pub async fn load<T: Loadable>(&self, id: impl IntoID<Id>) -> Result<T, DaggerError> {
        let type_name = T::graphql_type();
        // Asking for an ID through a concrete inline fragment makes a missing or
        // mismatched node fail before constructing the caller's typed handle.
        let check_selection = self
            .selection
            .select("node")
            .arg_lazy("id", {
                let id = id.clone();
                Box::new(move || {
                    let id = id.clone();
                    Box::pin(async move {
                        let resolved = id.into_id().await.unwrap();
                        format!("\"{}\"", resolved.0)
                    })
                })
            })
            .inline_fragment(type_name)
            .select("id");

        let _: Id = check_selection.execute(self.graphql_client.clone()).await?;
        Ok(self.r#ref(id))
    }
}

#[cfg(all(test, feature = "gen"))]
mod test {
    use super::connect_legacy;

    #[tokio::test]
    async fn test_connect() -> eyre::Result<()> {
        tracing_subscriber::fmt::init();

        connect_legacy(|client| async move {
            client
                .container()
                .from("alpine:latest")
                .with_exec(vec!["echo", "1"])
                .sync()
                .await?;
            Ok(())
        })
        .await?;

        Ok(())
    }
}
