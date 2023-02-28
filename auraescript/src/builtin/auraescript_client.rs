use anyhow::Result;
use client::{Client, AuraeConfig};
use deno_core::{OpState, Resource, ResourceId};
use std::{cell::RefCell, rc::Rc};

// `AuraeConfig` `try_default`
#[deno_core::op]
pub(crate) async fn as__aurae_config__try_default(
    opstate: Rc<RefCell<OpState>>
) -> Result<ResourceId> {
    let config = AuraeConfig::try_default()?;
    let mut opstate = opstate.borrow_mut();
    let rid = opstate.resource_table.add(AuraeScriptConfig(config));
    Ok(rid)
}

// `AuraeConfig` `from_options`
#[deno_core::op]
pub(crate) async fn as__aurae_config__from_options(
    opstate: Rc<RefCell<OpState>>,
    ca_crt: String,
    client_crt: String,
    client_key: String,
    socket: String,
) -> ResourceId {
    let config = AuraeConfig::from_options(ca_crt, client_crt, client_key, socket);
    let mut opstate = opstate.borrow_mut();
    opstate.resource_table.add(AuraeScriptConfig(config))
}

// `AuraeConfig` `parse_from_file`
#[deno_core::op]
pub(crate) async fn as__aurae_config__parse_from_file(
    opstate: Rc<RefCell<OpState>>,
    path: String,
) -> Result<ResourceId> {
    let config = AuraeConfig::parse_from_file(path)?;
    let mut opstate = opstate.borrow_mut();
    let rid = opstate.resource_table.add(AuraeScriptConfig(config));
    Ok(rid)
}


// Re export AuraeConfig in auraescript to be able
// to impl Resource on it
pub(crate) struct AuraeScriptConfig(pub AuraeConfig);

impl Resource for AuraeScriptConfig {} // Blank impl



// Create a `Client` with given `AuraeConfig`
#[deno_core::op]
pub(crate) async fn as__client_new(
    opstate: Rc<RefCell<OpState>>,
    config: ResourceId,
) -> Result<ResourceId> {
    let config = {
        let opstate = &opstate.borrow();
        let rt = &opstate.resource_table; // get `ResourceTable` from JsRuntime `OpState`
        rt.get::<AuraeScriptConfig>(config)?.0.clone() // get `Config` from its rid
    };
    let client = Client::new(config).await?;
    let mut opstate = opstate.borrow_mut();
    let rid = opstate.resource_table.add(AuraeScriptClient(client));
    Ok(rid)
}

pub(crate) fn op_decls() -> Vec<::deno_core::OpDecl> {
    vec![
        as__aurae_config__try_default::decl(),
        as__aurae_config__from_options::decl(),
        as__aurae_config__parse_from_file::decl(),
        as__client_new::decl(),
    ]
}

// Re export client in auraescript to be able
// to impl Resource on it
pub(crate) struct AuraeScriptClient(pub Client);

impl Resource for AuraeScriptClient {} // Blank impl