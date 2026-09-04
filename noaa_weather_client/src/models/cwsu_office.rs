//! CWSU office metadata returned by `/aviation/cwsus/{cwsuId}`.
//!
//! # Requiredness
//!
//! The 2026-09-03 census saw all 18 keys present and non-null in all 22 CWSU
//! offices, which is the complete ARTCC set. Because the population is
//! complete rather than sampled, 100% presence here is stronger evidence than
//! a 500-sample would be. Every field is consequently non-`Option`.

use serde::{Deserialize, Serialize};

use crate::ids::CwsuId;

/// Metadata for one Center Weather Service Unit office.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CwsuOffice {
    /// The JSON-LD vocabulary context. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    #[serde(rename = "@context")]
    pub at_context: CwsuOfficeContext,
    /// The office's canonical API URL. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// The JSON-LD organization type. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// The office's postal address. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    pub address: CwsuOfficeAddress,
    /// The office email address. Present and non-null in all 22 CWSU offices
    /// in the complete 2026-09-03 census; an empty string is NOAA's value
    /// when no address is published.
    pub email: String,
    /// The office fax number. Present and non-null in all 22 CWSU offices in
    /// the complete 2026-09-03 census.
    pub fax_number: String,
    /// The CWSU identifier. Present and non-null in all 22 CWSU offices in
    /// the complete 2026-09-03 census; every value parses as a [`CwsuId`].
    pub id: CwsuId,
    /// The office name. Present and non-null in all 22 CWSU offices in the
    /// complete 2026-09-03 census.
    pub name: String,
    /// NOAA's NWS-region label. Present and non-null in all 22 CWSU offices
    /// in the complete 2026-09-03 census.
    pub nws_region: String,
    /// The office's public web URL. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    pub same_as: String,
    /// The office telephone number. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    pub telephone: String,
}

/// The fixed JSON-LD context returned with every CWSU office.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CwsuOfficeContext {
    /// The JSON-LD version. Present and non-null in all 22 CWSU offices in
    /// the complete 2026-09-03 census.
    #[serde(rename = "@version")]
    pub at_version: String,
    /// The JSON-LD vocabulary URL. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    #[serde(rename = "@vocab")]
    pub at_vocab: String,
}

/// The postal address nested in a CWSU office response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CwsuOfficeAddress {
    /// The JSON-LD address type. Present and non-null in all 22 CWSU offices
    /// in the complete 2026-09-03 census.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// The office city. Present and non-null in all 22 CWSU offices in the
    /// complete 2026-09-03 census.
    pub address_locality: String,
    /// The office state or territory. Present and non-null in all 22 CWSU
    /// offices in the complete 2026-09-03 census.
    pub address_region: String,
    /// The office postal code. Present and non-null in all 22 CWSU offices in
    /// the complete 2026-09-03 census.
    pub postal_code: String,
    /// The office street address. Present and non-null in all 22 CWSU offices
    /// in the complete 2026-09-03 census.
    pub street_address: String,
}
