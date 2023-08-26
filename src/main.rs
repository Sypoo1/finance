pub mod handlers;

use dotenv::dotenv;
use handlers::logic::*;
use handlers::*;
use teloxide::prelude::*;

use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting throw dice bot...");

    let pool = match get_sqlx_database_client().await {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let bot = Bot::from_env();
    println!("🚀 Bot started successfully");
    // Command::repl(bot, move |bot, msg, cmd| answer(bot, msg, cmd, Database{pool: pool.clone()})).await;
    Command::repl(bot, move |bot, msg, cmd| {
        answer(bot, msg, cmd, pool.clone())
    })
    .await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command, pool: PgPool) -> ResponseResult<()> {
    match cmd {
        Command::Help => help_handler(bot, msg).await?,

        Command::Start => start_handler(bot, msg).await?,

        Command::Accounts => accounts_handler(bot, msg, pool).await?,

        Command::AddAccount { name, balance } => {
            add_account_handler(bot, msg, pool, name, balance).await?
        }

        Command::Total => total_handler(bot, msg, pool).await?,

        Command::AddCategory { name, description } => {
            add_category_handler(bot, msg, pool, name, description).await?
        }

        Command::Categories => categories_handler(bot, msg, pool).await?,

        Command::DelAccount(id) => del_account_handler(bot, msg, pool, id).await?,

        Command::DelCategory(id) => del_category_handler(bot, msg, pool, id).await?,

        Command::EditAccount { id, name, balance } => {
            edit_account_handler(bot, msg, pool, id, name, balance).await?
        }

        Command::EditCategory {
            id,
            name,
            description,
        } => edit_category_handler(bot, msg, pool, id, name, description).await?,

        Command::Expenses => expense_handler(bot, msg, pool).await?,

        Command::Income  => income_handler(bot, msg, pool).await?,
       
        Command::AddExpense {
            amount,
            category,
            account,
        } => add_expense_handler(bot, msg, pool, amount, category, account).await?,
        Command::AddIncome {
            amount,
            category,
            account,
        } => add_income_handler(bot, msg, pool, amount, category, account).await?,

         //--------------------------------------
        Command::DelExp(id) => del_expense_handler(bot, msg, pool, id).await?,
        Command::DelInc(id) => del_income_handler(bot, msg, pool, id).await?,
    }

    Ok(())
}
