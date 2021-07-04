/*! Contains the root `Resolver` type. */

use crate::{
    domain::{
        customers::resolver::CustomersResolver,
        orders::resolver::OrdersResolver,
        products::resolver::ProductsResolver,
    },
    store::resolver::StoreResolver,
};

/**
Resolver for the domain.

The goal of the resolver is to let consumers construct components without having to know what their dependencies are.

The `Resolver` type wraps resolvers from other modules.
Private implementation details live on the wrapped resolvers.
Commands and queries are resolved from this `Resolver`.
*/
pub struct Resolver {
    store_resolver: StoreResolver,
    product_resolver: ProductsResolver,
    order_resolver: OrdersResolver,
    customer_resolver: CustomersResolver,
}

impl Default for Resolver {
    fn default() -> Self {
        let store_resolver = StoreResolver::default();

        Resolver {
            product_resolver: ProductsResolver::new(&store_resolver),
            order_resolver: OrdersResolver::new(&store_resolver),
            customer_resolver: CustomersResolver::new(&store_resolver),
            store_resolver,
        }
    }
}

// TODO: Come up with a scoped resolver that lets us specify an `ActiveTransactionProvider`

impl Resolver {
    pub(in crate::domain) fn store_resolver(&self) -> &StoreResolver {
        &self.store_resolver
    }

    pub(in crate::domain) fn products(&self) -> &ProductsResolver {
        &self.product_resolver
    }

    pub(in crate::domain) fn orders(&self) -> &OrdersResolver {
        &self.order_resolver
    }

    pub(in crate::domain) fn customers(&self) -> &CustomersResolver {
        &self.customer_resolver
    }
}
