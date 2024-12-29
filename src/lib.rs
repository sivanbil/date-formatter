use chrono::{DateTime, Local, NaiveDate, ParseError, Utc};

/// 将日期格式化为指定的字符串格式。
pub fn format_date(date: NaiveDate, format: &str) -> String {
    date.format(format).to_string()
}

/// 将字符串解析为日期。
pub fn parse_date(date_str: &str, format: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date_str, format)
}

/// 计算两个日期之间的天数差异。
pub fn days_between(date1: NaiveDate, date2: NaiveDate) -> i64 {
    (date1 - date2).num_days()
}

/// 获取当前日期和时间。
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// 获取当前日期和时间（本地时间）。
pub fn now_local() -> DateTime<Local> {
    Local::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 29).unwrap();
        assert_eq!(format_date(date, "%Y-%m-%d"), "2024-12-29");
        assert_eq!(format_date(date, "%d/%m/%Y"), "29/12/2024");
    }

    #[test]
    fn test_parse_date() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 29).unwrap();
        assert_eq!(parse_date("2024-12-29", "%Y-%m-%d").unwrap(), date);
        assert_eq!(parse_date("29/12/2024", "%d/%m/%Y").unwrap(), date);
    }

    #[test]
    fn test_days_between() {
        let date1 = NaiveDate::from_ymd_opt(2024, 12, 29).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert_eq!(days_between(date1, date2), 4);
    }
}