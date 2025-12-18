#![cfg_attr(docsrs, feature(doc_cfg))]

#![allow(unused)]
pub mod server;
mod format;

pub use format::*;
pub use oasgen_macro::{OaSchema, oasgen};
pub use server::Server;
pub use oasgen_core::*;

#[cfg(feature = "swagger-ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "swagger-ui")))]
pub use swagger_ui;

pub mod __private {
    pub use inventory;
    pub use oasgen_core::{SchemaRegister, OperationRegister};

    pub fn fn_path_to_op_id(type_name: &str) -> Option<String> {
        Some(type_name.split("::").skip(1).collect::<Vec<_>>().join("_"))
    }

    pub fn tags_and_fn_path_to_op_id(tag: &str, module_path: &str, type_name: &str) -> Option<String> {
        let mut module = module_path.split("::").skip(1).take(1).map(|v| v.to_string()).collect::<Vec<_>>();

        if !tag.is_empty() {
            module.push(tag.to_string().to_lowercase());
        }
        module.push(type_name.to_string());

        let res = module.join("_");

        Some(res)
    }
}

#[macro_export]
macro_rules! register_schema {
    ($name:literal, $constructor:expr) => {
        ::oasgen::__private::inventory::submit!(::oasgen::__private::SchemaRegister {
            name: $name,
            constructor: &$constructor,
        });
    };
}

#[macro_export]
macro_rules! register_operation {
    ($name:expr, $constructor:expr) => {
        ::oasgen::__private::inventory::submit!(::oasgen::__private::OperationRegister {
            name: $name,
            constructor: &$constructor,
        });
    };
}

/// Use this function if you just want the OpenAPI spec and don't need the server machinery.
/// Note the server machinery is what registers the operations, so this schema only contains
/// the schemas.
pub fn generate_openapi() -> OpenAPI {
    let mut openapi = OpenAPI::default();
    for flag in inventory::iter::<oasgen_core::SchemaRegister> {
        let schema = (flag.constructor)();
        openapi.schemas.insert(flag.name.to_string(), ReferenceOr::Item(schema));
    }
    // This is required to have stable diffing between builds
    openapi.schemas.sort_keys();
    openapi
}