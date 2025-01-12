pub mod health_check;
use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::dynamodb::DynamoDB;
use crate::persistence::postgres::PostgresConn;

#[derive(new)]
pub struct PostgresRepositoryImpl<T> {
    _pool: PostgresConn,
    _marker: PhantomData<T>,
}
#[derive(new)]
pub struct DynamoDBRepositoryImpl<T> {
    _dynamodb: DynamoDB,
    _marker: PhantomData<T>,
}
