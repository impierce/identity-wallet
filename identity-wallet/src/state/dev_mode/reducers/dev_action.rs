use std::sync::Arc;

use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        common::{actions::reset::Reset, reducers::reset_state::reset_state},
        dev_mode::{
            actions::dev_profile::{DevAction, DevActionType},
            reducers::{ferris_static_profile::load_ferris_profile, ngdil_flow::ngdil_flow, selv_flow::selv_flow},
            DevMode,
        },
        profile_settings::{
            actions::create_new::CreateNew,
            reducers::{
                create_new::create_identity,
                update_sorting_preference::{sort_connections, sort_credentials},
            },
            AppTheme,
        },
        AppState,
    },
};

use log::info;

pub(super) const PASSWORD: &str = "sup3rSecr3t";

pub async fn dev_action(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(dev_action) = listen::<DevAction>(action.clone()) {
        info!("Performing dev action: {:?}", dev_action.action);

        match dev_action.action {
            DevActionType::DeleteProfile => return reset_state(state, Arc::new(Reset)).await,
            DevActionType::EmptyProfile => {
                let mut state = reset_state(state, Arc::new(Reset)).await?;

                let create_new = CreateNew {
                    name: "Me".to_string(),
                    picture: String::new(), // todo: check what value needs to be here
                    theme: AppTheme::Dark,
                    password: PASSWORD.to_string(),
                };
                state = create_identity(state, Arc::new(create_new)).await?;

                return Ok(state);
            }
            DevActionType::LoadStaticFerris => {
                let mut state = load_ferris_profile().await?;
                state = sort_connections(state, action.clone()).await?;
                state = sort_credentials(state, action).await?;
                state.dev_mode = DevMode::OnWithAutologin;
                return Ok(state);
            }
            DevActionType::NGDIL { step, auto_confirm } => return ngdil_flow(state, step, auto_confirm).await,
            DevActionType::Selv { step, auto_confirm } => return selv_flow(state, step, auto_confirm).await,
        }
    }

    Ok(state)
}
