// Threadbags error class, goal is to move all error handling here //flipchan

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::error::Error),
    /// Could not write to database
    DBwritefail,
    /// Anyhow error wrapper
    Anyhow(anyhow::Error),
    /// Could not find entry in database
    NoEntryInDb,
    DecodeProblem,
    /// the source chain does not have support for sending to this destination chain
    UnsupportedDestinationChain,

    /// Subxt could not draft tx error
    SubxtError(subxt::Error),

    /// Could not extract data from pill node
    PillDataError,

    /// Can not fetch webhook data
    CantFetchWebhook,

    /// Error parsing scenario
    ScenarioParseError,

    /// Invalid scenarioid
    ScenarioIdNotFound,
    /// polodb database problem
    Polodb(polodb_core::Error),

    /// Invalid destination chain
    InvalidDestinationChain,
    /// Invalid chain selected
    InvalidChainOption,

    /// problem parsing the uuid from the webhook
    CouldNotFindWebhookData,
    /// problems making a http request
    HTTPRequestProblem(reqwest::Error),

    /// Sled database error
    Sled(sled::Error),
    /// Error converting string to u8
    Utf8StringError(std::string::FromUtf8Error),
}

impl From<anyhow::Error> for Error {
    fn from(src: anyhow::Error) -> Error {
        Error::Anyhow(src)
    }
}

impl From<reqwest::Error> for Error {
    fn from(src: reqwest::Error) -> Error {
        Error::HTTPRequestProblem(src)
    }
}

impl From<sled::Error> for Error {
    fn from(src: sled::Error) -> Error {
        Error::Sled(src)
    }
}

impl From<polodb_core::Error> for Error {
    fn from(src: polodb_core::Error) -> Error {
        Error::Polodb(src)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(src: std::string::FromUtf8Error) -> Error {
        Error::Utf8StringError(src)
    }
}

impl From<subxt::Error> for Error {
    fn from(src: subxt::Error) -> Error {
        Error::SubxtError(src)
    }
}
