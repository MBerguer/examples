//! Example of generating code from ABI file to interact with the contract.

use alloy::{node_bindings::Anvil, providers::ProviderBuilder, rpc::client::RpcClient, sol};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[sol(rpc)]
    IERC20,
    "examples/abi/IERC20.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let provider =
        ProviderBuilder::<_>::new().on_client(RpcClient::new_http(anvil.endpoint().parse()?));

    // Create a contract instance.
    let contract = IERC20::new("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?, provider);

    // Call the contract, retrieve the total supply.
    let IERC20::totalSupplyReturn { _0 } = contract.totalSupply().call().await?;

    println!("WETH total supply is {_0}");

    Ok(())
}
