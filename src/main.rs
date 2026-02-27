use axum::response::Redirect;
use axum::routing::get;
use axum::{Router, response::IntoResponse};
use axum_login::tower_sessions::{MemoryStore, SessionManagerLayer};
use axum_login::{AuthManagerLayerBuilder, login_required};

use crate::auth::Backend;

mod auth {
    use axum_login::{AuthUser, AuthnBackend};
    use std::convert::Infallible;

    #[allow(unused)]
    #[derive(Debug, Clone)]
    pub(crate) struct User {}

    impl AuthUser for User {
        type Id = String;

        fn id(&self) -> Self::Id {
            todo!()
        }

        fn session_auth_hash(&self) -> &[u8] {
            todo!()
        }
    }

    #[allow(unused)]
    #[derive(Clone, Default)]
    pub(crate) struct Backend {}

    impl AuthnBackend for Backend {
        type User = User;
        type Credentials = String;
        type Error = Infallible;

        async fn authenticate(
            &self,
            _creds: Self::Credentials,
        ) -> Result<Option<Self::User>, Self::Error> {
            todo!()
        }

        async fn get_user(
            &self,
            _user_id: &axum_login::UserId<Self>,
        ) -> Result<Option<Self::User>, Self::Error> {
            todo!()
        }
    }
}

#[allow(unused)]
type AuthSession = axum_login::AuthSession<Backend>;

#[allow(unused)]
async fn get_location(auth_session: AuthSession) -> impl IntoResponse {
    Redirect::to("/testing").into_response()
}

#[allow(unused)]
fn app() -> Router {
    // Session layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth serivce
    let backend = Backend::default();
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    Router::new()
        .route("/location", get(get_location))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .layer(auth_layer)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn it_should_return_location_redirect_for_mock() {
        let server = TestServer::builder().mock_transport().build(app());
        let location = server.get("/location").await.header("location");
        assert_eq!(location, "/login");
    }

    #[tokio::test]
    async fn it_should_return_location_redirect_for_http() {
        let server = TestServer::builder().http_transport().build(app());
        let location = server.get("/location").await.header("location");
        assert_eq!(location, "/login");
    }
}
