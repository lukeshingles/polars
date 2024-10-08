use polars_core::prelude::*;

pub fn as_struct(s: &[Series]) -> PolarsResult<Series> {
    Ok(StructChunked::from_series(s[0].name().clone(), s)?.into_series())
}
