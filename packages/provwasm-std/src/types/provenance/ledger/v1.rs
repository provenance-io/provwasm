use provwasm_proc_macro::CosmwasmExt;
/// EventLedgerCreated is emitted when a new ledger is created for an asset.
/// This event is triggered by the MsgCreateLedger message handler when a
/// ledger is successfully created for a specific NFT or scope.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.EventLedgerCreated")]
pub struct EventLedgerCreated {
    /// asset class of the ledger.
    #[prost(string, tag = "1")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// nft id of the ledger (scope id or nft id).
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
}
/// EventLedgerUpdated is emitted when a ledger's configuration is updated.
/// This event is triggered by various update message handlers such as
/// MsgUpdateLedgerStatus, MsgUpdateLedgerInterestRate, MsgUpdateLedgerPayment,
/// and MsgUpdateLedgerMaturityDate when a ledger's configuration is
/// successfully modified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.EventLedgerUpdated")]
pub struct EventLedgerUpdated {
    /// asset class of the ledger.
    #[prost(string, tag = "1")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// nft id of the ledger (scope id or nft id).
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
    /// What type of data update caused this event to be emitted.
    #[prost(enumeration = "UpdateType", tag = "3")]
    pub update_type: i32,
}
/// EventLedgerEntryAdded is emitted when a new entry is added to a ledger.
/// This event is triggered by the MsgAppendLedgerEntry message handler when
/// one or more ledger entries are successfully added to an existing ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.EventLedgerEntryAdded")]
pub struct EventLedgerEntryAdded {
    /// asset class of the ledger.
    #[prost(string, tag = "1")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// nft id of the ledger (scope id or nft id).
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
    /// correlation id of the ledger entry.
    #[prost(string, tag = "3")]
    pub correlation_id: ::prost::alloc::string::String,
}
/// EventFundTransferWithSettlement is emitted when funds are transferred with
/// settlement instructions. This event is triggered by the
/// MsgTransferFundsWithSettlement message handler when a fund transfer with
/// settlement instructions is successfully processed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.EventFundTransferWithSettlement")]
pub struct EventFundTransferWithSettlement {
    /// asset class of the ledger.
    #[prost(string, tag = "1")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// nft id of the ledger (scope id or nft id).
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
    /// correlation id of the ledger entry.
    #[prost(string, tag = "3")]
    pub correlation_id: ::prost::alloc::string::String,
}
/// EventLedgerDestroyed is emitted when a ledger is destroyed.
/// This event is triggered by the MsgDestroyLedger message handler when
/// a ledger and all its associated data are successfully removed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.EventLedgerDestroyed")]
pub struct EventLedgerDestroyed {
    /// asset class of the ledger.
    #[prost(string, tag = "1")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// nft id of the ledger (scope id or nft id).
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
}
/// UpdateType is the type of data update that caused the EventLedgerUpdated event to be emitted.
/// This is used to identify the specific update that caused the event to be emitted.
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
pub enum UpdateType {
    /// The update type is unspecified.
    Unspecified = 0,
    /// The status of the ledger was updated.
    Status = 1,
    /// The interest rate of the ledger was updated.
    InterestRate = 2,
    /// The payment of the ledger was updated.
    Payment = 3,
    /// The maturity date of the ledger was updated.
    MaturityDate = 4,
}
impl UpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateType::Unspecified => "UPDATE_TYPE_UNSPECIFIED",
            UpdateType::Status => "UPDATE_TYPE_STATUS",
            UpdateType::InterestRate => "UPDATE_TYPE_INTEREST_RATE",
            UpdateType::Payment => "UPDATE_TYPE_PAYMENT",
            UpdateType::MaturityDate => "UPDATE_TYPE_MATURITY_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UPDATE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "UPDATE_TYPE_STATUS" => Some(Self::Status),
            "UPDATE_TYPE_INTEREST_RATE" => Some(Self::InterestRate),
            "UPDATE_TYPE_PAYMENT" => Some(Self::Payment),
            "UPDATE_TYPE_MATURITY_DATE" => Some(Self::MaturityDate),
            _ => None,
        }
    }
}
/// LedgerClass contains the configuration for a ledger related to a particular class of asset. The asset class
/// is defined by the either a scope spec `x/metadata`, or nft class `x/nft`. Ultimately, the configuration will
/// assist in verifying the types that are associated with particular ledger entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClass")]
pub struct LedgerClass {
    /// Unique ID for the ledger class (eg. 1, 2, 3, etc.).
    /// This is necessary since the nft class does not have an owner.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// Scope Specification ID or NFT Class ID.
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// Denom that this class of asset will be ledgered in.
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    /// Address of the maintainer for the ledger class.
    #[prost(string, tag = "4")]
    pub maintainer_address: ::prost::alloc::string::String,
}
/// LedgerClassEntryType defines the types of possible ledger entries for a given asset class. These type codes allow
/// for minimal data storage while providing a human readable description of the entry type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClassEntryType")]
pub struct LedgerClassEntryType {
    /// Unique ID for the entry type (eg. 1, 2, 3, etc.).
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// Code for the entry type (eg. "DISBURSEMENT", "PAYMENT", "ADJUSTMENT", "INTEREST", "FEE", "OTHER").
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Description of the entry type (eg. "Disbursement", "Payment", "Adjustment", "Interest", "Fee", "Other").
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// LedgerClassStatusType defines the status types for a ledger class.
/// These status types are used to track the status of underlying loan throughout the loan life cycle.
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
/// LedgerKey is used as the unique key for an asset's ledger in the keeper.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerKey")]
pub struct LedgerKey {
    /// Identifier for the nft that this ledger is linked to.
    /// This could be a `x/metadata` scope id or an `x/nft` nft id.
    /// In order to create a ledger for an nft, the nft class must be registered in the ledger module as a LedgerClass.
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    /// Scope Specification ID or NFT Class ID.
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
}
/// Ledger defines an servicing ledger for an asset.
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
    /// The units of this field are defined by the denom field in this ledger's class.
    #[prost(string, tag = "5")]
    pub next_pmt_amt: ::prost::alloc::string::String,
    /// The interest rate. Min = 0, Max = 100,000,000 = 100%, e.g. 4,321,987 = 4.321987%.
    #[prost(int32, tag = "6")]
    pub interest_rate: i32,
    /// The maturity date (days since epoch).
    #[prost(int32, tag = "7")]
    pub maturity_date: i32,
    /// The day count convention for interest calculations.
    #[prost(enumeration = "DayCountConvention", tag = "8")]
    pub interest_day_count_convention: i32,
    /// The interest accrual method for interest calculations.
    #[prost(enumeration = "InterestAccrualMethod", tag = "9")]
    pub interest_accrual_method: i32,
    /// The payment frequency.
    #[prost(enumeration = "PaymentFrequency", tag = "10")]
    pub payment_frequency: i32,
}
/// LedgerClassBucketType represents a bucket type for a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerClassBucketType")]
pub struct LedgerClassBucketType {
    /// Unique ID for the bucket type (eg. 1, 2, 3, etc.).
    #[prost(int32, tag = "1")]
    pub id: i32,
    /// Code for the bucket type (eg. "PRINCIPAL", "INTEREST", "FEE", "OTHER").
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Description of the bucket type (eg. "Principal", "Interest", "Fee", "Other").
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// LedgerEntry is an single entry in the ledger. An entry would be a payment, disbursement, adjustment, etc...
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
    /// Sequence number of the ledger entry (less than 100). This field is used to maintain the correct
    /// order of entries when multiple entries share the same effective date. Entries are sorted first
    /// by effective date, then by sequence.
    #[prost(uint32, tag = "4")]
    pub sequence: u32,
    /// The type of ledger entry specified by the LedgerClassEntryType.id.
    #[prost(int32, tag = "5")]
    pub entry_type_id: i32,
    /// The posted date (days since epoch).
    #[prost(int32, tag = "6")]
    pub posted_date: i32,
    /// The effective date (days since epoch).
    #[prost(int32, tag = "7")]
    pub effective_date: i32,
    /// The total amount of the ledger entry.
    /// The units of this field are defined by the denom field in this ledger's class.
    #[prost(string, tag = "8")]
    pub total_amt: ::prost::alloc::string::String,
    /// Applied amounts represent how the entry affects different buckets.
    #[prost(message, repeated, tag = "9")]
    pub applied_amounts: ::prost::alloc::vec::Vec<LedgerBucketAmount>,
    /// Bucket balances represent the current state of funds in each bucket.
    #[prost(message, repeated, tag = "10")]
    pub balance_amounts: ::prost::alloc::vec::Vec<BucketBalance>,
}
/// LedgerBucketAmount is the amount applied to a bucket. Applications to a bucket increase or
/// decrease the balance of the bucket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerBucketAmount")]
pub struct LedgerBucketAmount {
    /// The bucket type specified by the LedgerClassBucketType.id.
    #[prost(int32, tag = "1")]
    pub bucket_type_id: i32,
    /// The amount applied to the bucket.
    /// The units of this field are defined by the denom field in this ledger's class.
    #[prost(string, tag = "2")]
    pub applied_amt: ::prost::alloc::string::String,
}
/// BucketBalance represents the balance for a specific bucket type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.BucketBalance")]
pub struct BucketBalance {
    /// The bucket type specified by the LedgerClassBucketType.id.
    #[prost(int32, tag = "1")]
    pub bucket_type_id: i32,
    /// The balance of the bucket.
    /// The units of this field are defined by the denom field in this ledger's class.
    #[prost(string, tag = "2")]
    pub balance_amt: ::prost::alloc::string::String,
}
/// LedgerAndEntries represents a ledger with its associated entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.LedgerAndEntries")]
pub struct LedgerAndEntries {
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
/// Day Count Conventions used in interest calculations.
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
pub enum DayCountConvention {
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
impl DayCountConvention {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DayCountConvention::Unspecified => "DAY_COUNT_CONVENTION_UNSPECIFIED",
            DayCountConvention::Actual365 => "DAY_COUNT_CONVENTION_ACTUAL_365",
            DayCountConvention::Actual360 => "DAY_COUNT_CONVENTION_ACTUAL_360",
            DayCountConvention::Thirty360 => "DAY_COUNT_CONVENTION_THIRTY_360",
            DayCountConvention::ActualActual => "DAY_COUNT_CONVENTION_ACTUAL_ACTUAL",
            DayCountConvention::Days365 => "DAY_COUNT_CONVENTION_DAYS_365",
            DayCountConvention::Days360 => "DAY_COUNT_CONVENTION_DAYS_360",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DAY_COUNT_CONVENTION_UNSPECIFIED" => Some(Self::Unspecified),
            "DAY_COUNT_CONVENTION_ACTUAL_365" => Some(Self::Actual365),
            "DAY_COUNT_CONVENTION_ACTUAL_360" => Some(Self::Actual360),
            "DAY_COUNT_CONVENTION_THIRTY_360" => Some(Self::Thirty360),
            "DAY_COUNT_CONVENTION_ACTUAL_ACTUAL" => Some(Self::ActualActual),
            "DAY_COUNT_CONVENTION_DAYS_365" => Some(Self::Days365),
            "DAY_COUNT_CONVENTION_DAYS_360" => Some(Self::Days360),
            _ => None,
        }
    }
}
/// Interest Accrual Methods describing how interest is calculated over time.
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
pub enum InterestAccrualMethod {
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
impl InterestAccrualMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InterestAccrualMethod::Unspecified => "INTEREST_ACCRUAL_METHOD_UNSPECIFIED",
            InterestAccrualMethod::SimpleInterest => "INTEREST_ACCRUAL_METHOD_SIMPLE_INTEREST",
            InterestAccrualMethod::CompoundInterest => "INTEREST_ACCRUAL_METHOD_COMPOUND_INTEREST",
            InterestAccrualMethod::DailyCompounding => "INTEREST_ACCRUAL_METHOD_DAILY_COMPOUNDING",
            InterestAccrualMethod::MonthlyCompounding => {
                "INTEREST_ACCRUAL_METHOD_MONTHLY_COMPOUNDING"
            }
            InterestAccrualMethod::QuarterlyCompounding => {
                "INTEREST_ACCRUAL_METHOD_QUARTERLY_COMPOUNDING"
            }
            InterestAccrualMethod::AnnualCompounding => {
                "INTEREST_ACCRUAL_METHOD_ANNUAL_COMPOUNDING"
            }
            InterestAccrualMethod::ContinuousCompounding => {
                "INTEREST_ACCRUAL_METHOD_CONTINUOUS_COMPOUNDING"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTEREST_ACCRUAL_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
            "INTEREST_ACCRUAL_METHOD_SIMPLE_INTEREST" => Some(Self::SimpleInterest),
            "INTEREST_ACCRUAL_METHOD_COMPOUND_INTEREST" => Some(Self::CompoundInterest),
            "INTEREST_ACCRUAL_METHOD_DAILY_COMPOUNDING" => Some(Self::DailyCompounding),
            "INTEREST_ACCRUAL_METHOD_MONTHLY_COMPOUNDING" => Some(Self::MonthlyCompounding),
            "INTEREST_ACCRUAL_METHOD_QUARTERLY_COMPOUNDING" => Some(Self::QuarterlyCompounding),
            "INTEREST_ACCRUAL_METHOD_ANNUAL_COMPOUNDING" => Some(Self::AnnualCompounding),
            "INTEREST_ACCRUAL_METHOD_CONTINUOUS_COMPOUNDING" => Some(Self::ContinuousCompounding),
            _ => None,
        }
    }
}
/// Payment frequencies for loan repayments.
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
}
/// StoredSettlementInstructions is used as the stored version of settlement instructions against a ledger key and entry
/// correlation id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.StoredSettlementInstructions")]
pub struct StoredSettlementInstructions {
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
    /// Pending funding transfer status.
    Pending = 1,
    /// Processing funding transfer status.
    Processing = 2,
    /// Completed funding transfer status.
    Completed = 3,
    /// Failed funding transfer status.
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
/// GenesisState represents the initial state of the ledger store.
/// This structure matches the test.json format for bulk import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisState")]
pub struct GenesisState {
    /// Ledger classes configuration.
    #[prost(message, repeated, tag = "1")]
    pub ledger_classes: ::prost::alloc::vec::Vec<LedgerClass>,
    /// Ledger class entry types configuration.
    #[prost(message, repeated, tag = "2")]
    pub ledger_class_entry_types: ::prost::alloc::vec::Vec<GenesisLedgerClassEntryType>,
    /// Ledger class status types configuration.
    #[prost(message, repeated, tag = "3")]
    pub ledger_class_status_types: ::prost::alloc::vec::Vec<GenesisLedgerClassStatusType>,
    /// Ledger class bucket types configuration.
    #[prost(message, repeated, tag = "4")]
    pub ledger_class_bucket_types: ::prost::alloc::vec::Vec<GenesisLedgerClassBucketType>,
    /// Ledgers.
    #[prost(message, repeated, tag = "5")]
    pub ledgers: ::prost::alloc::vec::Vec<GenesisLedger>,
    /// Ledger entries.
    #[prost(message, repeated, tag = "6")]
    pub ledger_entries: ::prost::alloc::vec::Vec<GenesisLedgerEntry>,
    /// Settlement instructions for fund transfers.
    #[prost(message, repeated, tag = "7")]
    pub settlement_instructions: ::prost::alloc::vec::Vec<GenesisStoredSettlementInstructions>,
}
/// GenesisLedgerClassEntryType is a single ledger class entry type with its key. This is used for the genesis state
/// import/export so that we can avoid non-deterministic maps in the genesis functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisLedgerClassEntryType")]
pub struct GenesisLedgerClassEntryType {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<GenesisPair>,
    #[prost(message, optional, tag = "2")]
    pub entry_type: ::core::option::Option<LedgerClassEntryType>,
}
/// GenesisLedgerClassStatusType is a single ledger class status type with its key. This is used for the genesis state
/// import/export so that we can avoid non-deterministic maps in the genesis functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisLedgerClassStatusType")]
pub struct GenesisLedgerClassStatusType {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<GenesisPair>,
    #[prost(message, optional, tag = "2")]
    pub status_type: ::core::option::Option<LedgerClassStatusType>,
}
/// GenesisLedgerClassBucketType is a single ledger class bucket type with its key. This is used for the genesis state
/// import/export so that we can avoid non-deterministic maps in the genesis functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisLedgerClassBucketType")]
pub struct GenesisLedgerClassBucketType {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<GenesisPair>,
    #[prost(message, optional, tag = "2")]
    pub bucket_type: ::core::option::Option<LedgerClassBucketType>,
}
/// GenesisLedger is a single ledger type with its key. This is used for the genesis state import/export so that we
/// can avoid non-deterministic maps in the genesis functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisLedger")]
pub struct GenesisLedger {
    #[prost(message, optional, tag = "2")]
    pub ledger: ::core::option::Option<Ledger>,
}
/// GenesisLedgerEntry is a single ledger entry with its key. This is used for the genesis state import/export so that we
/// can avoid non-deterministic maps in the genesis functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisLedgerEntry")]
pub struct GenesisLedgerEntry {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    #[prost(message, optional, tag = "2")]
    pub entry: ::core::option::Option<LedgerEntry>,
}
/// GenesisStoredSettlementInstructions is a single settlement instruction with its key. This is used for the genesis
/// state import/export so that we can avoid non-deterministic maps in the genesis functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisStoredSettlementInstructions")]
pub struct GenesisStoredSettlementInstructions {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<GenesisPair>,
    #[prost(message, optional, tag = "2")]
    pub settlement_instructions: ::core::option::Option<StoredSettlementInstructions>,
}
/// GenesisPair represents a simple pair of strings that can be used as the key for a collections export.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.GenesisPair")]
pub struct GenesisPair {
    #[prost(string, tag = "1")]
    pub p1: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub p2: ::prost::alloc::string::String,
}
/// QueryLedgerClassRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClass",
    response_type = QueryLedgerClassResponse
)]
pub struct QueryLedgerClassRequest {
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassResponse")]
pub struct QueryLedgerClassResponse {
    #[prost(message, optional, tag = "1")]
    pub ledger_class: ::core::option::Option<LedgerClass>,
}
/// QueryLedgerClassesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClasses",
    response_type = QueryLedgerClassesResponse
)]
pub struct QueryLedgerClassesRequest {
    /// pagination is an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryLedgerClassesResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassesResponse")]
pub struct QueryLedgerClassesResponse {
    /// List of ledger classes.
    #[prost(message, repeated, tag = "1")]
    pub ledger_classes: ::prost::alloc::vec::Vec<LedgerClass>,
    /// pagination is the pagination details for this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryLedgerClassEntryTypesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassEntryTypesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClassEntryTypes",
    response_type = QueryLedgerClassEntryTypesResponse
)]
pub struct QueryLedgerClassEntryTypesRequest {
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassEntryTypesResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassEntryTypesResponse")]
pub struct QueryLedgerClassEntryTypesResponse {
    #[prost(message, repeated, tag = "1")]
    pub entry_types: ::prost::alloc::vec::Vec<LedgerClassEntryType>,
}
/// QueryLedgerClassStatusTypesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassStatusTypesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClassStatusTypes",
    response_type = QueryLedgerClassStatusTypesResponse
)]
pub struct QueryLedgerClassStatusTypesRequest {
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassStatusTypesResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassStatusTypesResponse")]
pub struct QueryLedgerClassStatusTypesResponse {
    #[prost(message, repeated, tag = "1")]
    pub status_types: ::prost::alloc::vec::Vec<LedgerClassStatusType>,
}
/// QueryLedgerClassBucketTypesRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassBucketTypesRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerClassBucketTypes",
    response_type = QueryLedgerClassBucketTypesResponse
)]
pub struct QueryLedgerClassBucketTypesRequest {
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
}
/// QueryLedgerClassBucketTypesResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerClassBucketTypesResponse")]
pub struct QueryLedgerClassBucketTypesResponse {
    #[prost(message, repeated, tag = "1")]
    pub bucket_types: ::prost::alloc::vec::Vec<LedgerClassBucketType>,
}
/// QueryLedgerRequest
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
/// QueryLedgerResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerResponse")]
pub struct QueryLedgerResponse {
    /// The ledger data.
    #[prost(message, optional, tag = "1")]
    pub ledger: ::core::option::Option<Ledger>,
}
/// QueryLedgersRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgersRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/Ledgers",
    response_type = QueryLedgersResponse
)]
pub struct QueryLedgersRequest {
    /// pagination is an  pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryLedgersResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgersResponse")]
pub struct QueryLedgersResponse {
    /// List of ledgers.
    #[prost(message, repeated, tag = "1")]
    pub ledgers: ::prost::alloc::vec::Vec<Ledger>,
    /// pagination is the pagination details for this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryLedgerEntriesRequest
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
/// QueryLedgerEntriesResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntriesResponse")]
pub struct QueryLedgerEntriesResponse {
    /// The ledger entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<LedgerEntry>,
}
/// QueryLedgerEntryRequest
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
/// QueryLedgerEntryResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerEntryResponse")]
pub struct QueryLedgerEntryResponse {
    /// The ledger entry.
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<LedgerEntry>,
}
/// QueryLedgerBalancesAsOfRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerBalancesAsOfRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerBalancesAsOf",
    response_type = QueryLedgerBalancesAsOfResponse
)]
pub struct QueryLedgerBalancesAsOfRequest {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The date to query balances as of (format: YYYY-MM-DD).
    #[prost(string, tag = "2")]
    pub as_of_date: ::prost::alloc::string::String,
}
/// QueryLedgerBalancesAsOfResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerBalancesAsOfResponse")]
pub struct QueryLedgerBalancesAsOfResponse {
    /// BucketBalances represents the current balances for principal, interest, and other amounts.
    #[prost(message, repeated, tag = "1")]
    pub bucket_balances: ::prost::alloc::vec::Vec<BucketBalance>,
}
/// QueryLedgerSettlementsRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerSettlementsRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerSettlements",
    response_type = QueryLedgerSettlementsResponse
)]
pub struct QueryLedgerSettlementsRequest {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
}
/// QueryLedgerSettlementsResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerSettlementsResponse")]
pub struct QueryLedgerSettlementsResponse {
    #[prost(message, repeated, tag = "1")]
    pub settlements: ::prost::alloc::vec::Vec<SettlementInstruction>,
}
/// QueryLedgerSettlementsByCorrelationIDRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerSettlementsByCorrelationIDRequest")]
#[proto_query(
    path = "/provenance.ledger.v1.Query/LedgerSettlementsByCorrelationID",
    response_type = QueryLedgerSettlementsByCorrelationIdResponse
)]
pub struct QueryLedgerSettlementsByCorrelationIdRequest {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The correlation ID to query settlements for.
    #[prost(string, tag = "2")]
    pub correlation_id: ::prost::alloc::string::String,
}
/// QueryLedgerSettlementsByCorrelationIDResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.QueryLedgerSettlementsByCorrelationIDResponse")]
pub struct QueryLedgerSettlementsByCorrelationIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub settlements: ::prost::alloc::vec::Vec<SettlementInstruction>,
}
/// MsgCreateLedgerRequest represents a request to create a new ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateLedgerRequest")]
pub struct MsgCreateLedgerRequest {
    /// The ledger to create.
    #[prost(message, optional, tag = "1")]
    pub ledger: ::core::option::Option<Ledger>,
    /// The signer address that can create ledgers.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgCreateLedgerResponse represents the response from creating a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateLedgerResponse")]
pub struct MsgCreateLedgerResponse {}
/// MsgUpdateStatusRequest represents a request to update the status of a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgUpdateStatusRequest")]
pub struct MsgUpdateStatusRequest {
    /// Ledger key of the ledger whose status is being updated.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The signer that is updating the status.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
    /// The new status type id of the ledger.
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
    /// Ledger key of the ledger whose interest rate is being updated.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The signer that is updating the interest rate.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
    /// The new interest rate of the ledger.
    #[prost(int32, tag = "3")]
    pub interest_rate: i32,
    /// The new interest day count convention of the ledger.
    #[prost(enumeration = "DayCountConvention", tag = "4")]
    pub interest_day_count_convention: i32,
    /// The new interest accrual method of the ledger.
    #[prost(enumeration = "InterestAccrualMethod", tag = "5")]
    pub interest_accrual_method: i32,
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
    /// Ledger key of the ledger whose payment is being updated.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The signer that is updating the payment.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
    /// The new next payment amount of the ledger.
    /// The units of this field are defined by the denom field in this ledger's class.
    #[prost(string, tag = "3")]
    pub next_pmt_amt: ::prost::alloc::string::String,
    /// The new next payment date in days since epoch.
    #[prost(int32, tag = "4")]
    pub next_pmt_date: i32,
    /// The new payment frequency of the ledger.
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
    /// Ledger key of the ledger whose maturity date is being updated.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The signer that is updating the maturity date.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
    /// The new maturity date in days since epoch.
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
    /// Ledger key of the ledger whose entries are being appended.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The entries to append to the ledger.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<LedgerEntry>,
    /// The signer that is appending the entries.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
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
    /// Ledger key of the ledger whose balances are being updated.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The signer that is updating the balances.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
    /// The correlation id of the ledger entry.
    #[prost(string, tag = "3")]
    pub correlation_id: ::prost::alloc::string::String,
    /// The total amount of the ledger entry.
    /// The units of this field are defined by the denom field in this ledger's class.
    #[prost(string, tag = "4")]
    pub total_amt: ::prost::alloc::string::String,
    /// Applied amounts represent how the entry affects different buckets.
    #[prost(message, repeated, tag = "5")]
    pub applied_amounts: ::prost::alloc::vec::Vec<LedgerBucketAmount>,
    /// Bucket balances represent the current state of funds in each bucket.
    #[prost(message, repeated, tag = "6")]
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
    /// The signer that is transferring the funds.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// The transfers to be made.
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
    /// Ledger key of the ledger to destroy.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<LedgerKey>,
    /// The signer that is destroying the ledger.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgDestroyResponse represents the response from destroying a ledger.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgDestroyResponse")]
pub struct MsgDestroyResponse {}
/// MsgCreateLedgerClassRequest represents a request to create a new ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateLedgerClassRequest")]
pub struct MsgCreateLedgerClassRequest {
    /// The ledger class to create.
    #[prost(message, optional, tag = "1")]
    pub ledger_class: ::core::option::Option<LedgerClass>,
    /// The signer that is creating the ledger class.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgCreateLedgerClassResponse represents the response from creating a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgCreateLedgerClassResponse")]
pub struct MsgCreateLedgerClassResponse {}
/// MsgAddLedgerClassStatusTypeRequest represents a request to add a status type to a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddLedgerClassStatusTypeRequest")]
pub struct MsgAddLedgerClassStatusTypeRequest {
    /// Ledger class id to add the status type to.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The status type to add to the ledger class.
    #[prost(message, optional, tag = "2")]
    pub status_type: ::core::option::Option<LedgerClassStatusType>,
    /// The signer that is adding the status type.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAddLedgerClassStatusTypeResponse represents the response from adding a status type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddLedgerClassStatusTypeResponse")]
pub struct MsgAddLedgerClassStatusTypeResponse {}
/// MsgAddLedgerClassEntryTypeRequest represents a request to add an entry type to a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddLedgerClassEntryTypeRequest")]
pub struct MsgAddLedgerClassEntryTypeRequest {
    /// Ledger class id to add the entry type to.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The entry type to add to the ledger class.
    #[prost(message, optional, tag = "2")]
    pub entry_type: ::core::option::Option<LedgerClassEntryType>,
    /// The signer that is adding the entry type.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAddLedgerClassEntryTypeResponse represents the response from adding an entry type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddLedgerClassEntryTypeResponse")]
pub struct MsgAddLedgerClassEntryTypeResponse {}
/// MsgAddLedgerClassBucketTypeRequest represents a request to add a bucket type to a ledger class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddLedgerClassBucketTypeRequest")]
pub struct MsgAddLedgerClassBucketTypeRequest {
    /// Ledger class id to add the bucket type to.
    #[prost(string, tag = "1")]
    pub ledger_class_id: ::prost::alloc::string::String,
    /// The bucket type to add to the ledger class.
    #[prost(message, optional, tag = "2")]
    pub bucket_type: ::core::option::Option<LedgerClassBucketType>,
    /// The signer that is adding the bucket type.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAddLedgerClassBucketTypeResponse represents the response from adding a bucket type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgAddLedgerClassBucketTypeResponse")]
pub struct MsgAddLedgerClassBucketTypeResponse {}
/// MsgBulkCreateRequest represents a request to bulk create ledgers and entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgBulkCreateRequest")]
pub struct MsgBulkCreateRequest {
    /// The signer that is bulk importing the ledger data.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// The genesis state to bulk import.
    #[prost(message, repeated, tag = "2")]
    pub ledger_and_entries: ::prost::alloc::vec::Vec<LedgerAndEntries>,
}
/// MsgBulkCreateResponse represents the response from bulk creating ledgers and entries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.ledger.v1.MsgBulkCreateResponse")]
pub struct MsgBulkCreateResponse {}
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
    pub fn ledger_classes(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryLedgerClassesResponse, cosmwasm_std::StdError> {
        QueryLedgerClassesRequest { pagination }.query(self.querier)
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
    pub fn ledgers(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryLedgersResponse, cosmwasm_std::StdError> {
        QueryLedgersRequest { pagination }.query(self.querier)
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
    pub fn ledger_balances_as_of(
        &self,
        key: ::core::option::Option<LedgerKey>,
        as_of_date: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerBalancesAsOfResponse, cosmwasm_std::StdError> {
        QueryLedgerBalancesAsOfRequest { key, as_of_date }.query(self.querier)
    }
    pub fn ledger_settlements(
        &self,
        key: ::core::option::Option<LedgerKey>,
    ) -> Result<QueryLedgerSettlementsResponse, cosmwasm_std::StdError> {
        QueryLedgerSettlementsRequest { key }.query(self.querier)
    }
    pub fn ledger_settlements_by_correlation_id(
        &self,
        key: ::core::option::Option<LedgerKey>,
        correlation_id: ::prost::alloc::string::String,
    ) -> Result<QueryLedgerSettlementsByCorrelationIdResponse, cosmwasm_std::StdError> {
        QueryLedgerSettlementsByCorrelationIdRequest {
            key,
            correlation_id,
        }
        .query(self.querier)
    }
}
