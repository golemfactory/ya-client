pub mod acceptance;
pub mod allocation;
pub mod debit_note;
pub mod debit_note_event;
pub mod document_status;
pub mod event_type;
pub mod invoice;
pub mod invoice_event;
pub mod payment;
pub mod rejection;
pub mod rejection_reason;

pub use self::acceptance::Acceptance;
pub use self::allocation::Allocation;
pub use self::allocation::NewAllocation;
pub use self::debit_note::DebitNote;
pub use self::debit_note::NewDebitNote;
pub use self::debit_note_event::DebitNoteEvent;
pub use self::document_status::DocumentStatus;
pub use self::event_type::EventType;
pub use self::invoice::Invoice;
pub use self::invoice::NewInvoice;
pub use self::invoice_event::InvoiceEvent;
pub use self::payment::Payment;
pub use self::rejection::Rejection;
pub use self::rejection_reason::RejectionReason;

pub const PAYMENT_API_PATH: &str = "payment-api/v1/";
