use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::{json_example, test_managers};
use identity_wallet::state::Profile;
use identity_wallet::state::{actions::Action, AppState};
use std::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_create_new() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/no_profile_redirect_welcome.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/active_pf_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/create_new.json");
    assert_state_update(
        // Initial state.
        AppState::default(),
        vec![
            // Get the initial state.
            action1, // Create a new profile.
            action2,
        ],
        vec![
            // There is no profile yet, so the user is redirected to the welcome page.
            Some(state1),
            // The profile was created, so the user is redirected to the profile page.
            Some(state2),
        ],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_unlock_storage() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/active_pf_password_required.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/active_pf_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/unlock_storage.json");
    assert_state_update(
        // Initial state.
        AppState {
            managers: test_managers(vec![]),
            active_profile: Mutex::new(Some(Profile {
                name: "Ferris Crabman".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
                primary_did: "did:example:placeholder".to_string(),
            })),
            ..AppState::default()
        },
        vec![
            // Get the initial state.
            action1, // Unlock the storage.
            action2,
        ],
        vec![
            // The storage is locked, so the user is prompted to unlock it.
            Some(state1),
            // The storage is unlocked, so the user is redirected to the profile page.
            Some(state2),
        ],
    )
    .await;
}

///////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
    use identity_wallet::state::{
        actions::{Action, ActionType},
        AppState,
    };
    pub use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "../src-tauri/flowchart.pest"]
    pub struct MermaidParser;

    #[tokio::test]
    async fn test() {
        use tokio::process::Command;

        let output = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg("tests/temp/Cargo.toml")
            .output();

        let output = output.await.unwrap();

        assert!(output.status.success());
        assert_eq!(
            std::str::from_utf8(output.stdout.as_slice()).unwrap(),
            "Hello, world!\n"
        );

        print_mermaid().await;
    }

    async fn print_mermaid() {
        use std::collections::HashMap;
        let mut idents = HashMap::new();
        let mut payloads = HashMap::new();
        let mut actions = HashMap::new();

        let unparsed_file = std::fs::read_to_string("1.mermaid").expect("cannot read file");

        let file = MermaidParser::parse(Rule::file, &unparsed_file)
            .unwrap()
            .next()
            .unwrap();

        // println!("file: {:#?}", file);
        println!("__________________________________________________________");
        println!("__________________________________________________________");

        let temp2 = file
            .clone()
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::body)
            .map(|body| {
                body.into_inner()
                    .filter(|pair| pair.as_rule() == Rule::payload_assign)
                    .for_each(|payload_assign| {
                        let mut pair = payload_assign.into_inner().nth(0).unwrap().into_inner();

                        let ident = pair.next().unwrap();
                        println!("ident: {:#?}", ident.as_str());

                        let vec: Vec<_> = pair
                            .next()
                            .unwrap()
                            .into_inner()
                            .nth(0)
                            .unwrap()
                            .into_inner()
                            .map(|pair| pair.as_str())
                            .collect();
                        let string = vec.iter().cloned().collect::<Vec<&str>>().join("\n");
                        let mapping: serde_json::Value = serde_yaml::from_str(&string).unwrap();
                        println!("mapping: {:#?}", mapping);
                        payloads.insert(ident.as_str(), mapping);
                    })
            });

        let mut temp = file
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::body)
            .map(|body| {
                body.into_inner()
                    .filter(|pair| pair.as_rule() == Rule::subgraph)
                    .map(|subgraph| {
                        subgraph
                            .into_inner()
                            .find(|pair| pair.as_rule() == Rule::graph)
                            .map(|graph| {
                                graph
                                    .into_inner()
                                    .filter(|pair| pair.as_rule() == Rule::chain)
                                    .filter_map(|chain| {
                                        // println!("graph: {:#?}", chain);
                                        // println!("__________________________________________________________");
                                        // println!("__________________________________________________________");
                                        let mut chain = chain.into_inner();
                                        match (
                                            chain.next(),
                                            chain.next(),
                                            chain.next(),
                                            chain.next(),
                                            chain.next(),
                                            chain.next(),
                                        ) {
                                            (
                                                Some(first),
                                                Some(second),
                                                Some(third),
                                                Some(fourth),
                                                Some(fifth),
                                                None,
                                            ) if first.as_rule() == Rule::ident_comb
                                                && second.as_rule() == Rule::link
                                                && third.as_rule() == Rule::ident_comb
                                                && fourth.as_rule() == Rule::link
                                                && fifth.as_rule() == Rule::ident_comb =>
                                            {
                                                let mut first = first.into_inner().next().unwrap().into_inner();
                                                let (ident, path) = match (first.next(), first.next(), first.next()) {
                                                    (Some(ident), Some(path), None) => {
                                                        let path = path.into_inner().nth(0).unwrap().as_str();
                                                        idents.insert(ident.as_str(), path);
                                                        (ident.as_str(), path)
                                                    }
                                                    (Some(ident), None, None) => {
                                                        let path = idents.get(ident.as_str()).unwrap();
                                                        (ident.as_str(), *path)
                                                    }
                                                    _ => panic!("first"),
                                                };

                                                // println!("state: {:#?} : {:#?}", ident, path);

                                                // println!("temp: {:#?}", third);

                                                // let temp = third.into_inner().nth(0).unwrap();
                                                // let temp = if temp.clone().into_inner().nth(1).is_some() {
                                                //     temp.into_inner()
                                                //         .nth(1)
                                                //         .unwrap()
                                                //         .into_inner()
                                                //         .nth(0)
                                                //         .unwrap()
                                                //         .as_str()
                                                // } else {
                                                //     temp.into_inner().nth(0).unwrap().as_str()
                                                // };
                                                let link = third
                                                    .into_inner()
                                                    .next()
                                                    .unwrap()
                                                    .into_inner()
                                                    .next()
                                                    .unwrap()
                                                    .as_str();

                                                // println!("action: {:#?}", link.as_str());

                                                let mut fifth = fifth.into_inner().next().unwrap().into_inner();
                                                let (ident2, path2) = match (fifth.next(), fifth.next(), fifth.next()) {
                                                    (Some(ident), Some(path), None) => {
                                                        let path = path.into_inner().nth(0).unwrap().as_str();
                                                        idents.insert(ident.as_str(), path);
                                                        (ident.as_str(), path)
                                                    }
                                                    (Some(ident), None, None) => {
                                                        let path = idents.get(ident.as_str()).unwrap();
                                                        (ident.as_str(), *path)
                                                    }
                                                    _ => panic!("fifth"),
                                                };
                                                // println!("state: {:#?} : {:#?}", ident2, path2);
                                                Some((path, link, path2))
                                            }
                                            (Some(first), Some(second), Some(third), None, None, None)
                                                if first.as_rule() == Rule::ident_comb
                                                    && second.as_rule() == Rule::link
                                                    && third.as_rule() == Rule::ident_comb =>
                                            {
                                                let ident = first
                                                    .into_inner()
                                                    .next()
                                                    .unwrap()
                                                    .into_inner()
                                                    .next()
                                                    .unwrap()
                                                    .as_str();

                                                let payload = payloads.get(ident).unwrap();

                                                actions.insert(third.as_str(), payload);
                                                // println!("first: {:#?}", payload);
                                                // println!("second: {:#?}", second);
                                                // println!("third: {:#?}", third.as_str());
                                                None
                                            }
                                            _ => panic!("third"),
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap();

        for (path, link, path2) in &temp[0] {
            setup_state_file();
            setup_stronghold();

            let payload = actions.get(link);
            println!("state: {:#?}", path);
            println!("link: {:#?} {:#?}", link, payload);
            println!("state2: {:#?}", path2);

            assert_state_update(
                AppState::default(),
                vec![Action {
                    r#type: ActionType::from_tag_str(*link),
                    payload: payload.cloned().cloned(),
                }],
                vec![],
            )
            .await;
        }
    }

    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    pub struct Temp {
        name: String,
        picture: String,
        theme: String,
        password: String,
    }

    #[test]
    fn test2() {
        let temp: serde_yaml::Value = serde_yaml::from_str(
            "
            name: Ferris Crabman
            picture: 129408
            theme: system
            password: sup3rSecr3t
        ",
        )
        .unwrap();
        dbg!(temp);
    }
}
