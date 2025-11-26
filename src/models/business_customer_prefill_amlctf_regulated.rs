use crate::models;
use serde::{Deserialize, Serialize};

/// BusinessCustomerPrefillAmlctfRegulated : This section is required if the business is subject to regulatory oversight that requires compliance with Anti-Money Laundering (AML) regulations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessCustomerPrefillAmlctfRegulated {
    /// Please provide the name of the supervisory authority.
    #[serde(
        rename = "SupervisoryAuthorityName",
        skip_serializing_if = "Option::is_none"
    )]
    pub supervisory_authority_name: Option<String>,
    /// Please provide the license number.
    #[serde(rename = "LicenseNumber", skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
    /// Has the business appointed an MLRO (Money Laundering Reporting Officer)?
    #[serde(rename = "HasAppointedMLRO", skip_serializing_if = "Option::is_none")]
    pub has_appointed_mlro: Option<bool>,
    #[serde(rename = "CustomerRiskSplit", skip_serializing_if = "Option::is_none")]
    pub customer_risk_split:
        Option<Box<models::BusinessCustomerPrefillAmlctfRegulatedCustomerRiskSplit>>,
    /// Does the company prohibit the opening and keeping of anonymous and fictitious named accounts?
    #[serde(
        rename = "ProhibitsAnonOrFictiousAccounts",
        skip_serializing_if = "Option::is_none"
    )]
    pub prohibits_anon_or_fictious_accounts: Option<bool>,
    /// Does the company prohibit the opening and keeping of accounts for unlicensed or shell customers?
    #[serde(
        rename = "ProhibitsAccountsForUnlicensedOrShellCustomers",
        skip_serializing_if = "Option::is_none"
    )]
    pub prohibits_accounts_for_unlicensed_or_shell_customers: Option<bool>,
    #[serde(
        rename = "CustomerIdentityVerification",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_identity_verification: Option<Box<models::BusinessCustomerPrefillScreeningInput>>,
    #[serde(rename = "PEPAndSanctions", skip_serializing_if = "Option::is_none")]
    pub pep_and_sanctions: Option<Box<models::BusinessCustomerPrefillScreeningInput>>,
    /// If the company does not screen its customers against PEP and sanctions, indicate what sanctions lists the Company uses to screen customers.
    #[serde(rename = "SanctionLists", skip_serializing_if = "Option::is_none")]
    pub sanction_lists: Option<Vec<String>>,
    /// Does the due diligence process result in customers receiving a risk classification?
    #[serde(
        rename = "CustomerRiskClassificationFromDueDiligence",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_risk_classification_from_due_diligence: Option<bool>,
    /// Does the company apply an enhanced due diligence process?
    #[serde(rename = "EDDProcess", skip_serializing_if = "Option::is_none")]
    pub edd_process: Option<bool>,
    #[serde(
        rename = "TransactionMonitoring",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_monitoring: Option<Box<models::BusinessCustomerPrefillScreeningInput>>,
    /// Does the company have procedures to identify transactions structured to avoid large cash/transactions reporting requirements?
    #[serde(
        rename = "ProceduresForTransactionMonitoring",
        skip_serializing_if = "Option::is_none"
    )]
    pub procedures_for_transaction_monitoring: Option<bool>,
    /// Has the company been subject to a money laundering or terrorist financing investigation? If yes, please provide further details:
    #[serde(
        rename = "SubjectToMLOrTFInvestigation",
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_to_mlor_tf_investigation: Option<String>,
    /// Has the company been the subject, in the past two years, of regulatory enforcement for inadequate AML/CTF policies and procedures and/or breaches of AML/CTF? If yes, please provide further details:
    #[serde(
        rename = "SubjectToRegulatoryEnforcementPast2Years",
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_to_regulatory_enforcement_past2_years: Option<String>,
    /// Does the company confirm that it does not provide services to individuals or entities which are residents of sanctioned countries. Sanctioned countries: Afghanistan, Belarus, Bhutan, Burundi, Democratic Republic of the Congo, Cuba, Gaza Strip, Guinea-Bissau, Haiti, Iran, Islamic Republic of Iraq, Kenya, Kosovo, Lebanon, Libya, Mozambique, Myanmar, Niger, North Korea(Democratic People’s Republic of Korea), Pakistan, Qatar, Russian Federation, Somalia, South Sudan, Sudan, Syria, Ukrainian Territories(Crimea, Sevastopol, Donetsk, Kherson, Luhansk, Zaporizhzhia), Venezuela, West Bank (Palestinian Territory, Occupied), Yemen, Zimbabwe
    #[serde(
        rename = "ConfirmsNoServiceToSanctionedCountries",
        skip_serializing_if = "Option::is_none"
    )]
    pub confirms_no_service_to_sanctioned_countries: Option<bool>,
    /// If the client's funds are transferred to the Company's external wallet or bank account, will they be accessible through:  Closed-loop – Funds can only be withdrawn by the client as instructed. Open-loop – Funds can be accessed and used freely by the client. Client funds will not be transferred to the external company wallet.
    #[serde(
        rename = "ClientFundsAccessibility",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_funds_accessibility: Option<ClientFundsAccessibility>,
    /// Has the company had AML/CTF audit?
    #[serde(rename = "AMLCTFAudit", skip_serializing_if = "Option::is_none")]
    pub amlctf_audit: Option<bool>,
    #[serde(rename = "AuditDate", skip_serializing_if = "Option::is_none")]
    pub audit_date: Option<String>,
}

impl BusinessCustomerPrefillAmlctfRegulated {
    /// This section is required if the business is subject to regulatory oversight that requires compliance with Anti-Money Laundering (AML) regulations.
    pub fn new() -> BusinessCustomerPrefillAmlctfRegulated {
        BusinessCustomerPrefillAmlctfRegulated {
            supervisory_authority_name: None,
            license_number: None,
            has_appointed_mlro: None,
            customer_risk_split: None,
            prohibits_anon_or_fictious_accounts: None,
            prohibits_accounts_for_unlicensed_or_shell_customers: None,
            customer_identity_verification: None,
            pep_and_sanctions: None,
            sanction_lists: None,
            customer_risk_classification_from_due_diligence: None,
            edd_process: None,
            transaction_monitoring: None,
            procedures_for_transaction_monitoring: None,
            subject_to_mlor_tf_investigation: None,
            subject_to_regulatory_enforcement_past2_years: None,
            confirms_no_service_to_sanctioned_countries: None,
            client_funds_accessibility: None,
            amlctf_audit: None,
            audit_date: None,
        }
    }
}
/// If the client's funds are transferred to the Company's external wallet or bank account, will they be accessible through:  Closed-loop – Funds can only be withdrawn by the client as instructed. Open-loop – Funds can be accessed and used freely by the client. Client funds will not be transferred to the external company wallet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientFundsAccessibility {
    #[serde(rename = "ClosedLoop")]
    ClosedLoop,
    #[serde(rename = "OpenLoop")]
    OpenLoop,
    #[serde(rename = "NotTransferred")]
    NotTransferred,
}

impl Default for ClientFundsAccessibility {
    fn default() -> ClientFundsAccessibility {
        Self::ClosedLoop
    }
}
