use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug)]
#[allow(unused)]
pub(crate) enum TestKind {
    Success,
    RuntimeError,
    StaticError,
}

#[macro_export]
macro_rules! success_tests {
    (subdir: $subdir:literal, $($tt:tt)*) => { $crate::tests!(Success, Some($subdir) => $($tt)*); };
    ($($tt:tt)*) => { $crate::tests!(Success, None => $($tt)*); }
}

#[macro_export]
macro_rules! runtime_error_tests {
    (subdir: $subdir:literal, $($tt:tt)*) => { $crate::tests!(RuntimeError, Some($subdir) => $($tt)*); };
    ($($tt:tt)*) => { $crate::tests!(RuntimeError, None => $($tt)*); }
}

#[macro_export]
macro_rules! static_error_tests {
    (subdir: $subdir:literal, $($tt:tt)*) => { $crate::tests!(StaticError, Some($subdir) => $($tt)*); };
    ($($tt:tt)*) => { $crate::tests!(StaticError, None => $($tt)*); }
}

#[macro_export]
macro_rules! tests {
    ($kind:ident, $subdir:expr =>
        $(
            {
                name: $name:ident,
                file: $file:literal,
                $(input: $input:literal,)?
                $(heap_size: $heap_size:literal,)?
                expected: $expected:literal $(,)?
                $(" $(tt:$tt)* ")?
            }
        ),*
        $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                #[allow(unused_assignments, unused_mut)]
                let mut input = None;
                $(input = Some($input);)?
                #[allow(unused_assignments, unused_mut)]
                let mut heap_size = None;
                $(heap_size = Some($heap_size);)?
                let kind = $crate::infra::TestKind::$kind;
                $crate::infra::run_test(stringify!($name), $subdir, $file, input, heap_size, $expected, kind);
            }
        )*
    };
}

pub(crate) fn run_test(
    name: &str,
    subdir: Option<&str>,
    file: &str,
    input: Option<&str>,
    heap_size: Option<usize>,
    expected: &str,
    kind: TestKind,
) {
    let mut path = PathBuf::new();
    path.push("tests");
    if let Some(subdir) = subdir {
        path.push(subdir);
    }
    path.push(file);

    match kind {
        TestKind::Success => run_success_test(name, &path, expected, input, heap_size),
        TestKind::RuntimeError => run_runtime_error_test(name, &path, expected, input, heap_size),
        TestKind::StaticError => run_static_error_test(name, &path, expected),
    }
}

fn run_success_test(
    name: &str,
    file: &Path,
    expected: &str,
    input: Option<&str>,
    heap_size: Option<usize>,
) {
    if let Err(err) = compile(name, file) {
        panic!("expected a successful compilation, but got an error: `{err}`");
    }
    match run(name, input, heap_size) {
        Err(err) => {
            panic!("expected a successful execution, but got an error: `{err}`");
        }
        Ok(actual_output) => {
            diff(expected, actual_output);
        }
    }
}

fn run_runtime_error_test(
    name: &str,
    file: &Path,
    expected: &str,
    input: Option<&str>,
    heap_size: Option<usize>,
) {
    if let Err(err) = compile(name, file) {
        panic!("expected a successful compilation, but got an error: `{err}`");
    }
    match run(name, input, heap_size) {
        Ok(out) => {
            panic!("expected a runtime error, but program executed succesfully - expected error: `{expected}`, output: `{out}`");
        }
        Err(err) => check_error_msg(&err, expected),
    }
}

fn run_static_error_test(name: &str, file: &Path, expected: &str) {
    match compile(name, file) {
        Ok(()) => {
            panic!(
                "expected a static error, but compilation succeeded - expected error: `{expected}`"
            )
        }
        Err(err) => check_error_msg(&err, expected),
    }
}

fn compile(name: &str, file: &Path) -> Result<(), String> {
    // Run the compiler
    let compiler: PathBuf = ["target", "debug", env!("CARGO_PKG_NAME")].iter().collect();
    let output = Command::new(&compiler)
        .arg(file)
        .arg(&mk_path(name, Ext::Asm))
        .output()
        .expect("could not run the compiler");
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap());
    }

    // Assemble and link
    let output = Command::new("make")
        .arg(&mk_path(name, Ext::Run))
        .output()
        .expect("could not run make");
    assert!(output.status.success(), "linking failed");

    Ok(())
}

fn run(name: &str, input: Option<&str>, heap_size: Option<usize>) -> Result<String, String> {
    let mut cmd = Command::new(&mk_path(name, Ext::Run));
    if let Some(input) = input {
        cmd.arg(input);
    }
    if let Some(heap_size) = heap_size {
        cmd.arg(heap_size.to_string());
    }
    let output = cmd.output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else {
        Err(String::from_utf8(output.stderr).unwrap().trim().to_string())
    }
}

fn check_error_msg(found: &str, expected: &str) {
    let lower_found = found.trim().to_lowercase();
    let lower_expected = expected.trim().to_lowercase();
    assert!(
        lower_found.contains(&lower_expected),
        "the reported error message does not contain the expected subtring - found: `{found}`, expected: `{expected}`",
    );
}

fn diff(expected: &str, found: String) {
    let expected = expected.trim();

    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = found.lines().collect();
    if expected_lines != actual_lines {
        eprintln!(
            "output differed!\n{}",
            prettydiff::diff_lines(&found, expected)
        );
        panic!("test failed");
    }
}

fn mk_path(name: &str, ext: Ext) -> PathBuf {
    Path::new("tests").join(format!("{name}.{ext}"))
}

#[derive(Copy, Clone)]
enum Ext {
    Asm,
    Run,
}

impl std::fmt::Display for Ext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ext::Asm => write!(f, "s"),
            Ext::Run => write!(f, "run"),
        }
    }
}
