use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
print("Hello World!")
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Swift;

impl LanguageConfig for Swift {
    fn id(&self) -> String {
        "swift".to_string()
    }

    fn name(&self) -> String {
        "Swift".to_string()
    }

    fn file_extension(&self) -> String {
        "swift".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/swift".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/swift:latest".to_string(),
            version_command: "swift --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" preserveAspectRatio="xMidYMid" {
                linearGradient id="swiftGradient1" x1="-1845.501" x2="-1797.134" y1="1255.639" y2="981.338" gradientTransform="rotate(180 -846.605 623.252)" gradientUnits="userSpaceOnUse" {
                    stop offset="0" style="stop-color:#faae42" {
                    }
                    stop offset="1" style="stop-color:#ef3e31" {
                    }
                }
                path fill="url(#swiftGradient1)" d="M56.9 0h141.8c6.9 0 13.6 1.1 20.1 3.4 9.4 3.4 17.9 9.4 24.3 17.2 6.5 7.8 10.8 17.4 12.3 27.4.6 3.7.7 7.4.7 11.1v138.3c0 4.4-.2 8.9-1.1 13.2-2 9.9-6.7 19.2-13.5 26.7-6.7 7.5-15.5 13.1-25 16.1-5.8 1.8-11.8 2.6-17.9 2.6-2.7 0-142.1 0-144.2-.1-10.2-.5-20.3-3.8-28.8-9.5-8.3-5.6-15.1-13.4-19.5-22.4-3.8-7.7-5.7-16.3-5.7-24.9V56.9C.2 48.4 2 40 5.7 32.4c4.3-9 11-16.9 19.3-22.5C33.5 4.1 43.5.7 53.7.2c1-.2 2.1-.2 3.2-.2" {
                }
                linearGradient id="swiftGradient2" x1="130.612" x2="95.213" y1="4.136" y2="204.893" gradientUnits="userSpaceOnUse" {
                    stop offset="0" style="stop-color:#e39f3a" {
                    }
                    stop offset="1" style="stop-color:#d33929" {
                    }
                }
                path fill="url(#swiftGradient2)" d="M216 209.4c-.9-1.4-1.9-2.8-3-4.1-2.5-3-5.4-5.6-8.6-7.8-4-2.7-8.7-4.4-13.5-4.6-3.4-.2-6.8.4-10 1.6-3.2 1.1-6.3 2.7-9.3 4.3-3.5 1.8-7 3.6-10.7 5.1-4.4 1.8-9 3.2-13.7 4.2-5.9 1.1-11.9 1.5-17.8 1.4-10.7-.2-21.4-1.8-31.6-4.8-9-2.7-17.6-6.4-25.7-11.1-7.1-4.1-13.7-8.8-19.9-14.1-5.1-4.4-9.8-9.1-14.2-14.1-3-3.5-5.9-7.2-8.6-11-1.1-1.5-2.1-3.1-3-4.7L0 121.2V56.7C0 25.4 25.3 0 56.6 0h50.5l37.4 38c84.4 57.4 57.1 120.7 57.1 120.7s24 27 14.4 50.7" {
                }
                path fill="#FFF" d="M144.7 38c84.4 57.4 57.1 120.7 57.1 120.7s24 27.1 14.3 50.8c0 0-9.9-16.6-26.5-16.6-16 0-25.4 16.6-57.6 16.6-71.7 0-105.6-59.9-105.6-59.9C91 192.1 135.1 162 135.1 162c-29.1-16.9-91-97.7-91-97.7 53.9 45.9 77.2 58 77.2 58-13.9-11.5-52.9-67.7-52.9-67.7 31.2 31.6 93.2 75.7 93.2 75.7C179.2 81.5 144.7 38 144.7 38" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, _other_files: Vec<PathBuf>) -> RunInstructions {
        RunInstructions {
            build_commands: vec![],
            run_command: format!("swift {}", main_file.display()).to_string(),
        }
    }
}