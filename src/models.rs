#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponseNodeInfo {
    #[serde(rename = "data")]
    pub data: models::NodeInfo,

    #[serde(rename = "meta")]
    pub meta: models::Metadata,

    #[serde(rename = "status")]
    pub status: models::ResponseStatus,

}

impl ApiResponseNodeInfo {
    pub fn new(data: models::NodeInfo, meta: models::Metadata, status: models::ResponseStatus, ) -> ApiResponseNodeInfo {
        ApiResponseNodeInfo {
            data: data,
            meta: meta,
            status: status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponseNodeSettings {
    #[serde(rename = "data")]
    pub data: models::NodeSettings,

    #[serde(rename = "meta")]
    pub meta: models::Metadata,

    #[serde(rename = "status")]
    pub status: models::ResponseStatus,

}

impl ApiResponseNodeSettings {
    pub fn new(data: models::NodeSettings, meta: models::Metadata, status: models::ResponseStatus, ) -> ApiResponseNodeSettings {
        ApiResponseNodeSettings {
            data: data,
            meta: meta,
            status: status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponseV1SoftwareVersion {
    #[serde(rename = "data")]
    pub data: models::V1SoftwareVersion,

    #[serde(rename = "meta")]
    pub meta: models::Metadata,

    #[serde(rename = "status")]
    pub status: models::ResponseStatus,

}

impl ApiResponseV1SoftwareVersion {
    pub fn new(data: models::V1SoftwareVersion, meta: models::Metadata, status: models::ResponseStatus, ) -> ApiResponseV1SoftwareVersion {
        ApiResponseV1SoftwareVersion {
            data: data,
            meta: meta,
            status: status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockchainHeight {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl BlockchainHeight {
    pub fn new(quantity: u64, unit: String, ) -> BlockchainHeight {
        BlockchainHeight {
            quantity: quantity,
            unit: unit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalTimeDifference {
    #[serde(rename = "quantity")]
    pub quantity: isize,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl LocalTimeDifference {
    pub fn new(quantity: isize, unit: String, ) -> LocalTimeDifference {
        LocalTimeDifference {
            quantity: quantity,
            unit: unit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxTxSize {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl MaxTxSize {
    pub fn new(quantity: u64, unit: String, ) -> MaxTxSize {
        MaxTxSize {
            quantity: quantity,
            unit: unit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "pagination")]
    pub pagination: models::PaginationMetadata,

}

impl Metadata {
    pub fn new(pagination: models::PaginationMetadata, ) -> Metadata {
        Metadata {
            pagination: pagination,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    #[serde(rename = "blockchainHeight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blockchain_height: Option<models::NodeInfoBlockchainHeight>,

    #[serde(rename = "localBlockchainHeight")]
    pub local_blockchain_height: models::NodeInfoLocalBlockchainHeight,

    #[serde(rename = "localTimeInformation")]
    pub local_time_information: models::NodeInfoLocalTimeInformation,

    /// Is the node connected to the network?
    #[serde(rename = "subscriptionStatus")]
    pub subscription_status: HashMap<String, models::SubscriptionStatus>,

    #[serde(rename = "syncProgress")]
    pub sync_progress: models::NodeInfoSyncProgress,

}

impl NodeInfo {
    pub fn new(local_blockchain_height: models::NodeInfoLocalBlockchainHeight, local_time_information: models::NodeInfoLocalTimeInformation, subscription_status: HashMap<String, models::SubscriptionStatus>, sync_progress: models::NodeInfoSyncProgress, ) -> NodeInfo {
        NodeInfo {
            blockchain_height: None,
            local_blockchain_height: local_blockchain_height,
            local_time_information: local_time_information,
            subscription_status: subscription_status,
            sync_progress: sync_progress,
        }
    }
}

/// If known, the current blockchain height, in number of blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoBlockchainHeight {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeInfoBlockchainHeight {
    pub fn new(quantity: u64, unit: String, ) -> NodeInfoBlockchainHeight {
        NodeInfoBlockchainHeight {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// Local blockchain height, in number of blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoLocalBlockchainHeight {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeInfoLocalBlockchainHeight {
    pub fn new(quantity: u64, unit: String, ) -> NodeInfoLocalBlockchainHeight {
        NodeInfoLocalBlockchainHeight {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// Information about the clock on this node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoLocalTimeInformation {
    #[serde(rename = "differenceFromNtpServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub difference_from_ntp_server: Option<models::NodeInfoLocalTimeInformationDifferenceFromNtpServer>,

}

impl NodeInfoLocalTimeInformation {
    pub fn new() -> NodeInfoLocalTimeInformation {
        NodeInfoLocalTimeInformation {
            difference_from_ntp_server: None,
        }
    }
}

/// The difference in microseconds between the node time and the NTP server. This value will be null if the NTP server is unavailable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoLocalTimeInformationDifferenceFromNtpServer {
    #[serde(rename = "quantity")]
    pub quantity: isize,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeInfoLocalTimeInformationDifferenceFromNtpServer {
    pub fn new(quantity: isize, unit: String, ) -> NodeInfoLocalTimeInformationDifferenceFromNtpServer {
        NodeInfoLocalTimeInformationDifferenceFromNtpServer {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// Syncing progression, in percentage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoSyncProgress {
    #[serde(rename = "quantity")]
    pub quantity: u8,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeInfoSyncProgress {
    pub fn new(quantity: u8, unit: String, ) -> NodeInfoSyncProgress {
        NodeInfoSyncProgress {
            quantity: quantity,
            unit: unit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettings {
    #[serde(rename = "feePolicy")]
    pub fee_policy: models::NodeSettingsFeePolicy,

    /// Git revision of this deployment.
    #[serde(rename = "gitRevision")]
    pub git_revision: String,

    #[serde(rename = "maxTxSize")]
    pub max_tx_size: models::NodeSettingsMaxTxSize,

    /// Current project's version.
    #[serde(rename = "projectVersion")]
    pub project_version: String,

    /// The security parameter.
    #[serde(rename = "securityParameter")]
    pub security_parameter: f64,

    /// The number of slots per epoch.
    #[serde(rename = "slotCount")]
    pub slot_count: f64,

    #[serde(rename = "slotDuration")]
    pub slot_duration: models::NodeSettingsSlotDuration,

    #[serde(rename = "slotId")]
    pub slot_id: models::NodeSettingsSlotId,

    #[serde(rename = "softwareInfo")]
    pub software_info: models::NodeSettingsSoftwareInfo,

}

impl NodeSettings {
    pub fn new(fee_policy: models::NodeSettingsFeePolicy, git_revision: String, max_tx_size: models::NodeSettingsMaxTxSize, project_version: String, security_parameter: f64, slot_count: f64, slot_duration: models::NodeSettingsSlotDuration, slot_id: models::NodeSettingsSlotId, software_info: models::NodeSettingsSoftwareInfo, ) -> NodeSettings {
        NodeSettings {
            fee_policy: fee_policy,
            git_revision: git_revision,
            max_tx_size: max_tx_size,
            project_version: project_version,
            security_parameter: security_parameter,
            slot_count: slot_count,
            slot_duration: slot_duration,
            slot_id: slot_id,
            software_info: software_info,
        }
    }
}

/// The fee policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsFeePolicy {
    #[serde(rename = "a")]
    pub a: models::NodeSettingsFeePolicyA,

    #[serde(rename = "b")]
    pub b: models::NodeSettingsFeePolicyB,

}

impl NodeSettingsFeePolicy {
    pub fn new(a: models::NodeSettingsFeePolicyA, b: models::NodeSettingsFeePolicyB, ) -> NodeSettingsFeePolicy {
        NodeSettingsFeePolicy {
            a: a,
            b: b,
        }
    }
}

/// Slope of the linear curve
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsFeePolicyA {
    #[serde(rename = "quantity")]
    pub quantity: f64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeSettingsFeePolicyA {
    pub fn new(quantity: f64, unit: String, ) -> NodeSettingsFeePolicyA {
        NodeSettingsFeePolicyA {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// Intercept of the linear curve
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsFeePolicyB {
    #[serde(rename = "quantity")]
    pub quantity: f64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeSettingsFeePolicyB {
    pub fn new(quantity: f64, unit: String, ) -> NodeSettingsFeePolicyB {
        NodeSettingsFeePolicyB {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// The largest allowed transaction size
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsMaxTxSize {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeSettingsMaxTxSize {
    pub fn new(quantity: u64, unit: String, ) -> NodeSettingsMaxTxSize {
        NodeSettingsMaxTxSize {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// Duration of a slot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsSlotDuration {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl NodeSettingsSlotDuration {
    pub fn new(quantity: u64, unit: String, ) -> NodeSettingsSlotDuration {
        NodeSettingsSlotDuration {
            quantity: quantity,
            unit: unit,
        }
    }
}

/// The current slot and epoch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsSlotId {
    #[serde(rename = "epoch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub epoch: Option<u64>,

    #[serde(rename = "slot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slot: Option<u16>,

}

impl NodeSettingsSlotId {
    pub fn new() -> NodeSettingsSlotId {
        NodeSettingsSlotId {
            epoch: None,
            slot: None,
        }
    }
}

/// Various pieces of information about the current software.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSettingsSoftwareInfo {
    #[serde(rename = "applicationName")]
    pub application_name: String,

    #[serde(rename = "version")]
    pub version: u32,

}

impl NodeSettingsSoftwareInfo {
    pub fn new(application_name: String, version: u32, ) -> NodeSettingsSoftwareInfo {
        NodeSettingsSoftwareInfo {
            application_name: application_name,
            version: version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct Page(i32);

impl ::std::convert::From<i32> for Page {
    fn from(x: i32) -> Self {
        Page(x)
    }
}

impl ::std::convert::From<Page> for i32 {
    fn from(x: Page) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for Page {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for Page {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaginationMetadata {
    #[serde(rename = "page")]
    pub page: models::Page,

    #[serde(rename = "perPage")]
    pub per_page: models::PerPage,

    #[serde(rename = "totalEntries")]
    pub total_entries: f64,

    #[serde(rename = "totalPages")]
    pub total_pages: f64,

}

impl PaginationMetadata {
    pub fn new(page: models::Page, per_page: models::PerPage, total_entries: f64, total_pages: f64, ) -> PaginationMetadata {
        PaginationMetadata {
            page: page,
            per_page: per_page,
            total_entries: total_entries,
            total_pages: total_pages,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct PerPage(i32);

impl ::std::convert::From<i32> for PerPage {
    fn from(x: i32) -> Self {
        PerPage(x)
    }
}

impl ::std::convert::From<PerPage> for i32 {
    fn from(x: PerPage) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for PerPage {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl ::std::ops::DerefMut for PerPage {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum ResponseStatus { 
    #[serde(rename = "success")]
    SUCCESS,
    #[serde(rename = "fail")]
    FAIL,
    #[serde(rename = "error")]
    ERROR,
}

impl ::std::fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            ResponseStatus::SUCCESS => write!(f, "{}", "success"),
            ResponseStatus::FAIL => write!(f, "{}", "fail"),
            ResponseStatus::ERROR => write!(f, "{}", "error"),
        }
    }
}

impl ::std::str::FromStr for ResponseStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "success" => Ok(ResponseStatus::SUCCESS),
            "fail" => Ok(ResponseStatus::FAIL),
            "error" => Ok(ResponseStatus::ERROR),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct SecurityParameter(f64);

impl ::std::convert::From<f64> for SecurityParameter {
    fn from(x: f64) -> Self {
        SecurityParameter(x)
    }
}

impl ::std::convert::From<SecurityParameter> for f64 {
    fn from(x: SecurityParameter) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for SecurityParameter {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl ::std::ops::DerefMut for SecurityParameter {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlotDuration {
    #[serde(rename = "quantity")]
    pub quantity: u64,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl SlotDuration {
    pub fn new(quantity: u64, unit: String, ) -> SlotDuration {
        SlotDuration {
            quantity: quantity,
            unit: unit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlotId {
    #[serde(rename = "epoch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub epoch: Option<u64>,

    #[serde(rename = "slot")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slot: Option<u16>,

}

impl SlotId {
    pub fn new() -> SlotId {
        SlotId {
            epoch: None,
            slot: None,
        }
    }
}

/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum SubscriptionStatus { 
    #[serde(rename = "subscribed")]
    SUBSCRIBED,
    #[serde(rename = "subscribing")]
    SUBSCRIBING,
}

impl ::std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            SubscriptionStatus::SUBSCRIBED => write!(f, "{}", "subscribed"),
            SubscriptionStatus::SUBSCRIBING => write!(f, "{}", "subscribing"),
        }
    }
}

impl ::std::str::FromStr for SubscriptionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "subscribed" => Ok(SubscriptionStatus::SUBSCRIBED),
            "subscribing" => Ok(SubscriptionStatus::SUBSCRIBING),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncPercentage {
    #[serde(rename = "quantity")]
    pub quantity: u8,

    // Note: inline enums are not fully supported by swagger-codegen
    #[serde(rename = "unit")]
    pub unit: String,

}

impl SyncPercentage {
    pub fn new(quantity: u8, unit: String, ) -> SyncPercentage {
        SyncPercentage {
            quantity: quantity,
            unit: unit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeInfo {
    #[serde(rename = "differenceFromNtpServer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub difference_from_ntp_server: Option<models::NodeInfoLocalTimeInformationDifferenceFromNtpServer>,

}

impl TimeInfo {
    pub fn new() -> TimeInfo {
        TimeInfo {
            difference_from_ntp_server: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxFeePolicy {
    #[serde(rename = "a")]
    pub a: models::NodeSettingsFeePolicyA,

    #[serde(rename = "b")]
    pub b: models::NodeSettingsFeePolicyB,

}

impl TxFeePolicy {
    pub fn new(a: models::NodeSettingsFeePolicyA, b: models::NodeSettingsFeePolicyB, ) -> TxFeePolicy {
        TxFeePolicy {
            a: a,
            b: b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxOut {
    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "coin")]
    pub coin: f64,

}

impl TxOut {
    pub fn new(address: String, coin: f64, ) -> TxOut {
        TxOut {
            address: address,
            coin: coin,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxOutAux {
    #[serde(rename = "toaOut")]
    pub toa_out: models::TxOut,

}

impl TxOutAux {
    pub fn new(toa_out: models::TxOut, ) -> TxOutAux {
        TxOutAux {
            toa_out: toa_out,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct V1CoreSlotCount(f64);

impl ::std::convert::From<f64> for V1CoreSlotCount {
    fn from(x: f64) -> Self {
        V1CoreSlotCount(x)
    }
}

impl ::std::convert::From<V1CoreSlotCount> for f64 {
    fn from(x: V1CoreSlotCount) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for V1CoreSlotCount {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl ::std::ops::DerefMut for V1CoreSlotCount {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V1SoftwareVersion {
    #[serde(rename = "applicationName")]
    pub application_name: String,

    #[serde(rename = "version")]
    pub version: u32,

}

impl V1SoftwareVersion {
    pub fn new(application_name: String, version: u32, ) -> V1SoftwareVersion {
        V1SoftwareVersion {
            application_name: application_name,
            version: version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct V1Version(String);

impl ::std::convert::From<String> for V1Version {
    fn from(x: String) -> Self {
        V1Version(x)
    }
}

impl ::std::convert::From<V1Version> for String {
    fn from(x: V1Version) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for V1Version {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::ops::DerefMut for V1Version {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

