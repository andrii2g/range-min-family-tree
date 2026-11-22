#![forbid(unsafe_code)]

use std::error::Error;
use std::io::{self, Write};
use std::process::ExitCode;

use range_min_family_tree::{CartesianTree, Query};

const USAGE: &str = "Usage: range-min-family-tree LEFT RIGHT -- VALUE [VALUE ...]\n\
                     LEFT and RIGHT are zero-based inclusive indices.";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match run(&args, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String], output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    if matches!(args, [flag] if flag == "--help" || flag == "-h") {
        writeln!(output, "{USAGE}")?;
        return Ok(());
    }

    let [left, right, separator, values @ ..] = args else {
        return Err("expected two endpoints followed by -- and the array values".into());
    };
    if separator != "--" || values.is_empty() {
        return Err("expected -- followed by at least one array value".into());
    }
    let left: usize = left
        .parse()
        .map_err(|_| format!("invalid left index {left:?}"))?;
    let right: usize = right
        .parse()
        .map_err(|_| format!("invalid right index {right:?}"))?;
    if left > right {
        return Err(format!("left index {left} exceeds right index {right}").into());
    }
    let values = values
        .iter()
        .map(|value| {
            value
                .parse::<i64>()
                .map_err(|_| format!("invalid integer {value:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let tree = CartesianTree::new(&values);
    let query = tree.query(left, right).ok_or_else(|| {
        format!(
            "right index {right} is out of bounds for {} values",
            values.len()
        )
    })?;

    write_tree(output, &tree, &query)?;
    writeln!(output)?;
    for (label, path) in [("Left", &query.left_path), ("Right", &query.right_path)] {
        let path = path
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(" -> ");
        writeln!(output, "{label} path: {path}")?;
    }
    let minimum = query.minimum_index;
    writeln!(
        output,
        "Minimum in [{left}, {right}] is at index {minimum} (value {})",
        values[minimum]
    )?;
    Ok(())
}

fn write_tree(output: &mut impl Write, tree: &CartesianTree, query: &Query) -> io::Result<()> {
    let mut annotations = vec![""; tree.nodes().len()];
    for (path, path_label, endpoint_label) in [
        (&query.left_path, "  <- left path", "  <- left endpoint"),
        (&query.right_path, "  <- right path", "  <- right endpoint"),
    ] {
        for &index in path {
            annotations[index] = path_label;
        }
        annotations[path[0]] = endpoint_label;
    }
    let ancestor = query.minimum_index;
    let root = tree
        .root()
        .expect("a successful query requires a nonempty tree");
    let mut pending = vec![(root, String::new(), "", true)];
    while let Some((index, prefix, side, last)) = pending.pop() {
        let node = &tree.nodes()[index];
        let connector = match (side, last) {
            ("", _) => "",
            (_, true) => "└── ",
            (_, false) => "├── ",
        };
        write!(output, "{prefix}{connector}{side}[{index}] {}", node.value)?;
        if index == ancestor {
            write!(output, "  * shared ancestor")?;
            if query.left_path[0] == ancestor {
                write!(output, "  <- left endpoint")?;
            }
            if query.right_path[0] == ancestor {
                write!(output, "  <- right endpoint")?;
            }
        } else {
            write!(output, "{}", annotations[index])?;
        }
        writeln!(output)?;

        if node.left.is_some() || node.right.is_some() {
            let extension = match (side, last) {
                ("", _) => "",
                (_, true) => "    ",
                (_, false) => "│   ",
            };
            let child_prefix = prefix + extension;
            if let Some(right) = node.right {
                pending.push((right, child_prefix.clone(), "R ", true));
            }
            if let Some(left) = node.left {
                pending.push((left, child_prefix, "L ", node.right.is_none()));
            }
        }
    }
    Ok(())
}
