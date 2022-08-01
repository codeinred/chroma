use std::path::PathBuf;

/// Compile option - there will be a Compiler object that allos translation of
/// options into flags
pub enum Opt {
    Cpp11 = 0,
    Cpp14 = 1,
    Cpp17 = 2,
    Cpp20 = 3,
    Cpp23 = 4,
}
/// List of dependencies
pub type DepBuf = Vec<u32>;
/// List of sources
pub type SrcBuf = Vec<PathBuf>;
/// Include directory w/ header files
pub type Include = PathBuf;
/// Group of compiler options
pub type Opts = Vec<Opt>;
/// Static library (.a file)
pub type StaticLib = PathBuf;
/// Shared library (.so file)
pub type SharedLib = PathBuf;
/// Group of dependencies
pub type Group = DepBuf;
/// Represents a library
pub struct Library {
    /// dependencies that are transitive (header files, other libraries, compile options, etc)
    /// for example, if this library adds some include path, anything depending on this library will also add that path
    pub deps: DepBuf,
    /// Source files (which are compiled into object files)
    pub sources: SrcBuf,
    /// Source dependencies (dependencies that are specific to the object, and are NOT transitive)
    pub source_deps: DepBuf,
}

pub struct Exe {
    /// Dependencies needed to build the executable
    pub deps: DepBuf,
    /// Sources needed to build the executable
    pub sources: SrcBuf,
}

/// Represents a node in the dependency graph
pub enum Node {
    Include(Include),
    Opts(Opts),
    StaticLib(StaticLib),
    SharedLib(SharedLib),
    Group(Group),
    Library(Library),
    Exe(Exe),
}

/// Dependency graph representing project. When combined with a Compiler object
/// (which stores information on how to pass libraries, etc to the compiler),
/// this can be used to generate a set of compile commands
pub struct Graph {
    pub targets: Vec<Node>,
}
