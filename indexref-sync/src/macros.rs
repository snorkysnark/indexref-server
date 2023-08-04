macro_rules! transaction {
    ($db:expr => $block:tt) => {{
        use sea_orm::TransactionTrait;
        let txn = $db.begin().await?;

        let return_value = $block;

        txn.commit().await?;
        return_value
    }};
}

pub(crate) use transaction;
