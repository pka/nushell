use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{
    dataframe::{NuDataFrame, PolarsData},
    Signature, UntaggedValue, Value,
};

use super::utils::parse_polars_error;

pub struct DataFrame;

impl WholeStreamCommand for DataFrame {
    fn name(&self) -> &str {
        "pls to_dummies"
    }

    fn usage(&self) -> &str {
        "Creates a new dataframe with dummy variables"
    }

    fn signature(&self) -> Signature {
        Signature::build("pls select")
    }

    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        command(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Create new dataframe with dummy variables",
            example: "[[a b]; [1 2] [3 4]] | pls convert | pls to_dummies",
            result: None,
        }]
    }
}

fn command(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let tag = args.call_info.name_tag.clone();
    let mut args = args.evaluate_once()?;

    match args.input.next() {
        None => Err(ShellError::labeled_error(
            "No input received",
            "missing dataframe input from stream",
            &tag,
        )),
        Some(value) => {
            if let UntaggedValue::DataFrame(PolarsData::EagerDataFrame(df)) = value.value {
                let res = df.as_ref().to_dummies().map_err(|e| {
                    parse_polars_error(
                        &e,
                        &tag.span,
                        Some("The only allowed column types for dummies are String or Int"),
                    )
                })?;

                let value = Value {
                    value: UntaggedValue::DataFrame(PolarsData::EagerDataFrame(NuDataFrame::new(
                        res,
                    ))),
                    tag: tag.clone(),
                };

                Ok(OutputStream::one(value))
            } else {
                Err(ShellError::labeled_error(
                    "No dataframe in stream",
                    "no dataframe found in input stream",
                    &tag,
                ))
            }
        }
    }
}
