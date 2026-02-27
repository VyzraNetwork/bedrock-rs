use crate::v662::V662;
use crate::version::proto_version::ProtoVersionPackets;
use bedrockrs_macros::gamepackets;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::sub_client::SubClientID;
use std::io::{Cursor, Write};
use varint_rs::{VarintReader, VarintWriter};

gamepackets! {
    Login: <V662 as ProtoVersionPackets>::LoginPacket,
    PlayStatus: <V662 as ProtoVersionPackets>::PlayStatusPacket,
    ServerToClientHandshake: <V662 as ProtoVersionPackets>::ServerToClientHandshakePacket,
    ClientToServerHandshake: <V662 as ProtoVersionPackets>::ClientToServerHandshakePacket,
    Disconnect: <V662 as ProtoVersionPackets>::DisconnectPacket,
    ResourcePacksInfo: <V662 as ProtoVersionPackets>::ResourcePacksInfoPacket,
    ResourcePackStack: <V662 as ProtoVersionPackets>::ResourcePackStackPacket,
    ResourcePackClientResponse: <V662 as ProtoVersionPackets>::ResourcePackClientResponsePacket,
    Text: <V662 as ProtoVersionPackets>::TextPacket,
    SetTime: <V662 as ProtoVersionPackets>::SetTimePacket,
    StartGame: <V662 as ProtoVersionPackets>::StartGamePacket,
    AddPlayer: <V662 as ProtoVersionPackets>::AddPlayerPacket,
    AddActor: <V662 as ProtoVersionPackets>::AddActorPacket,
    RemoveActor: <V662 as ProtoVersionPackets>::RemoveActorPacket,
    AddItemActor: <V662 as ProtoVersionPackets>::AddItemActorPacket,
    ServerPlayerPostMovePosition: <V662 as ProtoVersionPackets>::ServerPlayerPostMovePositionPacket,
    TakeItemActor: <V662 as ProtoVersionPackets>::TakeItemActorPacket,
    MoveActorAbsolute: <V662 as ProtoVersionPackets>::MoveActorAbsolutePacket,
    MovePlayer: <V662 as ProtoVersionPackets>::MovePlayerPacket,
    PassengerJump: <V662 as ProtoVersionPackets>::PassengerJumpPacket,
    UpdateBlock: <V662 as ProtoVersionPackets>::UpdateBlockPacket,
    AddPainting: <V662 as ProtoVersionPackets>::AddPaintingPacket,
    TickSync: <V662 as ProtoVersionPackets>::TickSyncPacket,
    LevelSoundEventV1: <V662 as ProtoVersionPackets>::LevelSoundEventV1Packet,
    LevelEvent: <V662 as ProtoVersionPackets>::LevelEventPacket,
    BlockEvent: <V662 as ProtoVersionPackets>::BlockEventPacket,
    ActorEvent: <V662 as ProtoVersionPackets>::ActorEventPacket,
    MobEffect: <V662 as ProtoVersionPackets>::MobEffectPacket,
    UpdateAttributes: <V662 as ProtoVersionPackets>::UpdateAttributesPacket,
    InventoryTransaction: <V662 as ProtoVersionPackets>::InventoryTransactionPacket,
    MobEquipment: <V662 as ProtoVersionPackets>::MobEquipmentPacket,
    MobArmorEquipment: <V662 as ProtoVersionPackets>::MobArmorEquipmentPacket,
    Interact: <V662 as ProtoVersionPackets>::InteractPacket,
    BlockPickRequest: <V662 as ProtoVersionPackets>::BlockPickRequestPacket,
    ActorPickRequest: <V662 as ProtoVersionPackets>::ActorPickRequestPacket,
    PlayerAction: <V662 as ProtoVersionPackets>::PlayerActionPacket,
    HurtArmor: <V662 as ProtoVersionPackets>::HurtArmorPacket,
    SetActorData: <V662 as ProtoVersionPackets>::SetActorDataPacket,
    SetActorMotion: <V662 as ProtoVersionPackets>::SetActorMotionPacket,
    SetActorLink: <V662 as ProtoVersionPackets>::SetActorLinkPacket,
    SetHealth: <V662 as ProtoVersionPackets>::SetHealthPacket,
    SetSpawnPosition: <V662 as ProtoVersionPackets>::SetSpawnPositionPacket,
    Animate: <V662 as ProtoVersionPackets>::AnimatePacket,
    Respawn: <V662 as ProtoVersionPackets>::RespawnPacket,
    ContainerOpen: <V662 as ProtoVersionPackets>::ContainerOpenPacket,
    ContainerClose: <V662 as ProtoVersionPackets>::ContainerClosePacket,
    PlayerHotbar: <V662 as ProtoVersionPackets>::PlayerHotbarPacket,
    InventoryContent: <V662 as ProtoVersionPackets>::InventoryContentPacket,
    InventorySlot: <V662 as ProtoVersionPackets>::InventorySlotPacket,
    ContainerSetData: <V662 as ProtoVersionPackets>::ContainerSetDataPacket,
    CraftingData: <V662 as ProtoVersionPackets>::CraftingDataPacket,
    GuiDataPickItem: <V662 as ProtoVersionPackets>::GuiDataPickItemPacket,
    BlockActorData: <V662 as ProtoVersionPackets>::BlockActorDataPacket,
    PlayerInput: <V662 as ProtoVersionPackets>::PlayerInputPacket,
    LevelChunk: <V662 as ProtoVersionPackets>::LevelChunkPacket,
    SetCommandsEnabled: <V662 as ProtoVersionPackets>::SetCommandsEnabledPacket,
    SetDifficulty: <V662 as ProtoVersionPackets>::SetDifficultyPacket,
    ChangeDimension: <V662 as ProtoVersionPackets>::ChangeDimensionPacket,
    SetPlayerGameType: <V662 as ProtoVersionPackets>::SetPlayerGameTypePacket,
    PlayerList: <V662 as ProtoVersionPackets>::PlayerListPacket,
    SimpleEvent: <V662 as ProtoVersionPackets>::SimpleEventPacket,
    LegacyTelemetryEvent: <V662 as ProtoVersionPackets>::LegacyTelemetryEventPacket,
    SpawnExperienceOrb: <V662 as ProtoVersionPackets>::SpawnExperienceOrbPacket,
    ClientboundMapItemData: <V662 as ProtoVersionPackets>::ClientBoundMapItemDataPacket,
    MapInfoRequest: <V662 as ProtoVersionPackets>::MapInfoRequestPacket,
    RequestChunkRadius: <V662 as ProtoVersionPackets>::RequestChunkRadiusPacket,
    ChunkRadiusUpdated: <V662 as ProtoVersionPackets>::ChunkRadiusUpdatedPacket,
    GameRulesChanged: <V662 as ProtoVersionPackets>::GameRulesChangedPacket,
    Camera: <V662 as ProtoVersionPackets>::CameraPacket,
    BossEvent: <V662 as ProtoVersionPackets>::BossEventPacket,
    ShowCredits: <V662 as ProtoVersionPackets>::ShowCreditsPacket,
    AvailableCommands: <V662 as ProtoVersionPackets>::AvailableCommandsPacket,
    CommandRequest: <V662 as ProtoVersionPackets>::CommandRequestPacket,
    CommandBlockUpdate: <V662 as ProtoVersionPackets>::CommandBlockUpdatePacket,
    CommandOutput: <V662 as ProtoVersionPackets>::CommandOutputPacket,
    UpdateTrade: <V662 as ProtoVersionPackets>::UpdateTradePacket,
    UpdateEquip: <V662 as ProtoVersionPackets>::UpdateEquipPacket,
    ResourcePackDataInfo: <V662 as ProtoVersionPackets>::ResourcePackDataInfoPacket,
    ResourcePackChunkData: <V662 as ProtoVersionPackets>::ResourcePackChunkDataPacket,
    ResourcePackChunkRequest: <V662 as ProtoVersionPackets>::ResourcePackChunkRequestPacket,
    TransferPlayer: <V662 as ProtoVersionPackets>::TransferPlayerPacket,
    PlaySound: <V662 as ProtoVersionPackets>::PlaySoundPacket,
    StopSound: <V662 as ProtoVersionPackets>::StopSoundPacket,
    SetTitle: <V662 as ProtoVersionPackets>::SetTitlePacket,
    AddBehaviourTree: <V662 as ProtoVersionPackets>::AddBehaviourTreePacket,
    StructureBlockUpdate: <V662 as ProtoVersionPackets>::StructureBlockUpdatePacket,
    ShowStoreOffer: <V662 as ProtoVersionPackets>::ShowStoreOfferPacket,
    PurchaseReceipt: <V662 as ProtoVersionPackets>::PurchaseReceiptPacket,
    PlayerSkin: <V662 as ProtoVersionPackets>::PlayerSkinPacket,
    SubClientLogin: <V662 as ProtoVersionPackets>::SubClientLoginPacket,
    AutomationClientConnect: <V662 as ProtoVersionPackets>::AutomationClientConnectPacket,
    SetLastHurtBy: <V662 as ProtoVersionPackets>::SetLastHurtByPacket,
    BookEdit: <V662 as ProtoVersionPackets>::BookEditPacket,
    NpcRequest: <V662 as ProtoVersionPackets>::NpcRequestPacket,
    PhotoTransfer: <V662 as ProtoVersionPackets>::PhotoTransferPacket,
    ModalFormRequest: <V662 as ProtoVersionPackets>::ModalFormRequestPacket,
    ModalFormResponse: <V662 as ProtoVersionPackets>::ModalFormResponsePacket,
    ServerSettingsRequest: <V662 as ProtoVersionPackets>::ServerSettingsRequestPacket,
    ServerSettingsResponse: <V662 as ProtoVersionPackets>::ServerSettingsResponsePacket,
    ShowProfile: <V662 as ProtoVersionPackets>::ShowProfilePacket,
    SetDefaultGameType: <V662 as ProtoVersionPackets>::SetDefaultGameTypePacket,
    RemoveObjective: <V662 as ProtoVersionPackets>::RemoveObjectivePacket,
    SetDisplayObjective: <V662 as ProtoVersionPackets>::SetDisplayObjectivePacket,
    SetScore: <V662 as ProtoVersionPackets>::SetScorePacket,
    LabTable: <V662 as ProtoVersionPackets>::LabTablePacket,
    UpdateBlockSynced: <V662 as ProtoVersionPackets>::UpdateBlockSyncedPacket,
    MoveActorDelta: <V662 as ProtoVersionPackets>::MoveActorDeltaPacket,
    SetScoreboardIdentity: <V662 as ProtoVersionPackets>::SetScoreboardIdentityPacket,
    SetLocalPlayerAsInitialized: <V662 as ProtoVersionPackets>::SetLocalPlayerAsInitializedPacket,
    UpdateSoftEnum: <V662 as ProtoVersionPackets>::UpdateSoftEnumPacket,
    NetworkStackLatency: <V662 as ProtoVersionPackets>::NetworkStackLatencyPacket,
    SpawnParticleEffect: <V662 as ProtoVersionPackets>::SpawnParticleEffectPacket,
    AvailableActorIdentifiers: <V662 as ProtoVersionPackets>::AvailableActorIdentifiersPacket,
    LevelSoundEventV2: <V662 as ProtoVersionPackets>::LevelSoundEventV2Packet,
    NetworkChunkPublisherUpdate: <V662 as ProtoVersionPackets>::NetworkChunkPublisherUpdatePacket,
    BiomeDefinitionList: <V662 as ProtoVersionPackets>::BiomeDefinitionListPacket,
    LevelSoundEvent: <V662 as ProtoVersionPackets>::LevelSoundEventPacket,
    LevelEventGeneric: <V662 as ProtoVersionPackets>::LevelEventGenericPacket,
    LecternUpdate: <V662 as ProtoVersionPackets>::LecternUpdatePacket,
    ClientCacheStatus: <V662 as ProtoVersionPackets>::ClientCacheStatusPacket,
    OnScreenTextureAnimation: <V662 as ProtoVersionPackets>::OnScreenTextureAnimationPacket,
    MapCreateLockedCopy: <V662 as ProtoVersionPackets>::MapCreateLockedCopyPacket,
    StructureDataRequest: <V662 as ProtoVersionPackets>::StructureDataRequestPacket,
    StructureDataResponse: <V662 as ProtoVersionPackets>::StructureDataResponsePacket,
    ClientCacheBlobStatus: <V662 as ProtoVersionPackets>::ClientCacheBlobStatusPacket,
    ClientCacheMissResponse: <V662 as ProtoVersionPackets>::ClientCacheMissResponsePacket,
    EducationSettings: <V662 as ProtoVersionPackets>::EducationSettingsPacket,
    Emote: <V662 as ProtoVersionPackets>::EmotePacket,
    MultiplayerSettings: <V662 as ProtoVersionPackets>::MultiplayerSettingsPacket,
    SettingsCommand: <V662 as ProtoVersionPackets>::SettingsCommandPacket,
    AnvilDamage: <V662 as ProtoVersionPackets>::AnvilDamagePacket,
    CompletedUsingItem: <V662 as ProtoVersionPackets>::CompletedUsingItemPacket,
    NetworkSettings: <V662 as ProtoVersionPackets>::NetworkSettingsPacket,
    PlayerAuthInput: <V662 as ProtoVersionPackets>::PlayerAuthInputPacket,
    CreativeContent: <V662 as ProtoVersionPackets>::CreativeContentPacket,
    PlayerEnchantOptions: <V662 as ProtoVersionPackets>::PlayerEnchantOptionsPacket,
    ItemStackRequest: <V662 as ProtoVersionPackets>::ItemStackRequestPacket,
    ItemStackResponse: <V662 as ProtoVersionPackets>::ItemStackResponsePacket,
    PlayerArmorDamage: <V662 as ProtoVersionPackets>::PlayerArmorDamagePacket,
    CodeBuilder: <V662 as ProtoVersionPackets>::CodeBuilderPacket,
    UpdatePlayerGameType: <V662 as ProtoVersionPackets>::UpdatePlayerGameTypePacket,
    EmoteList: <V662 as ProtoVersionPackets>::EmoteListPacket,
    PositionTrackingDbServerBroadcast: <V662 as ProtoVersionPackets>::PositionTrackingDBServerBroadcastPacket,
    PositionTrackingDbClientRequest: <V662 as ProtoVersionPackets>::PositionTrackingDBClientRequestPacket,
    DebugInfo: <V662 as ProtoVersionPackets>::DebugInfoPacket,
    PacketViolationWarning: <V662 as ProtoVersionPackets>::PacketViolationWarningPacket,
    MotionPredictionHints: <V662 as ProtoVersionPackets>::MotionPredictionHintsPacket,
    AnimateEntity: <V662 as ProtoVersionPackets>::AnimateEntityPacket,
    CameraShake: <V662 as ProtoVersionPackets>::CameraShakePacket,
    PlayerFog: <V662 as ProtoVersionPackets>::PlayerFogPacket,
    CorrectPlayerMovePrediction: <V662 as ProtoVersionPackets>::CorrectPlayerMovePredictionPacket,
    ItemComponent: <V662 as ProtoVersionPackets>::ItemComponentPacket,
    FilterText: <V662 as ProtoVersionPackets>::FilterTextPacket,
    ClientboundDebugRenderer: <V662 as ProtoVersionPackets>::ClientBoundDebugRendererPacket,
    SyncActorProperty: <V662 as ProtoVersionPackets>::SyncActorPropertyPacket,
    AddVolumeEntity: <V662 as ProtoVersionPackets>::AddVolumeEntityPacket,
    RemoveVolumeEntity: <V662 as ProtoVersionPackets>::RemoveVolumeEntityPacket,
    SimulationType: <V662 as ProtoVersionPackets>::SimulationTypePacket,
    NpcDialogue: <V662 as ProtoVersionPackets>::NpcDialoguePacket,
    EduUriResource: <V662 as ProtoVersionPackets>::EduUriResourcePacket,
    CreatePhoto: <V662 as ProtoVersionPackets>::CreatePhotoPacket,
    UpdateSubChunkBlocks: <V662 as ProtoVersionPackets>::UpdateSubChunkBlocksPacket,
    SubChunk: <V662 as ProtoVersionPackets>::SubChunkPacket,
    SubChunkRequest: <V662 as ProtoVersionPackets>::SubChunkRequestPacket,
    PlayerStartItemCooldown: <V662 as ProtoVersionPackets>::PlayerStartItemCooldownPacket,
    ScriptMessage: <V662 as ProtoVersionPackets>::ScriptMessagePacket,
    CodeBuilderSource: <V662 as ProtoVersionPackets>::CodeBuilderSourcePacket,
    TickingAreaLoadStatus: <V662 as ProtoVersionPackets>::TickingAreaLoadStatusPacket,
    DimensionData: <V662 as ProtoVersionPackets>::DimensionDataPacket,
    AgentActionEvent: <V662 as ProtoVersionPackets>::AgentActionEventPacket,
    ChangeMobProperty: <V662 as ProtoVersionPackets>::ChangeMobPropertyPacket,
    LessonProgress: <V662 as ProtoVersionPackets>::LessonProgressPacket,
    RequestAbility: <V662 as ProtoVersionPackets>::RequestAbilityPacket,
    RequestPermissions: <V662 as ProtoVersionPackets>::RequestPermissionsPacket,
    ToastRequest: <V662 as ProtoVersionPackets>::ToastRequestPacket,
    UpdateAbilities: <V662 as ProtoVersionPackets>::UpdateAbilitiesPacket,
    UpdateAdventureSettings: <V662 as ProtoVersionPackets>::UpdateAdventureSettingsPacket,
    DeathInfo: <V662 as ProtoVersionPackets>::DeathInfoPacket,
    EditorNetwork: <V662 as ProtoVersionPackets>::EditorNetworkPacket,
    FeatureRegistry: <V662 as ProtoVersionPackets>::FeatureRegistryPacket,
    ServerStats: <V662 as ProtoVersionPackets>::ServerStatsPacket,
    RequestNetworkSettings: <V662 as ProtoVersionPackets>::RequestNetworkSettingsPacket,
    GameTestRequest: <V662 as ProtoVersionPackets>::GameTestRequestPacket,
    GameTestResults: <V662 as ProtoVersionPackets>::GameTestResultsPacket,
    UpdateClientInputLocks: <V662 as ProtoVersionPackets>::UpdateClientInputLocksPacket,
    CameraPresets: <V662 as ProtoVersionPackets>::CameraPresetsPacket,
    UnlockedRecipes: <V662 as ProtoVersionPackets>::UnlockedRecipesPacket,
    CameraInstruction: <V662 as ProtoVersionPackets>::CameraInstructionPacket,
    CompressedBiomeDefinitionList: <V662 as ProtoVersionPackets>::CompressedBiomeDefinitionListPacket,
    TrimData: <V662 as ProtoVersionPackets>::TrimDataPacket,
    OpenSign: <V662 as ProtoVersionPackets>::OpenSignPacket,
    AgentAnimation: <V662 as ProtoVersionPackets>::AgentAnimationPacket,
    RefreshEntitlements: <V662 as ProtoVersionPackets>::RefreshEntitlementsPacket,
    PlayerToggleCrafterSlotRequest: <V662 as ProtoVersionPackets>::PlayerToggleCrafterSlotRequestPacket,
    SetPlayerInventoryOptions: <V662 as ProtoVersionPackets>::SetPlayerInventoryOptionsPacket,
    SetHud: <V662 as ProtoVersionPackets>::SetHudPacket
}

