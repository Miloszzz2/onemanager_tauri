use crate::commands::programs::getprogrampaths_fn;

pub fn get_apppath_from_name(name: &str) -> String {
    let program_paths = getprogrampaths_fn();
    if let Some(program) = program_paths
        .iter()
        .find(|p| p.path.to_lowercase().contains(name))
    {
        return program.path.clone(); // Assuming `path` is a `String`, clone it to return.
    }
    // Return a default value or handle the case where no program was found.
    String::new() // Return an empty string in case no program was found.
}
