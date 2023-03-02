use std::io;

pub struct Question<R, W> {
    reader: R,
    writer: W,
}

impl<'a> Default for Question<io::StdinLock<'a>, io::StdoutLock<'a>> {
    fn default() -> Self {
        Question::new(io::stdin().lock(), io::stdout().lock())
    }
}

impl<R: io::BufRead, W: io::Write> Question<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    pub fn ask<S: AsRef<str>>(&mut self, question: S) -> io::Result<Option<String>> {
        let question = ensure_ends_with_whitespace(question);

        self.writer.write_all(question.as_bytes())?;
        self.writer.flush()?;

        let mut buffer = String::new();
        let n = self.reader.read_line(&mut buffer)?;

        Ok(match n {
            0 => None,
            _ => Some(buffer.trim().to_owned()),
        })
    }
}

fn ensure_ends_with_whitespace<S: AsRef<str>>(source: S) -> String {
    let source = source.as_ref().to_owned();

    if source.ends_with(' ') {
        source
    } else {
        source + " "
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOW_ARE_YOU: &str = "How are you?";

    #[test]
    fn ask_should_display_the_question() {
        let mut output = io::BufWriter::new(Vec::new());

        Question::new(io::empty(), &mut output)
            .ask(HOW_ARE_YOU)
            .expect("ask() should not fail");

        let question =
            String::from_utf8(output.into_inner().expect("into_inner() should not fail"))
                .expect("from_utf8() should not fail");

        assert_eq!(question, HOW_ARE_YOU.to_owned() + " ");
    }

    #[test]
    fn ask_should_add_a_whitespace_after_the_question() {
        let mut output = io::BufWriter::new(Vec::new());

        Question::new(io::empty(), &mut output)
            .ask(HOW_ARE_YOU)
            .expect("ask() should not fail");

        let question =
            String::from_utf8(output.into_inner().expect("into_inner() should not fail"))
                .expect("from_utf8() should not fail");

        assert_eq!(question, HOW_ARE_YOU.to_owned() + " ");
    }

    #[test]
    fn ask_should_not_add_a_whitespace_after_the_question_if_already_present() {
        let mut output = io::BufWriter::new(Vec::new());

        Question::new(io::empty(), &mut output)
            .ask(HOW_ARE_YOU)
            .expect("ask() should not fail");

        let question =
            String::from_utf8(output.into_inner().expect("into_inner() should not fail"))
                .expect("from_utf8() should not fail");

        assert_eq!(question, HOW_ARE_YOU.to_owned() + " ");
    }

    #[test]
    fn ask_should_ensure_a_space_separator_between_the_question_and_the_answer() {
        let mut output = io::BufWriter::new(Vec::new());

        Question::new(io::empty(), &mut output)
            .ask(HOW_ARE_YOU)
            .expect("ask() should not fail");

        let question =
            String::from_utf8(output.into_inner().expect("into_inner() should not fail"))
                .expect("from_utf8() should not fail");

        assert_eq!(question, HOW_ARE_YOU.to_owned() + " ");
    }

    #[test]
    fn ask_should_return_none_for_eof() {
        let result = Question::new(io::empty(), io::sink())
            .ask(HOW_ARE_YOU)
            .expect("ask() should not fail");

        assert!(result.is_none());
    }

    #[test]
    fn ask_should_return_some_answer() {
        let answer = String::from("I'm fine, thank you!");
        let result = Question::new(answer.as_bytes(), io::sink())
            .ask(HOW_ARE_YOU)
            .expect("ask() should not fail");

        assert_eq!(result, Some(answer));
    }
}
