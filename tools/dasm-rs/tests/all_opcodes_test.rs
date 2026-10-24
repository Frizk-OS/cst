/*
 * Copyright (C) 2026 The AOSP and FrizkOS.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use dasm_rs::DasmParser;
use dasm_rs::DalvikStatement;
use dasm_rs::DalvikOpcode;
use std::fs;
use std::path::Path;

#[test]
fn test_parse_all_opcodes_d() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest_dir.parent().unwrap().join("dasm").join("test").join("all_opcodes.d"),
        manifest_dir.parent().unwrap().parent().unwrap().join("tools").join("dasm").join("test").join("all_opcodes.d"),
        manifest_dir.join("test").join("all_opcodes.d"),
    ];
    let test_file = candidates
        .iter()
        .find(|p| p.exists())
        .expect("all_opcodes.d must exist in repository");

    let content = fs::read_to_string(test_file).expect("Failed to read all_opcodes.d");
    let mut parser = DasmParser::new();
    let result = parser.parse_str(&content);
    assert!(result.is_ok(), "Failed to parse all_opcodes.d: {:?}", result.err());

    // Expect 2 classes/interfaces: dasm.test.all_opcodes and dasm.test.interface.test_interface
    assert_eq!(parser.classes.len(), 2, "Expected 2 classes parsed");

    let class1 = &parser.classes[0];
    assert_eq!(class1.name, "dasm.test.all_opcodes");
    assert_eq!(class1.super_class.as_deref(), Some("java/lang/Object"));
    assert_eq!(class1.interfaces, vec!["java/lang/Runnable"]);

    // Check statements count
    assert!(!class1.methods.is_empty(), "Should have parsed methods");

    let method = &class1.methods[0];
    assert_eq!(method.name, "<init>");
    assert_eq!(method.signature, "()V");
    assert_eq!(method.limit_regs, None);

    let m1 = &class1.methods[1];
    assert_eq!(m1.name, "run");
    assert_eq!(m1.signature, "()I");
    assert_eq!(m1.limit_regs, Some(4));

    // Verify presence of opcode statements
    let found_return_void = method.statements.iter().any(|st| matches!(st, DalvikStatement::Instruction(inst) if inst.opcode == DalvikOpcode::ReturnVoid));
    assert!(found_return_void, "Method should contain return-void statement");
}
