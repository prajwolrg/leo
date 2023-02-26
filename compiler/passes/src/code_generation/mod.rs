// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

pub mod generator;
pub use generator::*;

mod generate_expressions;

mod generate_program;

mod generate_statements;

use crate::SymbolTable;
use crate::{CallGraph, Pass, StructGraph};

use leo_ast::{Ast, ProgramConsumer};
use leo_errors::Result;

impl<'a> Pass for CodeGenerator<'a> {
    type Input = (Ast, &'a SymbolTable, &'a StructGraph, &'a CallGraph);
    type Output = Result<String>;

    fn do_pass((ast, symbol_table, struct_graph, call_graph): Self::Input) -> Self::Output {
        let mut generator = Self::new(symbol_table, struct_graph, call_graph);
        let bytecode = generator.consume_program(ast.into_repr());

        Ok(bytecode)
    }
}
