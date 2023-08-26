pub mod logic;

use logic::*;
use sqlx::postgres::PgPool;

use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn help_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub const BOT_INFO: &str = "Привет. \nЯ бот, который поможет тебе управлять финансами. \nЧтобы узнать мои команды отправь /help";

pub async fn start_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, BOT_INFO).await?;
    Ok(())
}

pub async fn accounts_handler(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    let accounts = get_accounts(pool, msg.chat.id.0).await.unwrap();
    for acc in accounts {
        let text = format!(
            "id: {id} name: {name} balance: {balance}",
            id = acc.id.unwrap(),
            name = acc.name,
            balance = acc.balance
        );
        bot.send_message(msg.chat.id, text).await?;
    }

    Ok(())
}

pub async fn categories_handler(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    let categories = get_categories(pool, msg.chat.id.0).await.unwrap();
    for cat in categories {
        let text = format!(
            "id: {id} name: {name} description: {description}",
            id = cat.id.unwrap(),
            name = cat.name,
            description = cat.description
        );
        bot.send_message(msg.chat.id, text).await?;
    }

    Ok(())
}

pub async fn add_account_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    name: String,
    balance: i64,
) -> ResponseResult<()> {
    let new_acc = Accounts {
        id: None,
        name: name,
        balance: balance,
        user_id: msg.chat.id.0,
    };

    let text = match new_acc.add(pool).await {
        Ok(()) => "Аккаунт успешно добавлен".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn add_category_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    name: String,
    description: String,
) -> ResponseResult<()> {
    let new_cat = Categories {
        id: None,
        name: name,
        user_id: msg.chat.id.0,
        description: description,
    };

    let text = match new_cat.add(pool).await {
        Ok(()) => "Категория успешно добавлена".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn total_handler(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    let accounts = get_accounts(pool, msg.chat.id.0).await.unwrap();
    let mut sum: i64 = 0;
    for acc in accounts {
        sum += acc.balance;
    }
    let text = format!("Общий баланс составляет {sum}");
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn del_account_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    id: i64,
) -> ResponseResult<()> {
    let text = match del_account(pool, id).await {
        Ok(()) => "Аккаунт успешно удален".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn del_category_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    id: i64,
) -> ResponseResult<()> {
    let text = match del_category(pool, id).await {
        Ok(()) => "Категория успешно удалена".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn edit_category_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    id: i64,
    name: String,
    description: String,
) -> ResponseResult<()> {
    let text = match edit_category(pool, id, name, description).await {
        Ok(()) => "Категория успешно изменена".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn edit_account_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    id: i64,
    name: String,
    balance: i64,
) -> ResponseResult<()> {
    let text = match edit_account(pool, id, name, balance).await {
        Ok(()) => "Аккаунт успешно изменен".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn income_handler(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    let income = get_income(pool, msg.chat.id.0).await.unwrap();
    for inc in income {
        let text = format!(
            "id: {id} account: {account} category: {category} amount: {amount} \n",
            id = inc.id,
            account = inc.account,
            category = inc.category,
            amount = inc.amount
        );
        bot.send_message(msg.chat.id, text).await?;
    }

    Ok(())
}

pub async fn expense_handler(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    let expenses = get_expense(pool, msg.chat.id.0).await.unwrap();
    for exp in expenses {
        let text = format!(
            "id: {id} account: {account} category: {category} amount: {amount} \n",
            id = exp.id,
            account = exp.account,
            category = exp.category,
            amount = exp.amount
        );
        bot.send_message(msg.chat.id, text).await?;
    }

    Ok(())
}

pub async fn add_expense_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    amount: i64,
    category: String,
    account: String,
) -> ResponseResult<()> {
    let text = match add_expense(pool, msg.chat.id.0, amount, category, account).await {
        Ok(()) => "Расход успешно добавлен".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn add_income_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    amount: i64,
    category: String,
    account: String,
) -> ResponseResult<()> {
    let text = match add_income(pool, msg.chat.id.0, amount, category, account).await {
        Ok(()) => "Доход успешно добавлен".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}


pub async fn del_income_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    id: i64,
) -> ResponseResult<()> {
    let text = match del_income(pool, id).await {
        Ok(()) => "Доход успешно удален".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

pub async fn del_expense_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
    id: i64,
) -> ResponseResult<()> {
    let text = match del_expense(pool, id).await {
        Ok(()) => "Расход успешно удален".to_string(),
        Err(e) => format!("Произошла ошибка {e}"),
    };
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}