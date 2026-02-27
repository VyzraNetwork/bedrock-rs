use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::proto_version::ProtoVersion;

#[gamepacket(id = 181)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct AgentActionEventPacket<V: ProtoVersion> {
    pub request_id: String,
    pub action_type: V::AgentActionType,
    pub response: String,
}