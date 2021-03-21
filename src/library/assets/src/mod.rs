#![allow(dead_code)]
pub mod asset_server;
mod asset;
mod loader;
mod handle;

#[cfg(test)]
mod tests {
    use crate::asset_server::AssetServer;
    use crate::asset::TextAssetLoader;
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_asset_server() {
        let mut server = AssetServer::new();
        server.register_asset_loader(&["txt"],TextAssetLoader {});
        server.load("res/test_load.txt");

        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(100);
        let now = time::Instant::now();
        thread::sleep(ten_millis);

    }
}
