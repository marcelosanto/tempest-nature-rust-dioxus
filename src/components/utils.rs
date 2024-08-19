use chrono::{Local, TimeZone, Utc};

pub fn get_day_and_hours_now() -> String {
    let local_now = Local::now();
    let day_and_hours = local_now.format("%A, %H:%M").to_string();

    return day_and_hours;
}

pub fn converte_unix_time_in_hours(time: i64) -> String {
    // O timestamp Unix em segundos (UTC)
    let timestamp_utc = time;

    // Converte o timestamp para um objeto DateTime no UTC
    let datetime_utc = Utc.timestamp_opt(timestamp_utc, 0).unwrap();

    // Converte para a hora no fuso horário UTC-3
    let datetime_utc_minus = datetime_utc.with_timezone(&Local);

    let hora_utc_minus = datetime_utc_minus.format("%H:%M:%S").to_string();

    return hora_utc_minus;
}
