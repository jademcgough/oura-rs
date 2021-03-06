use serde::Deserialize;
use url::Url;
// use main::
use clap::Clap;

pub struct Client {
    pub token: String, // personal access token
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    age: u8,
    weight: f32,
    height: u16,
    gender: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct Readiness {
    readiness: Vec<ReadinessPeriod>,
}

#[derive(Deserialize, Debug)]
struct ReadinessPeriod {
    summary_date: String, // change this to parse into date type
    period_id: u8,
    score: u8,
    score_previous_night: u8,
    score_sleep_balance: u8,
    score_previous_day: u8,
    score_activity_balance: u8,
    score_resting_hr: u8,
    score_hrv_balance: u8,
    score_recovery_index: u8,
    score_temperature: u8,
}

#[derive(Deserialize, Debug)]
struct Sleep {
    sleep: Vec<SleepPeriod>,
}

#[derive(Deserialize, Debug)]
struct SleepPeriod {
    summary_date: String, // change this to parse into date type
    period_id: u8,
    is_longest: u8,
    timezone: i32,
    bedtime_start: String, // should be date
    bedtime_end: String,   // shound be date
    score: u8,
    score_total: u8,
    score_disturbances: u8,
    score_efficiency: u8,
    score_latency: u8,
    score_rem: u8,
    score_deep: u8,
    score_alignment: u8,
    total: u16,
    duration: u16,
    awake: u16,
    light: u16,
    rem: u16,
    deep: u16,
    onset_latency: u16,
    restless: u8,
    efficiency: u8,
    midpoint_time: u16,
    hr_lowest: u16,
    hr_average: f32,
    rmssd: u16,
    breath_average: f32,
    temperature_delta: f32,
    hypnogram_5min: String, // this is a weird one
    hr_5min: Vec<u8>,
    rmssd_5min: Vec<u16>,
}

#[derive(Deserialize, Debug)]
struct Activity {
    activity: Vec<ActivityPeriod>,
}

#[derive(Deserialize, Debug)]
struct ActivityPeriod {
    summary_date: String, // should be date
    day_start: String,    // should be date
    day_end: String,      // should be date
    timezone: i32,
    score: u8,
    score_stay_active: u8,
    score_move_every_hour: u8,
    score_meet_daily_targets: u8,
    score_training_frequency: u8,
    score_training_volume: u8,
    score_recovery_time: u8,
    daily_movement: u32,
    non_wear: u16,
    rest: u16,
    inactive: u16,
    inactivity_alerts: u8,
    low: u16,
    medium: u16,
    high: u16,
    steps: u16,
    cal_total: u16,
    cal_active: u16,
    met_min_inactive: u16,
    met_min_low: u16,
    #[serde(default)]
    met_min_medium_plus: u16, // not included if no "high" activity
    met_min_medium: u16,
    met_min_high: u16,
    average_met: f32,
    class_5min: String, // also a weird one
    met_1min: Vec<f32>,
}

#[derive(Deserialize, Debug)]
struct Bedtime {
    ideal_bedtimes: Vec<IdealBedtimes>,
}

#[derive(Deserialize, Debug)]
struct IdealBedtimes {
    date: String, // change this to parse into date type
    bedtime_window: BedtimeWindow,
    status: String, // should probably be an enum
}

#[derive(Deserialize, Debug)]
struct BedtimeWindow {
    // both of these can be null
    start: Option<i32>,
    end: Option<i32>,
}

// TODO: we don't want clap to be required within lib.
// can probably get rid of this dependency by not using derive for clap within main.rs
#[derive(Clap)]
pub struct TimeOpts {
    /// <YYYY-MM-DD> Start date. Defaults to one week from now.
    #[clap(long)]
    start: Option<String>,
    /// <YYYY-MM-DD> End date. Defaults to today.
    #[clap(long)]
    end: Option<String>,
}

macro_rules! endpoint {
    ($name:ident, $type:ty, $url:literal) => {
        pub fn $name(&self) -> Result<(), Box<dyn std::error::Error>> {
            let url = Url::parse_with_params($url, &[("access_token", self.token.clone())])?;
            let resp = reqwest::blocking::get(url)?.json::<$type>()?;
            println!("{:#?}", resp);
            Ok(())
        }
    };
}

impl Client {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn activity(&self, opts: TimeOpts) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            "https://api.ouraring.com/v1/activity",
            &[("access_token", self.token.clone())],
        )?;
        let resp = reqwest::blocking::get(url)?.json::<Activity>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    // public endpoint functions
    // function name | struct it populates | api endpoint
    endpoint!(info, UserInfo, "https://api.ouraring.com/v1/userinfo");
    endpoint!(sleep, Sleep, "https://api.ouraring.com/v1/sleep");
    endpoint!(bedtime, Bedtime, "https://api.ouraring.com/v1/bedtime");
    endpoint!(
        readiness,
        Readiness,
        "https://api.ouraring.com/v1/readiness"
    );
}
