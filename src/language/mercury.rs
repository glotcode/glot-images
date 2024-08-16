use crate::language::EditorConfig;
use crate::language::LanguageConfig;
use crate::language::RunConfig;
use crate::language::RunInstructions;
use crate::utils;
use maud::html;
use maud::Markup;
use std::path::PathBuf;

const EXAMPLE_CODE: &str = r#"
:- module main.
:- interface.
:- import_module io.

:- pred main(io::di, io::uo) is det.

:- implementation.

main(!IO) :-
    io.write_string("Hello World!", !IO).
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mercury;

impl LanguageConfig for Mercury {
    fn id(&self) -> String {
        "mercury".to_string()
    }

    fn name(&self) -> String {
        "Mercury".to_string()
    }

    fn file_extension(&self) -> String {
        "m".to_string()
    }

    fn editor_config(&self) -> EditorConfig {
        EditorConfig {
            default_filename: format!("main.{}", self.file_extension()),
            mode: "ace/mode/plain_text".to_string(),
            use_soft_tabs: true,
            soft_tab_size: 4,
            example_code: EXAMPLE_CODE.trim_matches('\n').to_string(),
        }
    }

    fn run_config(&self) -> RunConfig {
        RunConfig {
            container_image: "glot/mercury:latest".to_string(),
            version_command: "mmc --version | head -n 1".to_string(),
        }
    }

    fn logo(&self) -> Markup {
        html! {
            svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" {
                path fill="#666" d="M280.522 250.364c-7.11-.939-5.193-5.615-7.72-5.615s-5.053 6.317-8.282 9.967-8.843 2.667-8.843-5.194-1.685-6.176-1.685-6.176c-1.825 2.807-11.483 21.28-16.246 20.118-4.635-1.13-7.348-1.91-8.14-4.522 0 0-8.705 6.897-15.488 11.758-6.213 4.452-11.54 3.043-11.54 3.043 0-4.949 3.967-15.549-.254-12.943-10.376 6.409-30.257 20.037-28.676 12.055 2.665-13.45 2.356-17.964 2.356-17.964-12.055 8.5-30.756 11.592-34.002 11.747-3.245.154-10.973-4.637 2.937-14.528s12.055-12.056 11.746-14.838c-.31-2.782-13.292 2.473-23.338 3.091-2.794.172-24.42-1.081-10.51-6.8s31.058-21.447 31.058-21.447c-22.052 13.57-45.8 9.45-52.342 5.573s-.242-5.088-.242-5.088c42.164-6.785 42.164-14.54 42.164-14.54-49.919 11.39-60.58 4.12-69.547-2.423s-.242-7.512-.242-7.512c54.038.485 71.97-11.632 71.97-11.632l-3.615-.747c-37.305 8.695-78.816 4.488-88.353-3.085s3.366-9.536 3.366-9.536c62.268 5.89 93.682-7.013 93.682-7.013s-53.012-.28-79.097-1.963-45.719-15.988-48.243-21.597 7.292-3.366 7.292-3.366c16.064 2.94 27.224 6.65 38.988 8.134 57.78 7.292 71.804-2.525 71.804-2.525s-1.963 0-15.707 1.403c-13.744 1.402-15.146-5.049-15.146-5.049-24.465 1.494-33.332-1.485-70.102-14.345C2.615 103.545 0 94.768 0 92.153s12.92-1.42 12.92-1.42c37.359 16.714 120.413 28.076 133.522 24.143s-.8-3.824-.8-3.824-33.787-1.6-70.039-10.485C42.436 92.438 14.526 81.049 3.738 70.679c-10.218-9.821 5.25-9.111 13.77-8.128s86.515 30.477 148.263 32.794c11.326 1.506 12.232.197 12.232.197-11.211-2.623-32.145-3.864-40.329-9.913s-6.387-14.707-1.27-10.888c13.02 9.716 47.613 14.464 47.613 14.464 37.361 2.491 51.643-5.823 77.924 8.54 20.813 11.375 13.893 25.315 11.77 38.643-5.422 34.04 2.47 50.39 14.046 63.437 13.53 15.25 30.76 18.899 30.76 18.899 8.895 1.779 7.828 8.183 2.846 10.674 0 0-33.401 21.949-40.84 20.966m-17.394-110.05s7.406-18.662-4.938-31.005c-10.861-10.861-28.136-8.222-28.136-8.222-1.574.524-1.328 2.17.489 2.298 0 0 15.206 1.185 22.71 10.466s8.69 21.724 8.69 24.686 1.185 1.777 1.185 1.777m123.509 47.97c21.254-13.792 73.032-12.888 73.032-12.888-72.467-45.13-126.167-38.89-171.473-40.094 11.343 1.152 82.692 32.199 98.44 52.982m-65.601 147.242s-16.363-10.698-25.313-24.179c-4.236-6.382-4.85-16.16-4.91-20.59-.187-13.583 10.389-17.19 16.433-16.056 7.22 1.354 15.595 8.48 15.595 8.48-3.694-18.492 2.818-24.12 55.998-23.403 8.977.121 16.245-1.889 29.492 6.713l29.509 19.094 41.893-9.59c-.809 16.802 7.977 48.393 7.977 48.393.557 2.957 2.551 5.891-3.013 7.495-2.165.623-5.415 1.668-8.342 2.228a8.54 8.54 0 0 0-6.821 7.068l-2.444 15.529h-20.577v6.01c1.82 1.895 10.145 4.117 13.966 3.606 6.16-.823 5.616 7.978 3.698 8.764s-5.704.073-7.785 1.784c-.85.699-2.35 2.533-2.27 4.703.114 3.14 2.19 6.982 3.08 9.083 3.66 8.633 8.279 21.464.071 25.352-4.64 2.198-18.52.436-29.851-1.002-41.071-5.209-76.933-41.508-77.935-43.312s-5.61-6.574-5.61-6.574c7.012 52.892 38.372 58.073 48.203 61.514-7.62 11.552-8.11 23.35-8.11 23.35 5.946-8.261 14.032-11.85 33.96-18.656 4.627-1.58 8.586-1.89 12.328-1.908 33.914-.165 36.882-13.824 38.998-18.762 2.212-5.16.245-21.383-1.721-25.07-1.966-3.686 6.145-9.585 6.145-9.585v-11.06c0-1.229 1.966-2.704 3.686-4.916s-.082-17.286-.082-17.286 21.73 2.239 20.747-7.265-8.294-23.868-9.932-31.077c-1.639-7.21 4.588-40.309 4.588-40.309 23.267-15.074 19.007-78.978 19.007-78.978-79.306 7.21-132.886 29.166-158.94 39.98-26.052 10.815-44.848 25.654-82.207 52.69-51.252 37.09-92.298 74.135-92.298 74.135s-14.747 15.73-9.831 27.036 11.306 11.306 11.306 11.306 60.462-78.16 101.262-89.957c18.642 34.214 28.846 32.595 40.05 35.222" {
                }
            }
        }
    }

    fn run_instructions(&self, main_file: PathBuf, other_files: Vec<PathBuf>) -> RunInstructions {
        let other_source_files = utils::filter_by_extension(other_files, "m");

        RunInstructions {
            build_commands: vec![format!(
                "mmc -o a.out {} {}",
                main_file.display(),
                utils::join_files(other_source_files)
            )],
            run_command: "./a.out".to_string(),
        }
    }
}
