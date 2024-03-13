pub struct DateUtils;

impl DateUtils {
    pub fn new_date_string() -> String {
        chrono::Utc::now().to_rfc3339()
    }
}