pub fn read_gamepacket_header(
    stream: &mut Cursor<&[u8]>,
) -> Result<(u32, u16, SubClientID, SubClientID), ProtoCodecError> {
    // Read the gamepacket length
    let length = stream.read_u32_varint()?;

    // Read the gamepacket header and parse it into an u16
    // Since the (var)int is only storing 14 bytes we can treat it as an u16
    // This is normally treated as u32 varint
    let gamepacket_header: u16 = stream.read_u16_varint()?;

    // Get the first 10 bits as the packet id
    // Can never be more than a 16-bit integer due to being 10-bits big
    // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
    let gamepacket_id = gamepacket_header & 0b0000_0011_1111_1111;

    // Get the next 2 bits as the sub client sender id
    // Can never be more than an 8-bit integer due to being 2 bits big
    let subclient_sender_id =
        SubClientID::try_from(((gamepacket_header & 0b0000_1100_0000_0000) >> 10) as u8)?;
    // Get the next 2 bits as the sub client target id
    // Never more than an 8-bit integer due to being 2 bits big
    let subclient_target_id =
        SubClientID::try_from(((gamepacket_header & 0b0011_0000_0000_0000) >> 12) as u8)?;

    Ok((
        length,
        gamepacket_id,
        subclient_sender_id,
        subclient_target_id,
    ))
}

