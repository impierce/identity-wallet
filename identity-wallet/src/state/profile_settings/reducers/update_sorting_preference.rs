use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        connections::{Connection, Connections},
        credentials::DisplayCredential,
        profile_settings::{
            actions::update_sorting_preference::UpdateSortingPreference, ConnectionSortMethod, CredentialSortMethod,
            Preferences, ProfileSettings,
        },
        AppState,
    },
};

use log::debug;

pub async fn update_sorting_preference(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(update_sorting) = listen::<UpdateSortingPreference>(action) {
        let mut sorting_preferences = state.profile_settings.sorting_preferences.clone();

        if let Some(credential_sorting) = update_sorting.credential_sorting {
            debug!(
                "Update credential sorting preference set to: `{:?}`",
                credential_sorting
            );
            sorting_preferences.credentials.sort_method = credential_sorting;
            // With this nested if let statement the user (should) automatically select the sort method when toggling the reverse icon on that sort method.
            // If not nested (and therefore not repeated but just checking once in a separate if let statement), the user would be able to toggle the reverse option on a sort method without selecting it.
            // Check the UX designs if the meaning of this comment not clear.
            if let Some(reverse) = update_sorting.reverse {
                debug!("Update credential sorting preference set to: `{:?}`", reverse);
                sorting_preferences.credentials.reverse = reverse;
            }
        } else if let Some(connection_sorting) = update_sorting.connection_sorting {
            debug!(
                "Update connection sorting preference set to: `{:?}`",
                connection_sorting
            );
            sorting_preferences.connections.sort_method = connection_sorting;
            if let Some(reverse) = update_sorting.reverse {
                debug!("Update connection sorting preference set to: `{:?}`", reverse);
                sorting_preferences.connections.reverse = reverse;
            }
        }

        return Ok(AppState {
            profile_settings: ProfileSettings {
                sorting_preferences,
                ..state.profile_settings
            },
            ..state
        });
    }
    Ok(state)
}

use icu::{collator::*, locid::locale};
use icu::locid::Locale;
/// Sort a Vector of words alphabetically, taking into account the locale of the words
/// `.sorted()` words -> ["Zambia", "abbey", "eager", "enlever", "ezra", "zoo", "énigme"]
/// sort_carefully words -> ["abbey", "eager", "énigme", "enlever", "ezra", "Zambia", "zoo"]
pub fn sort_carefully(list: Vec<String>, locale: Locale) -> Vec<String> {
    // https://github.com/unicode-org/icu4x/tree/main/components/collator#examples
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);
    let collator: Collator = Collator::try_new(&locale.into(), options).unwrap();

    let mut newly_sorted_list = list;
    newly_sorted_list.sort_by(|a, b| collator.compare(a, b));
    newly_sorted_list
}

pub async fn sort_credentials(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut credentials: Vec<DisplayCredential> = state.credentials.clone();
    let preferences: Preferences<CredentialSortMethod> = state.profile_settings.sorting_preferences.credentials.clone();

    let list: Vec<String> = credentials.iter().map(|x| x.display_name.clone()).collect();
    //let locale = stringify!(state.profile_settings.locale).to_string();
    let sorted_list = sort_carefully(list, locale!("en-US"));
    println!("credentials carefully sorted: \n{:?}\n", sorted_list);

    let name_az = |a: &DisplayCredential, b: &DisplayCredential| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase());
    let issuance_new_old =
        |a: &DisplayCredential, b: &DisplayCredential| a.metadata.date_issued.cmp(&b.metadata.date_issued);
    let added_new_old =
        |a: &DisplayCredential, b: &DisplayCredential| a.metadata.date_added.cmp(&b.metadata.date_added);

    credentials.sort_by(match preferences.sort_method {
        CredentialSortMethod::NameAZ => name_az,
        CredentialSortMethod::IssueDateNewOld => issuance_new_old,
        CredentialSortMethod::AddedDateNewOld => added_new_old,
    });

    if preferences.reverse {
        credentials.reverse();
    }

    Ok(AppState {
        credentials,
        current_user_prompt: None,
        ..state
    })
}

pub async fn sort_connections(state: AppState, _action: Action) -> Result<AppState, AppError> {
    let mut connections: Vec<Connection> = state.connections.0.clone();
    let preferences: Preferences<ConnectionSortMethod> = state.profile_settings.sorting_preferences.connections.clone();

    // Testing the sort_carefully function
    let list: Vec<String> = connections.iter().map(|x| x.name.clone()).collect();
    // Get locale from AppState and convert it to a string readable by the icu_locid::Locale parser.
    let locale: &str = state.profile_settings.locale.clone().into();
    let locale: Locale = locale.replace("_", "-").parse().unwrap();
    let sorted_list = sort_carefully(list, locale);
    println!("connections carefully sorted: \n{:?}\n", sorted_list);

    let name_az = |a: &Connection, b: &Connection| a.name.to_lowercase().cmp(&b.name.to_lowercase());
    let first_interacted_new_old = |a: &Connection, b: &Connection| a.first_interacted.cmp(&b.first_interacted);
    let last_interacted_new_old = |a: &Connection, b: &Connection| a.last_interacted.cmp(&b.last_interacted);

    connections.sort_by(match preferences.sort_method {
        ConnectionSortMethod::NameAZ => name_az,
        ConnectionSortMethod::FirstInteractedNewOld => first_interacted_new_old,
        ConnectionSortMethod::LastInteractedNewOld => last_interacted_new_old,
    });

    // connections.sort_by_key(|x| sorted_list.iter().position(|y| y == &x.name).unwrap());

    if preferences.reverse {
        connections.reverse();
    }

    Ok(AppState {
        connections: Connections { 0: connections },
        current_user_prompt: None,
        ..state
    })
}
