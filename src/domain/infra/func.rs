use crate::domain::infra::Resolver;

use std::future::Future;

pub trait CommandArgs {
    type Output;
}

#[async_trait]
pub trait Command<TArgs: CommandArgs> {
    async fn execute(self, input: TArgs) -> TArgs::Output;
}

#[async_trait]
impl<TArgs, TCommand, TFuture> Command<TArgs> for TCommand
where
    TArgs: CommandArgs + Send + 'static,
    TCommand: FnOnce(TArgs) -> TFuture + Send,
    TFuture: Future<Output = TArgs::Output> + Send,
{
    async fn execute(self, input: TArgs) -> TArgs::Output {
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
        command: TCommand,
    ) -> impl Command<TArgs>
    where
        TArgs: CommandArgs + Send + 'static,
        TCommand: FnOnce(Resolver, TArgs) -> TFuture + Send,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unsync_command_query() {
        struct AddValue {
            value: i32,
        }

        struct GetLen;

        impl CommandArgs for AddValue {
            type Output = ();
        }

        impl QueryArgs for GetLen {
            type Output = usize;
        }

        let mut data = vec![];

        fn add_value(data: &mut Vec<i32>) -> impl Command<AddValue> + '_ {
            move |command: AddValue| async move {
                data.push(command.value);
            }
        }

        fn get_len(data: &[i32]) -> impl Query<GetLen> + '_ {
            move |_: GetLen| async move { data.len() }
        }

        let command = add_value(&mut data);

        command.execute(AddValue { value: 1 }).await;
        // Commands can only be executed once

        let query = get_len(&data);

        assert_eq!(1, query.execute(GetLen).await);
        assert_eq!(1, query.execute(GetLen).await);
    }
}
