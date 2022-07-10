pub struct Config<'a> {
    pub path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("wrong number of arguments");
        }

        Ok(Config { path: &args[1] })
    }
}
