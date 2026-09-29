use itertools::Itertools;
use tower_lsp::lsp_types::{
    Documentation, Location, MarkedString, ParameterInformation, ParameterLabel,
    SignatureInformation, Url,
};

use crate::members::class_members;
use crate::{
    CodeSymbolDefinition, DefinitionKind, LocationDefinition, MarkupDefinition, SemanticInfo,
    SignatureParameters, Symbol, Usage,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Only definitions that accept the call's argument count.
    Goto,
    /// All candidates, best arity match first; a class is shown with its constructors.
    Hover,
}

pub fn get_declaration<'a, I, D>(info: &SemanticInfo, definitions: I) -> Vec<Location>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + LocationDefinition + 'a,
{
    resolve(info, definitions, Mode::Goto)
        .into_iter()
        .map(|(uri, d)| d.id_location(uri))
        .collect()
}

pub fn get_hover<'a, I, D>(info: &SemanticInfo, definitions: I) -> Vec<MarkedString>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + MarkupDefinition + 'a,
{
    resolve(info, definitions, Mode::Hover)
        .into_iter()
        .map(|(_, d)| MarkedString::String(d.full_markdown()))
        .collect()
}

pub fn get_signatures<'a, I, D>(
    info: &SemanticInfo,
    definitions: I,
    current_argument: u32,
) -> Vec<SignatureInformation>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + MarkupDefinition + SignatureParameters + 'a,
{
    let callables = match (&info.symbol, info.usage) {
        (Symbol::Function(_), Usage::Call(_))
        | (Symbol::Member { class: Some(_), .. }, Usage::Call(_))
        | (Symbol::Class(_), Usage::New(_) | Usage::Super(_)) => {
            resolve(info, definitions, Mode::Hover)
        }
        _ => vec![],
    };

    callables
        .into_iter()
        .filter(|(_, d)| d.kind() != DefinitionKind::Class)
        .map(|(_, d)| signature(d, current_argument))
        .collect()
}

fn signature<D: SignatureParameters>(d: &D, current_argument: u32) -> SignatureInformation {
    let offsets = d.offsets();
    let mut parameters = offsets
        .iter()
        .map(|offset| ParameterInformation {
            label: ParameterLabel::LabelOffsets(*offset),
            documentation: None,
        })
        .collect::<Vec<_>>();

    if d.has_rest() {
        if let Some(rest) = parameters.last_mut() {
            rest.documentation = Some(Documentation::String(String::from("... arg 1")));
        }

        if let Some(offset) = offsets.last() {
            for i in 2..100 {
                parameters.push(ParameterInformation {
                    label: ParameterLabel::LabelOffsets(*offset),
                    documentation: Some(Documentation::String(format!("... arg {i}"))),
                });
            }
        }
    }

    SignatureInformation {
        label: d.text(),
        parameters: Some(parameters),
        documentation: None,
        active_parameter: Some(current_argument),
    }
}

fn resolve<'a, I, D>(info: &SemanticInfo, definitions: I, mode: Mode) -> Vec<(Url, &'a D)>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let SemanticInfo { symbol, usage } = info;
    let arguments = usage.arguments();

    let Some(name) = symbol.name() else {
        return vec![];
    };

    if let Symbol::Class(_) = symbol
        && matches!(usage, Usage::Declaration | Usage::New(_) | Usage::Super(_))
    {
        return class_with_constructors(name, arguments, definitions, mode);
    }

    let pool = match symbol {
        Symbol::Member {
            class: Some(class), ..
        } => class_members(definitions, class),
        _ => definitions.into_iter().collect(),
    };

    let candidates = pool
        .into_iter()
        .filter(|(_, d)| accepts(symbol, *usage, d.kind()) && d.compare_id_with(name))
        .collect();

    by_arity(candidates, arguments, mode)
}

fn accepts(symbol: &Symbol, usage: Usage, kind: DefinitionKind) -> bool {
    match symbol {
        Symbol::Function(_) => kind == DefinitionKind::Function,
        Symbol::Class(_) => kind == DefinitionKind::Class,
        Symbol::AnyClass => false,
        Symbol::Member { .. } if usage == Usage::Access => kind.is_accessor(),
        Symbol::Member { .. } => kind == DefinitionKind::Method,
    }
}

fn by_arity<D>(candidates: Vec<(Url, &D)>, arguments: Option<usize>, mode: Mode) -> Vec<(Url, &D)>
where
    D: CodeSymbolDefinition,
{
    let Some(count) = arguments else {
        return candidates;
    };

    match mode {
        Mode::Goto => candidates
            .into_iter()
            .filter(|(_, d)| d.can_be_called(count))
            .collect(),
        Mode::Hover => candidates
            .into_iter()
            .sorted_by(|(_, a), (_, b)| a.call_priority(b, count))
            .collect(),
    }
}

/// Goto: matching constructors, or the class itself when there are none.
/// Hover: the class followed by its constructors.
fn class_with_constructors<'a, I, D>(
    class_name: &str,
    arguments: Option<usize>,
    definitions: I,
    mode: Mode,
) -> Vec<(Url, &'a D)>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let definitions = Vec::from_iter(definitions);
    let mut result = vec![];

    let classes = definitions
        .iter()
        .filter(|(_, d)| d.kind() == DefinitionKind::Class && d.compare_id_with(class_name));

    for (uri, class) in classes {
        let constructors = definitions
            .iter()
            .filter(|(_, d)| {
                d.kind() == DefinitionKind::Constructor && d.container().as_ref() == Some(*class)
            })
            .cloned()
            .collect();
        let constructors = by_arity(constructors, arguments, mode);

        if mode == Mode::Hover || constructors.is_empty() {
            result.push((uri.clone(), *class));
        }
        result.extend(constructors);
    }

    result
}
