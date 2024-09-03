use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, Binary};
use cw2::set_contract_version;
use serde::{Deserialize, Serialize};
use cosmwasm_std::entry_point;
use cosmwasm_std::to_json_binary;
use schemars::JsonSchema;
use std::fmt;

// Ajoutez les importations nécessaires pour ExecuteMsg et QueryMsg
use crate::msg::{ExecuteMsg, QueryMsg, TaskResponse};

const CONTRACT_NAME: &str = "crates.io:todo-list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

// Initialisation du contrat
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

// Fonction pour gérer les messages d'exécution
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddTask { description } => execute_add_task(deps, info, description),
        ExecuteMsg::CompleteTask { id } => execute_complete_task(deps, id),
    }
}

// Fonction pour ajouter une tâche
pub fn execute_add_task(
    deps: DepsMut,
    _info: MessageInfo,
    description: String,
) -> Result<Response, ContractError> {
    let mut tasks = load_tasks(deps.storage)?;
    let new_task = Task {
        id: tasks.len() as u64 + 1,
        description,
        completed: false,
    };
    tasks.push(new_task);
    save_tasks(deps.storage, &tasks)?;
    Ok(Response::new()
        .add_attribute("method", "execute_add_task")
        .add_attribute("task_count", tasks.len().to_string()))
}

// Fonction pour marquer une tâche comme complétée
pub fn execute_complete_task(deps: DepsMut, id: u64) -> Result<Response, ContractError> {
    let mut tasks = load_tasks(deps.storage)?;
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.completed = true;
        save_tasks(deps.storage, &tasks)?;
        Ok(Response::new()
            .add_attribute("method", "execute_complete_task")
            .add_attribute("completed_task_id", id.to_string()))
    } else {
        Err(ContractError::TaskNotFound {})
    }
}

// Fonction pour gérer les requêtes
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTasks {} => to_json_binary(&query_tasks(deps)?),
    }
}

// Fonction pour récupérer la liste des tâches
pub fn query_tasks(deps: Deps) -> StdResult<TaskResponse> {
    let tasks = load_tasks(deps.storage)?;
    Ok(TaskResponse { tasks })
}

// Charge les tâches depuis le stockage
fn load_tasks(storage: &dyn Storage) -> StdResult<Vec<Task>> {
    let data = storage.get(b"tasks").unwrap_or_default();
    Ok(bincode::deserialize(&data).unwrap_or_default())
}

// Sauvegarde les tâches dans le stockage
fn save_tasks(storage: &mut dyn Storage, tasks: &Vec<Task>) -> Result<(), ContractError> {
    let data = bincode::serialize(tasks).map_err(|e| ContractError::SerializationError { source: e })?;
    storage.set(b"tasks", &data);
    Ok(())
}

// Définition des erreurs possibles
#[derive(Debug)]
pub enum ContractError {
    TaskNotFound {},
    SerializationError { source: Box<bincode::ErrorKind> },
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::TaskNotFound {} => write!(f, "Task not found"),
            ContractError::SerializationError { source } => write!(f, "Serialization error: {:?}", source),
        }
    }
}

impl From<cosmwasm_std::StdError> for ContractError {
    fn from(err: cosmwasm_std::StdError) -> Self {
        ContractError::SerializationError { source: Box::new(bincode::ErrorKind::Custom(err.to_string())) }
    }
}

impl From<ContractError> for cosmwasm_std::StdError {
    fn from(err: ContractError) -> Self {
        cosmwasm_std::StdError::generic_err(err.to_string())
    }
}
