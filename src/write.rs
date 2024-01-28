#![cfg(not(feature = "readonly"))]

/// InsertCodeRequest is the request body for inserting a code into the database.
/// You will also need an API Key to insert codes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct InsertCodeRequest {
    /// The code itself that can be redeemed in-game.
    pub code: String,
    /// A unix timestamp of when the code expires, best guess - we recommend defaulting to next week if unknown.
    pub expires_at: u64,
    /// The creator is the person who "created" the code. This is usually a streamer or developer.
    pub creator: SourceLookup,
    /// The submitter is the person who submitted the code to some kind of list or channel
    pub submitter: Option<SourceLookup>,
}

/// SourceLookup represents a source of a code, such as a streamer or developer.
/// This object is used for PUT/POST requests and do not require an ID.
///
/// We try to maintain a list of names (always available) and URLs (best guess of where the source came from),
/// but do not guarantee complete accuracy.
///
/// Sources in the remote service are stored as unique (name, url) pairs
#[derive(Clone, Debug, serde::Serialize)]
pub struct SourceLookup {
    pub name: String,
    pub url: String,
}
