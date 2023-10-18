mod get_state;
mod load_dev_profile;
mod qr_code_scanned;

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

        println!("__________________________________________________________");
        println!("__________________________________________________________");

        file.clone()
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

        let temp = file
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
                                                let (_ident, path) = match (first.next(), first.next(), first.next()) {
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

                                                let link = third
                                                    .into_inner()
                                                    .next()
                                                    .unwrap()
                                                    .into_inner()
                                                    .next()
                                                    .unwrap()
                                                    .as_str();

                                                let mut fifth = fifth.into_inner().next().unwrap().into_inner();
                                                let (_ident2, path2) = match (fifth.next(), fifth.next(), fifth.next())
                                                {
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
}
