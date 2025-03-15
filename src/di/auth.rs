use shaku::module;

use crate::{AuthRepositoryImpl, AuthServiceImpl};

module! {
    pub(crate) AuthModule {
        components = [
            AuthServiceImpl,
            AuthRepositoryImpl,
        ],
        providers = []
    }
}
