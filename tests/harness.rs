use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/hello_sway-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let storage_config =
        StorageConfiguration::load_from("out/debug/hello_sway-storage_slots.json").unwrap();
    let load_config = LoadConfiguration::default().set_storage_configuration(storage_config);

    let id = Contract::load_from("./out/debug/hello_sway.bin", load_config)
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
    let (instance, _id) = get_contract_instance().await;

    // change the greeting
    instance.methods().change_greeting().call().await.unwrap();

    // Get the current greeting
    let result = instance.methods().greet().call().await.unwrap();

    
    assert_eq!(result.value, "Bonjour!!!");
}


#[tokio::test]
async fn can_test_default_greeting() {
    let (instance, _id) = get_contract_instance().await;

    
    // Get the current greeting
    let result = instance.methods().greet().call().await.unwrap();

    
    assert_eq!(result.value, "Hello!!!!!");
}