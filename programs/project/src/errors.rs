
use anchor_lang::prelude::*;

#[error_code]
pub enum ServiceHubError {
    #[msg("Customer name cannot be empty.")]
    CustomerNameEmpty,

    #[msg("Customer name cannot exceed 50 bytes.")]
    CustomerNameTooLong,

    #[msg("Customer is not registered.")]
    CustomerNotRegistered,

    #[msg("Provider is not registered.")]
    ProviderNotRegistered,

    #[msg("Provider price must be greater than zero.")]
    InvalidPrice,

    #[msg("This booking cannot be accepted in its current status.")]
    InvalidBookingStatus,

    #[msg("Customer cannot cancel this booking.")]
    UnauthorizedCancellation,

    #[msg("Only accepted bookings can be marked as completed.")]
    InvalidCompletionStatus,

    #[msg("Only completed bookings can be approved.")]
    InvalidApprovalStatus,


    #[msg("Only newly created bookings can be cancelled.")]
    InvalidCancellationStatus,

    #[msg("The escrow does not contain enough funds to release the payment.")]
InsufficientEscrowFunds,
}
