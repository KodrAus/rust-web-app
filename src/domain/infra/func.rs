use crate::domain::infra::Resolver;

use std::future::Future;

#[async_trait]
pub trait Command<I, O> {
    async fn execute(&mut self, input: I) -> O;
}

#[async_trait]
impl<C, F, I, O> Command<I, O> for C
where
    C: FnMut(I) -> F + Send,
    F: Future<Output = O> + Send,
    I: Send + 'static,
{
    async fn execute(&mut self, input: I) -> O {
        self(input).await
    }
}

#[async_trait]
pub trait Query<I, O> {
    async fn execute(&self, input: I) -> O;
}

#[async_trait]
impl<Q, F, I, O> Query<I, O> for Q
where
    Q: Fn(I) -> F + Sync,
    F: Future<Output = O> + Send,
    I: Send + 'static,
{
    async fn execute(&self, input: I) -> O {
        self(input).await
    }
}

impl Resolver {
    pub(in crate::domain) fn command<C, F, I, O>(&self, mut command: C) -> impl Command<I, O>
    where
        C: FnMut(Resolver, I) -> F + Send,
        F: Future<Output = O> + Send,
        I: Send + 'static,
    {
        let resolver = self.by_ref();
        move |input: I| {
            let resolver = resolver.by_ref();
            command(resolver, input)
        }
    }

    pub(in crate::domain) fn query<Q, F, I, O>(&self, query: Q) -> impl Query<I, O>
    where
        Q: Fn(Resolver, I) -> F + Sync,
        F: Future<Output = O> + Send,
        I: Send + 'static,
    {
        let resolver = self.by_ref();
        move |input: I| {
            let resolver = resolver.by_ref();
            query(resolver, input)
        }
    }
}
