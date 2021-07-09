/*! Contains the root `Resolver` type. */

use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::domain::{
    customers::resolver::CustomersResolver,
    infra::transactions::resolver::TransactionsResolver,
    orders::resolver::OrdersResolver,
    products::resolver::ProductsResolver,
};

/**
The app.
*/
pub struct App {
    pub(in crate::domain) root_resolver: Resolver,
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    pub fn new() -> Self {
        App {
            root_resolver: Resolver {
                transactions_resolver: Default::default(),
                products_resolver: Default::default(),
                orders_resolver: Default::default(),
                customers_resolver: Default::default(),
            },
        }
    }
}

/**
Resolver for the domain.

The goal of the resolver is to let consumers construct components without having to know what their dependencies are.

The `Resolver` type wraps resolvers from other modules.
Private implementation details live on the wrapped resolvers.
Commands and queries are resolved from this `Resolver`.
*/
pub struct Resolver {
    pub(in crate::domain) transactions_resolver: TransactionsResolver,
    pub(in crate::domain) products_resolver: ProductsResolver,
    pub(in crate::domain) orders_resolver: OrdersResolver,
    pub(in crate::domain) customers_resolver: CustomersResolver,
}

impl Resolver {
    pub(in crate::domain) fn by_ref(&self) -> Self {
        Resolver {
            transactions_resolver: self.transactions_resolver.clone(),
            products_resolver: self.products_resolver.clone(),
            orders_resolver: self.orders_resolver.clone(),
            customers_resolver: self.customers_resolver.clone(),
        }
    }

    pub(in crate::domain) fn resolve<T>(&self, register: &Register<T>) -> T
    where
        T: Clone,
    {
        (register.0)(self)
    }
}

/**
A registration in the resolver.

Registers produce values of a specific type when a command or query asks for them.
The values may be singletons that share a single value, or they may be created on-demand.
*/
#[derive(Clone)]
pub struct Register<T>(Arc<dyn Fn(&Resolver) -> T + Send + Sync>);

impl<T> Register<T> {
    /**
    Create a register that returns the same instance of a value.
    */
    pub fn once(f: impl Fn(&Resolver) -> T + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + Clone + 'static,
    {
        let cell = OnceCell::new();
        Register(Arc::new(move |resolver| {
            cell.get_or_init(|| f(resolver)).clone()
        }))
    }

    /**
    Create a register that returns a new instance of a value each time.
    */
    pub fn factory(f: impl Fn(&Resolver) -> T + Send + Sync + 'static) -> Self {
        Register(Arc::new(f))
    }
}
