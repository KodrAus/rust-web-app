/*! Contains the root `Resolver` type. */

use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::domain::{
    customers::resolver::CustomersResolver,
    infra::transactions::TransactionsResolver,
    orders::resolver::OrdersResolver,
    products::resolver::ProductsResolver,
};

/**
Resolver for the domain.

The goal of the resolver is to let consumers construct components without having to know what their dependencies are.

The `Resolver` type wraps resolvers from other modules.
Private implementation details live on the wrapped resolvers.
Commands and queries are resolved from this `Resolver`.
*/
#[derive(Default)]
pub struct Resolver {
    pub(in crate::domain) transactions_resolver: TransactionsResolver,
    pub(in crate::domain) products_resolver: ProductsResolver,
    pub(in crate::domain) orders_resolver: OrdersResolver,
    pub(in crate::domain) customers_resolver: CustomersResolver,
}

impl Resolver {
    pub(in crate::domain) fn resolve<T>(&self, register: &Register<T>) -> T
    where
        T: Clone,
    {
        (register.0)(self)
    }
}

#[derive(Clone)]
pub struct Register<T>(Arc<dyn Fn(&Resolver) -> T + Send + Sync>);

impl<T> Register<T> {
    pub fn once(f: impl Fn(&Resolver) -> T + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + Clone + 'static,
    {
        let cell = OnceCell::new();
        Register(Arc::new(move |resolver| {
            cell.get_or_init(|| f(resolver)).clone()
        }))
    }

    pub fn factory(f: impl Fn(&Resolver) -> T + Send + Sync + 'static) -> Self {
        Register(Arc::new(f))
    }
}
