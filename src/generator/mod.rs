// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


mod visitor;

mod pgsql;
mod mysql;
mod oracle;

use ast::*;
use self::visitor::Visitor;
use std::result;
use std::fmt::{Write, Error, Result};
type Formatter = String;

pub use self::pgsql::PgsqlGenerator;
pub use self::oracle::OracleGenerator;
pub use self::mysql::MySQLGenerator;

pub trait Generator<T> {
    fn to_sql(&self) -> result::Result<String, Error>;
}


struct InternalGenerator;

impl Visitor for InternalGenerator {
    fn visit_extract_fn(&self, function: &ExtractFn, f: &mut Formatter) -> Result {
        self.visit_datetime_type(&function.extract_type, f)?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_char(')')
    }
}


impl Generator<Expression> for Expression {
    fn to_sql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_expression(self, &mut s)?;
        Ok(s)
    }
}


impl Generator<PredicateExpression> for PredicateExpression {
    fn to_sql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

impl Generator<Statement> for Statement {
    fn to_sql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_statement(self, &mut s)?;
        Ok(s)
    }
}