pub fn write_gamepacket_header(
    stream: &mut Vec<u8>,
    length: u32,
    gamepacket_id: u16,
    subclient_sender_id: SubClientID,
    subclient_target_id: SubClientID,
) -> Result<(), ProtoCodecError> {
    // Since the (var)int is only storing 14 bytes, we can treat it as an u16
    // This is normally treated as u32 varint
    let mut gamepacket_header: u16 = 0;

    // Set the first 10 bits as the packet id
    // Can never be more than a 16-bit integer due to being 10-bits big
    // Gamepacket IDs through 200-299 are used for spin-offs, they are free to use for custom packets
    gamepacket_header |= 0b0000_0011_1111_1111 & gamepacket_id;

    // Set the next 2 bits as the sub client sender id
    // Never more than an 8-bit integer due to being 2 bits big
    gamepacket_header |= (Into::<u16>::into(subclient_sender_id) >> 10) & 0b0000_1100_0000_0000;
    // Set the next 2 bits as the sub client target id
    // Never more than an 8-bit integer due to being 2 bits big
    gamepacket_header |= (Into::<u16>::into(subclient_target_id) >> 12) & 0b0011_0000_0000_0000;

    // Since the size of the header is also included in the batched packet size,
    // we need to write it to a temporary buffer
    let mut gamepacket_header_buf = Vec::new();

    // Write the gamepacket header into temporary buffer
    gamepacket_header_buf.write_u16_varint(gamepacket_header)?;

    // Write the gamepacket length and the header length
    stream.write_u32_varint(length + gamepacket_header_buf.len() as u32)?;

    // Write the final game packet header
    stream.write_all(gamepacket_header_buf.as_slice())?;

    Ok(())
}

pub const fn get_gamepacket_header_size_prediction() -> usize {
    // 2 = gamepacket header (varint u32, only 14 bites can be treated as an u16)
    // 4 = gamepacket length size (varint u32)
    2 + 4
}
