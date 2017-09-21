/*! Contains the root `Resolver` type. */

use domain::products::resolver::ProductsResolver;
use domain::orders::resolver::OrdersResolver;
use domain::customers::resolver::CustomersResolver;

/**
Resolver for the domain.

The goal of the resolver is to let consumers construct components without having to know what their dependencies are.

The `Resolver` type wraps resolvers from other modules.
Private implementation details live on the wrapped resolvers.
Commands and queries are resolved from this `Resolver`.
*/
pub struct Resolver {
    product_resolver: ProductsResolver,
    order_resolver: OrdersResolver,
    customer_resolver: CustomersResolver,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            product_resolver: ProductsResolver::default(),
            order_resolver: OrdersResolver::default(),
            customer_resolver: CustomersResolver::default(),
        }
    }
}

impl Resolver {
    pub(in domain) fn products(&self) -> &ProductsResolver {
        &self.product_resolver
    }

    pub(in domain) fn orders(&self) -> &OrdersResolver {
        &self.order_resolver
    }

    pub(in domain) fn customers(&self) -> &CustomersResolver {
        &self.customer_resolver
    }
}
