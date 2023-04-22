use automerge as am;

mod sequence_tree;
use sequence_tree::SequenceTree;

use crate::{
    obj_id::ObjId,
    path::{PathElement, Prop},
    value::Value,
    scalar_value::ScalarValue,
};
mod observer;
pub(crate) use observer::Observer;
use observer::Patch as ObserverPatch;

pub struct Patch {
    pub path: Vec<PathElement>,
    pub action: PatchAction,
}

pub enum PatchAction {
    Put {
        obj: ObjId,
        prop: Prop,
        value: Value,
    },
    Insert {
        obj: ObjId,
        index: u64,
        values: Vec<Value>,
    },
    SpliceText {
        obj: ObjId,
        index: u64,
        value: String,
    },
    Increment {
        obj: ObjId,
        prop: Prop,
        value: i64,
    },
    DeleteMap {
        obj: ObjId,
        key: String,
    },
    DeleteSeq {
        obj: ObjId,
        index: u64,
        length: u64,
    },
    Mark {
        obj: ObjId,
        name: String,
        start: u64,
        end: u64,
        value: ScalarValue,
    },
    Unmark {
        obj: ObjId,
        name: String,
        start: u64,
        end: u64,
    },
}

impl From<ObserverPatch> for Patch {
    fn from(value: ObserverPatch) -> Self {
        match value {
            ObserverPatch::PutMap {
                obj,
                path,
                key,
                value,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::Put {
                    obj: obj.into(),
                    prop: Prop::Key { value: key },
                    value: value.into(),
                },
            },
            ObserverPatch::PutSeq {
                obj,
                path,
                index,
                value,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::Put {
                    obj: obj.into(),
                    prop: Prop::Index {
                        value: index as u64,
                    },
                    value: value.into(),
                },
            },
            ObserverPatch::Insert {
                obj,
                path,
                index,
                values,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::Insert {
                    obj: obj.into(),
                    index: index as u64,
                    values: values
                        .into_iter()
                        .map(|(v, id)| Value::from((v.clone(), id.clone())))
                        .collect(),
                },
            },
            ObserverPatch::SpliceText {
                obj,
                path,
                index,
                value,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::SpliceText {
                    obj: obj.into(),
                    index: index as u64,
                    value: value.into_iter().collect(),
                },
            },
            ObserverPatch::Increment {
                obj,
                path,
                prop,
                value,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::Increment {
                    obj: obj.into(),
                    prop: prop.into(),
                    value,
                },
            },
            ObserverPatch::DeleteMap { obj, path, key } => Patch {
                path: convert_path(path),
                action: PatchAction::DeleteMap {
                    obj: obj.into(),
                    key,
                },
            },
            ObserverPatch::DeleteSeq {
                obj,
                path,
                index,
                length,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::DeleteSeq {
                    obj: obj.into(),
                    index: index as u64,
                    length: length as u64,
                },
            },
            ObserverPatch::Mark { obj, path, mark } => Patch {
                path: convert_path(path),
                action: PatchAction::Mark {
                    obj: obj.into(),
                    name: mark.name().to_owned(),
                    start: mark.start.try_into().unwrap(),
                    end: mark.end.try_into().unwrap(),
                    value: mark.value().into(),
                },
            },
            ObserverPatch::Unmark {
                obj,
                path,
                name,
                range,
            } => Patch {
                path: convert_path(path),
                action: PatchAction::Unmark {
                    obj: obj.into(),
                    name,
                    start: range.start.try_into().unwrap(),
                    end: range.end.try_into().unwrap(),
                },
            },
        }
    }
}

fn convert_path(p: Vec<(am::ObjId, am::Prop)>) -> Vec<PathElement> {
    p.into_iter()
        .map(|(obj, prop)| PathElement {
            obj: obj.into(),
            prop: prop.into(),
        })
        .collect()
}
