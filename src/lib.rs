pub mod client;
#[cfg(feature = "write")]
pub mod write;

#[cfg(feature = "write")]
pub mod api_key;

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
