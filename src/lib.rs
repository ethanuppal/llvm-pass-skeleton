// SPDX-License: MIT

use llvm_plugin::{LlvmModulePass, PassBuilder, PipelineParsing, PreservedAnalyses};

#[llvm_plugin::plugin(name = "SkeletonPass", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name == "skeleton-pass" {
            manager.add_pass(SkeletonPass);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}

struct SkeletonPass;

impl LlvmModulePass for SkeletonPass {
    fn run_pass(
        &self,
        module: &mut llvm_plugin::inkwell::module::Module<'_>,
        _manager: &llvm_plugin::ModuleAnalysisManager,
    ) -> PreservedAnalyses {
        for function in module.get_functions() {
            eprintln!("I saw a function called {:?}!", function.get_name(),);
        }
        PreservedAnalyses::All
    }
}
