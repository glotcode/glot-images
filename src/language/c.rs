use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
#include <stdio.h>

int main(void) {
    printf("Hello World!\n");
    return 0;
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct C;

impl LanguageConfig for C {
    fn id(&self) -> String {
        "c".to_string()
    }

    fn name(&self) -> String {
        "C".to_string()
    }

    fn file_extension(&self) -> String {
        "c".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/c_cpp".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/clang:latest".to_string(),
            version_command: "clang --version | head -n 1".to_string(),
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "c");

        RunInstructions {
            build_commands: vec![format!(
                "clang -o a.out -lm {} {}",
                main_file.display(),
                utils::join_files(other_source_files),
            )],
            run_command: "./a.out".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 288" preserveAspectRatio="xMidYMid" {
                path fill="#A9B9CB" d="M255.987 85.672c-.002-4.843-1.037-9.122-3.129-12.794-2.055-3.612-5.134-6.638-9.262-9.032-34.081-19.67-68.195-39.28-102.264-58.97-9.185-5.307-18.091-5.114-27.208.27-13.565 8.008-81.481 46.956-101.719 58.689C4.071 68.665.015 76.056.013 85.663 0 125.221.013 164.777 0 204.336c.002 4.736.993 8.932 2.993 12.55 2.056 3.72 5.177 6.83 9.401 9.278 20.239 11.733 88.164 50.678 101.726 58.688 9.121 5.387 18.027 5.579 27.215.27 34.07-19.691 68.186-39.3 102.272-58.97 4.224-2.447 7.345-5.559 9.401-9.276 1.997-3.618 2.99-7.814 2.992-12.551 0 0 0-79.094-.013-118.653" {
                }
                path fill="#7F8B99" d="M141.101 5.134c-9.17-5.294-18.061-5.101-27.163.269C100.395 13.39 32.59 52.237 12.385 63.94 4.064 68.757.015 76.129.013 85.711 0 125.166.013 164.62 0 204.076c.002 4.724.991 8.909 2.988 12.517 2.053 3.711 5.169 6.813 9.386 9.254a9009 9009 0 0 0 20.159 11.62L219.625 50.375c-26.178-15.074-52.363-30.136-78.524-45.241" {
                }
                path fill="#FFF" d="m154.456 126.968 39.839.281c0-16.599-16.802-57.249-64.973-57.249-30.691 0-71.951 19.512-71.951 75.61S97.818 220 129.322 220c51.017 0 63.21-35.302 63.21-55.252l-38.007-2.173s1.017 23.075-25.406 23.075c-24.39 0-28.46-29.878-28.46-40.04 0-15.447 5.493-40.244 28.46-40.244 22.968 0 25.337 21.602 25.337 21.602" {
                }
            }
        }
    }
}
