//! Onboarding API

use crate::client::NoahClient;
use crate::error::Result;
use crate::models::common::*;
use crate::models::onboarding::{
    HostedOnboardingRequest, HostedSessionResponse, PrefillDocumentUploadURLResponse,
};
use serde::Serialize;

/// Prefill onboarding request (simplified - can be extended)
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PrefillOnboardingRequest {
    /// SumSub token
    SumSubToken {
        /// Type
        #[serde(rename = "Type")]
        request_type: String,
        /// Token
        #[serde(rename = "Token")]
        token: String,
    },
    /// Business customer prefill
    BusinessCustomerPrefill(serde_json::Value),
    /// Individual customer prefill
    IndividualCustomerPrefill(serde_json::Value),
}

impl NoahClient {
    /// Create onboarding session (async)
    #[cfg(feature = "async")]
    pub async fn create_onboarding_session(
        &self,
        customer_id: &CustomerID,
        request: &HostedOnboardingRequest,
    ) -> Result<HostedSessionResponse> {
        let path = format!("/onboarding/{customer_id}");
        self.post(&path, request).await
    }

    /// Create onboarding session (blocking)
    #[cfg(feature = "sync")]
    pub fn create_onboarding_session_blocking(
        &self,
        customer_id: &CustomerID,
        request: &HostedOnboardingRequest,
    ) -> Result<HostedSessionResponse> {
        let path = format!("/onboarding/{customer_id}");
        self.post_blocking(&path, request)
    }

    /// Prefill customer details (async)
    #[cfg(feature = "async")]
    pub async fn prefill_onboarding(
        &self,
        customer_id: &CustomerID,
        request: &PrefillOnboardingRequest,
    ) -> Result<()> {
        let path = format!("/onboarding/{customer_id}/prefill");
        let _response: Option<serde_json::Value> = self.post(&path, request).await?;
        Ok(())
    }

    /// Prefill customer details (blocking)
    #[cfg(feature = "sync")]
    pub fn prefill_onboarding_blocking(
        &self,
        customer_id: &CustomerID,
        request: &PrefillOnboardingRequest,
    ) -> Result<()> {
        let path = format!("/onboarding/{customer_id}/prefill");
        let _response: Option<serde_json::Value> = self.post_blocking(&path, request)?;
        Ok(())
    }

    /// Get document upload URL (async)
    #[cfg(feature = "async")]
    pub async fn get_document_upload_url(
        &self,
        customer_id: &CustomerID,
        document_type: &str,
        country_code: &CountryCode,
        side: Option<&str>,
        associate_id: Option<&str>,
    ) -> Result<PrefillDocumentUploadURLResponse> {
        let mut path = format!("/onboarding/{customer_id}/prefill/documents/upload-url");
        let mut query_params = vec![
            format!("Type={document_type}"),
            format!("CountryCode={country_code}"),
        ];

        if let Some(s) = side {
            query_params.push(format!("Side={s}"));
        }
        if let Some(aid) = associate_id {
            query_params.push(format!("AssociateID={aid}"));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get(&path).await
    }

    /// Get document upload URL (blocking)
    #[cfg(feature = "sync")]
    pub fn get_document_upload_url_blocking(
        &self,
        customer_id: &CustomerID,
        document_type: &str,
        country_code: &CountryCode,
        side: Option<&str>,
        associate_id: Option<&str>,
    ) -> Result<PrefillDocumentUploadURLResponse> {
        let mut path = format!("/onboarding/{customer_id}/prefill/documents/upload-url");
        let mut query_params = vec![
            format!("Type={document_type}"),
            format!("CountryCode={country_code}"),
        ];

        if let Some(s) = side {
            query_params.push(format!("Side={s}"));
        }
        if let Some(aid) = associate_id {
            query_params.push(format!("AssociateID={aid}"));
        }

        path.push('?');
        path.push_str(&query_params.join("&"));

        self.get_blocking(&path)
    }
}
