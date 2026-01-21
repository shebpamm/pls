pub struct Context {
    pub verbose: bool,
    pub only_print: bool,
    pub dotfiles: String,
}
fn default_dotfiles() -> String {
    std::env::var("HOME")
        .map(|home| format!("{}/dotfiles", home))
        .unwrap()
}

impl From<&crate::cli::Arguments> for Context {
    fn from(cli: &crate::cli::Arguments) -> Self {
        Context {
            verbose: cli.global.verbose,
            only_print: cli.global.only_print,
            dotfiles: default_dotfiles(),
        }
    }
}
