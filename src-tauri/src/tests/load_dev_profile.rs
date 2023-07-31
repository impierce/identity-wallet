use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use crate::state::Profile;
use crate::tests::assert_state_update;

use crate::state::{
    actions::{Action, ActionType},
    AppState, TransferState,
};

#[tokio::test]
async fn test_load_dev_profile() {
    assert_state_update(
        AppState::default(),
        Action {
            r#type: ActionType::LoadDevProfile,
            payload: None,
        },
        TransferState {
            active_profile: Some(Profile {
                display_name: "Ferris Crabman".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            }),
            current_user_flow: Some(CurrentUserFlow::Redirect(Redirect {
                r#type: CurrentUserFlowType::Redirect,
                target: "profile".to_string(),
            })),
            ..TransferState::default()
        },
    );
}
