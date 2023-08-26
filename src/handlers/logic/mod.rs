use futures::TryStreamExt;
use sqlx::{Postgres, Row};

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn get_sqlx_database_client() -> Result<PgPool, Box<dyn Error>> {
    let database_url = dotenv::var("POSTGRESQL_URL").expect("POSTGRESQL_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "greet and bot info")]
    Start,
    #[command(description = "display this text")]
    Help,
    #[command(description = "total amount of money", parse_with = "split")]
    Total,
    #[command(description = "accounts amount of money")]
    Accounts,
    #[command(description = "add account\nexample: /addaccount sber 150", parse_with = "split")]
    AddAccount { name: String, balance: i64 },
    #[command(description = "edit account", parse_with = "split")]
    EditAccount { id: i64, name: String, balance: i64 },
    #[command(description = "delete account")]
    DelAccount(i64),
    #[command(description = "available categories")]
    Categories,
    #[command(description = "add category\nexample: /addcategory продукта продукты_из_магазина", parse_with = "split")]
    AddCategory { name: String, description: String },
    #[command(description = "edit category", parse_with = "split")]
    EditCategory {
        id: i64,
        name: String,
        description: String,
    },
    #[command(description = "delete category")]
    DelCategory(i64),
    #[command(description = "display expenses", parse_with = "split")]
    Expenses,
    #[command(description = "display income", parse_with = "split")]
    Income,
    #[command(description = "add expense\nexample: /addexpense 200 кафе tinkoff", parse_with = "split")]
    AddExpense {
        amount: i64,
        category: String,
        account: String,
    },
    #[command(description = "add income\nexample: /addincome 500 зарплата tinkoff", parse_with = "split")]
    AddIncome {
        amount: i64,
        category: String,
        account: String,
    },
    #[command(description = "delete expense")]
    DelExp(i64),
    #[command(description = "delete income")]
    DelInc(i64),
}
pub struct Categories {
    pub id: Option<i64>,
    pub name: String,
    pub user_id: i64,
    pub description: String,
}
impl Categories {
    pub async fn add(&self, pool: PgPool) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO categories (name, user_id, description) VALUES ($1, $2, $3)";
        sqlx::query(query)
            .bind(&self.name)
            .bind(&self.user_id)
            .bind(&self.description)
            .execute(&pool)
            .await?;
        Ok(())
    }
}

pub struct Accounts {
    pub id: Option<i64>,
    pub name: String,
    pub balance: i64,
    pub user_id: i64,
}

impl Accounts {
    pub async fn add(&self, pool: PgPool) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO accounts (name, balance, user_id) VALUES ($1, $2, $3)";
        sqlx::query(query)
            .bind(&self.name)
            .bind(&self.balance)
            .bind(&self.user_id)
            .execute(&pool)
            .await?;
        Ok(())
    }
}

pub struct Expenses {
    pub id: i64,
    pub account: String,
    pub category: String,
    pub amount: i64,
    pub user_id: i64,
}

pub struct Income {
    pub id: i64,
    pub account: String,
    pub category: String,
    pub amount: i64,
    pub user_id: i64,
}
// pub struct Database {
//     pub pool: PgPool
// }

// impl Database {
//     fn get_total(&self) -> i128{
//         32
//     }
// }

pub async fn get_accounts(pool: PgPool, user_id: i64) -> Result<Vec<Accounts>, Box<dyn Error>> {
    let q = "SELECT * FROM accounts WHERE user_id = $1 ";
    let query = sqlx::query(q).bind(user_id);
    let mut rows = query.fetch(&pool);

    let mut accounts = vec![];

    while let Some(row) = rows.try_next().await? {
        accounts.push(Accounts {
            id: row.get("id"),
            name: row.get("name"),
            balance: row.get("balance"),
            user_id: row.get("user_id"),
        })
    }

    Ok(accounts)
}

pub async fn get_categories(pool: PgPool, user_id: i64) -> Result<Vec<Categories>, Box<dyn Error>> {
    let q = "SELECT * FROM categories WHERE user_id = $1 ";
    let query = sqlx::query(q).bind(user_id);
    let mut rows = query.fetch(&pool);

    let mut categories = vec![];

    while let Some(row) = rows.try_next().await? {
        categories.push(Categories {
            id: row.get("id"),
            name: row.get("name"),
            user_id: row.get("user_id"),
            description: row.get("description"),
        })
    }

    Ok(categories)
}

