use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use lsp_definition::{CodeSymbolDefinition, DefinitionKind};
use mlang_core::AnyMCoreDefinition;
use mlang_semantic::{AnyMDefinition, SemanticModel};
use rustc_hash::{FxHashMap, FxHasher};
use unicase::UniCase;

/// Case-insensitive key of a name, computed without allocating. Keys can collide,
/// so candidates are rechecked against the name.
fn key(name: &str) -> u64 {
    let mut hasher = FxHasher::default();
    UniCase::new(name).hash(&mut hasher);
    hasher.finish()
}

fn member_key(class: &str, name: &str) -> u64 {
    let mut hasher = FxHasher::default();
    UniCase::new(class).hash(&mut hasher);
    UniCase::new(name).hash(&mut hasher);
    hasher.finish()
}

type Names<E> = FxHashMap<u64, Vec<E>>;

#[derive(Clone, Copy)]
enum Group {
    Function,
    Class,
    /// Keyed by the class name.
    Constructor,
    /// Keyed by the class and method names.
    Method,
}

fn group<T: CodeSymbolDefinition>(d: &T) -> Option<(Group, u64)> {
    let class = || d.container().map(|c| c.id().to_string());
    match d.kind() {
        DefinitionKind::Function => Some((Group::Function, key(d.id()))),
        DefinitionKind::Class => Some((Group::Class, key(d.id()))),
        DefinitionKind::Constructor => Some((Group::Constructor, key(&class()?))),
        DefinitionKind::Method => Some((Group::Method, member_key(&class()?, d.id()))),
        _ => None,
    }
}

/// Definitions the lint resolves calls against, by kind and name.
pub(crate) struct Definitions<E> {
    functions: Names<E>,
    classes: Names<E>,
    constructors: Names<E>,
    methods: Names<E>,
}

impl<E> Default for Definitions<E> {
    fn default() -> Self {
        Self {
            functions: Default::default(),
            classes: Default::default(),
            constructors: Default::default(),
            methods: Default::default(),
        }
    }
}

impl<E: Copy + PartialEq> Definitions<E> {
    fn names(&mut self, group: Group) -> &mut Names<E> {
        match group {
            Group::Function => &mut self.functions,
            Group::Class => &mut self.classes,
            Group::Constructor => &mut self.constructors,
            Group::Method => &mut self.methods,
        }
    }

    pub(crate) fn insert(&mut self, d: &AnyMDefinition, entry: E) {
        if let Some((group, key)) = group(d) {
            self.names(group).entry(key).or_default().push(entry);
        }
    }

    fn remove(&mut self, d: &AnyMDefinition, entry: E) {
        let Some((group, key)) = group(d) else { return };
        let names = self.names(group);
        if let Some(entries) = names.get_mut(&key) {
            entries.retain(|e| *e != entry);
            if entries.is_empty() {
                names.remove(&key);
            }
        }
    }

    pub(crate) fn functions(&self, name: &str) -> &[E] {
        get(&self.functions, key(name))
    }

    pub(crate) fn classes(&self, name: &str) -> &[E] {
        get(&self.classes, key(name))
    }

    pub(crate) fn constructors(&self, class: &str) -> &[E] {
        get(&self.constructors, key(class))
    }

    pub(crate) fn methods(&self, class: &str, name: &str) -> &[E] {
        get(&self.methods, member_key(class, name))
    }
}

fn get<E>(names: &Names<E>, key: u64) -> &[E] {
    names.get(&key).map_or(&[], Vec::as_slice)
}

/// Functions of the built-in API; built once, it never changes.
pub struct CoreIndex {
    core: Arc<[AnyMCoreDefinition]>,
    functions: Names<u32>,
}

impl CoreIndex {
    pub fn new(core: Arc<[AnyMCoreDefinition]>) -> Self {
        let mut functions = Names::<u32>::default();
        for (i, d) in core.iter().enumerate() {
            if d.kind() == DefinitionKind::Function {
                functions.entry(key(d.id())).or_default().push(i as u32);
            }
        }
        Self { core, functions }
    }

    pub(crate) fn functions<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a AnyMCoreDefinition> {
        get(&self.functions, key(name))
            .iter()
            .map(|&i| &self.core[i as usize])
            .filter(move |d| unicase::eq(d.id(), name))
    }
}

/// A definition of an indexed file.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Entry {
    file: u32,
    definition: u32,
}

/// Definitions of the project's files, updated one file at a time.
pub struct ProjectIndex<F> {
    files: FxHashMap<F, u32>,
    models: Vec<Option<Arc<SemanticModel>>>,
    free: Vec<u32>,
    definitions: Definitions<Entry>,
}

impl<F> Default for ProjectIndex<F> {
    fn default() -> Self {
        Self {
            files: Default::default(),
            models: Default::default(),
            free: Default::default(),
            definitions: Default::default(),
        }
    }
}

impl<F: Hash + Eq> ProjectIndex<F> {
    /// Replaces the definitions of `file` with the ones of `model`.
    pub fn insert(&mut self, file: F, model: Arc<SemanticModel>) {
        let slot = match self.files.get(&file) {
            Some(&slot) => {
                self.unindex(slot);
                slot
            }
            None => {
                let slot = self.free.pop().unwrap_or_else(|| {
                    self.models.push(None);
                    (self.models.len() - 1) as u32
                });
                self.files.insert(file, slot);
                slot
            }
        };

        for (i, d) in model.definitions().enumerate() {
            let entry = Entry {
                file: slot,
                definition: i as u32,
            };
            self.definitions.insert(d, entry);
        }
        self.models[slot as usize] = Some(model);
    }

    pub fn remove<Q: Hash + Eq + ?Sized>(&mut self, file: &Q)
    where
        F: Borrow<Q>,
    {
        if let Some(slot) = self.files.remove(file) {
            self.unindex(slot);
            self.free.push(slot);
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    fn unindex(&mut self, slot: u32) {
        let Some(model) = self.models[slot as usize].take() else {
            return;
        };
        for (i, d) in model.definitions().enumerate() {
            let entry = Entry {
                file: slot,
                definition: i as u32,
            };
            self.definitions.remove(d, entry);
        }
    }

    /// The index without `file`, whose definitions are taken from its open document instead.
    pub(crate) fn without<Q: Hash + Eq + ?Sized>(&self, file: &Q) -> Project<'_>
    where
        F: Borrow<Q>,
    {
        Project {
            models: &self.models,
            definitions: &self.definitions,
            skip: self.files.get(file).copied(),
        }
    }
}

/// Borrowed view of a [ProjectIndex], without one of its files.
pub(crate) struct Project<'a> {
    models: &'a [Option<Arc<SemanticModel>>],
    pub(crate) definitions: &'a Definitions<Entry>,
    skip: Option<u32>,
}

impl<'a> Project<'a> {
    pub(crate) fn resolve(&self, entries: &'a [Entry]) -> impl Iterator<Item = &'a AnyMDefinition> {
        let models = self.models;
        let skip = self.skip;
        entries
            .iter()
            .filter(move |e| Some(e.file) != skip)
            .filter_map(move |e| {
                let model = models[e.file as usize].as_deref()?;
                model.definitions().as_slice().get(e.definition as usize)
            })
    }
}
