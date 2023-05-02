// c:v
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NumericValue {
    text: String,
}
impl NumericValue {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut NumericValue {
        self.text = value.into();
        self
    }

    pub(crate) fn _set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    self.set_text(e.unescape().unwrap());
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:v" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:v"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn _write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, &self.text);
        write_end_tag(writer, "c:v");
    }
}
