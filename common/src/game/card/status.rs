
use super::duration::Duration;

#[derive(Debug, Clone)]
pub enum Status {
    Frozen{duration: Duration},
    Silenced{duration: Duration}
}