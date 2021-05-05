use bevy_ecs::world::World;
use assets::asset_server::AssetServer;
use assets::asset::Asset;

pub struct App {
    pub world: World,
}

impl App {
    fn startup() -> Self {
        let mut world = World::default();
        let asset_server = AssetServer::new();
        world.insert_resource(asset_server);
        

        return Self {
            world
        };
    }

     fn register_asset<T>(&self) -> &mut Self where T: Asset {
         
     }
     
    // fn register_asset_loader<T>() -> &mut Self where T: Asset {}
}