pub struct ScheduledTransaction {
    pub id: String,
    pub date_first: String,
    pub date_next: String,
    pub frequency: String,
    pub amount: f64,
    pub account_id: String,
    pub account_name: String,
    pub payee_id: String,
    pub payee_name: String,
    pub category_id: String,
    pub category_name: String,
    pub deleted: bool,
}
