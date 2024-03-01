use serde_repr::{Deserialize_repr, Serialize_repr};

/// Status code from the REST API.
/// This is not for HTTP status codes.
/// Codes documented at https://documentation.ubuntu.com/lxd/en/latest/rest-api/#status-codes
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum ResourceStatus {
    OperationCreated = 100,
    Started = 101,
    Stopped = 102,
    Running = 103,
    Canceling = 104,
    Pending = 105,
    Starting = 106,
    Stopping = 107,
    Aborting = 108,
    Freezing = 109,
    Frozen = 110,
    Thawed = 111,
    Error = 112,
    Ready = 113,
    Success = 200,
    Failure = 400,
    Canceled = 401,
}
