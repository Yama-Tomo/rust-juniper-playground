pub mod errors;
pub mod post;
pub mod user;

// data_sourcesモジュール以外のスコープへ公開するのはstructだけに限定して実装をなるべく隠蔽する
pub mod public {
    pub mod models {
        pub use crate::data_sources::entities::errors::ValidationErrors;
        pub use crate::data_sources::entities::post::Model as Post;
        pub use crate::data_sources::entities::user::Model as User;
    }
}
