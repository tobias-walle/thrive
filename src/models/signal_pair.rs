use leptos::{ReadSignal, WriteSignal};

pub type SignalPair<T> = (ReadSignal<T>, WriteSignal<T>);
