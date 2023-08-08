use core::fmt::Debug;
use core::prelude::rust_2018::derive;
use scale::Decode;
use scale::Encode;

/// Represents the possible error conditions in football_match.
#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Errors {
    /// An general error occurred
    DontWork,
    /// An error occurred due caller is not admin.
    OnlyAdmin,
    /// An error occurred due value of participant_chelsea is already set.
    ParticipantChelseaIsAlreadySet,
    /// An error occurred due value of participant_manchester is already set.
    ParticipantManchesterIsAlreadySet,
    /// An error occurred due value is not 1 or 2.
    OnlyOneOrTwo,
}
