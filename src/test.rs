mod tests {
    use std::num::NonZero;

    use dotenv::dotenv;
    use serde_json::json;

    // use crate::manage_plugin::get_plugin_list;
    // #[test]
    // fn test_get_plugin_list() -> Result<(), Box<dyn std::error::Error>> {
    //     let result = get_plugin_list::new("anime");
    //     match result {
    //         Ok(data) => {
    //             println!("Test [get installed plugin list] passed with result: {:?}", data);
    //             return Ok(().into());
    //         },
    //         Err(e) => {
    //             return Err(e.into());
    //         },
    //     }
    // }

    // use crate::manage_plugin::{
    //     install_plugin,
    //     install_plugin::PluginManifest
    // };

    // #[test]
    // fn test_download_plugin() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenv().ok();
    //     let result = install_plugin::new(
    //         "anime",
    //         "hianime", 
    //         "latest",
    //         PluginManifest{
    //             title: "HiAnime".to_string(),
    //             manifest: "https://raw.githubusercontent.com/chlaty/chlaty-lib-hianime/refs/heads/main/manifes.json".to_string()
    //         },
    //         |_, _| {}
    //     );
    //     match result {
    //         Ok(data) => {
    //             println!("Test [get installed plugin list] passed with result: {:?}", data);
    //             return Ok(().into());
    //         },
    //         Err(e) => {
    //             return Err(e.into());
    //         },
    //     }
    // }

    // use crate::manage_plugin::get_installed_plugin_list;

    // #[test]
    // fn test_get_installed_plugin_list() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenv().ok();
    //     let result = get_installed_plugin_list::new("anime");
    //     match result {
    //         Ok(data) => {
    //             println!("Test [get installed plugin list] passed with result: {:?}", data);
    //             return Ok(().into());
    //         },
    //         Err(e) => {
    //             return Err(e.into());
    //         },
    //     }
    // }

    // use crate::manage_plugin::get_plugin_release;

    // #[test]
    // fn request_plugin_search() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenv().ok();
    //     let result = get_plugin_release::new("https://raw.githubusercontent.com/chlaty/chlaty-lib-hianime/refs/heads/main/manifes.json", "latest");
    //     match result {
    //         Ok(data) => {
    //             println!("Test [request plugin: search] passed with result: {:?}", data);
    //             return Ok(().into());
    //         },
    //         Err(e) => {
    //             return Err(e.into());
    //         },
    //     }
    // }

    // use crate::request_plugin::search;

    // #[test]
    // fn request_plugin_search() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenv().ok();
    //     let result = search::new("anime","hianime", "one peace", NonZero::new(1).unwrap());
    //     match result {
    //         Ok(data) => {
    //             println!("Test [request plugin: search] passed with result: {:?}", data);
    //             return Ok(().into());
    //         },
    //         Err(e) => {
    //             return Err(e.into());
    //         },
    //     }
    // }

    


    // use crate::request_plugin::get_episode_list;
    // use tokio::{self, task::JoinHandle};

    // #[tokio::test]
    // async fn request_plugin_get_episode_list() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenv().ok();
    //     use crate::init;

    //     init();
    //     let mut handles: Vec<JoinHandle<()>> = Vec::new();
    //     for _ in 0..10 {
    //         let handle =  tokio::spawn(async move {
    //             let result = get_episode_list::new("anime", "hianime", "112");
    //             match result {
    //                 Ok(data) => {
    //                     println!("Test [request plugin: get_episode_list] passed with result: {:?}", data);
    //                     return;
    //                 },
    //                 Err(e) => {
    //                     println!("Test [request plugin: get_episode_list] failed with error: {}", e);
    //                     return;
    //                 },
    //             }
    //         });
    //         handles.push(handle);
    //     };
    //     for handle in handles {
    //         let _ = handle.await;
    //     }

    //     return Ok(());
    // }

    // use crate::request_plugin::get_episode_server;

    // #[test]
    // fn request_plugin_get_episode_server() -> Result<(), Box<dyn std::error::Error>> {
    //     dotenv().ok();
    //     let result = get_episode_server::new("anime", "hianime", "0", "100");
    //     match result {
    //         Ok(data) => {
    //             println!("Test [request plugin: get_episode_server] passed with result: {:?}", data);
    //             return Ok(().into());
    //         },
    //         Err(e) => {
    //             return Err(e.into());
    //         },
    //     }
    // }

    use crate::request_plugin::get_server;

    #[test]
    fn request_plugin_get_episode_server() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let result = get_server::new("movie", "hydrahd", 2, "https%3A%2F%2Fhyhd.org%2Fembed%2Ftt9140554%2F1-1%2F");
        match result {
            Ok(data) => {
                println!("Test [request plugin: get_episode_server] passed with result: {:?}", data);
                return Ok(().into());
            },
            Err(e) => {
                return Err(e.into());
            },
        }
    }
}