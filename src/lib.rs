mod api_key;
pub mod client;

pub use api_key::ApiKey;

/// Code represents a code that can be redeemed in Idle Champions of the Forgotten Realms.
/// For more information, visit https://idlechampions.fandom.com/wiki/Combinations
#[derive(Debug, Clone)]
pub struct Code {
    /// The code itself that can be redeemed in-game.
    pub code: String,
    /// Whether the code has likely expired, based on the expires_at timestamp.
    /// This information is often not incredibly accurate and the code may still work.
    pub expired: bool,
    /// A string RFC3339 timestamp of when the code expires.
    /// This information is often not incredibly accurate and the code may still work.
    pub expires_at: Option<String>,
    /// The creator is the person who "created" the code. This is usually a streamer or developer.
    pub creator: Option<Source>,
    /// The submitter is the person who submitted the code to some kind of list or channel,
    /// Our service tries to give credit where credit is due, when this is unknown or not provided
    /// it maps to the Creator.
    pub submitter: Option<Source>,
    /// The lister is the person who added the code to our service.
    /// We run some internal services that crawl various sources (discord, wiki, etc) and add codes
    pub lister: Option<Source>,
}
//
// /// InsertCodeRequest is the request body for inserting a code into the database.
// /// You will also need an API Key to insert codes.
// pub struct InsertCodeRequest {
//     /// The code itself that can be redeemed in-game.
//     pub code: String,
//     /// A unix timestamp of when the code expires, best guess - we recommend defaulting to next week if unknown.
//     pub expires_at: u64,
//     /// The creator is the person who "created" the code. This is usually a streamer or developer.
//     pub creator: SourceLookup,
//     /// The submitter is the person who submitted the code to some kind of list or channel
//     pub submitter: Option<SourceLookup>,
// }

/// Source represents a source of a code, such as a streamer or developer.
/// We try to maintain a list of names (always available) and URLs (best guess of where the source came from),
/// but do not guarantee complete accuracy.
///
/// Sources in the remote service are stored as unique (name, url) pairs
#[derive(Clone, Debug, serde::Deserialize)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub url: String,
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
