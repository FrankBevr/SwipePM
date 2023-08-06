use scale::Decode;
use scale::Encode;

#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Errors {
    DontWork,
    OnlyAdmin,
    ParticipantChelseaIsAlreadySet,
    ParticipantManchesterIsAlreadySet,
}
