#[cfg(feature = "serde")]
use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Accounts/getAccountById
pub struct Account {
    pub id: String,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub account_type: String,
    pub on_budget: bool,
    pub closed: bool,
    pub note: String,
    pub balance: i64,
    pub cleared_balance: i64,
    pub uncleared_balance: i64,
    pub transfer_payee_id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Transactions/getTransactionById
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: String,
    pub cleared: String,
    pub approved: bool,
    pub flag_color: Option<String>,
    pub account_id: String,
    pub payee_id: String,
    pub category_id: String,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: bool,
    pub account_name: String,
    pub payee_name: String,
    pub category_name: String,
    pub subtransactions: Vec<SubTransaction>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubTransaction {
    pub id: String,
    pub scheduled_transaction_id: Option<String>,
    pub amount: i64,
    pub memo: String,
    pub payee_id: String,
    pub category_id: String,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Scheduled_Transactions/getScheduledTransactionById
pub struct ScheduledTransaction {
    pub id: String,
    pub date_first: String,
    pub date_next: String,
    pub frequency: Option<RecurFrequency>,
    pub amount: i64,
    pub memo: Option<String>,
    pub flag_color: Option<String>,
    pub account_id: String,
    pub payee_id: String,
    pub category_id: String,
    pub transfer_account_id: Option<String>,
    pub deleted: bool,
    pub account_name: String,
    pub payee_name: String,
    pub category_name: String,
    pub subtransactions: Vec<SubTransaction>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Months/getBudgetMonth
pub struct Month {
    pub month: String,
    pub note: String,
    pub income: i64,
    pub budgeted: i64,
    pub activity: i64,
    pub to_be_budgeted: i64,
    pub age_of_money: i64,
    pub deleted: bool,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Categories/getCategoryById
pub struct Category {
    pub id: String,
    pub category_group_id: String,
    pub name: String,
    pub hidden: bool,
    pub original_category_group_id: Option<String>,
    pub note: Option<String>,
    pub budgeted: i64,
    pub activity: i64,
    pub balance: i64,
    pub goal_type: Option<String>,
    pub goal_creation_month: Option<String>,
    pub goal_target: i64,
    pub goal_target_month: Option<String>,
    pub goal_percentage_complete: Option<i64>,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub hidden: Option<bool>,
    pub deleted: bool,
    pub transfer_account_id: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Payees/getPayeeById
pub struct Payee {
    pub id: String,
    pub name: String,
    pub transfer_account_id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///See https://api.youneedabudget.com/v1#/Payee_Locations/getPayeeLocationById
pub struct PayeeLocation {
    pub id: String,
    pub payee_id: String,
    pub latitude: String,
    pub longitude: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CurrencyFormat {
    pub iso_code: String,
    pub example_format: String,
    pub decimal_digits: i64,
    pub decimal_separator: String,
    pub symbol_first: bool,
    pub group_separator: String,
    pub currency_symbol: String,
    pub display_symbol: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateFormat {
    pub format: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/Budgets/getBudgetById
pub struct Budget {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: DateFormat,
    pub currency_format: CurrencyFormat,
    pub accounts: Vec<Account>,
    pub payees: Vec<CategoryGroup>,
    pub payee_locations: Vec<PayeeLocation>,
    pub category_groups: Vec<CategoryGroup>,
    pub categories: Vec<Category>,
    pub months: Vec<Month>,
    pub transactions: Vec<Transaction>,
    pub subtransactions: Vec<SubTransaction>,
    pub scheduled_transactions: Vec<ScheduledTransaction>,
    pub scheduled_subtransactions: Vec<SubTransaction>,
    pub server_knowledge: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// See https://api.youneedabudget.com/v1#/User/getUser
pub struct User {
    pub id: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum RecurFrequency {
    Never,
    Daily,
    Weekly,
    EveryOtherWeek,
    TwiceAMonth,
    Every4Weeks,
    Monthly,
    EveryOtherMonth,
    Every3Months,
    Every4Months,
    TwiceAYear,
    Yearly,
    EveryOtherYear,
}