use std::ops::Drop;

use std::ffi::CString;

use llvm;
use llvm::core::*;
use llvm::prelude::*;
use libc;

pub fn c_str(b: &[u8]) -> *const libc::c_char {
    b.as_ptr() as *const _
}

#[derive(Debug, Clone)]
pub struct Context {
    pub context: LLVMContextRef
}

impl Context {
    pub fn new() -> Context {
        let context = unsafe {
            LLVMContextCreate()
        };

        Context { context }
    }

    pub fn create_module(&self, name: &[u8]) -> LLVMModuleRef {
        unsafe {
            LLVMModuleCreateWithNameInContext(c_str(name), self.context)
        }
    }

    pub fn void_ty(&self) -> LLVMTypeRef {
        unsafe {
            LLVMVoidTypeInContext(self.context)
        }
    }

    pub fn i1_ty(&self) -> LLVMTypeRef {
        unsafe {
            LLVMInt1TypeInContext(self.context)
        }
    }

    pub fn i8_ty(&self) -> LLVMTypeRef {
        unsafe {
            LLVMInt8TypeInContext(self.context)
        }
    }

    pub fn i32_ty(&self) -> LLVMTypeRef {
        unsafe {
            LLVMInt32TypeInContext(self.context)
        }
    }

    pub fn double_ty(&self) -> LLVMTypeRef {
        unsafe {
            LLVMDoubleTypeInContext(self.context)
        }
    }

    pub fn pointer_ty(sub_ty: LLVMTypeRef) -> LLVMTypeRef {
        unsafe {
            LLVMPointerType(sub_ty, 0)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context)
        };
    }
}

#[derive(Debug, Clone)]
pub struct IRBuilder {
    builder: LLVMBuilderRef
}

impl IRBuilder {
    pub fn new_in_context(context: &Context) -> IRBuilder {
        let builder = unsafe {
            LLVMCreateBuilderInContext(context.context)
        };

        IRBuilder {
            builder
        }
    }

    pub fn position_at_end(&self, bb: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, bb);
        }
    }

    pub fn build_unreachable(&self) {
        unsafe {
            LLVMBuildUnreachable(self.builder);
        }
    }

    pub fn build_ret(&self, value: LLVMValueRef) {
        unsafe {
            LLVMBuildRet(self.builder, value);
        }
    }

    pub fn build_ret_void(&self) {
        unsafe {
            LLVMBuildRetVoid(self.builder);
        }
    }

    pub fn build_alloca(&self, ty: LLVMTypeRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildAlloca(self.builder, ty, c_str(name))
        }
    }

    pub fn build_store(&self, value: LLVMValueRef, ptr: LLVMValueRef) {
        unsafe {
            LLVMBuildStore(self.builder, value, ptr);
        }
    }

    pub fn build_load(&self, ptr: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildLoad(self.builder, ptr, c_str(name))
        }
    }

    pub fn build_cond_br(&self, cond: LLVMValueRef, true_bb: LLVMBasicBlockRef, false_bb: LLVMBasicBlockRef) {
        unsafe {
            LLVMBuildCondBr(self.builder, cond, true_bb, false_bb);
        }
    }

    pub fn build_br(&self, bb: LLVMBasicBlockRef) {
        unsafe {
            LLVMBuildBr(self.builder, bb);
        }
    }

    pub fn build_global_string_ptr(&self, s: String, name: &[u8]) -> LLVMValueRef {
        let cs = CString::new(s).unwrap();

        unsafe {
            LLVMBuildGlobalStringPtr(self.builder, cs.as_ptr(), c_str(name))
        }
    }

    pub fn build_bitcast(&self, expr: LLVMValueRef, ty: LLVMTypeRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildBitCast(self.builder, expr, ty, c_str(name))
        }
    }

    pub fn build_not(&self, expr: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildNot(self.builder, expr, c_str(name))
        }
    }

    pub fn build_phi(&self, ty: LLVMTypeRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildPhi(self.builder, ty, c_str(name))
        }
    }

    pub fn build_call(&self, func: LLVMValueRef, mut args: Vec<LLVMValueRef>, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildCall(self.builder, func, args.as_mut_ptr(), args.len() as _, c_str(name))
        }
    }

    pub fn build_add(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildAdd(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_sub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildSub(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_mul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildMul(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_sdiv(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildSDiv(self.builder, lhs, rhs, c_str(name))
        }
    }
    
    pub fn build_srem(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildSRem(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_fadd(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildFAdd(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_fsub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildFSub(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_fmul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildFMul(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_fdiv(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildFDiv(self.builder, lhs, rhs, c_str(name))
        }
    }

    pub fn build_icmp(&self, pred: llvm::LLVMIntPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildICmp(self.builder, pred, lhs, rhs, c_str(name))
        }
    }

    pub fn build_fcmp(&self, pred: llvm::LLVMRealPredicate, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &[u8]) -> LLVMValueRef {
        unsafe {
            LLVMBuildFCmp(self.builder, pred, lhs, rhs, c_str(name))
        }
    }

    pub fn get_insert_block(&self) -> LLVMBasicBlockRef {
        unsafe {
            LLVMGetInsertBlock(self.builder)
        }
    }
}