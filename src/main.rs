extern crate cpython;

use cpython::Python;
use cpython::ObjectProtocol;

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    /*
    from pygments import highlight
    from pygments.lexers import PythonLexer
    from pygments.formatters import HtmlFormatter

    code = 'print "Hello World"'
    print highlight(code, PythonLexer(), HtmlFormatter())
    */

    let pygments = py.import("pygments").unwrap();
    let highlight = pygments.get(py, "highlight").unwrap();

    let lexers = py.import("pygments.lexers").unwrap();
    let python_lexer = lexers.get(py, "PythonLexer").unwrap();
    let python_lexer_obj = python_lexer.call(py, cpython::NoArgs, None).unwrap();

    let formatters = py.import("pygments.formatters").unwrap();
    let html_formatter = formatters.get(py, "HtmlFormatter").unwrap();
    let html_formatter_obj = html_formatter.call(py, cpython::NoArgs, None).unwrap();

    let code = "print \'Hello World\'";
    //println!(html_formatter.call(py, (code,), None).unwrap().extract(py).unwrap());
    let formatted: String =  highlight.call(py, (code, &python_lexer_obj, &html_formatter_obj), None).unwrap().extract(py).unwrap();

    println!("{}", formatted);
}
