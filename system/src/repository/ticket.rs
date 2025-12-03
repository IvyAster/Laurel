use diesel::SelectableHelper;
use diesel_async::AsyncConnection;
use laurel_actix::types::repository;
use laurel_pg::{AsyncDsl, DbPool};
use crate::model::ticket::{InsertableTicket, Ticket};
use crate::schema::schema::ticket::dsl as TicketDsl;
//use crate::schema::schema::ticket as TicketSchema;

#[derive(Clone, Debug)]
pub struct Repository {
    pool: DbPool,
}

impl Repository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn save<'a>(&self, insertable: &InsertableTicket<'a>) -> repository::Result<Ticket>{
        let mut conn = self.pool.get().await?;
        let ticket = conn
            .transaction::<Ticket, anyhow::Error, _>(|mut tx| {
                Box::pin(async move {
                    let ticket = AsyncDsl::get_result(
                        diesel::insert_into(TicketDsl::ticket)
                            .values(insertable)
                            .returning(Ticket::as_returning()),
                        &mut tx,
                    )
                        .await?;
                    Ok(ticket)
                })
            })
            .await?;
        Ok(ticket)
    }
}