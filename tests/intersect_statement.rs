// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod common;

use common::*;

#[test]
fn test(){
    test_statement("select name from student where gender ='男' intersect select name from student where gender ='女'",
                   "select name from student where gender = '男' intersect select name from student where gender = '女'");
}