use serde::Deserialize;

use mlang_lsp_definition::Arity;

use crate::{AnyMCoreDefinition, MCoreFunctionDefinition};

pub(crate) fn load() -> BuiltInData {
    let yaml_data = include_str!("../coreApi/api.yaml");

    let parsed_data: BuiltInData = serde_yaml::from_str(yaml_data).unwrap();

    parsed_data
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub(crate) struct Function {
    name: String,
    description: String,
    aliases: Vec<String>,
    #[serde(default, rename = "argsCount")]
    args_count: Option<usize>,
    #[serde(default, rename = "optionalArgsCount")]
    optional_args_count: usize,
    #[serde(default, rename = "hasRest")]
    has_rest: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub(crate) struct BuiltInData {
    functions: Vec<Function>,
}

impl From<Function> for Vec<MCoreFunctionDefinition> {
    fn from(val: Function) -> Self {
        let arity = val.args_count.map(|total_count| Arity {
            total_count,
            optional_count: val.optional_args_count,
            has_rest: val.has_rest,
        });

        val.aliases
            .into_iter()
            .chain([val.name])
            .map(|name| MCoreFunctionDefinition {
                id: name,
                description: val.description.clone(),
                arity,
            })
            .collect()
    }
}

impl From<BuiltInData> for Vec<AnyMCoreDefinition> {
    fn from(val: BuiltInData) -> Self {
        val.functions
            .into_iter()
            .flat_map(Into::<Vec<MCoreFunctionDefinition>>::into)
            .map(AnyMCoreDefinition::MCoreFunctionDefinition)
            .collect::<Vec<_>>()
    }
}