pub async fn del_account(pool: PgPool, id: i64) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM accounts WHERE id = $1 ";
    sqlx::query(q).bind(id).execute(&pool).await?;

    Ok(())
}

pub async fn del_category(pool: PgPool, id: i64) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM categories WHERE id = $1 ";
    sqlx::query(q).bind(id).execute(&pool).await?;

    Ok(())
}

pub async fn edit_category(
    pool: PgPool,
    id: i64,
    name: String,
    description: String,
) -> Result<(), Box<dyn Error>> {
    let q = "UPDATE categories SET name = $1, description = $2 WHERE id = $3 ";
    sqlx::query(q)
        .bind(name)
        .bind(description)
        .bind(id)
        .execute(&pool).await?;

    Ok(())
}

pub async fn edit_account(
    pool: PgPool,
    id: i64,
    name: String,
    balance: i64,
) -> Result<(), Box<dyn Error>> {
    let q = "UPDATE accounts SET name = $1, balance = $2 WHERE id = $3 ";
    sqlx::query(q)
        .bind(name)
        .bind(balance)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_expense(pool: PgPool, user_id: i64) -> Result<Vec<Expenses>, Box<dyn Error>> {
    let q = "SELECT expenses.id, accounts.name AS account_name, categories.name AS category_name, expenses.amount, expenses.user_id
    FROM expenses
    JOIN accounts ON expenses.account_id = accounts.id
    JOIN categories ON expenses.category_id = categories.id
    WHERE expenses.user_id = $1; ";
    let query = sqlx::query(q).bind(user_id);
    let mut rows = query.fetch(&pool);

    let mut expenses = vec![];

    while let Some(row) = rows.try_next().await? {

        expenses.push(Expenses {
            id: row.get("id"),
            account: row.get("account_name"),
            category: row.get("category_name"),
            amount: row.get("amount"),
            user_id: row.get("user_id")
        });
    }

    Ok(expenses)
}

pub async fn get_income(pool: PgPool, user_id: i64) -> Result<Vec<Income>, Box<dyn Error>> {
    let q = "select income.id, accounts.name AS account_name, categories.name AS category_name, income.amount, income.user_id
    FROM income
    JOIN accounts ON income.account_id = accounts.id
    JOIN categories ON income.category_id = categories.id
    WHERE income.user_id = $1";
    let query = sqlx::query(q).bind(user_id);
    let mut rows = query.fetch(&pool);

    let mut income = vec![];

    while let Some(row) = rows.try_next().await? {

        income.push(Income {
            id: row.get("id"),
            account: row.get("account_name"),
            category: row.get("category_name"),
            amount: row.get("amount"),
            user_id: row.get("user_id")
        });
    }

    Ok(income)
}

pub async fn add_expense(
    pool: PgPool,
    user_id: i64,
    amount: i64,
    category: String,
    account: String,
) -> Result<(), Box<dyn Error>> {
    let first_q = "SELECT id FROM categories WHERE user_id = $1 AND name = $2 ";
    let firts_query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
        sqlx::query(first_q).bind(user_id).bind(category);
    let cat_id: i64 = firts_query.fetch_one(&pool).await?.get("id");

    let second_q = "SELECT id FROM accounts WHERE user_id = $1 AND name = $2 ";
    let second_query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
        sqlx::query(second_q).bind(user_id).bind(account);

    let acc_id: i64 = second_query.fetch_one(&pool).await?.get("id");

    let balance_q = "SELECT balance FROM accounts WHERE id = $1 ";
    let balance_query = sqlx::query(balance_q).bind(acc_id);

    let mut balance: i64 = balance_query.fetch_one(&pool).await?.get("balance");

    balance = balance - amount;

    let set_balance_q = "UPDATE accounts SET balance = $1 WHERE id = $2 ";
    sqlx::query(set_balance_q)
        .bind(balance)
        .bind(acc_id)
        .execute(&pool)
        .await?;

    let query =
        "INSERT INTO expenses (account_id, category_id, amount, user_id) VALUES ($1, $2, $3, $4)";
    sqlx::query(query)
        .bind(acc_id)
        .bind(cat_id)
        .bind(amount)
        .bind(user_id)
        .execute(&pool)
        .await?;

    Ok(())
}


pub async fn add_income(
    pool: PgPool,
    user_id: i64,
    amount: i64,
    category: String,
    account: String,
) -> Result<(), Box<dyn Error>> {
    let first_q = "SELECT id FROM categories WHERE user_id = $1 AND name = $2 ";
    let firts_query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
        sqlx::query(first_q).bind(user_id).bind(category);
    let cat_id: i64 = firts_query.fetch_one(&pool).await?.get("id");

    let second_q = "SELECT id FROM accounts WHERE user_id = $1 AND name = $2 ";
    let second_query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
        sqlx::query(second_q).bind(user_id).bind(account);

    let acc_id: i64 = second_query.fetch_one(&pool).await?.get("id");

    let balance_q = "SELECT balance FROM accounts WHERE id = $1 ";
    let balance_query = sqlx::query(balance_q).bind(acc_id);

    let mut balance: i64 = balance_query.fetch_one(&pool).await?.get("balance");

    balance = balance + amount;

    let set_balance_q = "UPDATE accounts SET balance = $1 WHERE id = $2 ";
    sqlx::query(set_balance_q)
        .bind(balance)
        .bind(acc_id)
        .execute(&pool)
        .await?;

    let query =
        "INSERT INTO income (account_id, category_id, amount, user_id) VALUES ($1, $2, $3, $4)";
    sqlx::query(query)
        .bind(acc_id)
        .bind(cat_id)
        .bind(amount)
        .bind(user_id)
        .execute(&pool)
        .await?;

    Ok(())
}


pub async fn del_income(pool: PgPool, id: i64,) -> Result<(), Box<dyn Error>> {

    // let first_q = "SELECT id FROM categories WHERE user_id = $1 AND name = $2 ";
    // let firts_query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
    //     sqlx::query(first_q).bind(user_id).bind(category);
    // let cat_id: i64 = firts_query.fetch_one(&pool).await?.get("id");

    let acc_id_q = "SELECT account_id FROM income WHERE id = $1";
    let acc_id_query = sqlx::query(acc_id_q).bind(id);
    let acc_id: i64 = acc_id_query.fetch_one(&pool).await?.get("account_id");

    let amount_q = "SELECT amount FROM income WHERE id = $1";
    let amount_query = sqlx::query(amount_q).bind(id);
    let amount: i64 = amount_query.fetch_one(&pool).await?.get("amount");

    let balance_q = "SELECT balance FROM accounts WHERE id = $1 ";
    let balance_query = sqlx::query(balance_q).bind(acc_id);

    let mut balance: i64 = balance_query.fetch_one(&pool).await?.get("balance");
    balance = balance - amount;

    let set_balance_q = "UPDATE accounts SET balance = $1 WHERE id = $2 ";
    sqlx::query(set_balance_q)
        .bind(balance)
        .bind(acc_id)
        .execute(&pool)
        .await?;

    let q = "DELETE FROM income WHERE id = $1";
    sqlx::query(q).bind(id).execute(&pool).await?;

    Ok(())
}


pub async fn del_expense(pool: PgPool, id: i64,) -> Result<(), Box<dyn Error>> {

    // let first_q = "SELECT id FROM categories WHERE user_id = $1 AND name = $2 ";
    // let firts_query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments> =
    //     sqlx::query(first_q).bind(user_id).bind(category);
    // let cat_id: i64 = firts_query.fetch_one(&pool).await?.get("id");

    let acc_id_q = "SELECT account_id FROM expenses WHERE id = $1";
    let acc_id_query = sqlx::query(acc_id_q).bind(id);
    let acc_id: i64 = acc_id_query.fetch_one(&pool).await?.get("account_id");

    let amount_q = "SELECT amount FROM expenses WHERE id = $1";
    let amount_query = sqlx::query(amount_q).bind(id);
    let amount: i64 = amount_query.fetch_one(&pool).await?.get("amount");

    let balance_q = "SELECT balance FROM accounts WHERE id = $1 ";
    let balance_query = sqlx::query(balance_q).bind(acc_id);

    let mut balance: i64 = balance_query.fetch_one(&pool).await?.get("balance");
    balance = balance + amount;

    let set_balance_q = "UPDATE accounts SET balance = $1 WHERE id = $2 ";
    sqlx::query(set_balance_q)
        .bind(balance)
        .bind(acc_id)
        .execute(&pool)
        .await?;

    let q = "DELETE FROM expenses WHERE id = $1";
    sqlx::query(q).bind(id).execute(&pool).await?;

    Ok(())
}