extern crate cpython;

use cpython::Python;
use cpython::ObjectProtocol;

/// A simple wrapper around Pygments, the awesome code syntax highlighter
/// This basically calls the following Python code from Rust:
///
/// from pygments import highlight
/// from pygments.lexers import PythonLexer
/// from pygments.formatters import HtmlFormatter
///
/// code = 'print "Hello World"'
/// print highlight(code, PythonLexer(), HtmlFormatter())
pub fn highlight<'a>(input: &'a str) -> String {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let pygments = py.import("pygments").unwrap();
    let highlight = pygments.get(py, "highlight").unwrap();

    let lexers = py.import("pygments.lexers").unwrap();
    let python_lexer = lexers.get(py, "PythonLexer").unwrap();
    let python_lexer_obj = python_lexer.call(py, cpython::NoArgs, None).unwrap();

    let formatters = py.import("pygments.formatters").unwrap();
    let html_formatter = formatters.get(py, "HtmlFormatter").unwrap();
    let html_formatter_obj = html_formatter.call(py, cpython::NoArgs, None).unwrap();

    //println!(html_formatter.call(py, (code,), None).unwrap().extract(py).unwrap());
    let formatted: String = highlight.call(py, (input, &python_lexer_obj, &html_formatter_obj), None).unwrap().extract(py).unwrap();
    formatted
}
