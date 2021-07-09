use crate::domain::infra::Resolver;

use std::future::Future;

pub trait CommandArgs {
    type Output;
}

#[async_trait]
pub trait Command<TArgs: CommandArgs> {
    async fn execute(&mut self, input: TArgs) -> TArgs::Output;
}

#[async_trait]
impl<TArgs, TCommand, TFuture> Command<TArgs> for TCommand
where
    TArgs: CommandArgs + Send + 'static,
    TCommand: FnMut(TArgs) -> TFuture + Send,
    TFuture: Future<Output = TArgs::Output> + Send,
{
    async fn execute(&mut self, input: TArgs) -> TArgs::Output {
        self(input).await
    }
}

pub trait QueryArgs {
    type Output;
}

#[async_trait]
pub trait Query<TArgs: QueryArgs> {
    async fn execute(&self, input: TArgs) -> TArgs::Output;
}

#[async_trait]
impl<TArgs, TQuery, TFuture> Query<TArgs> for TQuery
where
    TArgs: QueryArgs + Send + 'static,
    TQuery: Fn(TArgs) -> TFuture + Sync,
    TFuture: Future<Output = TArgs::Output> + Send,
{
    async fn execute(&self, input: TArgs) -> TArgs::Output {
        self(input).await
    }
}

impl Resolver {
    pub(in crate::domain) fn command<TArgs, TCommand, TFuture>(
        &self,
        mut command: TCommand,
    ) -> impl Command<TArgs>
    where
        TArgs: CommandArgs + Send + 'static,
        TCommand: FnMut(Resolver, TArgs) -> TFuture + Send,
        TFuture: Future<Output = TArgs::Output> + Send,
    {
        let resolver = self.by_ref();
        move |input: TArgs| {
            let resolver = resolver.by_ref();
            command(resolver, input)
        }
    }

    pub(in crate::domain) fn query<TArgs, TQuery, TFuture>(
        &self,
        query: TQuery,
    ) -> impl Query<TArgs>
    where
        TArgs: QueryArgs + Send + 'static,
        TQuery: Fn(Resolver, TArgs) -> TFuture + Sync,
        TFuture: Future<Output = TArgs::Output> + Send,
    {
        let resolver = self.by_ref();
        move |input: TArgs| {
            let resolver = resolver.by_ref();
            query(resolver, input)
        }
    }
}
