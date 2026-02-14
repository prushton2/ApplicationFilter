#[derive(Debug)]
pub enum Argument {
    Category(String),
    Type(String),
    Keywords(String),
    Exclude(Box<Argument>),
    Output(String),
    Stdin(String),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidArgument(String)
}


impl Argument {
    pub fn parse(arg_string: Vec<String>) -> Result<Vec<Argument>, ParserError> {

        let mut start_index = 0;
        let mut arguments: Vec<Argument> = vec![];
        
        loop {
            let mut slice = &arg_string[start_index..];
            
            let (offset, argument) = match Self::enumize(slice) {
                Ok(t) => t,
                Err(t) => return Err(t)
            };

            start_index += offset;
            arguments.push(argument);

            if start_index == arg_string.len() {
                break
            }
        }

        return Ok(arguments);
    }

    fn enumize(arguments: &[String]) -> Result<(usize, Self), ParserError> {
        return match arguments[0].as_str() {
            "--category" => {
                Ok((2, Argument::Category(arguments[1].clone())))
            }
            "--type" => {
                Ok((2, Argument::Type(arguments[1].clone())))
            }
            "--keywords" => {
                Ok((2, Argument::Keywords(arguments[1].clone())))
            }
            "--exclude" => {
                let sub_arg = match Self::enumize(&arguments[1..]) {
                    Ok(t) => t,
                    Err(t) => return Err(t)
                };

                Ok((1 + sub_arg.0, Argument::Exclude(Box::new(sub_arg.1))))
            }
            "--output" => {
                Ok((2, Argument::Output(arguments[1].clone())))
            }
            "--stdin" => {
                Ok((2, Argument::Stdin(arguments[1].clone())))
            }
            _ => {
                Err(ParserError::InvalidArgument(arguments[0].clone()))
            }
        }
    }
}