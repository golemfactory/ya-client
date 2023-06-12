#![allow(missing_docs)]
pub mod acceptance;
pub mod account;
pub mod activity_payment;
pub mod agreement_payment;
pub mod allocation;
pub mod debit_note;
pub mod debit_note_event;
pub mod document_status;
pub mod driver_details;
pub mod invoice;
pub mod invoice_event;
pub mod market_decoration;
pub mod network;
pub mod params;
#[allow(clippy::module_inception)]
pub mod payment;
pub mod rejection;
pub mod rejection_reason;

pub use self::acceptance::Acceptance;
pub use self::account::Account;
pub use self::activity_payment::ActivityPayment;
pub use self::agreement_payment::AgreementPayment;
pub use self::allocation::Allocation;
pub use self::allocation::NewAllocation;
pub use self::debit_note::DebitNote;
pub use self::debit_note::NewDebitNote;
pub use self::debit_note_event::{DebitNoteEvent, DebitNoteEventType};
pub use self::document_status::DocumentStatus;
pub use self::driver_details::DriverDetails;
pub use self::invoice::Invoice;
pub use self::invoice::NewInvoice;
pub use self::invoice_event::{InvoiceEvent, InvoiceEventType};
pub use self::market_decoration::MarketDecoration;
pub use self::market_decoration::MarketProperty;
pub use self::network::Network;
pub use self::payment::Payment;
pub use self::rejection::Rejection;
pub use self::rejection_reason::RejectionReason;

pub const PAYMENT_API_PATH: &str = "/payment-api/v1";
