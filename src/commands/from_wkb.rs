use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, CustomValue, Example, LabeledError, Signature, SyntaxShape, Type, Value};
use crate::geometry::GeometryCustomValue;

use crate::GeoSpatialPlugin;

pub struct FromWkb;

impl SimplePluginCommand for FromWkb {
    type Plugin = GeoSpatialPlugin;

    fn name(&self) -> &str {
        "from wkb"
    }

    fn description(&self) -> &str {
        "Decodes wkb into a geospatial data type"
    }

    fn extra_description(&self) -> &str {
        "I'll tell you later"
    }

    fn signature(&self) -> Signature {
        // The signature defines the usage of the command inside Nu, and also automatically
        // generates its help page.
        Signature::build(self.name())
            .required("wkb_geometry", SyntaxShape::Binary, "required integer value")
            .category(Category::Experimental)
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["example"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "example one 3 bb",
            description: "running example with an int value and string value",
            result: None,
        }]
    }

    fn run(
        &self,
        plugin: &GeoSpatialPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        plugin.print_values(1, call, input)?;

        Ok(Value::nothing(call.head))
    }
}
