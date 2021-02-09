use serde::Deserialize;
use chrono::{
    Duration,
    Local,
    NaiveDateTime
};

use crate::utils::error_mapper::ElogError;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct DateQueryParameters {
    since_when: Option<i64>,
    until_when: Option<i64>
}

// Default implementation for when user didn't send any date on endpoint
impl Default for DateQueryParameters {
    fn default() -> Self {
        DateQueryParameters {
            since_when: Some(0),
            until_when: Some(Local::now().timestamp_millis())
        }
    }
}

impl DateQueryParameters {
    pub fn get_naive_date_time_from_millis(&self) -> Result<(NaiveDateTime, NaiveDateTime), ElogError> {
        if self.since_when > self.until_when {
            return Err(ElogError::BadQueryParameters(
                format!("Since date {} is greater than Until date {}",
                Self::naive_date_time_from_duration(self.since_when.unwrap_or_default()),
                Self::naive_date_time_from_duration(self.until_when.unwrap_or_default())
            )));
        }
        Ok((
            Self::naive_date_time_from_duration(self.since_when.unwrap_or_default()),
            Self::naive_date_time_from_duration(self.until_when.unwrap_or_default()),
        ))
    }

    fn naive_date_time_from_duration(millis: i64) -> NaiveDateTime {
        let duration = Duration::milliseconds(millis).to_std().unwrap();
        NaiveDateTime::from_timestamp(
                duration.as_secs() as i64,
                duration.subsec_nanos()
        )
    }
}
