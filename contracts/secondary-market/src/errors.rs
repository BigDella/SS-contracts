use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidAmount = 3,
    InvalidPrice = 4,
    InsufficientBalance = 5,
    ListingNotFound = 6,
    ListingNotActive = 7,
    ListingExpired = 8,
    Unauthorized = 9,
    ExceedsListedQuantity = 10,
    InvalidExpiry = 11,
}
