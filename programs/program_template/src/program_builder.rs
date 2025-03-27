use cosmwasm_std::Uint128;
use cw_denom::UncheckedDenom;
use valence_authorization_utils::{
    authorization_message::{Message, MessageDetails, MessageType, ParamRestriction},
    builders::{AtomicFunctionBuilder, AtomicSubroutineBuilder, AuthorizationBuilder},
};
use valence_forwarder_library::msg::ForwardingConstraints;
use valence_program_manager::{
    account::{AccountInfo, AccountType},
    library::{LibraryConfig, LibraryInfo},
    program_config::ProgramConfig,
    program_config_builder::ProgramConfigBuilder,
};

pub fn program_builder(params: deployer_lib::ProgramParams) -> ProgramConfig {
    let owner = params.get("owner");
    let denom = params.get("denom");

    //---- Set builder ----//
    let mut builder = ProgramConfigBuilder::new(owner.to_string());

    // Juno domain
    let juno_domain = valence_program_manager::domain::Domain::CosmosCosmwasm("juno".to_string());

    //---- Accounts ----//
    // First account
    let acc_first = builder.add_account(AccountInfo::new(
        "first_account".to_string(),
        &juno_domain,
        AccountType::default(),
    ));
    // Second account
    let acc_second = builder.add_account(AccountInfo::new(
        "second_account".to_string(),
        &juno_domain,
        AccountType::default(),
    ));

    //---- Libraries ----//
    // Forward funds from first account to second account
    let first_forwarder_config = valence_forwarder_library::msg::LibraryConfig {
        input_addr: acc_first.clone(),
        output_addr: acc_second.clone(),
        forwarding_configs: vec![valence_forwarder_library::msg::UncheckedForwardingConfig {
            denom: UncheckedDenom::Native(denom.clone()),
            max_amount: Uint128::MAX,
        }],
        forwarding_constraints: ForwardingConstraints::new(None),
    };

    let lib_first_forwarder = builder.add_library(LibraryInfo::new(
        "first_forwarder".to_string(),
        &juno_domain,
        LibraryConfig::ValenceForwarderLibrary(first_forwarder_config),
    ));

    builder.add_link(&lib_first_forwarder, vec![&acc_first], vec![&acc_second]);

    //---- Authorizations ----//
    // First authorization to forward funds from the first account to the second account
    let function = AtomicFunctionBuilder::new()
        .with_domain(valence_authorization_utils::domain::Domain::External("juno".to_string()))
        .with_contract_address(lib_first_forwarder.clone())
        .with_message_details(MessageDetails {
            message_type: MessageType::CosmwasmExecuteMsg,
            message: Message {
                name: "process_function".to_string(),
                params_restrictions: Some(vec![ParamRestriction::MustBeIncluded(vec![
                    "process_function".to_string(),
                    "forward".to_string(),
                ])]),
            },
        })
        .build();

    let subroutine = AtomicSubroutineBuilder::new()
        .with_function(function)
        .build();
    let authorization = AuthorizationBuilder::new()
        .with_label("Forward from first to second")
        .with_subroutine(subroutine)
        .build();

    builder.add_authorization(authorization);

    // Build program config
    builder.build()
}
