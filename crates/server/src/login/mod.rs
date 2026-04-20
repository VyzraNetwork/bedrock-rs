mod handler;

use crate::error::LoginError;
use crate::login::handler::LoginHandler;
use bedrockrs_network::connection::Connection;
use bedrockrs_proto::V975;
use shipyard::World;

pub async fn login(
    _connection: Connection<V975>,
    _world: &mut World,
    _login_handler: impl LoginHandler,
) -> Result<(), LoginError> {
    // let mut shard = shard::<V729>(connection);

    todo!()
}
