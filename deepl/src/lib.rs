cfg_if::cfg_if! {
    if #[cfg(feature = "online")] {
        mod api;
        mod lang;
        pub use lang::Lang;
        mod online;
        pub use online::translate;
    } else if #[cfg(feature = "local")] {
        mod local;
        pub use local::{translate, Lang};
    }
}

// 都看到这里了，如果不介意，可以尝试一下这个 https://hub.docker.com/repository/docker/zu1k/deepl
// docker run -itd -p 8080:80 zu1k/deepl
