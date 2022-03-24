use aesm_client::AesmClient;
use enclave_runner::EnclaveBuilder;
use sgxs_loaders::isgx::Device as IsgxDevice;

/// Find .sgxs executable file and run it.
/// Connection to the enclave program is established via EnclaveBuilder.usercall_extension().
fn run_server(file: String) -> Result<(), ()> {
    let mut device = IsgxDevice::new().unwrap().einittoken_provider(AesmClient::new()).build();

    let mut enclave_builder = EnclaveBuilder::new(file.as_ref());
    enclave_builder.dummy_signature();
    let enclave = enclave_builder.build(&mut device).unwrap();

    enclave.run().map_err(|e| {
        eprintln!("Error in running enclave {}", e);
    })
}

// Todo: Add some softcode solution
const PATH: &str = "../../target/x86_64-fortanix-unknown-sgx/release/enclave.sgxs";
fn main() {
    run_server(PATH.to_string()).unwrap();
}