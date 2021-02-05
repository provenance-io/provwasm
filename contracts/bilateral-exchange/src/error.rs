use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Must send collateral")]
    MissingCollateral {},

    #[error("Must send an ask")]
    MissingAsk {},

    #[error("Must send an offer")]
    MissingOffer {},

    #[error("Offer must match ask (offer {offer:?}, ask: {ask:?})")]
    AskOfferMismatch { ask: Vec<Coin>, offer: Vec<Coin> },

    #[error("Cannot send funds when canceling contract")]
    CancelWithFunds {},
}
