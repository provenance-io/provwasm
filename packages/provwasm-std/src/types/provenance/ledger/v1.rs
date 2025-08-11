use provwasm_proc_macro::CosmwasmExt;
/// LedgerClass contains the configuration for a ledger related to a particular class of asset. The asset class
/// is defined by the either a scope spec `x/metadata`, or nft class `x/nft`. Ultimately, the configuration will
/// assist in verifying the types that are associated with particular ledger entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClass")]
pub struct LedgerClass {
    /// Unique ID for the ledger class (eg. 1, 2, 3, etc.)
    /// This is necessary since the nft class does not have an owner.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// Scope Specification ID or NFT Class ID
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// Denom that this class of asset will be ledgered in
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    /// Address of the maintainer for the ledger class
    #[prost(string, tag = "4")]
    pub maintainer_address: ::prost::alloc::string::String,
}
/// LedgerClassEntryType defines the types of possible ledger entries for a given asset class. These type codes allow
/// for minimal data storage while providing a human readable description of the entry type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClassEntryType")]
pub struct LedgerClassEntryType {
    /// Unique ID for the entry type (eg. 1, 2, 3, etc.)
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// Code for the entry type (eg. "DISBURSEMENT", "PAYMENT", "ADJUSTMENT", "INTEREST", "FEE", "OTHER")
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Description of the entry type (eg. "Disbursement", "Payment", "Adjustment", "Interest", "Fee", "Other")
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// LedgerClassStatusType defines the types of possible status values for a given asset class. These type codes allow
/// for minimal data storage while providing a human readable description of the status type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClassStatusType")]
pub struct LedgerClassStatusType {
    /// Unique ID for the status type (eg. 1, 2, 3, etc.).
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// Code for the status type (eg. "IN_REPAYMENT", "IN_FORECLOSURE", "FORBEARANCE", "DEFERMENT", "BANKRUPTCY",
    /// "CLOSED", "CANCELLED", "SUSPENDED", "OTHER").
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Description of the status type (eg. "In Repayment", "In Foreclosure", "Forbearance", "Deferment", "Bankruptcy",
    /// "Closed", "Cancelled", "Suspended", "Other").
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// LedgerKey represents a unique key to identify an NFT/Asset for ledger transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerKey")]
pub struct LedgerKey {
    /// Identifier for the nft that this ledger is linked to.
    /// This could be a `x/metadata` scope id or an `x/nft` nft id.
    /// In order to create a ledger for an nft, the nft class must be registered in the ledger module as a LedgerClass.
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    /// Scope Specification ID or NFT Class ID
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
}
/// Ledger represents a ledger for tracking financial transactions and balances for a specific NFT.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.Ledger")]
pub struct Ledger {
    /// The ledger key identifying the ledger.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The ledger class ID for the ledger.
    #[prost(string, tag = "2")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The status of the ledger.
    #[prost(int32, tag = "3")]
    pub status_type_id: i32,
    /// The next payment date (days since epoch).
    #[prost(int32, tag = "4")]
    pub next_pmt_date: i32,
    /// The next payment amount.
    #[prost(int64, tag = "5")]
    pub next_pmt_amt: i64,
    /// The interest rate (10000000 = 10.000000%) - 6 decimal places.
    #[prost(int32, tag = "6")]
    pub interest_rate: i32,
    /// The maturity date (days since epoch).
    #[prost(int32, tag = "7")]
    pub maturity_date: i32,
    /// The day count convention for interest calculations.
    #[prost(enumeration = "DayCount", tag = "8")]
    pub interest_day_count: i32,
    /// The interest accrual method for interest calculations.
    #[prost(enumeration = "InterestAccrual", tag = "9")]
    pub interest_accrual: i32,
    /// The payment frequency.
    #[prost(enumeration = "PaymentFrequency", tag = "10")]
    pub payment_frequency: i32,
}
/// LedgerClassBucketType represents a bucket type for a ledger class
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClassBucketType")]
pub struct LedgerClassBucketType {
    /// Unique ID for the bucket type (eg. 1, 2, 3, etc.)
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// Code for the bucket type (eg. "PRINCIPAL", "INTEREST", "FEE", "OTHER")
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Description of the bucket type (eg. "Principal", "Interest", "Fee", "Other")
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// LedgerEntry represents a single entry in a ledger for tracking financial transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerEntry")]
pub struct LedgerEntry {
    /// The correlation ID for tracking ledger entries with external systems (max 50 characters).
    #[prost(string, tag = "1")]
    pub correlation_id: ::prost::alloc::string::String,
    /// If this entry reverses another entry, the correlation ID of the entry it reverses.
    #[prost(string, tag = "2")]
    pub reverses_correlation_id: ::prost::alloc::string::String,
    /// If true, this entry is a void and should not be included in the ledger balance calculations.
    #[prost(bool, tag = "3")]
    pub is_void: bool,
    /// The sequence number of the ledger entry (less than 100).
    /// This field is used to maintain the correct order of entries when multiple entries
    /// share the same effective date. Entries are sorted first by effective date, then by sequence.
    #[prost(uint32, tag = "4")]
    pub sequence: u32,
    /// The type of ledger entry specified by the LedgerClassEntryType.id.
    #[prost(int32, tag = "5")]
    pub entry_type_id: i32,
    /// The posted date (days since epoch).
    #[prost(int32, tag = "7")]
    pub posted_date: i32,
    /// The effective date (days since epoch).
    #[prost(int32, tag = "8")]
    pub effective_date: i32,
    /// The total amount of the ledger entry.
    #[prost(string, tag = "9")]
    pub total_amt: ::prost::alloc::string::String,
    /// The applied amounts for each bucket.
    #[prost(message, repeated, tag = "10")]
    pub applied_amounts: ::prost::alloc::vec::Vec<LedgerBucketAmount>,
    /// The balances for each bucket.
    #[prost(message, repeated, tag = "11")]
    pub balance_amounts: ::prost::alloc::vec::Vec<BucketBalance>,
}
/// LedgerBucketAmount represents the amount applied to a specific bucket in a ledger entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerBucketAmount")]
pub struct LedgerBucketAmount {
    /// The bucket type specified by the LedgerClassBucketType.id.
    #[prost(int32, tag = "1")]
    pub bucket_type_id: i32,
    /// The amount applied to the bucket.
    #[prost(string, tag = "2")]
    pub applied_amt: ::prost::alloc::string::String,
}
/// Balances represents the current balances for principal, interest, and other amounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.Balances")]
pub struct Balances {
    /// The bucket balances.
    #[prost(message, repeated, tag = "1")]
    pub bucket_balances: ::prost::alloc::vec::Vec<BucketBalance>,
}
/// BucketBalance represents the balance for a specific bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.BucketBalance")]
pub struct BucketBalance {
    /// The bucket type specified by the LedgerClassBucketType.id.
    #[prost(int32, tag = "1")]
    pub bucket_type_id: i32,
    /// The balance of the bucket.
    #[prost(string, tag = "2")]
    pub balance_amt: ::prost::alloc::string::String,
}
/// Ledgers represents a collection of ledgers with their entries, used for conversion and bulk operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.Ledgers")]
pub struct Ledgers {
    /// The ledgers with their entries.
    #[prost(message, repeated, tag = "1")]
    pub ledger_to_entries: ::prost::alloc::vec::Vec<LedgerToEntries>,
}
/// LedgerToEntries represents a ledger with its associated entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerToEntries")]
pub struct LedgerToEntries {
    /// The ledger key identifying the ledger.
    #[prost(message, optional, tag = "1")]
    pub ledger_key: ::core::option::Option<LedgerKey>,
    /// The ledger data.
    #[prost(message, optional, tag = "2")]
    pub ledger: ::core::option::Option<Ledger>,
    /// The ledger entries.
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<LedgerEntry>,
}
/// Day Count Conventions used in interest calculations
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
    ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum DayCount {
    /// Unspecified day count convention.
    Unspecified = 0,
    /// Actual/365: Uses the actual number of days in the period with a fixed denominator of 365
    /// (or sometimes 365.25 to adjust for leap years).
    Actual365 = 1,
    /// Actual/360: Uses the actual number of days in the period but divides by 360.
    Actual360 = 2,
    /// 30/360: Assumes each month has 30 days and the year has 360 days.
    Thirty360 = 3,
    /// Actual/Actual: Uses the actual number of days in the period and the actual days in the year
    /// (365 or 366, depending on the year).
    ActualActual = 4,
    /// 365/365: Always uses 365 days in the denominator regardless of leap years.
    Days365 = 5,
    /// 360/360: Always uses 360 days in both the numerator and denominator.
    Days360 = 6,
}
impl DayCount {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DayCount::Unspecified => "DAY_COUNT_UNSPECIFIED",
            DayCount::Actual365 => "DAY_COUNT_ACTUAL_365",
            DayCount::Actual360 => "DAY_COUNT_ACTUAL_360",
            DayCount::Thirty360 => "DAY_COUNT_THIRTY_360",
            DayCount::ActualActual => "DAY_COUNT_ACTUAL_ACTUAL",
            DayCount::Days365 => "DAY_COUNT_DAYS_365",
            DayCount::Days360 => "DAY_COUNT_DAYS_360",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DAY_COUNT_UNSPECIFIED" => Some(Self::Unspecified),
            "DAY_COUNT_ACTUAL_365" => Some(Self::Actual365),
            "DAY_COUNT_ACTUAL_360" => Some(Self::Actual360),
            "DAY_COUNT_THIRTY_360" => Some(Self::Thirty360),
            "DAY_COUNT_ACTUAL_ACTUAL" => Some(Self::ActualActual),
            "DAY_COUNT_DAYS_365" => Some(Self::Days365),
            "DAY_COUNT_DAYS_360" => Some(Self::Days360),
            _ => None,
        }
    }
}
/// Interest Accrual Methods describing how interest is calculated over time
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
    ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum InterestAccrual {
    /// Unspecified interest accrual method.
    Unspecified = 0,
    /// Simple Interest: Calculated only on the principal amount.
    SimpleInterest = 1,
    /// Compound Interest: Calculated on both the principal and on previously accumulated interest.
    CompoundInterest = 2,
    /// Daily Compounding: Interest is compounded on a daily basis.
    DailyCompounding = 3,
    /// Monthly Compounding: Interest is compounded each month.
    MonthlyCompounding = 4,
    /// Quarterly Compounding: Interest is compounded every quarter.
    QuarterlyCompounding = 5,
    /// Annually Compounding: Interest is compounded once per year.
    AnnualCompounding = 6,
    /// Continuous Compounding: The theoretical limit of compounding frequency where interest is compounded continuously.
    ContinuousCompounding = 7,
}
impl InterestAccrual {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InterestAccrual::Unspecified => "INTEREST_ACCRUAL_UNSPECIFIED",
            InterestAccrual::SimpleInterest => "INTEREST_ACCRUAL_SIMPLE_INTEREST",
            InterestAccrual::CompoundInterest => "INTEREST_ACCRUAL_COMPOUND_INTEREST",
            InterestAccrual::DailyCompounding => "INTEREST_ACCRUAL_DAILY_COMPOUNDING",
            InterestAccrual::MonthlyCompounding => "INTEREST_ACCRUAL_MONTHLY_COMPOUNDING",
            InterestAccrual::QuarterlyCompounding => "INTEREST_ACCRUAL_QUARTERLY_COMPOUNDING",
            InterestAccrual::AnnualCompounding => "INTEREST_ACCRUAL_ANNUAL_COMPOUNDING",
            InterestAccrual::ContinuousCompounding => "INTEREST_ACCRUAL_CONTINUOUS_COMPOUNDING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTEREST_ACCRUAL_UNSPECIFIED" => Some(Self::Unspecified),
            "INTEREST_ACCRUAL_SIMPLE_INTEREST" => Some(Self::SimpleInterest),
            "INTEREST_ACCRUAL_COMPOUND_INTEREST" => Some(Self::CompoundInterest),
            "INTEREST_ACCRUAL_DAILY_COMPOUNDING" => Some(Self::DailyCompounding),
            "INTEREST_ACCRUAL_MONTHLY_COMPOUNDING" => Some(Self::MonthlyCompounding),
            "INTEREST_ACCRUAL_QUARTERLY_COMPOUNDING" => Some(Self::QuarterlyCompounding),
            "INTEREST_ACCRUAL_ANNUAL_COMPOUNDING" => Some(Self::AnnualCompounding),
            "INTEREST_ACCRUAL_CONTINUOUS_COMPOUNDING" => Some(Self::ContinuousCompounding),
            _ => None,
        }
    }
}
/// Payment frequencies for loan repayments
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
    ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum PaymentFrequency {
    /// Unspecified payment frequency.
    Unspecified = 0,
    /// Daily payments.
    Daily = 1,
    /// Weekly or biweekly payments.
    Weekly = 2,
    /// Monthly payments (most common for consumer loans and mortgages).
    Monthly = 3,
    /// Quarterly payments.
    Quarterly = 4,
    /// Annual payments.
    Annually = 5,
}
impl PaymentFrequency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentFrequency::Unspecified => "PAYMENT_FREQUENCY_UNSPECIFIED",
            PaymentFrequency::Daily => "PAYMENT_FREQUENCY_DAILY",
            PaymentFrequency::Weekly => "PAYMENT_FREQUENCY_WEEKLY",
            PaymentFrequency::Monthly => "PAYMENT_FREQUENCY_MONTHLY",
            PaymentFrequency::Quarterly => "PAYMENT_FREQUENCY_QUARTERLY",
            PaymentFrequency::Annually => "PAYMENT_FREQUENCY_ANNUALLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAYMENT_FREQUENCY_UNSPECIFIED" => Some(Self::Unspecified),
            "PAYMENT_FREQUENCY_DAILY" => Some(Self::Daily),
            "PAYMENT_FREQUENCY_WEEKLY" => Some(Self::Weekly),
            "PAYMENT_FREQUENCY_MONTHLY" => Some(Self::Monthly),
            "PAYMENT_FREQUENCY_QUARTERLY" => Some(Self::Quarterly),
            "PAYMENT_FREQUENCY_ANNUALLY" => Some(Self::Annually),
            _ => None,
        }
    }
}
/// GenesisState represents the initial state of the ledger store.
/// This structure matches the test.json format for bulk import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisState")]
pub struct GenesisState {
    /// Ledgers with their entries - matches the test.json structure.
    #[prost(message, repeated, tag = "1")]
    pub ledger_to_entries: ::prost::alloc::vec::Vec<LedgerToEntries>,
}
/// LedgerPlainText represents a ledger in plain text format for human-readable display.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerPlainText")]
pub struct LedgerPlainText {
    /// The ledger key identifying the ledger.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The status of the ledger as a human-readable string.
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    /// The next payment date as a human-readable string.
    #[prost(string, tag = "3")]
    pub next_pmt_date: ::prost::alloc::string::String,
    /// The next payment amount as a human-readable string.
    #[prost(string, tag = "4")]
    pub next_pmt_amt: ::prost::alloc::string::String,
    /// The interest rate as a human-readable string.
    #[prost(string, tag = "5")]
    pub interest_rate: ::prost::alloc::string::String,
    /// The maturity date as a human-readable string.
    #[prost(string, tag = "6")]
    pub maturity_date: ::prost::alloc::string::String,
    /// The day count convention for interest calculations.
    #[prost(enumeration = "DayCount", tag = "7")]
    pub interest_day_count: i32,
    /// The interest accrual method for interest calculations.
    #[prost(enumeration = "InterestAccrual", tag = "8")]
    pub interest_accrual: i32,
    /// The payment frequency.
    #[prost(enumeration = "PaymentFrequency", tag = "9")]
    pub payment_frequency: i32,
}
/// LedgerEntryPlainText represents a ledger entry in plain text format for human-readable display.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerEntryPlainText")]
pub struct LedgerEntryPlainText {
    /// The correlation ID for tracking ledger entries with external systems (max 50 characters).
    #[prost(string, tag = "1")]
    pub correlation_id: ::prost::alloc::string::String,
    /// The sequence number of the ledger entry (less than 100).
    /// This field is used to maintain the correct order of entries when multiple entries
    /// share the same effective date. Entries are sorted first by effective date, then by sequence.
    #[prost(uint32, tag = "2")]
    pub sequence: u32,
    /// The type of ledger entry.
    #[prost(message, optional, tag = "3")]
    pub r#type: ::core::option::Option<LedgerClassEntryType>,
    /// The posted date as a human-readable string.
    #[prost(string, tag = "5")]
    pub posted_date: ::prost::alloc::string::String,
    /// The effective date as a human-readable string.
    #[prost(string, tag = "6")]
    pub effective_date: ::prost::alloc::string::String,
    /// The total amount of the ledger entry as a human-readable string.
    #[prost(string, tag = "7")]
    pub total_amt: ::prost::alloc::string::String,
    /// The amounts applied to each bucket in plain text format.
    #[prost(message, repeated, tag = "8")]
    pub applied_amounts: ::prost::alloc::vec::Vec<LedgerBucketAmountPlainText>,
}
/// LedgerBucketAmountPlainText represents a bucket amount in plain text format for human-readable display.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerBucketAmountPlainText")]
pub struct LedgerBucketAmountPlainText {
    /// The bucket type for this amount.
    #[prost(message, optional, tag = "1")]
    pub bucket: ::core::option::Option<LedgerClassBucketType>,
    /// The amount applied to this bucket as a human-readable string.
    #[prost(string, tag = "2")]
    pub applied_amt: ::prost::alloc::string::String,
    /// The balance amount for this bucket as a human-readable string.
    #[prost(string, tag = "3")]
    pub balance_amt: ::prost::alloc::string::String,
}
/// QueryLedgerEntryResponsePlainText represents a response containing ledger entries in plain text format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntryResponsePlainText")]
pub struct QueryLedgerEntryResponsePlainText {
    /// The ledger entries in plain text format.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<LedgerEntryPlainText>,
}
/// FundTransferWithSettlement represents a fund transfer with settlement instructions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.FundTransferWithSettlement")]
pub struct FundTransferWithSettlement {
    /// The ledger key identifying the ledger for this transfer.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The correlation ID of the ledger entry associated with this transfer.
    #[prost(string, tag = "2")]
    pub ledger_entry_correlation_id: ::prost::alloc::string::String,
    /// The settlement instructions for this transfer.
    #[prost(message, repeated, tag = "3")]
    pub settlement_instructions: ::prost::alloc::vec::Vec<SettlementInstruction>,
}
/// SettlementInstruction represents blockchain-specific settlement instructions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.SettlementInstruction")]
pub struct SettlementInstruction {
    /// The amount to be transferred.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The recipient's blockchain address.
    #[prost(string, tag = "2")]
    pub recipient_address: ::prost::alloc::string::String,
    /// The current status of the funding transfer.
    #[prost(enumeration = "FundingTransferStatus", tag = "3")]
    pub status: i32,
    /// An optional memo or note for the transaction.
    #[prost(string, tag = "4")]
    pub memo: ::prost::alloc::string::String,
    /// The minimum block height or timestamp for settlement.
    #[prost(int64, tag = "5")]
    pub settlement_block: i64,
}
/// StoredSettlementInstructions represents the stored version of settlement instructions against a ledger key and entry correlation ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.StoredSettlementInstructions")]
pub struct StoredSettlementInstructions {
    /// The settlement instructions.
    #[prost(message, repeated, tag = "1")]
    pub settlement_instructions: ::prost::alloc::vec::Vec<SettlementInstruction>,
}
/// FundingTransferStatus represents the current status of a funding transfer.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
    ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum FundingTransferStatus {
    /// Unspecified funding transfer status.
    Unspecified = 0,
    /// The funding transfer is pending.
    Pending = 1,
    /// The funding transfer is being processed.
    Processing = 2,
    /// The funding transfer has been completed.
    Completed = 3,
    /// The funding transfer has failed.
    Failed = 4,
}
impl FundingTransferStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FundingTransferStatus::Unspecified => "FUNDING_TRANSFER_STATUS_UNSPECIFIED",
            FundingTransferStatus::Pending => "FUNDING_TRANSFER_STATUS_PENDING",
            FundingTransferStatus::Processing => "FUNDING_TRANSFER_STATUS_PROCESSING",
            FundingTransferStatus::Completed => "FUNDING_TRANSFER_STATUS_COMPLETED",
            FundingTransferStatus::Failed => "FUNDING_TRANSFER_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FUNDING_TRANSFER_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "FUNDING_TRANSFER_STATUS_PENDING" => Some(Self::Pending),
            "FUNDING_TRANSFER_STATUS_PROCESSING" => Some(Self::Processing),
            "FUNDING_TRANSFER_STATUS_COMPLETED" => Some(Self::Completed),
            "FUNDING_TRANSFER_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// QueryLedgerRequest represents a request to query a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/Ledger",
    response_type = QueryLedgerResponse
)]
pub struct QueryLedgerRequest {
    /// The ledger key identifying the ledger to query.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
}
/// QueryLedgerResponse represents the response from querying a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerResponse")]
pub struct QueryLedgerResponse {
    /// The ledger data.
    #[prost(message, optional, tag = "1")]
    pub ledger: ::core::option::Option<Ledger>,
}
/// QueryLedgerEntriesRequest represents a request to query ledger entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntriesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerEntries",
    response_type = QueryLedgerEntriesResponse
)]
pub struct QueryLedgerEntriesRequest {
    /// The ledger key identifying the ledger to query entries for.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
}
/// QueryLedgerEntriesResponse represents the response from querying ledger entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntriesResponse")]
pub struct QueryLedgerEntriesResponse {
    /// The ledger entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<LedgerEntry>,
}
/// QueryLedgerEntryRequest represents a request to query a specific ledger entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntryRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerEntry",
    response_type = QueryLedgerEntryResponse
)]
pub struct QueryLedgerEntryRequest {
    /// The ledger key identifying the ledger.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The correlation ID of the entry to query (free-form string up to 50 characters).
    #[prost(string, tag = "2")]
    pub correlation_id: ::prost::alloc::string::String,
}
/// QueryLedgerEntryResponse represents the response from querying a specific ledger entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntryResponse")]
pub struct QueryLedgerEntryResponse {
    /// The ledger entry.
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<LedgerEntry>,
}
/// QueryBalancesAsOfRequest represents a request to query balances as of a specific date.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryBalancesAsOfRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/BalancesAsOf",
    response_type = QueryBalancesAsOfResponse
)]
pub struct QueryBalancesAsOfRequest {
    /// The ledger key identifying the ledger.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The date to query balances as of (format: YYYY-MM-DD).
    #[prost(string, tag = "2")]
    pub as_of_date: ::prost::alloc::string::String,
}
/// QueryBalancesAsOfResponse represents the response from querying balances as of a specific date.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryBalancesAsOfResponse")]
pub struct QueryBalancesAsOfResponse {
    /// The balances as of the specified date.
    #[prost(message, optional, tag = "1")]
    pub balances: ::core::option::Option<Balances>,
}
/// QueryLedgerClassEntryTypesRequest represents a request to query entry types for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassEntryTypesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClassEntryTypes",
    response_type = QueryLedgerClassEntryTypesResponse
)]
pub struct QueryLedgerClassEntryTypesRequest {
    /// The ledger class ID to query entry types for.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassEntryTypesResponse represents the response from querying entry types for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassEntryTypesResponse")]
pub struct QueryLedgerClassEntryTypesResponse {
    /// The entry types for the ledger class.
    #[prost(message, repeated, tag = "1")]
    pub entry_types: ::prost::alloc::vec::Vec<LedgerClassEntryType>,
}
/// QueryLedgerClassStatusTypesRequest represents a request to query status types for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassStatusTypesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClassStatusTypes",
    response_type = QueryLedgerClassStatusTypesResponse
)]
pub struct QueryLedgerClassStatusTypesRequest {
    /// The ledger class ID to query status types for.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassStatusTypesResponse represents the response from querying status types for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassStatusTypesResponse")]
pub struct QueryLedgerClassStatusTypesResponse {
    /// The status types for the ledger class.
    #[prost(message, repeated, tag = "1")]
    pub status_types: ::prost::alloc::vec::Vec<LedgerClassStatusType>,
}
/// QueryLedgerClassBucketTypesRequest represents a request to query bucket types for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassBucketTypesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClassBucketTypes",
    response_type = QueryLedgerClassBucketTypesResponse
)]
pub struct QueryLedgerClassBucketTypesRequest {
    /// The ledger class ID to query bucket types for.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassBucketTypesResponse represents the response from querying bucket types for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassBucketTypesResponse")]
pub struct QueryLedgerClassBucketTypesResponse {
    /// The bucket types for the ledger class.
    #[prost(message, repeated, tag = "1")]
    pub bucket_types: ::prost::alloc::vec::Vec<LedgerClassBucketType>,
}
/// QueryLedgerClassRequest represents a request to query a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClass",
    response_type = QueryLedgerClassResponse
)]
pub struct QueryLedgerClassRequest {
    /// The ledger class ID to query.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassResponse represents the response from querying a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassResponse")]
pub struct QueryLedgerClassResponse {
    /// The ledger class data.
    #[prost(message, optional, tag = "1")]
    pub ledger_class: ::core::option::Option<LedgerClass>,
}
/// QuerySettlementsRequest represents a request to query settlements for a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QuerySettlementsRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/Settlements",
    response_type = QuerySettlementsResponse
)]
pub struct QuerySettlementsRequest {
    /// The ledger key identifying the ledger to query settlements for.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
}
/// QuerySettlementsResponse represents the response from querying settlements for a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QuerySettlementsResponse")]
pub struct QuerySettlementsResponse {
    /// The settlements for the ledger.
    #[prost(message, repeated, tag = "1")]
    pub settlements: ::prost::alloc::vec::Vec<StoredSettlementInstructions>,
}
/// QuerySettlementsByCorrelationIdRequest represents a request to query settlements by correlation ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QuerySettlementsByCorrelationIdRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/SettlementsByCorrelationId",
    response_type = QuerySettlementsByCorrelationIdResponse
)]
pub struct QuerySettlementsByCorrelationIdRequest {
    /// The ledger key identifying the ledger.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The correlation ID to query settlements for.
    #[prost(string, tag = "2")]
    pub correlation_id: ::prost::alloc::string::String,
}
/// QuerySettlementsByCorrelationIdResponse represents the response from querying settlements by correlation ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QuerySettlementsByCorrelationIdResponse")]
pub struct QuerySettlementsByCorrelationIdResponse {
    /// The settlement instructions for the correlation ID.
    #[prost(message, optional, tag = "1")]
    pub settlement: ::core::option::Option<StoredSettlementInstructions>,
}
/// MsgCreateRequest represents a request to create a new ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateRequest")]
pub struct MsgCreateRequest {
    /// The ledger to create.
    #[prost(message, optional, tag = "1")]
    pub ledger: ::core::option::Option<Ledger>,
    /// The authority address that can create ledgers.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgCreateResponse represents the response from creating a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateResponse")]
pub struct MsgCreateResponse {}
/// MsgUpdateStatusRequest represents a request to update the status of a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateStatusRequest")]
pub struct MsgUpdateStatusRequest {
    /// The ledger key identifying the ledger to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The authority address that can update ledgers.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// The new status type ID for the ledger.
    #[prost(int32, tag = "3")]
    pub status_type_id: i32,
}
/// MsgUpdateStatusResponse represents the response from updating a ledger status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateStatusResponse")]
pub struct MsgUpdateStatusResponse {}
/// MsgUpdateInterestRateRequest represents a request to update the interest rate configuration of a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateInterestRateRequest")]
pub struct MsgUpdateInterestRateRequest {
    /// The ledger key identifying the ledger to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The authority address that can update ledgers.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// The new interest rate (10000000 = 10.000000%) - 6 decimal places.
    #[prost(int32, tag = "3")]
    pub interest_rate: i32,
    /// The day count convention for interest calculations.
    #[prost(enumeration = "DayCount", tag = "4")]
    pub interest_day_count: i32,
    /// The interest accrual method.
    #[prost(enumeration = "InterestAccrual", tag = "5")]
    pub interest_accrual: i32,
}
/// MsgUpdateInterestRateResponse represents the response from updating a ledger interest rate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateInterestRateResponse")]
pub struct MsgUpdateInterestRateResponse {}
/// MsgUpdatePaymentRequest represents a request to update payment configuration of a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdatePaymentRequest")]
pub struct MsgUpdatePaymentRequest {
    /// The ledger key identifying the ledger to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The authority address that can update ledgers.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// The next payment amount.
    #[prost(int64, tag = "3")]
    pub next_pmt_amt: i64,
    /// The next payment date (days since epoch).
    #[prost(int32, tag = "4")]
    pub next_pmt_date: i32,
    /// The payment frequency.
    #[prost(enumeration = "PaymentFrequency", tag = "5")]
    pub payment_frequency: i32,
}
/// MsgUpdatePaymentResponse represents the response from updating a ledger payment configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdatePaymentResponse")]
pub struct MsgUpdatePaymentResponse {}
/// MsgUpdateMaturityDateRequest represents a request to update the maturity date of a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateMaturityDateRequest")]
pub struct MsgUpdateMaturityDateRequest {
    /// The ledger key identifying the ledger to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The authority address that can update ledgers.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// The new maturity date (days since epoch).
    #[prost(int32, tag = "3")]
    pub maturity_date: i32,
}
/// MsgUpdateMaturityDateResponse represents the response from updating a ledger maturity date.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateMaturityDateResponse")]
pub struct MsgUpdateMaturityDateResponse {}
/// MsgAppendRequest represents a request to append entries to a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAppendRequest")]
pub struct MsgAppendRequest {
    /// The ledger key identifying the ledger to append to.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The ledger entries to append.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<LedgerEntry>,
    /// The authority address that can append to ledgers.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgAppendResponse represents the response from appending entries to a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAppendResponse")]
pub struct MsgAppendResponse {}
/// MsgUpdateBalancesRequest represents a request to update balances for a ledger entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateBalancesRequest")]
pub struct MsgUpdateBalancesRequest {
    /// The ledger key identifying the ledger to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The authority address that can update balances.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// The correlation ID of the entry to update.
    #[prost(string, tag = "3")]
    pub correlation_id: ::prost::alloc::string::String,
    /// The applied amounts to be updated.
    #[prost(message, repeated, tag = "4")]
    pub applied_amounts: ::prost::alloc::vec::Vec<LedgerBucketAmount>,
    /// The bucket balances to update.
    #[prost(message, repeated, tag = "5")]
    pub balance_amounts: ::prost::alloc::vec::Vec<BucketBalance>,
}
/// MsgUpdateBalancesResponse represents the response from updating ledger balances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateBalancesResponse")]
pub struct MsgUpdateBalancesResponse {}
/// MsgTransferFundsWithSettlementRequest represents a request to transfer funds with settlement instructions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgTransferFundsWithSettlementRequest")]
pub struct MsgTransferFundsWithSettlementRequest {
    /// The authority address that can transfer funds.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The fund transfers with settlement instructions.
    #[prost(message, repeated, tag = "2")]
    pub transfers: ::prost::alloc::vec::Vec<FundTransferWithSettlement>,
}
/// MsgTransferFundsWithSettlementResponse represents the response from transferring funds with settlement.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgTransferFundsWithSettlementResponse")]
pub struct MsgTransferFundsWithSettlementResponse {}
/// MsgDestroyRequest represents a request to destroy a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgDestroyRequest")]
pub struct MsgDestroyRequest {
    /// The ledger key identifying the ledger to destroy.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The authority address that can destroy ledgers.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgDestroyResponse represents the response from destroying a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgDestroyResponse")]
pub struct MsgDestroyResponse {}
/// MsgCreateClassRequest represents a request to create a new ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateClassRequest")]
pub struct MsgCreateClassRequest {
    /// The ledger class to create.
    #[prost(message, optional, tag = "1")]
    pub ledger_class: ::core::option::Option<LedgerClass>,
    /// The authority address that can create ledger classes.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgCreateClassResponse represents the response from creating a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateClassResponse")]
pub struct MsgCreateClassResponse {}
/// MsgAddClassStatusTypeRequest represents a request to add a status type to a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddClassStatusTypeRequest")]
pub struct MsgAddClassStatusTypeRequest {
    /// The ledger class ID to add the status type to.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The status type to add.
    #[prost(message, optional, tag = "2")]
    pub status_type: ::core::option::Option<LedgerClassStatusType>,
    /// The authority address that can add status types.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgAddClassStatusTypeResponse represents the response from adding a status type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddClassStatusTypeResponse")]
pub struct MsgAddClassStatusTypeResponse {}
/// MsgAddClassEntryTypeRequest represents a request to add an entry type to a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddClassEntryTypeRequest")]
pub struct MsgAddClassEntryTypeRequest {
    /// The ledger class ID to add the entry type to.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The entry type to add.
    #[prost(message, optional, tag = "2")]
    pub entry_type: ::core::option::Option<LedgerClassEntryType>,
    /// The authority address that can add entry types.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgAddClassEntryTypeResponse represents the response from adding an entry type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddClassEntryTypeResponse")]
pub struct MsgAddClassEntryTypeResponse {}
/// MsgAddClassBucketTypeRequest represents a request to add a bucket type to a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddClassBucketTypeRequest")]
pub struct MsgAddClassBucketTypeRequest {
    /// The ledger class ID to add the bucket type to.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The bucket type to add.
    #[prost(message, optional, tag = "2")]
    pub bucket_type: ::core::option::Option<LedgerClassBucketType>,
    /// The authority address that can add bucket types.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgAddClassBucketTypeResponse represents the response from adding a bucket type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddClassBucketTypeResponse")]
pub struct MsgAddClassBucketTypeResponse {}
/// MsgBulkImportRequest represents a request to bulk import ledger data from genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgBulkImportRequest")]
pub struct MsgBulkImportRequest {
    /// The authority address that can perform bulk imports.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The genesis state containing the ledger data to import.
    #[prost(message, optional, tag = "2")]
    pub genesis_state: ::core::option::Option<GenesisState>,
}
/// MsgBulkImportResponse represents the response from bulk importing ledger data from genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgBulkImportResponse")]
pub struct MsgBulkImportResponse {}
pub struct LedgerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> LedgerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn ledger_class(
        &self,
        ledger_class_id: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerClassResponse, cosmwasm_std::StdError> {
        QueryLedgerClassRequest { ledger_class_id }.query(self.querier)
    }
    pub fn ledger_class_entry_types(
        &self,
        ledger_class_id: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerClassEntryTypesResponse, cosmwasm_std::StdError> {
        QueryLedgerClassEntryTypesRequest { ledger_class_id }.query(self.querier)
    }
    pub fn ledger_class_status_types(
        &self,
        ledger_class_id: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerClassStatusTypesResponse, cosmwasm_std::StdError> {
        QueryLedgerClassStatusTypesRequest { ledger_class_id }.query(self.querier)
    }
    pub fn ledger_class_bucket_types(
        &self,
        ledger_class_id: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerClassBucketTypesResponse, cosmwasm_std::StdError> {
        QueryLedgerClassBucketTypesRequest { ledger_class_id }.query(self.querier)
    }
    pub fn ledger(
        &self,
        key: ::core::option::Option<LedgerKey>,
    ) -> Result<QueryLedgerResponse, cosmwasm_std::StdError> {
        QueryLedgerRequest { key }.query(self.querier)
    }
    pub fn ledger_entries(
        &self,
        key: ::core::option::Option<LedgerKey>,
    ) -> Result<QueryLedgerEntriesResponse, cosmwasm_std::StdError> {
        QueryLedgerEntriesRequest { key }.query(self.querier)
    }
    pub fn ledger_entry(
        &self,
        key: ::core::option::Option<LedgerKey>,
        correlation_id: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerEntryResponse, cosmwasm_std::StdError> {
        QueryLedgerEntryRequest {
            key,
            correlation_id,
        }
        .query(self.querier)
    }
    pub fn balances_as_of(
        &self,
        key: ::core::option::Option<LedgerKey>,
        as_of_date: ::prost::alloc::string::String,
    ) -> Result<QueryBalancesAsOfResponse, cosmwasm_std::StdError> {
        QueryBalancesAsOfRequest { key, as_of_date }.query(self.querier)
    }
    pub fn settlements(
        &self,
        key: ::core::option::Option<LedgerKey>,
    ) -> Result<QuerySettlementsResponse, cosmwasm_std::StdError> {
        QuerySettlementsRequest { key }.query(self.querier)
    }
    pub fn settlements_by_correlation_id(
        &self,
        key: ::core::option::Option<LedgerKey>,
        correlation_id: ::prost::alloc::string::String,
    ) -> Result<QuerySettlementsByCorrelationIdResponse, cosmwasm_std::StdError> {
        QuerySettlementsByCorrelationIdRequest {
            key,
            correlation_id,
        }
        .query(self.querier)
    }
}
