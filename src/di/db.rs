use shaku::module;

use crate::infrastructure::postgres::PostgresConnectionImpl;

module! {
    pub(crate) DatabaseModule {
        components = [
            PostgresConnectionImpl,
        ],
        providers = []
    }
}
