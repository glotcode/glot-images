use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
void main() {
    print('Hello World!');
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dart;

impl LanguageConfig for Dart {
    fn id(&self) -> String {
        "dart".to_string()
    }

    fn name(&self) -> String {
        "Dart".to_string()
    }

    fn file_extension(&self) -> String {
        "dart".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/dart".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/dart:latest".to_string(),
            version_command: "dart --version".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg id="Layer_1" xmlns="http://www.w3.org/2000/svg" data-name="Layer 1" viewBox="0 0 502.87 502.87" {
                defs {
                    radialGradient id="radial-gradient" cx="251.42" cy="631.97" r="251.4" gradientTransform="translate(0 -380.56)" gradientUnits="userSpaceOnUse" {
                        stop offset="0" stop-color="#fff" stop-opacity=".1" {
                        }
                        stop offset="1" stop-color="#fff" stop-opacity="0" {
                        }
                    }
                    style {
                        ".cls-1{fill:#01579b}.cls-2{fill:#40c4ff}.cls-4{fill:#fff;opacity:.2;isolation:isolate}"
                    }
                }
                path class="cls-1" d="m102.56 400.31-86-86C6.32 303.82 0 289 0 274.58c0-6.69 3.77-17.16 6.62-23.15L86 86Z" {
                }
                path class="cls-2" d="m397 102.56-86-86C303.49 9 287.85 0 274.61 0c-11.38 0-22.55 2.29-29.76 6.62L86.07 86ZM205.11 502.87h208.44v-89.32l-155.5-49.65-142.26 49.65z" {
                }
                path d="M86 354c0 26.54 3.33 33.05 16.53 46.32l13.23 13.24h297.79L268 248.14 86 86Z" style="fill:#29b6f6" {
                }
                path class="cls-1" d="M350.7 86H86l327.55 327.51h89.32V208.4L397 102.52C382.12 87.62 368.92 86 350.7 86" {
                }
                path class="cls-4" d="M105.88 403.6c-13.23-13.27-16.52-26.36-16.52-49.6V89.32L86.07 86v268c0 23.25 0 29.69 19.81 49.61l9.91 9.91Z" {
                }
                path d="M499.58 205.11v205.11h-89.32l3.29 3.33h89.32V208.4z" style="opacity:.2;isolation:isolate;fill:#263238" {
                }
                path class="cls-4" d="M397 102.56C380.61 86.14 367.19 86 347.41 86H86.07l3.29 3.29h258.05c9.87 0 34.79-1.66 49.61 13.24Z" {
                }
                path d="M499.58 205.11 397 102.56l-86-86C303.49 9 287.85 0 274.61 0c-11.38 0-22.55 2.29-29.76 6.62L86.07 86 6.65 251.43C3.81 257.46 0 267.92 0 274.58c0 14.45 6.36 29.2 16.52 39.7L95.83 393a92 92 0 0 0 6.73 7.32l3.29 3.29 9.9 9.91 86 86 3.29 3.29h208.4v-89.3h89.32V208.4Z" style="opacity:.2;isolation:isolate;fill:url(#radial-gradient)" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("dart {}", main_file.display()),
        }
    }
}
