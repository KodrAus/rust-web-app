use domain::products::resolver::Resolver as ProductResolver;
use domain::orders::resolver::Resolver as OrderResolver;

/// Resolver for the domain.
///
/// The `Resolver` type wraps resolvers from other modules.
/// Private implementation details live on the wrapped resolvers.
/// Commands and queries are resolved from this `Resolver`.
pub struct Resolver {
    product_resolver: ProductResolver,
    order_resolver: OrderResolver,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            product_resolver: ProductResolver::default(),
            order_resolver: OrderResolver::default(),
        }
    }
}

impl Resolver {
    pub fn products(&self) -> &ProductResolver {
        &self.product_resolver
    }

    pub fn orders(&self) -> &OrderResolver {
        &self.order_resolver
    }
}
