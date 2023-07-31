macro_rules! transaction {
    ($db:expr => $block:tt) => {
        {
            use sea_orm::TransactionTrait;
            let txn = $db.begin().await?;

            $block

            txn.commit().await?;
        }
    };
}

pub(crate) use transaction;
