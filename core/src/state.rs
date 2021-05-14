use crate::{
    command::Command,
    emitter::{Emitter, Subscribable},
    error::Result,
    event::Event,
    sheet::Sheet,
};

#[derive(Debug)]
pub struct State<'a> {
    emitter: Emitter<'a, Event>,
    pub sheet: Sheet,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self {
            emitter: Emitter::new(),
            sheet: Sheet::new(),
        }
    }

    pub fn subscribable(&mut self) -> &mut impl Subscribable<'a> {
        &mut self.emitter
    }

    pub fn apply_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::ChangeCellText { id, text } => {
                self.sheet.set_cell_text(&id, text)?;
                self.emitter.emit(Event::CellTextChanged {
                    id: id.clone(),
                    text: self.sheet.get_cell_text(&id).unwrap().to_string(),
                })
            }
        };
        Ok(())
    }
}

impl Default for State<'_> {
    fn default() -> Self {
        Self::new()
    }
}
