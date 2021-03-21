use legion::{World, Resources};
use assets::asset_server::AssetServer;

pub struct App {
    pub world: World,
    pub resources: Resources,
}

impl App {
    fn startup() -> Self {
        let world = World::default();
        let mut resources = Resources::default();
        let asset_server = AssetServer::new();
        resources.insert(asset_server);


        return Self {
            world,
            resources,
        };
    }

    fn register_asset<T>(&self) {

    }
}