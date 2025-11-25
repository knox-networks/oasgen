use crate::{impl_oa_schema, Schema};

impl_oa_schema!(::bigdecimal::BigDecimal, Schema::new_string().with_format("decimal"));
