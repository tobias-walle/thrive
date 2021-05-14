use crate::{command::Command, error::Result, event::Event, notifier::Notifier, sheet::Sheet};

pub struct State<'a> {
    pub notifier: Notifier<'a, Event>,
    pub sheet: Sheet,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self {
            notifier: Notifier::new(),
            sheet: Sheet::new(),
        }
    }

    pub fn apply_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::ChangeCellText { id, text } => {
                self.sheet.set_cell_text(&id, text.clone())?;
                self.notifier.notify(Event::CellTextChanged {
                    id: id.clone(),
                    text: self.sheet.get_cell_text(&id).unwrap().to_string(),
                })
            }
        };
        Ok(())
    }
}
