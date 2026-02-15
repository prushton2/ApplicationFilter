#[derive(Debug, Clone)]
pub struct Arguments {
    pub filters: Vec<Argument>,
    pub output: Option<String>,
    pub stdin: Option<String>,
}


#[derive(Debug, Clone)]
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


impl Arguments {
    pub fn parse(arg_string: Vec<String>) -> Result<Self, ParserError> {

        let mut start_index = 0;
        let mut arguments: Vec<Argument> = vec![];
        
        loop {
            let slice = &arg_string[start_index..];
            
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

        let mut arg_struct: Self = Self {
            filters: vec![],
            output: None,
            stdin: None
        };

        for i in arguments {
            match i {
                Argument::Category(t) => arg_struct.filters.push(Argument::Category(t)),
                Argument::Type(t) => arg_struct.filters.push(Argument::Type(t)),
                Argument::Keywords(t) => arg_struct.filters.push(Argument::Keywords(t)),
                Argument::Exclude(t) => arg_struct.filters.push(Argument::Exclude(t)),

                Argument::Output(t) => arg_struct.output = Some(t),
                Argument::Stdin(t) => arg_struct.stdin = Some(t)
            }
        }

        return Ok(arg_struct);
    }

    fn enumize(arguments: &[String]) -> Result<(usize, Argument), ParserError> {
        return match arguments[0].as_str() {
            "--category" => {
                Ok((2, Argument::Category(arguments[1].clone().to_lowercase())))
            }
            "--type" => {
                Ok((2, Argument::Type(arguments[1].clone().to_lowercase())))
            }
            "--keywords" => {
                Ok((2, Argument::Keywords(arguments[1].clone().to_lowercase())))
            }
            "--exclude" => {
                let sub_arg = match Self::enumize(&arguments[1..]) {
                    Ok(t) => t,
                    Err(t) => return Err(t)
                };

                Ok((1 + sub_arg.0, Argument::Exclude(Box::new(sub_arg.1))))
            }
            "--output" => {
                Ok((2, Argument::Output(arguments[1].clone().to_lowercase())))
            }
            "--stdin" => {
                Ok((2, Argument::Stdin(arguments[1].clone().to_lowercase())))
            }
            _ => {
                Err(ParserError::InvalidArgument(arguments[0].clone()))
            }
        }
    }
}