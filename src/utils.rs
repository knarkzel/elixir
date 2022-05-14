use chrono::{prelude::*, Utc};

pub fn time_ago(published: &str) -> String {
    let previous = published.parse::<DateTime<Utc>>().unwrap();
    let current = Utc::now();
    let delta = current.signed_duration_since(previous);

    let days = delta.num_days();
    if days == 0 {
        let hours = delta.num_hours();
        if hours == 0 {
            let minutes = delta.num_minutes();
            let suffix = match minutes {
                1 => "minute ago",
                _ => "minutes ago",
            };
            format!("{} {}", minutes, suffix)
        } else {
            let suffix = match hours {
                1 => "hour ago",
                _ => "hours ago",
            };
            format!("{} {}", hours, suffix)
        }
    } else {
        let suffix = match days {
            1 => "day ago",
            _ => "days ago",
        };
        format!("{} {}", days, suffix)
    }
}
