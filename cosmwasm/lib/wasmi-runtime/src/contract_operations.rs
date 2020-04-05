use enclave_ffi_types::{Ctx, EnclaveError};

use super::results::{HandleSuccess, InitSuccess, QuerySuccess};
use crate::exports;

use wasmi::{ImportsBuilder, ModuleInstance};

use crate::runtime::{Engine, EnigmaImportResolver, Runtime};

/*
Each contract is compiled with these functions already implemented in wasm:
fn cosmwasm_api_0_6() -> i32;  // Seems unused, but we should support it anyways
fn allocate(size: usize) -> *mut c_void;
fn deallocate(pointer: *mut c_void);
fn init(env_ptr: *mut c_void, msg_ptr: *mut c_void) -> *mut c_void
fn handle(env_ptr: *mut c_void, msg_ptr: *mut c_void) -> *mut c_void
fn query(msg_ptr: *mut c_void) -> *mut c_void

Re `init`, `handle` and `query`: We need to pass `env` & `msg`
down to the wasm implementations, but because they are buffers
we need to allocate memory regions inside the VM's instance and copy
`env` & `msg` into those memory regions inside the VM's instance.
*/

pub fn init(
    context: Ctx,    // need to pass this to read_db & write_db
    contract: &[u8], // contract wasm bytes
    env: &[u8],      // blockchain state
    msg: &[u8],      // probably function call and args
) -> Result<InitSuccess, EnclaveError> {
    let mut engine = start_engine(context, contract)?;

    let env_ptr = engine
        .write_to_memory(env)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let msg_ptr = engine
        .write_to_memory(msg)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let vec_ptr = engine
        .init(env_ptr, msg_ptr)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let output = engine
        .extract_vector(vec_ptr)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    Ok(InitSuccess {
        output,
        used_gas: 0,        // TODO gas
        signature: [0; 65], // TODO enclave sign
    })
}

pub fn handle(
    context: Ctx,
    contract: &[u8],
    env: &[u8],
    msg: &[u8],
) -> Result<HandleSuccess, EnclaveError> {
    let mut engine = start_engine(context, contract)?;

    let env_ptr = engine
        .write_to_memory(env)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let msg_ptr = engine
        .write_to_memory(msg)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let vec_ptr = engine
        .handle(env_ptr, msg_ptr)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let output = engine
        .extract_vector(vec_ptr)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    Ok(HandleSuccess {
        output,
        used_gas: 0,        // TODO gas
        signature: [0; 65], // enclave sign
    })
}

pub fn query(context: Ctx, contract: &[u8], msg: &[u8]) -> Result<QuerySuccess, EnclaveError> {
    let mut engine = start_engine(context, contract)?;

    let msg_ptr = engine
        .write_to_memory(msg)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let vec_ptr = engine
        .query(msg_ptr)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    let output = engine
        .extract_vector(vec_ptr)
        .map_err(|_err| EnclaveError::FailedFunctionCall)?;

    Ok(QuerySuccess {
        output,
        used_gas: 0,        // TODO gas
        signature: [0; 65], // enclave sign
    })
}

fn start_engine(context: Ctx, contract: &[u8]) -> Result<Engine, EnclaveError> {
    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(contract).map_err(|_err| EnclaveError::InvalidWasm)?;

    // Create new imports resolver.
    // These are the signatures of rust functions available to invoke from wasm code.
    // We want to limit to 4GiB of memory. `with_limit` accepts number of memory pages.
    // 1 memory page is 64KiB, therefore 4GiB/64KiB == num of pages == 64*1024
    let imports = EnigmaImportResolver::with_limit(64 * 1024);
    let module_imports = ImportsBuilder::new().with_resolver("env", &imports);

    // Instantiate a module with our imports and assert that there is no `start` function.
    let instance =
        ModuleInstance::new(&module, &module_imports).map_err(|_err| EnclaveError::InvalidWasm)?;
    if instance.has_start() {
        return Err(EnclaveError::WasmModuleWithStart);
    }
    let instance = instance.not_started_instance().clone();
    let runtime = Runtime {
        context,
        memory: imports.memory_ref(),
    };

    Ok(Engine::new(runtime, instance, imports))
}
