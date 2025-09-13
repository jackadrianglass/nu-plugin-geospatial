use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_geospatial::GeoSpatialPlugin;

fn main() {
    serve_plugin(&GeoSpatialPlugin {}, MsgPackSerializer {})
}
