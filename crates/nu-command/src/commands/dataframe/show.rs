use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{dataframe::PolarsData, Signature, SyntaxShape, UntaggedValue};

use nu_source::Tagged;

pub struct DataFrame;

impl WholeStreamCommand for DataFrame {
    fn name(&self) -> &str {
        "pls show"
    }

    fn usage(&self) -> &str {
        "Converts a section of the dataframe to a Table or List value"
    }

    fn signature(&self) -> Signature {
        Signature::build("pls show")
            .named(
                "n_rows",
                SyntaxShape::Number,
                "number of rows to be shown",
                Some('n'),
            )
            .switch("tail", "shows tail rows", Some('t'))
    }

    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        command(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Shows head rows from dataframe",
                example: "[[a b]; [1 2] [3 4]] | pls convert | pls show",
                result: None,
            },
            Example {
                description: "Shows tail rows from dataframe",
                example: "[[a b]; [1 2] [3 4] [5 6]] | pls convert | pls show -t -n 1",
                result: None,
            },
        ]
    }
}

fn command(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let tag = args.call_info.name_tag.clone();
    let mut args = args.evaluate_once()?;

    let rows: Option<Tagged<usize>> = args.get_flag("n_rows")?;
    let tail: bool = args.has_flag("tail");

    match args.input.next() {
        None => Err(ShellError::labeled_error(
            "No input received",
            "missing dataframe input from stream",
            &tag,
        )),
        Some(value) => {
            if let UntaggedValue::DataFrame(PolarsData::EagerDataFrame(df)) = value.value {
                let rows = rows.map(|v| v.item);
                let values = if tail { df.tail(rows)? } else { df.head(rows)? };

                Ok(OutputStream::from_stream(values.into_iter()))
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
