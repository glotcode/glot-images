use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
% escript will ignore the first line

main(_) ->
    io:format("Hello World!~n").
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Erlang;

impl LanguageConfig for Erlang {
    fn id(&self) -> String {
        "erlang".to_string()
    }

    fn name(&self) -> String {
        "Erlang".to_string()
    }

    fn file_extension(&self) -> String {
        "erl".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/erlang".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/erlang:latest".to_string(),
            version_command: "erl -version 2>&1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 225" preserveAspectRatio="xMidYMid" {
                path fill="#FFF" d="M0 0h256v225H0z" {
                }
                g fill="#A90533" {
                    path d="M44.34 159.66c-18.803-19.926-29.805-47.452-29.777-80.295-.026-29.045 9.1-54.01 24.789-73.008l-.026.01H6.351v153.295h37.966zM218.01 159.672c8.1-8.676 15.357-18.893 21.934-30.578l-36.499-18.25c-12.818 20.84-31.564 40.022-57.486 40.15-37.726-.128-52.549-32.388-52.467-73.91h140.977c.189-4.689.189-6.868 0-9.125.92-24.703-5.627-45.468-17.536-61.638l-.062.046h31.742v153.296H217.94z" {
                    }
                    path d="M95.774 41.497c1.56-18.8 16.383-31.443 33.761-31.48 17.498.037 30.14 12.68 30.568 31.48z" {
                    }
                }
                path d="M26.426 185.668v-6.387H6.807v37.868h19.619v-6.388H14.107v-10.037H25.97v-6.387H14.107v-8.67zM59.731 201.18c4.89-.726 7.576-5.573 7.756-10.494-.18-8.05-5.399-11.381-12.775-11.406H44.675v37.868h7.3v-15.056l9.125 15.056h9.124zm-7.757-15.968h.913c3.982.15 6.971 1.058 6.843 5.931.128 4.465-2.76 5.677-6.843 5.475h-.913zM93.036 179.281h-7.3v37.868h17.793v-6.388H93.036zM140.94 209.392l3.194 7.756h7.756l-14.143-38.78h-5.931l-15.056 38.78h7.755l3.195-7.756zm-1.824-5.93h-9.125l4.106-14.144zM165.578 217.149h7.756v-25.55l20.075 26.462h5.474v-38.78h-7.756v25.55l-20.075-26.463h-5.474zM230.82 197.074v5.93h8.212c-.17 4.767-4.072 8.806-8.668 8.67-7.26.136-10.857-6.88-10.95-13.232.093-6.266 3.64-13.585 10.95-13.687 3.836.101 7.08 2.726 8.668 5.931l6.388-3.193c-2.81-5.917-8.484-9.199-15.056-9.125-11.313-.073-18.558 9.265-18.706 20.074.148 10.54 7.191 19.93 18.25 20.075 11.943-.146 17.465-9.686 17.337-20.53v-.913z" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let build_commands = utils::filter_by_extension(other_files, "erl")
            .iter()
            .map(|file| format!("erlc {}", file.to_string_lossy()))
            .collect();

        RunInstructions {
            build_commands,
            run_command: format!("escript {}", main_file.display()),
        }
    }
}