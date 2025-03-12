// SPDX-License: MIT

use either::Left;
use llvm_plugin::{
    inkwell::values::{BasicValueEnum, InstructionOpcode},
    LlvmModulePass, PassBuilder, PipelineParsing, PreservedAnalyses,
};

#[llvm_plugin::plugin(name = "ReplaceFirstAddPass", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name == "replace-first-add-pass" {
            manager.add_pass(SkeletonPass);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}

struct ReplaceFirstAddPass;

impl LlvmModulePass for ReplaceFirstAddPass {
    fn run_pass(
        &self,
        module: &mut llvm_plugin::inkwell::module::Module<'_>,
        manager: &llvm_plugin::ModuleAnalysisManager,
    ) -> PreservedAnalyses {
        let context = module.get_context();
        let builder = context.create_builder();
        for function in module.get_functions() {
            eprintln!(
                "[replace-first-add] Visiting function {:?}",
                function.get_name()
            );

            for basic_block in function.get_basic_block_iter() {
                eprintln!(
                    "[replace-first-add] Visiting basic block {:?}",
                    basic_block.get_name()
                );

                for instruction in basic_block.get_instructions() {
                    if instruction.get_opcode() == InstructionOpcode::Add {
                        eprintln!(
                            "[replace-first-add] Found add instruction, replacing with mul and exiting"
                        );

                        let Some(Left(BasicValueEnum::IntValue(lhs))) = instruction.get_operand(0)
                        else {
                            panic!("Add instruction missing lhs");
                        };
                        let Some(Left(BasicValueEnum::IntValue(rhs))) = instruction.get_operand(1)
                        else {
                            panic!("Add instruction missing rhs");
                        };

                        builder.position_at(basic_block, &instruction);
                        let mul = builder
                            .build_int_mul(lhs, rhs, "")
                            .expect("Failed to build mul instruction")
                            .as_instruction()
                            .expect("We just built an instruction");

                        instruction.replace_all_uses_with(&mul);

                        return PreservedAnalyses::None;
                    }
                }
            }
        }

        PreservedAnalyses::All
    }
}
