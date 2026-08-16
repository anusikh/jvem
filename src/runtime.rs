//! The runtimes jvem manages: java, node and maven.
//!
//! Centralising the per-runtime names here replaces the stringly-typed
//! `"java"` / `"node"` / `"maven"` constants that were previously passed
//! around the codebase.

/// A tool managed by jvem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    Java,
    Node,
    Maven,
}

impl Runtime {
    /// Directory under `~/.jvem` that holds installed versions,
    /// e.g. `java_versions` for java.
    pub fn versions_dir(self) -> &'static str {
        match self {
            Runtime::Java => "java_versions",
            Runtime::Node => "node_versions",
            Runtime::Maven => "maven",
        }
    }

    /// Name of the symlink/junction pointing at the active version,
    /// e.g. `~/.jvem/java` for java.
    pub fn symlink_name(self) -> &'static str {
        match self {
            Runtime::Java => "java",
            Runtime::Node => "node",
            Runtime::Maven => "maven",
        }
    }

    /// Short noun used in user-facing messages, e.g. "jdk".
    pub fn label(self) -> &'static str {
        match self {
            Runtime::Java => "jdk",
            Runtime::Node => "node",
            Runtime::Maven => "maven",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Runtime;

    #[test]
    fn runtime_names_are_distinct() {
        let runtimes = [Runtime::Java, Runtime::Node, Runtime::Maven];
        let mut names: Vec<&str> = runtimes.iter().map(|r| r.symlink_name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), runtimes.len());
    }

    #[test]
    fn labels_are_small_lowercase_words() {
        for runtime in [Runtime::Java, Runtime::Node, Runtime::Maven] {
            let label = runtime.label();
            assert!(label.chars().all(|c| c.is_ascii_lowercase()));
            assert!(label.len() >= 3);
        }
    }
}
