mod tests {
    use dotenv::dotenv;

    // use crate::download_plugin;
    // #[test]
    // fn test_1() {
    //     download_plugin::new();
    // }

    // use crate::manage_plugin::get_plugin_list;
    // #[test]
    // fn test_1() {
    //     let result = get_plugin_list::new();
    //     println!("Result: {:?}", result.unwrap());
    // }

    

    // use crate::manage_plugin::get_plugin;
    // #[test]
    // fn test_1() {
    //     dotenv().ok();
    //     let result = get_plugin::new("https://raw.githubusercontent.com/chlaty/chlaty-lib-hianime/refs/heads/main/manifes.json");
    //     println!("Result: {:?}", result.unwrap());
    // }

    use crate::manage_plugin::download_plugin;

    #[test]
    fn test_1() {
        dotenv().ok();
        let result = download_plugin::new("hianime", "https://github.com/chlaty/chlaty-lib-hianime/releases/download/0.1.0/manifest.json");
        println!("Result: {:?}", result.unwrap());
    }
}